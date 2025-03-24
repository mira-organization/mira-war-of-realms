use bevy::prelude::*;
use bevy_mod_outline::{OutlineMode, OutlineStencil, OutlineVolume};
use system::battle_commons::{BattleCurrentEntities, BattleSelectedStatus};
use system::commons::OutlineTargetBundle;

/// Performs the operation for selecting a single target during the battle.
/// This will clear any existing outlines, and if a target is selected, it will highlight it with an outline.
///
/// # Parameters:
/// - `commands`: The `Commands` resource to issue entity modification commands.
/// - `battle_members`: A reference to the current battle entities to check the selected entities.
/// - `selected`: A mutable reference to the `BattleSelectedStatus` that stores the currently selected target.
pub fn single_target_operation(
    commands: &mut Commands,
    battle_members: &BattleCurrentEntities,
    selected: &mut BattleSelectedStatus,
    existing_entities: Query<Entity>,
) {
    // Clear any existing outlines from previous selections.
    clear_outline(commands, battle_members, &existing_entities);

    // Clear any sub-selected targets if there are any.
    if !selected.sub_selected.is_empty() {
        selected.sub_selected.clear();
    }

    // If there is a selected entity, apply the target outline.
    insert_outline_target(commands, &selected, &existing_entities);
}

/// Performs the operation for selecting an expansion of targets around the currently selected one.
/// The selected target and adjacent enemies will be highlighted.
///
/// # Parameters:
/// - `commands`: The `Commands` resource to issue entity modification commands.
/// - `battle_members`: A reference to the current battle entities to check for selection and adjacent entities.
/// - `selected`: A mutable reference to the `BattleSelectedStatus` that stores the currently selected target and sub-selections.
pub fn expansion_target_operation(
    commands: &mut Commands,
    battle_members: &BattleCurrentEntities,
    selected: &mut BattleSelectedStatus,
    existing_entities: Query<Entity>,
) {
    // If there is a selected entity, find the corresponding slot.
    if let Some((_, selected_entity)) = selected.selected {
        let mut selected_slot = None;

        // Clear any existing outlines.
        clear_outline(commands, battle_members, &existing_entities);

        // Clear sub-selected targets.
        if !selected.sub_selected.is_empty() {
            selected.sub_selected.clear();
        }

        // Find the slot of the selected entity among the enemies.
        for (slot, entity) in battle_members.enemies.iter() {
            if *entity == selected_entity {
                selected_slot = Some(*slot);
                continue;
            }
        }

        // If the selected slot was found, apply the outline to the selected entity and its adjacent ones.
        if let Some(slot) = selected_slot {
            // If the selected entity is not valid anymore, stop the operation.
            if commands.get_entity(selected_entity).is_none() {
                return;
            }

            // Apply the outline to the selected entity.
            insert_outline_target(commands, &selected, &existing_entities);

            let mut adjacent_slots = Vec::new();

            // Get the slots of the adjacent enemies.
            if let Some(&_left) = battle_members.enemies.get(&(slot - 1)) {
                adjacent_slots.push(slot - 1);
            }
            if let Some(&_right) = battle_members.enemies.get(&(slot + 1)) {
                adjacent_slots.push(slot + 1);
            }

            // Apply outline to adjacent enemies and add them to sub-selected.
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

/// Performs the operation for selecting all enemies in the battle for an AoE (Area of Effect) attack.
/// This will highlight all enemies with an outline to indicate they are targets of the AoE attack.
///
/// # Parameters:
/// - `commands`: The `Commands` resource to issue entity modification commands.
/// - `battle_members`: A reference to the current battle entities to check the enemies.
/// - `selected`: A mutable reference to the `BattleSelectedStatus` that stores the currently selected target.
pub fn aoe_target_operation(
    commands: &mut Commands,
    battle_members: &BattleCurrentEntities,
    selected: &mut BattleSelectedStatus,
) {
    let mut slot = 0;

    // Loop through all enemies and apply outline for AoE selection.
    for (_, enemies) in battle_members.enemies.clone() {
        // If a target is selected, apply the outline to it.
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

            // Add each enemy to the sub-selected targets.
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
/// - `existing_entities`: Query for find world entity instance of selected entity
///
/// # Behavior
/// - Removes all outline-related components from enemy entities.
fn clear_outline(commands: &mut Commands, battle_members: &BattleCurrentEntities, existing_entities: &Query<Entity>) {
    for (_, entity) in battle_members.enemies.iter() {
        if existing_entities.get(*entity).is_ok() {
            commands.entity(*entity)
                .remove::<OutlineVolume>()
                .remove::<OutlineMode>()
                .remove::<OutlineStencil>()
                .remove::<OutlineTargetBundle>();
        }
    }
}

/// Adds an `OutlineTargetBundle` to the selected entity if it exists and is not already marked.
///
/// # Arguments
/// * `commands` - Provides access to the `Commands` struct for modifying entities.
/// * `selected` - A reference to the current `BattleSelectedStatus`, which stores the currently selected entity.
/// * `existing_entities` - A query to check if the entity exists and whether it already has the `OutlineTargetBundle`.
fn insert_outline_target(
    commands: &mut Commands,
    selected: &BattleSelectedStatus,
    existing_entities: &Query<(Entity, Option<&OutlineTargetBundle>)>,
) {
    // Checks if an entity is currently selected.
    if let Some((_, entity)) = selected.selected {
        // Verifies that the entity exists and does not already have an `OutlineTargetBundle`.
        if let Ok((_, None)) = existing_entities.get(entity) {
            // Adds the `OutlineTargetBundle` to visually highlight the entity.
            commands.entity(entity).insert(OutlineTargetBundle::default());
        }
    }
}

