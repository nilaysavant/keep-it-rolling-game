use bevy::prelude::*;

use crate::state::GameState;

pub fn auto_start_game_on_menu(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::SceneLoading);
}
