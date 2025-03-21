use bevy::prelude::*;
use system::battle_commons::TurnCurrentMemberInfo;
use system::commons::{Character, CharacterAbilitySet, Enemy, TurnOrder};
use system::states::{GameState, InGameState};
use crate::logic::detect_current_character_operation;
use crate::setup::{spawn_entities};

pub struct BattleTurnLogicPlugin;

impl Plugin for BattleTurnLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame(InGameState::Battle)), setup_battle_order
            .after(spawn_entities));
        app.add_systems(Update, battle_order_system
            .run_if(in_state(GameState::InGame(InGameState::Battle)))
            .before(detect_current_character_operation));
    }
}

fn setup_battle_order(
    mut turn_order: ResMut<TurnOrder>,
    characters: Query<(Entity, &Character)>,
    enemies: Query<(Entity, &Enemy)>
) {
    let mut participants: Vec<(Entity, f64)> = Vec::new();
    for (entity, character) in characters.iter() {
        participants.push((entity, character.current_stats.speed));
    }

    for (entity, enemy) in enemies.iter() {
        participants.push((entity, enemy.current_stats.speed));
    }

    participants.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    turn_order.order = participants.into_iter().map(|(entity, _)| entity).collect();
    info!("Turn logic start, entries: [ {} ]", turn_order.order.len());
}

fn battle_order_system(
    mut turn_order: ResMut<TurnOrder>,
    characters: Query<(&Character, &Name)>,
    enemies: Query<(&Enemy, &Name)>,
    mut turn_current_member_info: ResMut<TurnCurrentMemberInfo>,
    abilities_query: Query<&CharacterAbilitySet>
) {
    if turn_order.order.is_empty() || !turn_order.next {
        return;
    }

    turn_order.next = false;
    let mut current_entity = turn_order.order[turn_order.current_index];
    let mut entity_name = Name::new(String::from("Unknown"));

    loop {
        let (name, is_alive) = characters
            .get(current_entity)
            .map(|(stats, name)| (name.clone(), stats.current_stats.hp > 0.0))
            .or_else(|_| {
                enemies
                    .get(current_entity)
                    .map(|(stats, name)| (name.clone(), stats.current_stats.hp > 0.0))
            })
            .unwrap_or((entity_name, false));

        entity_name = name;

        if !is_alive {
            let index = turn_order.current_index;
            turn_order.order.remove(index);
            if turn_order.order.is_empty() {
                return;
            }
            current_entity = turn_order.order[turn_order.current_index];
            continue;
        } else {
            break;
        }
    }

    if let Ok((character, _)) = characters.get(current_entity) {
        turn_current_member_info.character = Some(character.clone());
        info!("{}'s turn!", character.name);

        let abilities = match abilities_query.get(current_entity) {
            Ok(abilities) => abilities,
            Err(_) => return,
        };

        turn_current_member_info.pre_operation = abilities.0.get(0).cloned();
    } else {
        info!("{}'s turn!", entity_name);
        turn_order.next = true;
    }

    turn_order.current_index = (turn_order.current_index + 1) % turn_order.order.len();
    info!(
        "Turn logic | current_entity: {:?}, index: {:?}",
        current_entity, turn_order.current_index
    );
}

