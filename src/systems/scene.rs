use bevy::prelude::*;
use bevy_rapier3d::{na::Translation, prelude::*};

use crate::{
    components::{
        BelongsToGround, Ground, GroundMesh, GroundMidSensor, GroundSurfaceSensor, MyCamera,
        MyLight, RollingBall,
    },
    constants::{GROUND_ANGLE, GROUND_LENGTH},
    resources::GroundsResource,
};

/// set up a simple 3D scene
pub fn scene_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ground...
    let Some(ground_ent) = spawn_ground(&mut commands, &mut meshes, &mut materials) else { return; };
    // rotate by 45 deg...
    commands
        .entity(ground_ent)
        .insert(TransformBundle::from_transform(Transform::from_rotation(
            Quat::from_axis_angle(Vec3::X, GROUND_ANGLE),
        )));

    // ball...
    let ball_mesh = Mesh::from(shape::UVSphere {
        radius: 0.2,
        ..default()
    });
    let Some(ball_collider) = Collider::from_bevy_mesh(&ball_mesh, &ComputedColliderShape::ConvexDecomposition(VHACDParameters::default())) else { return; };
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(ball_mesh),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 2.5, -1.0),
            ..default()
        },
        ball_collider,
        RigidBody::Dynamic,
        RollingBall,
        Velocity::default(),
    ));

    // light...
    let light_transform = Transform::from_xyz(4.0, 8.0, 4.0);
    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: light_transform,
            ..default()
        },
        MyLight {
            init_transform: light_transform,
        },
    ));

    let zoom_out_fact = 1.2;
    let cam_transform = Transform::from_xyz(
        -2.0 * zoom_out_fact,
        4.5 * zoom_out_fact,
        5.0 * zoom_out_fact,
    );
    // camera...
    commands.spawn((
        Camera3dBundle {
            transform: cam_transform.looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MyCamera {
            init_transform: cam_transform,
        },
    ));
}

pub fn spawn_ground(
    commands: &mut Commands<'_, '_>,
    meshes: &mut ResMut<'_, Assets<Mesh>>,
    materials: &mut ResMut<'_, Assets<StandardMaterial>>,
) -> Option<Entity> {
    let ground_ent = commands
        .spawn_empty()
        .insert(VisibilityBundle {
            visibility: Visibility::Visible,
            ..default()
        })
        .id();
    commands
        .entity(ground_ent)
        .insert((Ground, BelongsToGround(ground_ent)))
        .with_children(|commands| {
            let ground_mesh: Mesh = shape::Box::new(5.0, 0.2, GROUND_LENGTH).into();
            let Some(ground_collider) = Collider::from_bevy_mesh(
                &ground_mesh, &ComputedColliderShape::TriMesh) else { return; };
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(ground_mesh.clone()),
                    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                    ..default()
                },
                ground_collider.clone(),
                RigidBody::Fixed,
                Ground,
                GroundMesh,
                BelongsToGround(ground_ent),
            ));
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(ground_mesh.clone()),
                    material: materials.add(Color::rgb(0.3, 0.5, 0.9).into()),
                    transform: Transform::from_translation(Vec3::Y * 0.2),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                ground_collider.clone(),
                Sensor,
                GroundSurfaceSensor,
                BelongsToGround(ground_ent),
            ));
            let ground_mid_sensor_mesh: Mesh =
                shape::Box::new(5.0, 1.0, GROUND_LENGTH * 0.1).into();
            let Some(ground_mid_collider) = Collider::from_bevy_mesh(
                &ground_mid_sensor_mesh, &ComputedColliderShape::TriMesh) else { return; };
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(ground_mid_sensor_mesh.clone()),
                    material: materials.add(Color::rgb(0.3, 0.5, 0.9).into()),
                    transform: Transform::from_translation(Vec3::Y * 0.2 - Vec3::Z * GROUND_LENGTH * 0.2),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                ground_mid_collider.clone(),
                Sensor,
                GroundMidSensor,
                BelongsToGround(ground_ent),
            ));
        });
    Some(ground_ent)
}
