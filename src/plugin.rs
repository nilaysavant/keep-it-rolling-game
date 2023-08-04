//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::systems::{
    physics::{ball_not_touching_any_ground, ball_touching_ground, color_active_grounds},
    scene::scene_setup,
    window::setup_window,
};

pub struct KeepItRollingGamePlugin;

impl Plugin for KeepItRollingGamePlugin {
    fn build(&self, app: &mut App) {
        app //
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
            .add_systems(
                Update,
                (
                    ball_touching_ground,
                    ball_not_touching_any_ground,
                    color_active_grounds,
                ),
            )
            // other...
            .add_systems(Startup, || info!("Game Started..."));
    }
}
