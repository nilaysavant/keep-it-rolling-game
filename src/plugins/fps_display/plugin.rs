use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

use super::systems::fps_text_update_system;

/// # FPS Display Plugin
///
/// Plugin to display frames per second stat.
pub struct FpsDisplayPlugin;

impl Plugin for FpsDisplayPlugin {
    fn build(&self, app: &mut App) {
        app // app
            .add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Update, fps_text_update_system)
            // rest...
            .add_systems(Startup, || info!("Starting FpsDisplayPlugin..."));
    }
}
