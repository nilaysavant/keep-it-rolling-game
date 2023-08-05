use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    components::{
        BelongsToGround, Cleanup, Ground, GroundGameOverSensor, GroundMesh, GroundMidSensor,
        GroundSurfaceSensor, RollingBall,
    },
    constants::{GROUND_ANGLE, GROUND_LENGTH, GROUND_OVERHEAT_DURATION_SECS, GROUND_THICKNESS},
    events::SceneEvent,
    resources::GroundsResource,
};

use super::scene::spawn_ground;

#[allow(clippy::type_complexity)]
pub fn handle_ground_sensor(
    mut commands: Commands,
    balls: Query<Entity, (With<RollingBall>, With<Collider>)>,
    ground_sensors: Query<(Entity, &BelongsToGround), (With<GroundSurfaceSensor>, With<Collider>)>,
    mut ground_res: ResMut<GroundsResource>,
    rapier_context: Res<RapierContext>,
) {
    let Ok(ball_ent) = balls.get_single() else { return; };
    // temp var for current ground init to none.
    let mut active_ground = None;
    for (sensor_ent, BelongsToGround(ground_ent)) in ground_sensors.iter() {
        let Some(is_intersecting) = rapier_context.intersection_pair(ball_ent, sensor_ent ) else { continue; };
        if is_intersecting {
            // set the active current ground if intersecting.
            active_ground = Some(*ground_ent);
        }
    }
    if active_ground.is_some() && ground_res.current_ground != active_ground {
        // if active ground was set and current ground is not the same as the new active ground
        // then rotate the active ground in state res...
        ground_res.previous_ground = ground_res.current_ground;
        ground_res.current_ground = active_ground;
        ground_res.next_ground = None;
        if let Some(active_ground) = active_ground {
            commands.entity(active_ground).insert(Cleanup::OnTimeout {
                timer: Timer::from_seconds(GROUND_OVERHEAT_DURATION_SECS, TimerMode::Once),
            });
        }
    }
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn handle_mid_ground_sensor(
    mut commands: Commands,
    balls: Query<Entity, (With<RollingBall>, With<Collider>)>,
    ground_mid_sensors: Query<(Entity, &BelongsToGround), (With<GroundMidSensor>, With<Collider>)>,
    query_grounds: Query<&Transform, With<Ground>>,
    mut ground_res: ResMut<GroundsResource>,
    rapier_context: Res<RapierContext>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok(ball_ent) = balls.get_single() else { return; };
    for (sensor_ent, BelongsToGround(ground_ent)) in ground_mid_sensors.iter() {
        let Some(is_intersecting) = rapier_context.intersection_pair(ball_ent, sensor_ent ) else { continue; };
        if is_intersecting
            && ground_res.current_ground == Some(*ground_ent)
            && ground_res.next_ground.is_none()
        {
            // spawn new ground relative to prev ground transform...
            let mut transform = Transform::default();
            if let Some(current_ground) = ground_res.current_ground {
                let Ok(current_transform) = query_grounds.get(current_ground) else { continue; };
                transform = *current_transform;
            }
            let Some(ground_ent) = spawn_ground(&mut commands, &mut meshes, &mut materials) else { continue; };
            // rotate by 45 deg...
            transform.rotation = Quat::from_axis_angle(Vec3::X, GROUND_ANGLE);
            transform.translation.y +=
                -(GROUND_LENGTH / GROUND_ANGLE.cos()) * 0.385 * GROUND_THICKNESS;
            transform.translation.z += GROUND_LENGTH - 0.1;
            commands
                .entity(ground_ent)
                .insert(TransformBundle::from_transform(transform));
            println!("Spawning new ground at: {:?}", transform.translation);
            // set it as next
            ground_res.next_ground = Some(ground_ent);
        }
    }
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn move_game_over_sensors_with_current_ground(
    grounds: Query<&Transform, With<Ground>>,
    mut ground_game_over_sensor: Query<
        (&mut Transform, &GroundGameOverSensor),
        (With<Collider>, Without<Ground>),
    >,
    ground_res: Res<GroundsResource>,
) {
    for (mut sensor_transform, GroundGameOverSensor { init_transform }) in
        ground_game_over_sensor.iter_mut()
    {
        let Some(current_ground) = ground_res.current_ground else { continue; };
        let Ok(curr_ground_transform) = grounds.get(current_ground) else { continue; };
        let mut new_transform = *curr_ground_transform;
        new_transform.translation += init_transform.translation;
        *sensor_transform = new_transform;
    }
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn handle_ground_game_over_sensor(
    balls: Query<Entity, (With<RollingBall>, With<Collider>)>,
    ground_game_over_sensor: Query<Entity, (With<GroundGameOverSensor>, With<Collider>)>,
    mut game_event: EventWriter<SceneEvent>,
    rapier_context: Res<RapierContext>,
) {
    let Ok(ball_ent) = balls.get_single() else { return; };
    for sensor_ent in ground_game_over_sensor.iter() {
        let Some(is_intersecting) = rapier_context.intersection_pair(ball_ent, sensor_ent ) else { continue; };
        if is_intersecting {
            game_event.send(SceneEvent::Restart);
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn color_grounds(
    ground_materials: Query<(&BelongsToGround, &Handle<StandardMaterial>), With<GroundMesh>>,
    ground_res: Res<GroundsResource>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let GroundsResource {
        previous_ground,
        current_ground,
        next_ground,
    } = *ground_res;

    if let Some(previous_ground) = previous_ground {
        for (BelongsToGround(ground_ent), mat_hdl) in ground_materials.iter() {
            if *ground_ent == previous_ground {
                let Some(mat)=materials.get_mut(mat_hdl) else { continue; };
                mat.base_color = Color::YELLOW;
            }
        }
    }
    if let Some(current_ground) = current_ground {
        for (BelongsToGround(ground_ent), mat_hdl) in ground_materials.iter() {
            if *ground_ent == current_ground {
                let Some(mat)=materials.get_mut(mat_hdl) else { continue; };
                mat.base_color = Color::RED;
            }
        }
    }
    if let Some(next_ground) = next_ground {
        for (BelongsToGround(ground_ent), mat_hdl) in ground_materials.iter() {
            if *ground_ent == next_ground {
                let Some(mat)=materials.get_mut(mat_hdl) else { continue; };
                mat.base_color = Color::BLUE;
            }
        }
    }
}

pub fn mark_cleanup_prev_grounds(mut commands: Commands, ground_res: Res<GroundsResource>) {
    if !ground_res.is_changed() {
        return;
    }
    let GroundsResource {
            previous_ground: Some(previous_ground),
            ..
    } = *ground_res else { return; };
    let Some(mut ent_commands) = commands.get_entity(previous_ground) else { return; };
    ent_commands.insert(Cleanup::OnTimeout {
        timer: Timer::from_seconds(15.0, TimerMode::Once),
    });
}
