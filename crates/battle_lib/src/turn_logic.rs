use bevy::prelude::*;
use system::battle_commons::TurnCurrentMemberInfo;
use system::commons::{Character, CharacterAbilitySet, Enemy, TurnOrder};
use system::states::{GameState, InGameState};
use crate::logic::detect_current_character_operation;
use crate::setup::{spawn_entities};

/// Plugin that handles the turn-based battle logic.
pub struct BattleTurnLogicPlugin;

impl Plugin for BattleTurnLogicPlugin {
    /// Builds the plugin by adding systems for setting up and managing battle turns.
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InGame(InGameState::Battle)),
            setup_battle_order.after(spawn_entities),
        );
        app.add_systems(
            Update,
            battle_order_system
                .run_if(in_state(GameState::InGame(InGameState::Battle))).after(detect_current_character_operation)
        );
    }
}

/// Initializes the turn order at the start of a battle.
///
/// # Parameters
/// - `turn_order`: A mutable resource that stores the turn order.
/// - `characters`: Query for all player-controlled characters.
/// - `enemies`: Query for all enemy-controlled characters.
pub fn setup_battle_order(
    mut turn_order: ResMut<TurnOrder>,
    characters: Query<(Entity, &Character)>,
    enemies: Query<(Entity, &Enemy)>,
) {
    let mut participants: Vec<(Entity, f64)> = Vec::new();

    // Collect character speeds
    for (entity, character) in characters.iter() {
        participants.push((entity, character.current_stats.speed));
    }

    // Collect enemy speeds
    for (entity, enemy) in enemies.iter() {
        participants.push((entity, enemy.current_stats.speed));
    }

    // Sort by speed (descending order)
    participants.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Store sorted entities in turn order
    turn_order.order = participants.into_iter().map(|(entity, _)| entity).collect();

    info!("Turn logic start, entries: [ {} ]", turn_order.order.len());
}

/// Manages the battle turn logic, determining the next entity's turn.
///
/// # Parameters
/// - `turn_order`: A mutable resource tracking the turn order and current turn index.
/// - `characters`: Query for all player-controlled characters, used for checking status.
/// - `enemies`: Query for all enemy-controlled characters, used for checking status.
/// - `turn_current_member_info`: A mutable resource storing the current turn's entity and abilities.
/// - `abilities_query`: Query for retrieving ability sets of characters.
///
/// # Behavior
/// - Skips defeated characters and removes them from the turn order.
/// - Moves to the next available entity in the order.
/// - If the current entity is a character, their ability set is loaded.
/// - If no valid entity remains, the function exits early.
pub fn battle_order_system(
    mut turn_order: ResMut<TurnOrder>,
    characters: Query<(&Character, &Name), (With<Character>, Without<Enemy>)>,
    enemies: Query<(&Enemy, &Name), (With<Enemy>, Without<Character>)>,
    mut turn_current_member_info: ResMut<TurnCurrentMemberInfo>,
    abilities_query: Query<&CharacterAbilitySet>,
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
            if turn_order.order.len() == 1 {
                turn_order.order.clear();
                return;
            }
            let index = turn_order.current_index;
            turn_order.order.remove(index);

            if turn_order.current_index >= turn_order.order.len() {
                turn_order.current_index = 0;
            }
            current_entity = turn_order.order[turn_order.current_index];
            continue;
        } else {
            break;
        }
    }

    if let Ok((character, _)) = characters.get(current_entity) {
        turn_current_member_info.character = Some(character.clone());

        // Fetch the character's abilities
        let abilities = match abilities_query.get(current_entity) {
            Ok(abilities) => abilities,
            Err(_) => return,
        };

        turn_current_member_info.pre_operation = abilities.0.get(0).cloned();
    } else {
        turn_order.next = true;
    }

    // Move to the next turn
    turn_order.current_index = (turn_order.current_index + 1) % turn_order.order.len();
    info!(
        "Turn logic | current_entity: {:?}, index: {:?}",
        current_entity, turn_order.current_index
    );
}


