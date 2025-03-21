use bevy::prelude::*;
use system::battle_commons::{BattleCurrentEntities, BattleSelectedStatus, TurnCurrentMemberInfo};
use system::commons::{Character, Enemy, TurnOrder};
use system::states::{GameState, InGameState};

pub struct BattleFightPlugin;

impl Plugin for BattleFightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, character_perform_attack.run_if(in_state(GameState::InGame(InGameState::Battle))));
    }
}

fn character_perform_attack(
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
            for (entity, mut enemy) in enemy_query.iter_mut() {
                if let Some((_, selected_entity)) = selected.selected {
                    if entity == selected_entity {
                        apply_damage(&mut enemy, entity, attack, &mut commands, &mut battle_members);
                        continue;
                    }
                }

                for sub_entity in selected.sub_selected.values() {
                    if entity == *sub_entity {
                        apply_damage(&mut enemy, entity, attack, &mut commands, &mut battle_members);
                    }
                }
            }

            if turn_current_member_info.character.is_some() {
                turn_current_member_info.character = None;
            }

            if turn_current_member_info.selected_operation.is_some() {
                turn_current_member_info.selected_operation = None;
            }
            turn_order.next = true;
            if battle_members.enemies.is_empty() {
                battle_members.characters.clear();
                next_state.set(GameState::InGame(InGameState::BattleEnd));
                info!("Leaving Battle Scenes");
            }
        }
    }
}

fn apply_damage(
    enemy: &mut Enemy,
    entity: Entity,
    attack: f64,
    commands: &mut Commands,
    battle_members: &mut ResMut<BattleCurrentEntities>,
) {
    let reduced_defense = enemy.current_stats.defense * 0.2;
    let damage = (attack - reduced_defense).max(0.0);

    enemy.current_stats.hp = (enemy.current_stats.hp - damage).max(0.0);
    info!(
        "Target {:?} takes {} damage, remaining HP: {}",
        enemy.name, damage, enemy.current_stats.hp
    );

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
