use bevy::prelude::*;
use bevy_rapier3d::{na::Translation, prelude::*};

use crate::components::{
    BelongsToGround, Ground, GroundMesh, GroundSensor, MyCamera, MyLight, RollingBall,
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
            Quat::from_axis_angle(Vec3::X, std::f32::consts::FRAC_PI_6),
        )));

    // ball...
    let ball_mesh = Mesh::from(shape::UVSphere {
        radius: 0.5,
        ..default()
    });
    let Some(ball_collider) = Collider::from_bevy_mesh(&ball_mesh, &ComputedColliderShape::ConvexDecomposition(VHACDParameters::default())) else { return; };
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(ball_mesh),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 1.5, 0.0),
            ..default()
        },
        ball_collider,
        RigidBody::Dynamic,
        RollingBall,
    ));

    // light...
    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        },
        MyLight,
    ));

    let zoom_out = 5.;
    // camera...
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0 * zoom_out, 2.5 * zoom_out, 5.0 * zoom_out)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MyCamera,
    ));
}

pub fn spawn_ground(
    commands: &mut Commands<'_, '_>,
    meshes: &mut ResMut<'_, Assets<Mesh>>,
    materials: &mut ResMut<'_, Assets<StandardMaterial>>,
) -> Option<Entity> {
    let ground_mesh: Mesh = shape::Plane::from_size(5.0).into();
    let Some(ground_collider) = Collider::from_bevy_mesh(&ground_mesh, &ComputedColliderShape::TriMesh) else { return None; };
    let ground_ent = commands
        .spawn_empty()
        .insert((
            VisibilityBundle {
                visibility: Visibility::Visible,
                ..default()
            },
            // ,
        ))
        .id();
    commands
        .entity(ground_ent)
        .insert(BelongsToGround(ground_ent))
        .with_children(|commands| {
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
                    transform: Transform::from_translation(Vec3::Y * 0.1),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                ground_collider.clone(),
                Sensor,
                GroundSensor,
                BelongsToGround(ground_ent),
            ));
        });
    Some(ground_ent)
}
