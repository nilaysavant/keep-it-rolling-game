use bevy::prelude::*;

use crate::constants::GAME_TITLE;

/// # Setup Window
///
/// System updates and sets up the window attributes
pub fn setup_window(mut windows: Query<&mut Window>) {
    let mut window = windows.get_single_mut().unwrap();
    window.title = GAME_TITLE.to_string();
}
