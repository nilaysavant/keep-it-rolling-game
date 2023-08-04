//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;

use crate::systems::{basic::setup, scene::scene_setup, window::setup_window};

pub struct KeepItRollingGamePlugin;

impl Plugin for KeepItRollingGamePlugin {
    fn build(&self, app: &mut App) {
        app //
            // window...
            .add_systems(Startup, setup_window)
            // logic...
            .add_systems(Startup, scene_setup)
            // other...
            .add_systems(Startup, || info!("Game Started..."));
    }
}
