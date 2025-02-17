use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_rapier3d::prelude::{AsyncSceneCollider, ComputedColliderShape, RigidBody, TriMeshFlags};
use fluent_bundle::types::AnyEq;
use crate::environment::{Area, CurrentAreaScenes, CurrentEnvironment, Environment, EnvironmentListResource, EnvironmentScene};
use crate::manager::{DummySaveData, GameState, InGameState};

pub struct ReadyUpHandles;

impl Plugin for ReadyUpHandles {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::EnvironmentPreLoad), pre_load_environments);
        app.add_systems(OnEnter(GameState::EnvironmentLoad), pre_load_area);
        app.add_systems(OnEnter(GameState::EnvironmentPostLoad), load_active_area);
    }
}

/// Preloads the environment based on saved data.
///
/// This function selects the environment and area to be loaded based on the dummy save data.
/// If the environment map is empty, an error is logged, and the function returns early.
/// Once the correct environment and area are found, they are stored in the `CurrentEnvironment`
/// resource, and the game state transitions to `GameState::EnvironmentLoad`.
///
/// # Arguments
///
/// * `commands` - Used to insert the `CurrentEnvironment` resource.
/// * `environment` - The list of available environments.
/// * `dummy_save_data` - Holds the current environment and area index.
/// * `next_state` - Used to transition to the next game state.
fn pre_load_environments(mut commands: Commands,
                         environment: Res<EnvironmentListResource>,
                         dummy_save_data: Res<DummySaveData>,
                         mut next_state: ResMut<NextState<GameState>>
) {
    let env_map = environment.0.clone();
    if env_map.is_empty() {
        error!("Empty environment map");
        return;
    }

    let mut to_load: Option<Area> = None;
    let mut founded_env: Option<Environment> = None;
    for (key, value) in env_map.iter() {
        if key.equals(&dummy_save_data.current_environment) {
            for (_a_key, area) in value.areas.iter() {
                if area.index == dummy_save_data.current_area {
                    to_load = Some(area.clone());
                }
            }
            founded_env = Some(value.clone());
        }
    }

    if let Some(env) = founded_env {
        if let Some(area) = to_load {
            commands.insert_resource(CurrentEnvironment {
                environment: env.clone(),
                area: area.clone(),
            });
            info!("Loading environments [{:?}]", env.name);
        }
    }

    next_state.set(GameState::EnvironmentLoad);
}

/// Preloads the area assets before they are fully loaded into the world.
///
/// This function constructs the file paths for the collider, environment, and object scenes
/// and loads them using the `AssetServer`. The loaded scenes are then stored in a
/// `CurrentAreaScenes` resource, and the game state transitions to `GameState::EnvironmentPostLoad`.
///
/// # Arguments
///
/// * `commands` - Used to insert the `CurrentAreaScenes` resource.
/// * `asset_server` - Handles asset loading.
/// * `environment` - Contains the currently selected environment and area.
/// * `next_state` - Used to transition to the next game state.
fn pre_load_area(mut commands: Commands,
                 asset_server: Res<AssetServer>,
                 environment: Res<CurrentEnvironment>,
                 mut next_state: ResMut<NextState<GameState>>
) {
    let path = format!("environments/{}/{}", environment.environment.name, environment.area.name);
    let collider_scene =
        asset_server.load::<Scene>(GltfAssetLabel::Scene(0).from_asset(path.clone()));

    let env_scene =
        asset_server.load::<Scene>(GltfAssetLabel::Scene(1).from_asset(path.clone()));

    let object_scene =
        asset_server.load::<Scene>(GltfAssetLabel::Scene(2).from_asset(path.clone()));

    let mut map = HashMap::new();
    map.insert(String::from("first_layer"), collider_scene);
    map.insert(String::from("second_layer"), env_scene);
    map.insert(String::from("last_layer"), object_scene);

    commands.insert_resource(CurrentAreaScenes(map.clone()));
    next_state.set(GameState::EnvironmentPostLoad);
    info!("Pre Loading [{:?}]", path);
}

/// Spawns the loaded area assets into the game world.
///
/// This function retrieves the preloaded area scenes from the `CurrentAreaScenes` resource
/// and spawns them into the game world. The first and last layers include colliders,
/// while the second layer is purely visual. After spawning, the game state transitions to `GameState::InGame(InGameState::Main)`.
///
/// # Arguments
///
/// * `commands` - Used to spawn entities into the world.
/// * `current_area_scenes` - Holds the loaded area scenes.
/// * `next_state` - Used to transition to the next game state.
fn load_active_area(mut commands: Commands,
                    current_area_scenes: Res<CurrentAreaScenes>,
                    mut next_state: ResMut<NextState<GameState>>
) {
    let first_layer = current_area_scenes.0.get(&String::from("first_layer")).cloned();
    let second_layer = current_area_scenes.0.get(&String::from("second_layer")).cloned();
    let last_layer = current_area_scenes.0.get(&String::from("last_layer")).cloned();

    if let Some(first_layer) = first_layer {
        commands.spawn(SceneRoot(first_layer.clone()))
            .insert(Name::new("Area First Layer"))
            .insert(EnvironmentScene)
            .insert(RigidBody::Fixed)
            .insert(AsyncSceneCollider {
                shape: Some(ComputedColliderShape::TriMesh(TriMeshFlags::MERGE_DUPLICATE_VERTICES)),
                ..default()
            });
    }

    if let Some(second_layer) = second_layer {
        commands.spawn(SceneRoot(second_layer.clone()))
            .insert(Name::new("Area Second Layer"))
            .insert(EnvironmentScene);
    }

    if let Some(last_layer) = last_layer {
        commands.spawn(SceneRoot(last_layer.clone()))
            .insert(Name::new("Area Last Layer"))
            .insert(EnvironmentScene)
            .insert(RigidBody::Fixed)
            .insert(AsyncSceneCollider {
                shape: Some(ComputedColliderShape::TriMesh(TriMeshFlags::MERGE_DUPLICATE_VERTICES)),
                ..default()
            });
    }

    next_state.set(GameState::InGame(InGameState::Main));
}