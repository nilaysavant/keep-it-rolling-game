use bevy::prelude::*;
use keep_it_rolling_game::KeepItRollingGamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(KeepItRollingGamePlugin)
        .run();
}
