use bevy::prelude::*;
use system::data::ChangeCharacter;
use system::states::GameState;

pub struct AccountScreenState;

impl Plugin for AccountScreenState {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::AccountScreen), not_implemented_yet);
    }
}

fn not_implemented_yet(
    mut next_state: ResMut<NextState<GameState>>,
    mut load_character: ResMut<ChangeCharacter>
) {
    load_character.0 = true;
    next_state.set(GameState::EnvironmentPreLoad);
}