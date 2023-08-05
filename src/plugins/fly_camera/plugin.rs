use bevy::prelude::*;

use super::systems::{camera_controller, camera_tracker, setup, CameraTracker, FlyCameraSettings};

/// # Fly Camera Plugin
///
/// Fly Camera Plugin adapted from bevy examples.
///
/// ```ignore
///Controls:
///    MOUSE       - Move camera orientation
///    LClick/M    - Enable mouse movement
///    WSAD        - forward/back/strafe left/right
///    LShift      - 'run'
///    E           - up
///    Q           - down
///    L           - animate light direction
///    U           - toggle shadows
///    C           - cycle through cameras
/// ```
pub struct FlyCameraPlugin;

impl Plugin for FlyCameraPlugin {
    fn build(&self, app: &mut App) {
        app // app
            .insert_resource(FlyCameraSettings::default())
            .insert_resource(CameraTracker::default())
            .add_systems(Startup, setup)
            .add_systems(Update, camera_controller)
            .add_systems(Update, camera_tracker);
    }
}
