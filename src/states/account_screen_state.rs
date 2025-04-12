use bevy::prelude::*;
use system::states::GameState;

pub struct AccountScreenState;

impl Plugin for AccountScreenState {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::AccountScreen), not_implemented_yet);
    }
}

fn not_implemented_yet(
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::MainMenu);
}