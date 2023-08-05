use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    components::{
        BelongsToGround, Cleanup, GameOverSensor, Ground, GroundMesh, GroundMidSensor,
        GroundSurfaceSensor, MyCamera, MyLight, RollingBall,
    },
    constants::{GROUND_ANGLE, GROUND_LENGTH, GROUND_THICKNESS, GROUND_WIDTH},
    events::SceneEvent,
    resources::{GroundsResource, PreviousScoresRes, ScoresResource},
    state::GameState,
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
    // de-spawn sensor(s)...
    let game_over_sensor_mesh: Mesh = shape::Box::new(
        GROUND_WIDTH * 1.5,
        GROUND_THICKNESS * 20.,
        GROUND_LENGTH * 1.5,
    )
    .into();
    let Some(game_over_sen_collider) = Collider::from_bevy_mesh(
        &game_over_sensor_mesh, &ComputedColliderShape::TriMesh) else { return; };
    let game_over_sensor_transform = Transform::from_translation(Vec3::Y * 2.0 + Vec3::Z * 1.0);
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(game_over_sensor_mesh.clone()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.9).into()),
            transform: game_over_sensor_transform,
            visibility: Visibility::Hidden,
            ..default()
        },
        game_over_sen_collider.clone(),
        Sensor,
        GameOverSensor {
            init_transform: game_over_sensor_transform,
        },
    ));

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
            transform: Transform::from_xyz(0.0, 2.5, -1.0),
            ..default()
        },
        ball_collider,
        RigidBody::Dynamic,
        RollingBall,
        Velocity::default(),
    ));

    // light...
    let light_transform = Transform::from_xyz(1.0, 8.0, 0.0);
    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                intensity: 2500.0,
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

    let zoom_out_fact = 2.2;
    let cam_transform = Transform::from_xyz(
        -4.0 * zoom_out_fact,
        4.5 * zoom_out_fact,
        6.0 * zoom_out_fact,
    )
    .looking_at(Vec3::Z * 5., Vec3::Y);
    // camera...
    commands.spawn((
        Camera3dBundle {
            transform: cam_transform,
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
        .insert((Ground, BelongsToGround(ground_ent), Ccd::default()))
        .with_children(|commands| {
            // main ground mesh...
            let ground_mesh: Mesh =
                shape::Box::new(GROUND_WIDTH, GROUND_THICKNESS, GROUND_LENGTH).into();
            let Some(ground_collider) = Collider::from_bevy_mesh(
                &ground_mesh, &ComputedColliderShape::TriMesh) else { return; };
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(ground_mesh.clone()),
                    material: materials.add(Color::GREEN.into()),
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
            // mid sensor...
            let ground_mid_sensor_mesh: Mesh =
                shape::Box::new(GROUND_WIDTH, GROUND_THICKNESS * 4.0, GROUND_LENGTH * 0.1).into();
            let Some(ground_mid_collider) = Collider::from_bevy_mesh(
                &ground_mid_sensor_mesh, &ComputedColliderShape::TriMesh) else { return; };
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(ground_mid_sensor_mesh.clone()),
                    material: materials.add(Color::rgb(0.3, 0.5, 0.9).into()),
                    transform: Transform::from_translation(
                        Vec3::Y * GROUND_THICKNESS * 2.0 - Vec3::Z * GROUND_LENGTH * 0.2,
                    ),
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

#[allow(clippy::too_many_arguments)]
pub fn handle_scene_events(
    mut commands: Commands,
    balls: Query<Entity, With<RollingBall>>,
    grounds: Query<Entity, With<Ground>>,
    lights: Query<Entity, With<MyLight>>,
    cameras: Query<Entity, With<MyCamera>>,
    game_over_sensor: Query<Entity, With<GameOverSensor>>,
    mut events: EventReader<SceneEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    mut ground_res: ResMut<GroundsResource>,
    scores_res: ResMut<ScoresResource>,
    mut prev_scores_res: ResMut<PreviousScoresRes>,
) {
    for event in events.iter() {
        match event {
            SceneEvent::Start => {}
            SceneEvent::Restart => {
                // push to prev scores
                prev_scores_res.0.push(scores_res.clone());
                // reset any resources...
                *ground_res = GroundsResource::default();
                // mark for cleanup
                for entity in balls.iter() {
                    commands.entity(entity).insert(Cleanup::Recursive);
                }
                for entity in grounds.iter() {
                    commands.entity(entity).insert(Cleanup::Recursive);
                }
                for entity in lights.iter() {
                    commands.entity(entity).insert(Cleanup::Recursive);
                }
                for entity in cameras.iter() {
                    commands.entity(entity).insert(Cleanup::Recursive);
                }
                for entity in game_over_sensor.iter() {
                    commands.entity(entity).insert(Cleanup::Recursive);
                }
                next_state.set(GameState::Menu);
            }
        }
    }
}
