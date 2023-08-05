use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    components::{
        BelongsToGround, Cleanup, GameOverSensor, Ground, GroundMesh, GroundMidSensor,
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
            transform.translation.z += GROUND_LENGTH - 0.2;
            commands
                .entity(ground_ent)
                .insert(TransformBundle::from_transform(transform));
            println!("Spawning new ground at: {:?}", transform.translation);
            // set it as next
            ground_res.next_ground = Some(ground_ent);
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn color_grounds(
    grounds: Query<&Cleanup, With<Ground>>,
    ground_materials: Query<(&BelongsToGround, &Handle<StandardMaterial>), With<GroundMesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (BelongsToGround(ground_ent), mat_hdl) in ground_materials.iter() {
        // Get the grounds cleanup timer (i.e. also used as overheat timer)
        let Ok(Cleanup::OnTimeout { timer }) = grounds.get(*ground_ent) else { continue; };
        let remaining = timer.remaining_secs();
        let Some(mat) = materials.get_mut(mat_hdl) else { continue; };
        let mut new_color = mat.base_color.as_hsla_f32();
        new_color[0] = Color::GREEN.as_hsla_f32()[0] * remaining / GROUND_OVERHEAT_DURATION_SECS;
        mat.base_color = Color::hsla(new_color[0], new_color[1], new_color[2], new_color[3]);
    }
}

// pub fn mark_cleanup_prev_grounds(mut commands: Commands, ground_res: Res<GroundsResource>) {
//     if !ground_res.is_changed() {
//         return;
//     }
//     let GroundsResource {
//             previous_ground: Some(previous_ground),
//             ..
//     } = *ground_res else { return; };
//     let Some(mut ent_commands) = commands.get_entity(previous_ground) else { return; };
//     ent_commands.insert(Cleanup::OnTimeout {
//         timer: Timer::from_seconds(15.0, TimerMode::Once),
//     });
// }
