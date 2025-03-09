use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy_rapier3d::dynamics::RigidBody;
use bevy_rapier3d::geometry::{AsyncSceneCollider, ComputedColliderShape, TriMeshFlags};
use system::commons::{InBattle, WorldPlayer};
use system::states::{GameState, InGameState};
use system::events::world_events::WorldEntityHitEntityEvent;
use crate::environment::{BattleEnvironment, CurrentAreaScenes, EnvironmentScene};

/// The `EnvSwapSystemPlugin` manages the transition between different game states,
/// specifically handling environment swaps for battle scenes.
pub struct EnvSwapSystemPlugin;

impl Plugin for EnvSwapSystemPlugin {
    /// Registers systems for handling battle scene transitions.
    ///
    /// - Preloads battle conditions in `Main` state.
    /// - Loads the battle scene when entering `Battle` state.
    /// - Allows leaving battle with a key press in `Battle` state.
    /// - Swaps back to `Main` state after `BattleEnd`.
    fn build(&self, app: &mut App) {
        app.add_systems(Update, pre_load_battle.run_if(in_state(GameState::InGame(InGameState::Main))));
        app.add_systems(OnEnter(GameState::InGame(InGameState::Battle)), load_battle_scene);
        app.add_systems(Update, temp_leave_battle.run_if(in_state(GameState::InGame(InGameState::Battle))));
        app.add_systems(OnEnter(GameState::InGame(InGameState::BattleEnd)), temp_swap_to_main);
    }
}

/// Preloads battle conditions when a player hits an enemy.
///
/// # Parameters
/// - `hit_event`: Reads world entity hit events.
/// - `area`: Stores current area scenes.
/// - `commands`: Used to insert battle state markers.
/// - `world_player_query`: Checks if the attacker is a player.
/// - `next_state`: Transitions the game state to `Battle` if conditions are met.
fn pre_load_battle(
    mut hit_event: EventReader<WorldEntityHitEntityEvent>,
    area: Res<CurrentAreaScenes>,
    mut commands: Commands,
    world_player_query: Query<Entity, With<WorldPlayer>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in hit_event.read() {
        let sender_a_player = world_player_query.get(event.sender).is_ok();
        if area.0.len() > 3 {
            if sender_a_player {
                commands.entity(event.sender).insert(InBattle);
            } else {
                commands.entity(event.entity).insert(InBattle);
            }
            info!("Starting Battle [{:?}]", area.0.len());
            next_state.set(GameState::InGame(InGameState::Battle));
        } else {
            warn!("No Battle Scenes found! Scenes [ {:?} ]", area.0.len());
        }
    }
}

/// Loads the battle scene when transitioning to `Battle` state.
///
/// # Parameters
/// - `area`: Holds the current battle scene references.
/// - `commands`: Spawns the battle scene and its components.
/// - `players`: Updates player positions in battle.
///
/// The function spawns the battle scene, sets up collision, and positions the players.
fn load_battle_scene(
    area: Res<CurrentAreaScenes>,
    mut commands: Commands
) {
    let battle_scene = area.0.get("battle_1");

    if let Some(scene) = battle_scene {
        commands.spawn(SceneRoot(scene.clone()))
            .insert(Name::new("Battle Scene"))
            .insert(EnvironmentScene)
            .insert(NoFrustumCulling)
            .insert(RigidBody::Fixed)
            .insert(BattleEnvironment)
            .insert(AsyncSceneCollider {
                shape: Some(ComputedColliderShape::TriMesh(TriMeshFlags::MERGE_DUPLICATE_VERTICES)),
                ..default()
            });
    }

    info!("Loading Battle Scenes");
}

/// Allows players to leave the battle scene when pressing `L`.
///
/// # Parameters
/// - `commands`: Removes battle markers and de-spawns battle environment.
/// - `players`: Moves players out of the battle area.
/// - `battle_query`: Identifies battle environment entities to remove.
/// - `keyboard`: Detects key input for leaving.
/// - `next_state`: Transitions game state to `BattleEnd`.
fn temp_leave_battle(
    mut commands: Commands,
    mut players: Query<(Entity, &mut Transform), With<InBattle>>,
    battle_query: Query<Entity, With<BattleEnvironment>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::KeyL) {
        for (entity, mut transform) in players.iter_mut() {
            commands.entity(entity).remove::<InBattle>();
            transform.translation = Vec3::new(40.0, 12.0, 40.0);
        }

        for entity in battle_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        next_state.set(GameState::InGame(InGameState::BattleEnd));
        info!("Leaving Battle Scenes");
    }
}

/// Transitions the game state back to `Main` after the battle ends.
///
/// # Parameters
/// - `next_state`: Transitions the game state to `Main`.
fn temp_swap_to_main(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::InGame(InGameState::Main));
}
