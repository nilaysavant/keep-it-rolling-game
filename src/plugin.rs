//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};
use bevy_rapier3d::prelude::*;

use crate::{
    events::{SceneEvent, WallEvent},
    plugins::FlyCameraPlugin,
    resources::GroundsResource,
    systems::{
        camera::move_camera_focus_with_grounds,
        ground::{
            color_grounds, handle_ground_sensor, handle_mid_ground_sensor,
            mark_cleanup_prev_grounds,
        },
        lights::move_lighting_with_grounds,
        scene::scene_setup,
        walls::{handle_wall_events, pick_ground_point_raycast},
        window::setup_window, cleanup::cleanup,
    },
};

pub struct KeepItRollingGamePlugin;

impl Plugin for KeepItRollingGamePlugin {
    fn build(&self, app: &mut App) {
        app //
            // background...
            .insert_resource(ClearColor(Color::BLACK))
            // window...
            .add_systems(Startup, setup_window)
            // physics setup...
            .insert_resource(RapierConfiguration {
                gravity: Vec3::new(0., -10., 0.),
                ..default()
            })
            // physics plugins...
            .add_plugins((
                RapierPhysicsPlugin::<NoUserData>::default(),
                RapierDebugRenderPlugin::default(),
            ))
            // fly cam
            // .add_plugins(FlyCameraPlugin)
            // scene...
            .add_event::<SceneEvent>()
            .add_systems(Startup, scene_setup)
            // ground...
            .insert_resource(GroundsResource::default())
            .add_systems(
                Update,
                (
                    handle_ground_sensor,
                    handle_mid_ground_sensor,
                    color_grounds,
                    mark_cleanup_prev_grounds,
                ),
            )
            // walls...
            .add_event::<WallEvent>()
            .add_systems(Update, (pick_ground_point_raycast, handle_wall_events))
            // camera
            .add_systems(Update, move_camera_focus_with_grounds)
            // lights
            .add_systems(Update, move_lighting_with_grounds)
            // cleanup
            .add_systems(First, cleanup)
            // debug...
            .add_plugins(WorldInspectorPlugin::default())
            .register_type::<GroundsResource>()
            .add_plugins(ResourceInspectorPlugin::<GroundsResource>::default())
            // other...
            .add_systems(Startup, || info!("Game Started..."));
    }
}
