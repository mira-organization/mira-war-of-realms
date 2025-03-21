use bevy::prelude::*;
use system::battle_commons::{ActiveCharacterOption, AttackOperation, BattleCurrentEntities, BattleSelectedStatus, ObserveAble};
use system::commons::{OutlineTargetBundle};
use system::states::{GameState, InGameState};
use crate::character_operations::{aoe_target_operation, expansion_target_operation, single_target_operation};
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
) {
    match action_operation.selected_operation {
        AttackOperation::Attack(1) => single_target_operation(&mut commands, &battle_members, &mut selected),
        AttackOperation::Ability(1) => expansion_target_operation(&mut commands, &battle_members, &mut selected),
        AttackOperation::Ultimate => aoe_target_operation(&mut commands, &battle_members, &mut selected),
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
            selected.selected = Some((1, target.clone()));
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