use bevy::prelude::*;
use system::battle_commons::{BattleSelectedStatus, CharacterTurnState};
use system::commons::{Character, Enemy};
use system::states::{GameState, InGameState};

pub struct BattleFightPlugin;

impl Plugin for BattleFightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, character_perform_attack.run_if(in_state(GameState::InGame(InGameState::Battle))));
    }
}

fn character_perform_attack(
    mut character_turn_state: ResMut<CharacterTurnState>,
    mut enemy_query: Query<(Entity, &mut Enemy), (With<Enemy>, Without<Character>)>,
    mut selected: ResMut<BattleSelectedStatus>,
) {
    if character_turn_state.entity.is_some() {
        let perform = character_turn_state.action.clone();
        let character = character_turn_state.entity.clone().unwrap();
        info!("Character {:?} perform: {:?}", character.name, perform);

        for (entity, mut enemy) in enemy_query.iter_mut() {
            if let Some(selected_entity) = selected.selected {
                let attack = character.current_stats.attack * 0.5;

                if selected_entity == entity {

                    let reduced_defense = enemy.current_stats.defense * 0.2;
                    let calc_dmg = attack - reduced_defense;
                    info!("correct damage ({})", calc_dmg);

                    enemy.current_stats.hp -= calc_dmg;

                    info!("Main target hp from {:?}, {}", enemy.name, enemy.current_stats.hp);
                    continue;
                }

                if !selected.sub_selected.is_empty() {
                    for (_slot, sub_entity) in selected.sub_selected.iter_mut() {
                        if entity == *sub_entity {
                            let reduced_defense = enemy.current_stats.defense * 0.2;
                            let calc_dmg = attack - reduced_defense;
                            info!("correct damage ({})", calc_dmg);

                            enemy.current_stats.hp -= calc_dmg;

                            info!("Sub target hp from {:?}, {}", enemy.name, enemy.current_stats.hp);
                        }
                    }
                }
            }
        }
        character_turn_state.entity = None;
    }
}

