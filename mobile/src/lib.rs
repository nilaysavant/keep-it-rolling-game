use bevy::prelude::*;
use bevy::window::WindowMode;
use keep_it_rolling_game::KeepItRollingGamePlugin;

#[bevy_main]
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    mode: WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            }),
            KeepItRollingGamePlugin,
        ))
        .run()
}
