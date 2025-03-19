use bevy::prelude::*;
use bevy_mod_outline::{OutlineMode, OutlineStencil, OutlineVolume};
use system::battle_commons::{ActiveCharacterOption, AttackOperation, BattleCurrentEntities, BattleSelectedStatus, CharacterTurnState, ObserveAble};
use system::commons::{Character, OutlineTargetBundle};
use system::states::{GameState, InGameState};
use crate::observes::{on_mouse_click, on_mouse_enter, on_mouse_leave};
use crate::setup::{setup_battle_entities, spawn_entities};

/// The `BattleLogicPlugin` handles the battle logic, including entity selection, action detection,
/// and applying visual effects such as outlines for targeted entities.
///
/// This plugin registers systems for handling battle mechanics during the `InGame::Battle` state.
pub struct BattleLogicPlugin;

impl Plugin for BattleLogicPlugin {
    /// Builds the battle logic plugin by adding relevant systems to the app.
    ///
    /// - `set_observe_entities`: Sets up entities that can be observed.
    /// - `select_encounter_target`: Automatically selects an initial battle target.
    /// - `detect_current_character_operation`: Detects when a character's action changes and updates outlines accordingly.
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame(InGameState::Battle)), set_observe_entities.after(spawn_entities));
        app.add_systems(OnEnter(GameState::InGame(InGameState::Battle)),
                        select_encounter_target.after(setup_battle_entities));
        app.add_systems(Update, detect_current_character_operation
            .run_if(in_state(GameState::InGame(InGameState::Battle)))
            .run_if(
                resource_changed::<ActiveCharacterOption>
                    .or(resource_changed::<BattleSelectedStatus>)
            )
        );
    }
}

/// Detects the currently selected character's operation and applies appropriate effects.
///
/// This system reacts to changes in `ActiveCharacterOption` and `BattleSelectedStatus`, updating
/// entity outlines based on the chosen attack or ability.
///
/// # Parameters
/// - `commands`: Command buffer for modifying entities.
/// - `action_operation`: The currently selected action.
/// - `battle_members`: The current battle participants.
/// - `selected`: The selected target entity.
/// - `parent`: Query to retrieve an entity’s parent.
///
/// # Behavior
/// - **Basic Attack**: Highlights the selected entity.
/// - **Ability**: Highlights the selected entity and its adjacent targets.
/// - **Ultimate**: Highlights all enemies.
pub fn detect_current_character_operation(
    mut commands: Commands,
    action_operation: Res<ActiveCharacterOption>,
    battle_members: Res<BattleCurrentEntities>,
    mut selected: ResMut<BattleSelectedStatus>,
    mut character_state: ResMut<CharacterTurnState>,
    parent: Query<&Parent>
) {
    if character_state.entity.is_none() {
        character_state.entity = Some(action_operation.clone().character);
        character_state.action = action_operation.selected_operation.clone();
    } else {
        return;
    }

    info!("Detected current character operation");
    match action_operation.selected_operation {
        AttackOperation::Attack(1) => {
            clear_outline(&mut commands, &battle_members);

            if !selected.sub_selected.is_empty() {
                selected.sub_selected.clear();
            }

            if let Some(entity) = selected.selected {
                if let Ok(parent_entity) = parent.get(entity) {
                    commands.entity(parent_entity.get()).insert(OutlineTargetBundle::default());
                } else {
                    commands.entity(entity).insert(OutlineTargetBundle::default());
                }
            }
        }
        AttackOperation::Ability(1) => {
            if let Some(selected_entity) = selected.selected {
                let mut selected_slot = None;

                clear_outline(&mut commands, &battle_members);

                if !selected.sub_selected.is_empty() {
                    selected.sub_selected.clear();
                }

                for (slot, entity) in battle_members.enemies.iter() {
                    if *entity == selected_entity {
                        selected_slot = Some(*slot);
                        break;
                    }
                }

                if let Some(slot) = selected_slot {
                    commands.entity(selected_entity).insert(OutlineTargetBundle::default());

                    let adjacent_slots = match slot {
                        1 => vec![2],
                        2 => vec![1, 3],
                        3 => vec![2, 4],
                        4 => vec![3],
                        _ => vec![],
                    };

                    for adj_slot in adjacent_slots {
                        if let Some(&adj_entity) = battle_members.enemies.get(&adj_slot) {
                            commands.entity(adj_entity).insert(OutlineTargetBundle {
                                volume: OutlineVolume {
                                    visible: true,
                                    width: 3.0,
                                    colour: Color::srgb(0.7, 0.0, 0.3),
                                },
                                ..default()
                            });
                        }
                    }
                }
            }
        }
        AttackOperation::Ultimate => {
            let mut slot = 0;
            for (_, enemies) in battle_members.enemies.clone() {
                if selected.selected.is_some() {
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
        _ => {}
    }
}

/// Selects an initial target for the battle encounter.
///
/// This system assigns the first enemy in the list as the selected target if no selection exists.
///
/// # Parameters
/// - `commands`: Command buffer to modify entities.
/// - `battle_members`: The entities currently engaged in battle.
/// - `selected`: The selected target status.
///
/// # Behavior
/// - If no entity is selected, the first enemy (slot 1) is chosen and highlighted.
fn select_encounter_target(
    mut commands: Commands,
    battle_members: Res<BattleCurrentEntities>,
    mut selected: ResMut<BattleSelectedStatus>,
) {
    let target = battle_members.enemies.get(&1);
    if let Some(target) = target {
        if selected.selected.is_none() {
            selected.selected = Some(target.clone());
            commands.entity(target.clone()).insert(OutlineTargetBundle::default());
        }
    }
}

/// Sets up entities that can be observed by the player.
///
/// This system adds child entities to all entities with the `ObserveAble` component, making them interactable.
///
/// # Parameters
/// - `commands`: Command buffer for modifying entities.
/// - `query`: Entities that can be observed.
/// - `meshes`: Asset resource to create visual representations.
///
/// # Behavior
/// - Adds a 3D capsule mesh as a child.
/// - Registers interaction handlers for mouse events.
fn set_observe_entities(
    mut commands: Commands,
    query: Query<Entity, With<ObserveAble>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for entity in query.iter() {
        commands.entity(entity).with_children(|children| {
            children.spawn((
                Transform::from_xyz(0.0, 0.9, 0.0),
                Mesh3d(meshes.add(Capsule3d::new(0.275, 1.6)))
            ))
                .observe(on_mouse_click)
                .observe(on_mouse_enter)
                .observe(on_mouse_leave);
        });
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
fn clear_outline(commands: &mut Commands, battle_members: &Res<BattleCurrentEntities>) {
    for (_, entities) in battle_members.enemies.clone() {
        commands.entity(entities)
            .remove::<OutlineVolume>()
            .remove::<OutlineMode>()
            .remove::<OutlineStencil>()
            .remove::<OutlineTargetBundle>();
    }
}
