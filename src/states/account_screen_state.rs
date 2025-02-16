use bevy::prelude::*;
use crate::manager::{GameState, InGameState};

pub struct AccountScreenState;

impl Plugin for AccountScreenState {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::AccountScreen), not_implemented_yet);
    }
}

fn not_implemented_yet(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::InGame(InGameState::Main));
}