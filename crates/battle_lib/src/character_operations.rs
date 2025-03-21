use bevy::prelude::*;
use bevy_mod_outline::{OutlineMode, OutlineStencil, OutlineVolume};
use system::battle_commons::{BattleCurrentEntities, BattleSelectedStatus};
use system::commons::OutlineTargetBundle;

pub fn single_target_operation(
    commands: &mut Commands,
    battle_members: &BattleCurrentEntities,
    selected: &mut BattleSelectedStatus,
) {
    clear_outline(commands, battle_members);

    if !selected.sub_selected.is_empty() {
        selected.sub_selected.clear();
    }

    if let Some((_, entity)) = selected.selected {
        if commands.get_entity(entity).is_some() {
            commands.entity(entity).insert(OutlineTargetBundle::default());
        }
    }
}

pub fn expansion_target_operation(
    commands: &mut Commands,
    battle_members: &BattleCurrentEntities,
    selected: &mut BattleSelectedStatus,
) {
    if let Some((_, selected_entity)) = selected.selected {
        let mut selected_slot = None;

        clear_outline(commands, battle_members);

        if !selected.sub_selected.is_empty() {
            selected.sub_selected.clear();
        }

        for (slot, entity) in battle_members.enemies.iter() {
            if *entity == selected_entity {
                selected_slot = Some(*slot);
                continue;
            }
        }

        if let Some(slot) = selected_slot {
            if commands.get_entity(selected_entity).is_none() {
                return;
            }

            if commands.get_entity(selected_entity).is_some() {
                commands.entity(selected_entity).insert(OutlineTargetBundle::default());
            }

            let mut adjacent_slots = Vec::new();

            if let Some(&_left) = battle_members.enemies.get(&(slot - 1)) {
                adjacent_slots.push(slot - 1);
            }
            if let Some(&_right) = battle_members.enemies.get(&(slot + 1)) {
                adjacent_slots.push(slot + 1);
            }

            for adj_slot in adjacent_slots {
                if let Some(&adj_entity) = battle_members.enemies.get(&adj_slot) {
                    if commands.get_entity(adj_entity).is_none() {
                        continue;
                    }
                    commands.entity(adj_entity).insert(OutlineTargetBundle {
                        volume: OutlineVolume {
                            visible: true,
                            width: 3.0,
                            colour: Color::srgb(0.7, 0.0, 0.3),
                        },
                        ..default()
                    });

                    if !selected.sub_selected.contains_key(&adj_slot) {
                        selected.sub_selected.insert(adj_slot, adj_entity);
                    }
                }
            }
        }
    }
}

pub fn aoe_target_operation(
    commands: &mut Commands,
    battle_members: &BattleCurrentEntities,
    selected: &mut BattleSelectedStatus,
) {
    let mut slot = 0;
    for (_, enemies) in battle_members.enemies.clone() {
        if selected.selected.is_some() {
            if commands.get_entity(enemies).is_none() {
                continue;
            }
            commands.entity(enemies).insert(OutlineTargetBundle {
                volume: OutlineVolume {
                    visible: true,
                    width: 3.0,
                    colour: Color::srgb(0.7, 0.0, 0.3),
                },
                ..default()
            });
            slot += 1;

            if !selected.sub_selected.contains_key(&slot) {
                selected.sub_selected.insert(slot, enemies);
            }
        }
    }
}

/// Clears all outline effects from enemies.
///
/// This function removes visual indicators used for selecting or targeting enemies.
///
/// # Parameters
/// - `commands`: Command buffer to modify entities.
/// - `battle_members`: The entities currently engaged in battle.
///
/// # Behavior
/// - Removes all outline-related components from enemy entities.
fn clear_outline(commands: &mut Commands, battle_members: &BattleCurrentEntities) {
    for (_, entity) in battle_members.enemies.iter() {
        if commands.get_entity(*entity).is_some() {
            commands.entity(*entity)
                .remove::<OutlineVolume>()
                .remove::<OutlineMode>()
                .remove::<OutlineStencil>()
                .remove::<OutlineTargetBundle>();
        }
    }
}
