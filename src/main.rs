use std::time::Duration;

use bevy::{asset::ChangeWatcher, prelude::*, window::PresentMode};
use keep_it_rolling_game::KeepItRollingGamePlugin;

/// Html Canvas selector
const CANVAS_SELECTOR: &str = "#my-game";

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "my bevy game".to_string(),
                        canvas: Some(CANVAS_SELECTOR.to_string()),
                        fit_canvas_to_parent: true,
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                    ..default()
                }),
        )
        .add_plugins(KeepItRollingGamePlugin)
        .run();
}
