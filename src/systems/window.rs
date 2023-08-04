use bevy::prelude::*;

/// # Setup Window
///
/// System updates and sets up the window attributes
pub fn setup_window(mut windows: Query<&mut Window>) {
    let mut window = windows.get_single_mut().unwrap();
    window.title = "Keep It Rolling".to_string();
}
