use bevy::prelude::*;
use system::battle_commons::{BattleSelectedStatus, CharacterTurnState};
use system::commons::Character;
use system::config::ConfigService;
use system::states::{GameState, InGameState};
use system::utils::key_code::convert;
use crate::logic::detect_current_character_operation;

pub struct BattleFightPlugin;

impl Plugin for BattleFightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, execute_character_attack
            .run_if(in_state(GameState::InGame(InGameState::Battle)))
            .after(detect_current_character_operation));
    }
}

fn execute_character_attack(
    mut character_query: Query<&mut CharacterTurnState, With<Character>>,
    // battle_members: Res<BattleCurrentEntities>,
    selected: Res<BattleSelectedStatus>,
    keyboard: Res<ButtonInput<KeyCode>>,
    general_config: Res<ConfigService>
) {
    for mut turn_state in character_query.iter_mut() {
        if turn_state.eq(&CharacterTurnState::Waiting)
            || turn_state.eq(&CharacterTurnState::Selecting) {
            continue;
        }

        let attack_key = convert(general_config.input_config.battle_attack_0.as_str()).expect("Fetch key for (attack 0) was failed!");
        let spell_key = convert(general_config.input_config.battle_spell_0.as_str()).expect("Fetch key for (spell 0) was failed!");
        let ultimate_key = convert(general_config.input_config.battle_ultimate.as_str()).expect("Fetch key for (ultimate) was failed!");

        if keyboard.just_pressed(attack_key) {
            if let Some(entity) = selected.selected {
                info!("Normal Attack entity: {:?} and sub: {:?}", entity, selected.sub_selected);
            }
        } else if keyboard.just_pressed(spell_key) {
            if let Some(entity) = selected.selected {
                info!("Ability Attack entity: {:?} and sub: {:?}", entity, selected.sub_selected);
            }
        } else if keyboard.just_pressed(ultimate_key) {
            if let Some(entity) = selected.selected {
                info!("Ultimate Attack entity: {:?} and sub: {:?}", entity, selected.sub_selected);
            }
        }

        *turn_state = CharacterTurnState::Selecting;
    }
}

