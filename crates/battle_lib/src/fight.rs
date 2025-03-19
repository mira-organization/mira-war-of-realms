use bevy::prelude::*;
use system::battle_commons::CharacterTurnState;
use system::run_conditions::detect_inputs;
use system::states::{GameState, InGameState};
use crate::logic::detect_current_character_operation;

pub struct BattleFightPlugin;

impl Plugin for BattleFightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, perform_attack.run_if(in_state(GameState::InGame(InGameState::Battle)))
            .run_if(detect_inputs)
            .before(detect_current_character_operation));
    }
}

fn perform_attack(mut character_turn_state: ResMut<CharacterTurnState>) {
    if character_turn_state.entity.is_some() {
        let perform = character_turn_state.action.clone();
        let name = character_turn_state.entity.clone().unwrap().name;
        info!("Character {:?} perform: {:?}", name, perform);
        character_turn_state.entity = None;
    }
}

