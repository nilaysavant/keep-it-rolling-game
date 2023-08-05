use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    window::PrimaryWindow,
};
use bevy_rapier3d::prelude::*;

use crate::{
    components::{
        BelongsToGround, Ground, GroundMesh, GroundMidSensor, GroundSurfaceSensor, TempWall, Wall,
    },
    constants::{GROUND_LENGTH, GROUND_THICKNESS, GROUND_WIDTH},
    events::WallEvent,
    resources::GroundsResource,
};

#[allow(clippy::too_many_arguments)]
pub fn pick_ground_point_raycast(
    windows: Query<&Window, With<PrimaryWindow>>,
    query_grounds: Query<&GlobalTransform, With<Ground>>,
    query_ground_meshes: Query<&BelongsToGround, (With<GroundMesh>, With<Collider>)>,
    mut temp_walls: Query<With<TempWall>>,
    ground_res: Res<GroundsResource>,
    rapier_context: Res<RapierContext>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut gizmos: Gizmos,
    mouse_btn_input: Res<Input<MouseButton>>,
    key_input: Res<Input<KeyCode>>,
    mut scroll_evr: EventReader<MouseWheel>,
    mut wall_event: EventWriter<WallEvent>,
    mut wall_angle: Local<f32>,
) {
    let window = windows.single();

    let Some(cursor_position) = window.cursor_position() else { return; };

    let predicate = |entity| !temp_walls.contains(entity);
    let filter = QueryFilter::exclude_dynamic()
        .exclude_sensors()
        .predicate(&predicate);

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
            if ground_res.current_ground == Some(*ground_ent)
                || ground_res.next_ground == Some(*ground_ent)
            {
                let Ok(ground_transform) = query_grounds.get(*ground_ent) else { continue; };
                let RayIntersection { point, normal, .. } = intersection;
                let point_local = ground_transform.affine().inverse().transform_point(point);
                let normal_local = ground_transform.affine().inverse().transform_point(normal);
                gizmos.ray(point, normal, Color::CYAN);
                gizmos.circle(point, normal, 0.1, Color::CYAN);

                for ev in scroll_evr.iter() {
                    match ev.unit {
                        MouseScrollUnit::Line => {
                            *wall_angle = ev.y * 0.5;
                        }
                        MouseScrollUnit::Pixel => {
                            *wall_angle = ev.y * 0.1;
                        }
                    }
                }
                if key_input.pressed(KeyCode::A) {
                    *wall_angle += 0.05;
                } else if key_input.pressed(KeyCode::D) {
                    *wall_angle -= 0.05;
                }
                let mut transform =
                    Transform::from_translation(point_local + Vec3::Y * GROUND_THICKNESS * 1.5);
                transform.rotation = Quat::from_axis_angle(Vec3::Y, *wall_angle);
                if mouse_btn_input.just_pressed(MouseButton::Left) {
                    wall_event.send(WallEvent::Draw);
                } else {
                    wall_event.send(WallEvent::HoverUpdate {
                        ground: *ground_ent,
                        transform,
                    });
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn handle_wall_events(
    mut commands: Commands,
    mut temp_walls: Query<(Entity, &mut Transform, &mut Visibility, &Parent), With<TempWall>>,
    mut wall_events: EventReader<WallEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in wall_events.iter() {
        match event {
            WallEvent::HoverUpdate { ground, transform } => {
                if temp_walls.is_empty() {
                    let Some(wall_ent) = draw_wall(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        *transform,
                        ground,
                    ) else { continue; };
                    commands.entity(wall_ent).insert(TempWall);
                } else {
                    let Ok((entity, mut temp_wall_transform, mut visibility, parent)) = temp_walls.get_single_mut() else { continue; };
                    *temp_wall_transform = *transform;
                    *visibility = Visibility::Visible;
                    if parent.get() != *ground {
                        commands.entity(entity).remove_parent();
                        commands.entity(*ground).push_children(&[entity]);
                    }
                }
            }
            WallEvent::HoverStop => {
                let Ok((entity, mut temp_wall_transform, mut visibility, parent)) = temp_walls.get_single_mut() else { continue; };
                *visibility = Visibility::Hidden;
            }
            WallEvent::Draw => {
                let Ok((entity, mut temp_wall_transform, mut visibility, parent)) = temp_walls.get_single_mut() else { continue; };
                commands.entity(entity).insert(Wall).remove::<TempWall>();
            }
        }
    }
}

fn draw_wall(
    commands: &mut Commands<'_, '_>,
    meshes: &mut ResMut<'_, Assets<Mesh>>,
    materials: &mut ResMut<'_, Assets<StandardMaterial>>,
    transform: Transform,
    ground_ent: &Entity,
) -> Option<Entity> {
    let wall_x = GROUND_LENGTH / 3.5;
    let wall_y = GROUND_THICKNESS * 3.;
    let wall_z = GROUND_LENGTH * 0.01;
    let wall: Mesh = shape::Box::new(wall_x, wall_y, wall_z).into();
    let Some(ground_collider) = Collider::from_bevy_mesh(
        &wall, &ComputedColliderShape::TriMesh) else { return None; };
    let wall_ent = commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(wall.clone()),
                material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                transform,
                ..default()
            },
            ground_collider.clone(),
            RigidBody::Fixed,
            BelongsToGround(*ground_ent),
        ))
        .id();
    commands.entity(*ground_ent).push_children(&[wall_ent]);
    Some(wall_ent)
}
