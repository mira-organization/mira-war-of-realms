use bevy::prelude::*;
use system::battle_commons::{BattleCurrentEntities, BattleSelectedStatus, TurnCurrentMemberInfo};
use system::commons::{Character, Enemy, TurnOrder};
use system::states::{GameState, InGameState};

pub struct BattleFightPlugin;

impl Plugin for BattleFightPlugin {

    #[cfg(not(coverage))]
    fn build(&self, app: &mut App) {
        app.add_systems(Update, character_perform_attack.run_if(in_state(GameState::InGame(InGameState::Battle))));
    }
}

/// A plugin for handling battle logic, including character attacks during the battle phase.
///
/// This plugin manages the execution of a character's attack and the application of damage to enemies. It checks
/// if the current state is `InGame` and `Battle`, then performs an attack and applies the resulting damage to the
/// targeted enemies. If all enemies are defeated, the game state transitions to `BattleEnd`.
///
/// # Parameters:
/// - `commands`: A resource to issue commands to the ECS, such as de-spawning entities.
/// - `enemy_query`: A query to access mutable references to the enemy entities and their components.
/// - `battle_members`: A resource to manage the current entities participating in the battle (characters and enemies).
/// - `selected`: A resource indicating the currently selected target(s) for the attack.
/// - `next_state`: A mutable reference to manage the transition between game states.
/// - `turn_order`: A mutable reference to manage the current turn order in the battle.
/// - `turn_current_member_info`: A resource to manage the current character and selected operation for the turn.
///
/// # Behavior:
/// 1. The system checks if there is a character in the current turn and if an operation (attack) is selected.
/// 2. If so, the character performs the selected attack, calculating damage based on the character's stats.
/// 3. The attack is applied to the targeted enemies (either the selected enemy or sub-selected enemies).
/// 4. The turn is completed, and the next turn is prepared in the turn order.
/// 5. If all enemies are defeated, the battle ends and the game transitions to `BattleEnd` state.
///
/// # Returns:
/// This function does not return any value but affects the game state, battle entities, and the next turn order.
pub fn character_perform_attack(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &mut Enemy), (With<Enemy>, Without<Character>)>,
    mut battle_members: ResMut<BattleCurrentEntities>,
    selected: Res<BattleSelectedStatus>,
    mut next_state: ResMut<NextState<GameState>>,
    mut turn_order: ResMut<TurnOrder>,
    mut turn_current_member_info: ResMut<TurnCurrentMemberInfo>
) {
    if let Some(character) = turn_current_member_info.character.clone() {
        if let Some(operation) = turn_current_member_info.selected_operation.clone() {

            info!("Character: {:?} performs: {:?}", character.name, operation.name);
            let attack = character.current_stats.attack * 0.5;

            // Iterate through enemies to apply attack
            for (entity, mut enemy) in enemy_query.iter_mut() {
                // Apply attack to the selected enemy
                if let Some((_, selected_entity)) = selected.selected {
                    if entity == selected_entity {
                        apply_damage(&mut enemy, entity, attack, &mut commands, &mut battle_members);
                        continue;
                    }
                }

                // Apply attack to sub-selected enemies
                for sub_entity in selected.sub_selected.values() {
                    if entity == *sub_entity {
                        apply_damage(&mut enemy, entity, attack, &mut commands, &mut battle_members);
                    }
                }
            }

            // Clear selected character and operation after attack
            turn_current_member_info.character = None;
            turn_current_member_info.selected_operation = None;

            // Proceed to next turn
            turn_order.next = true;

            // If all enemies are defeated, end the battle
            if battle_members.enemies.is_empty() {
                battle_members.characters.clear();
                next_state.set(GameState::InGame(InGameState::BattleEnd));
                info!("Leaving Battle Scenes");
            }
        }
    }
}

/// Applies damage to an enemy based on the calculated attack value.
///
/// This function computes the final damage by reducing the enemy's defense from the attack, and updates the enemy's
/// health points. If the enemy's HP reaches zero or below, the enemy is de-spawned from the battle.
///
/// # Parameters:
/// - `enemy`: A mutable reference to the `Enemy` component of the target enemy.
/// - `entity`: The entity ID of the enemy.
/// - `attack`: The calculated attack value to apply to the enemy.
/// - `commands`: A resource to issue ECS commands (e.g., de-spawn the enemy).
/// - `battle_members`: A mutable reference to the `BattleCurrentEntities` resource to manage enemy entities.
///
/// # Behavior:
/// 1. The defense of the enemy is reduced by a factor of 20% before applying the attack.
/// 2. The resulting damage is subtracted from the enemy's HP.
/// 3. If the enemy's HP falls below or equals zero, the enemy is removed from the battle and de-spawned.
///
/// # Returns:
/// This function does not return a value but updates the enemy's HP and potentially de-spawns the enemy.
fn apply_damage(
    enemy: &mut Enemy,
    entity: Entity,
    attack: f64,
    commands: &mut Commands,
    battle_members: &mut ResMut<BattleCurrentEntities>,
) {
    // Calculate the damage after reducing enemy defense
    let reduced_defense = enemy.current_stats.defense * 0.2;
    let damage = (attack - reduced_defense).max(0.0);

    // Apply the damage to the enemy's health
    enemy.current_stats.hp = (enemy.current_stats.hp - damage).max(0.0);
    info!(
        "Target {:?} takes {} damage, remaining HP: {}",
        enemy.name, damage, enemy.current_stats.hp
    );

    // If the enemy's HP is zero or below, remove them from the battle
    if enemy.current_stats.hp <= 0.0 {
        let mut slot_to_remove = 0;
        for (slot, entity_member) in battle_members.enemies.iter() {
            if entity_member.eq(&entity) {
                slot_to_remove = *slot;
                break;
            }
        }
        info!("De-spawning enemy {:?}!", enemy.name);
        battle_members.enemies.remove(&slot_to_remove);
        battle_members.need_patch = true;
        commands.entity(entity).despawn_recursive();
    }
}

