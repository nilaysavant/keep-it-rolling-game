//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;

use crate::systems::basic::setup;

pub struct KeepItRollingGamePlugin;

impl Plugin for KeepItRollingGamePlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_systems(Startup, setup);
    }
}
