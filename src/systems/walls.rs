use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    window::PrimaryWindow,
};
use bevy_rapier3d::prelude::*;

use crate::{
    components::{
        BelongsToGround, Cleanup, Ground, GroundMesh, GroundMidSensor, GroundSurfaceSensor,
        MyCamera, TempWall, Wall,
    },
    constants::{GROUND_LENGTH, GROUND_THICKNESS, GROUND_WIDTH},
    events::WallEvent,
    resources::{GroundsResource, SettingsResource},
};

#[allow(clippy::too_many_arguments)]
pub fn pick_ground_point_raycast(
    windows: Query<&Window, With<PrimaryWindow>>,
    query_grounds: Query<&GlobalTransform, With<Ground>>,
    query_ground_meshes: Query<&BelongsToGround, (With<GroundMesh>, With<Collider>)>,
    temp_walls: Query<With<TempWall>>,
    ground_res: Res<GroundsResource>,
    settings_res: Res<SettingsResource>,
    rapier_context: Res<RapierContext>,
    cameras: Query<(&Camera, &GlobalTransform), With<MyCamera>>,
    mut gizmos: Gizmos,
    mouse_btn_input: Res<Input<MouseButton>>,
    key_input: Res<Input<KeyCode>>,
    mut wall_event: EventWriter<WallEvent>,
    mut wall_angle: Local<f32>,
) {
    let window = windows.single();

    let Some(cursor_position) = window.cursor_position() else { return; };

    let predicate = |entity| !temp_walls.contains(entity);
    let filter = QueryFilter::exclude_dynamic()
        .exclude_sensors()
        .predicate(&predicate);

    let Ok((camera, camera_transform)) = cameras.get_single() else { return; };

    // First, compute a ray from the mouse position.
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else { return; };

    // Then cast the ray.
    let hit =
        rapier_context.cast_ray_and_get_normal(ray.origin, ray.direction, f32::MAX, true, filter);

    let Some((entity, intersection)) = hit else {
        // if not hit send hover stop
        wall_event.send(WallEvent::HoverStop);
        return;
    };
    // if hit continue to evaluate...
    let Ok(BelongsToGround(ground_ent)) =  query_ground_meshes.get(entity) else { return; };
    if ground_res.current_ground == Some(*ground_ent) || ground_res.next_ground == Some(*ground_ent)
    {
        let Ok(ground_transform) = query_grounds.get(*ground_ent) else { return; };
        let RayIntersection { point, normal, .. } = intersection;
        let point_local = ground_transform.affine().inverse().transform_point(point);
        let normal_local = ground_transform.affine().inverse().transform_point(normal);
        gizmos.ray(point, normal, Color::CYAN);
        gizmos.circle(point, normal, 0.1, Color::CYAN);

        if key_input.pressed(KeyCode::A) {
            *wall_angle += settings_res.wall_rotation_sensitivity;
        } else if key_input.pressed(KeyCode::D) {
            *wall_angle -= settings_res.wall_rotation_sensitivity;
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

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn handle_wall_events(
    mut commands: Commands,
    mut temp_walls: Query<
        (
            Entity,
            &mut Transform,
            &mut Visibility,
            &Parent,
            &Handle<Mesh>,
            &Handle<StandardMaterial>,
        ),
        With<TempWall>,
    >,
    mut wall_events: EventReader<WallEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in wall_events.iter() {
        match event {
            WallEvent::HoverUpdate { ground, transform } => {
                if commands.get_entity(*ground).is_none() {
                    continue;
                }
                if temp_walls.is_empty() {
                    let Some(wall_ent) = draw_wall(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        *transform,
                        ground,
                        false,
                    ) else { continue; };
                    commands.entity(wall_ent).insert(TempWall);
                } else {
                    let Ok((
                        entity,
                        mut temp_wall_transform,
                        mut visibility,
                        parent,
                        mesh_hdl,
                        mat_hdl,
                    )) = temp_walls.get_single_mut() else { continue; };
                    *temp_wall_transform = *transform;
                    *visibility = Visibility::Visible;
                    let Some(wall_mat) = materials.get_mut(mat_hdl) else { continue; };
                    wall_mat.alpha_mode = AlphaMode::Blend;
                    wall_mat.base_color.set_a(0.6);
                    if parent.get() != *ground {
                        commands.entity(entity).remove_parent();
                        commands.entity(*ground).push_children(&[entity]);
                    }
                }
            }
            WallEvent::HoverStop => {
                let Ok((
                        entity,
                        mut temp_wall_transform,
                        mut visibility,
                        parent,
                        mesh_hdl,
                        mat_hdl,
                    )) = temp_walls.get_single_mut() else { continue; };
                *visibility = Visibility::Hidden;
            }
            WallEvent::Draw => {
                let Ok((
                        entity,
                        mut temp_wall_transform,
                        mut visibility,
                        parent,
                        mesh_hdl,
                        mat_hdl,
                    )) = temp_walls.get_single_mut() else { continue; };
                let Some(wall_mesh) = meshes.get(mesh_hdl) else { continue; };
                let Some(collider) = Collider::from_bevy_mesh(
                        wall_mesh, &ComputedColliderShape::TriMesh) else { continue; };
                let Some(wall_mat) = materials.get_mut(mat_hdl) else { continue; };
                wall_mat.alpha_mode = AlphaMode::Opaque;
                wall_mat.base_color.set_a(1.);
                commands
                    .entity(entity)
                    .insert(collider.clone())
                    .insert(Wall)
                    .remove::<TempWall>();
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
    add_collider: bool,
) -> Option<Entity> {
    let wall_x = GROUND_LENGTH / 3.5;
    let wall_y = GROUND_THICKNESS * 3.;
    let wall_z = GROUND_LENGTH * 0.01;
    let wall: Mesh = shape::Box::new(wall_x, wall_y, wall_z).into();
    let wall_ent = commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(wall.clone()),
                material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                transform,
                ..default()
            },
            RigidBody::Fixed,
            BelongsToGround(*ground_ent),
        ))
        .id();
    if add_collider {
        let Some(collider) = Collider::from_bevy_mesh(
            &wall, &ComputedColliderShape::TriMesh) else { return None; };
        commands.entity(wall_ent).insert(collider.clone());
    }
    commands.entity(*ground_ent).push_children(&[wall_ent]);
    Some(wall_ent)
}
