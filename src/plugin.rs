//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};
use bevy_rapier3d::prelude::*;

use crate::{
    resources::GroundsResource,
    systems::{
        camera::move_camera_focus_with_grounds,
        lights::move_lighting_with_grounds,
        physics::{
            cleanup_marked, color_grounds, handle_ground_sensor, handle_mid_ground_sensor,
            mark_cleanup_prev_grounds,
        },
        scene::scene_setup,
        window::setup_window,
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
            .add_plugins((
                RapierPhysicsPlugin::<NoUserData>::default(),
                RapierDebugRenderPlugin::default(),
            ))
            // logic...
            .add_systems(Startup, scene_setup)
            // physics...
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
            .add_systems(First, cleanup_marked)
            // camera
            .add_systems(Update, move_camera_focus_with_grounds)
            // lights
            .add_systems(Update, move_lighting_with_grounds)
            // debug...
            .add_plugins(WorldInspectorPlugin::default())
            .register_type::<GroundsResource>()
            .add_plugins(ResourceInspectorPlugin::<GroundsResource>::default())
            // other...
            .add_systems(Startup, || info!("Game Started..."));
    }
}
