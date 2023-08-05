use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;

use crate::{
    components::{BelongsToGround, Ground, GroundMesh, GroundMidSensor, GroundSurfaceSensor},
    constants::{GROUND_LENGTH, GROUND_THICKNESS, GROUND_WIDTH},
    resources::GroundsResource,
};

#[allow(clippy::too_many_arguments)]
pub fn pick_ground_point_raycast(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    query_grounds: Query<&GlobalTransform, With<Ground>>,
    query_ground_meshes: Query<&BelongsToGround, (With<GroundMesh>, With<Collider>)>,
    ground_res: Res<GroundsResource>,
    rapier_context: Res<RapierContext>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut gizmos: Gizmos,
    mouse_btn_input: Res<Input<MouseButton>>,
) {
    let window = windows.single();

    let Some(cursor_position) = window.cursor_position() else { return; };

    let filter = QueryFilter::exclude_dynamic().exclude_sensors();

    // We will color in read the colliders hovered by the mouse.
    for (camera, camera_transform) in &cameras {
        // First, compute a ray from the mouse position.
        let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else { return; };

        // Then cast the ray.
        let hit = rapier_context.cast_ray_and_get_normal(
            ray.origin,
            ray.direction,
            f32::MAX,
            true,
            filter,
        );

        if let Some((entity, intersection)) = hit {
            let Ok(BelongsToGround(ground_ent)) =  query_ground_meshes.get(entity) else { continue; };
            if ground_res.current_ground == Some(*ground_ent) {
                let Ok(ground_transform) = query_grounds.get(*ground_ent) else { continue; };
                let RayIntersection { point, normal, .. } = intersection;
                let point_local = ground_transform.affine().inverse().transform_point(point);
                let normal_local = ground_transform.affine().inverse().transform_point(normal);
                gizmos.ray(point, normal, Color::CYAN);
                gizmos.circle(point, normal, 0.1, Color::CYAN);

                if mouse_btn_input.just_pressed(MouseButton::Left) {
                    let ground_mesh: Mesh = shape::Box::new(
                        GROUND_LENGTH / 4.,
                        GROUND_THICKNESS * 4.,
                        GROUND_LENGTH * 0.1,
                    )
                    .into();
                    let Some(ground_collider) = Collider::from_bevy_mesh(
                        &ground_mesh, &ComputedColliderShape::TriMesh) else { return; };
                    let wall_ent = commands
                        .spawn((
                            PbrBundle {
                                mesh: meshes.add(ground_mesh.clone()),
                                material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                                transform: Transform::from_translation(point_local),
                                ..default()
                            },
                            ground_collider.clone(),
                            RigidBody::Fixed,
                            BelongsToGround(*ground_ent),
                        ))
                        .id();
                    commands.entity(*ground_ent).push_children(&[wall_ent]);
                }
            }
        }
    }
}
