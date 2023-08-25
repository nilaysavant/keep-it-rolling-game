use std::{io::Cursor, time::Duration};

use bevy::{
    asset::ChangeWatcher,
    prelude::*,
    window::{PresentMode, PrimaryWindow},
    winit::WinitWindows,
};
use keep_it_rolling_game::KeepItRollingGamePlugin;
use winit::window::Icon;

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
        .add_systems(Startup, set_window_icon)
        .run();
}

// Sets the icon on windows and X11
fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single();
    let primary = windows.get_window(primary_entity).unwrap();
    let icon_buf = Cursor::new(include_bytes!(
        "../build/macos/AppIcon.iconset/icon_256x256.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
