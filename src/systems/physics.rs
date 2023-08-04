use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::components::{
    ActiveGround, BelongsToGround, Ground, GroundSensor, PrevActiveGround, RollingBall, GroundMesh,
};

use super::scene::spawn_ground;

#[allow(clippy::type_complexity)]
pub fn ball_touching_ground(
    mut commands: Commands,
    balls: Query<Entity, (With<RollingBall>, With<Collider>)>,
    ground_sensors: Query<(Entity, &BelongsToGround), (With<GroundSensor>, With<Collider>)>,
    active_grounds: Query<Entity, With<ActiveGround>>,
    prev_active_grounds: Query<Entity, (With<PrevActiveGround>, Without<ActiveGround>)>,
    rapier_context: Res<RapierContext>,
) {
    let Ok(ball_ent) = balls.get_single() else { return; };

    for (sensor_ent, BelongsToGround(ground_ent)) in ground_sensors.iter() {
        let Some(is_intersecting) = rapier_context.intersection_pair(ball_ent, sensor_ent ) else { continue; };
        // if intersecting set ground as active else inactive...
        if is_intersecting {
            commands.entity(*ground_ent).insert(ActiveGround);
        } else {
            if active_grounds.contains(*ground_ent) {
                // clear any prev active
                for prev_active_ground in prev_active_grounds.iter() {
                    commands
                        .entity(prev_active_ground)
                        .remove::<PrevActiveGround>();
                }
                // insert new prev active
                commands.entity(*ground_ent).insert(PrevActiveGround);
            }
            commands.entity(*ground_ent).remove::<ActiveGround>();
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn ball_not_touching_any_ground(
    mut commands: Commands,
    balls: Query<Entity, (With<RollingBall>, With<Collider>)>,
    prev_active_grounds: Query<&Transform, With<PrevActiveGround>>,
    active_grounds: Query<Entity, With<ActiveGround>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok(ball_ent) = balls.get_single() else { return; };
    if active_grounds.is_empty() {
        let Ok(prev_active_ground_transform) = prev_active_grounds.get_single() else { return; };
        let Some(ground_ent) = spawn_ground(&mut commands, &mut meshes, &mut materials) else { return; };
        // rotate by 45 deg...
        let mut transform = *prev_active_ground_transform;
        transform.rotation = Quat::from_axis_angle(Vec3::X, std::f32::consts::FRAC_PI_6);
        transform.translation.y += -2.5;
        transform.translation.z += 3.5;
        commands
            .entity(ground_ent)
            .insert(TransformBundle::from_transform(transform));
    }
}

#[allow(clippy::type_complexity)]
pub fn color_active_grounds(
    prev_active_grounds: Query<Entity, With<PrevActiveGround>>,
    active_grounds: Query<Entity, (With<ActiveGround>, Without<PrevActiveGround>)>,
    ground_materials: Query<(&BelongsToGround, &Handle<StandardMaterial>), With<GroundMesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for ground in prev_active_grounds.iter() {
        for (BelongsToGround(ground_ent), mat_hdl) in ground_materials.iter() {
            if *ground_ent == ground {
                let Some(mat)=materials.get_mut(mat_hdl) else { continue; };
                mat.base_color = Color::YELLOW;
            }
        }
    }
    for ground in active_grounds.iter() {
        for (BelongsToGround(ground_ent), mat_hdl) in ground_materials.iter() {
            if *ground_ent == ground {
                let Some(mat)=materials.get_mut(mat_hdl) else { continue; };
                mat.base_color = Color::RED;
            }
        }
    }
}
