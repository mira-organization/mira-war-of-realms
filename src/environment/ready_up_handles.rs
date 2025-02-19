use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy::utils::HashMap;
use bevy_rapier3d::prelude::{AsyncSceneCollider, ComputedColliderShape, RigidBody, TriMeshFlags};
use fluent_bundle::types::AnyEq;
use crate::environment::{Area, CurrentAreaScenes, CurrentEnvironment, Environment, EnvironmentListResource, EnvironmentScene, WaitingForAreaAssets};
use crate::manager::{DummySaveData, GameState, InGameState};

pub struct ReadyUpHandles;

impl Plugin for ReadyUpHandles {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::EnvironmentPreLoad), pre_load_environments);
        app.add_systems(OnEnter(GameState::EnvironmentLoad), pre_load_area);
        app.add_systems(Update, process_loaded_area.run_if(in_state(GameState::EnvironmentLoad)));
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

/// Pre-loads the `.glb` file of the current area before it is fully loaded into the game world.
/// This ensures that the asset is available in the asset pipeline before rendering.
///
/// Parameters:
/// - `commands`: Bevy's command buffer used to insert resources.
/// - `asset_server`: The asset server responsible for loading assets asynchronously.
/// - `environment`: The currently active environment, containing the area name.
///
/// Behavior:
/// - Constructs the asset path using the current environment and area name.
/// - Requests the asset server to load the `.glb` file.
/// - Stores the loading handle in a `WaitingForAreaAssets` resource.
///
/// Logging:
/// - Outputs an informational log message indicating that the `.glb` file is being preloaded.
fn pre_load_area(mut commands: Commands,
                 asset_server: Res<AssetServer>,
                 environment: Res<CurrentEnvironment>
) {
    let path = format!("environments/{}/{}", environment.environment.name, environment.area.name);
    let glb_handle = asset_server.load::<Gltf>(path.as_str());
    commands.insert_resource(WaitingForAreaAssets(glb_handle.clone()));

    info!("Pre Loading glb [{:?}]", path);
}

/// Processes a previously preloaded `.glb` area once it is fully loaded by Bevy's asset system.
/// This function extracts scenes from the `.glb` file and stores them in a resource for rendering.
///
/// Parameters:
/// - `commands`: Bevy's command buffer used to insert and remove resources.
/// - `gltf_assets`: The collection of all loaded GLTF assets.
/// - `next_state`: A mutable reference to the game's state, used to transition after loading.
/// - `waiting`: An optional resource that holds the handle for the area being loaded.
///
/// Behavior:
/// - Checks if the `.glb` file has finished loading.
/// - Retrieves up to three scenes (layers) from the `.glb` asset:
///   - **Layer 0**: Mandatory scene, causes a panic if missing.
///   - **Layer 1 & 2**: Optional layers; warnings are logged if they are missing.
/// - If additional scenes exist, they are considered potential battle scenes.
/// - Stores the loaded scenes in a `CurrentAreaScenes` resource.
/// - Removes the `WaitingForAreaAssets` resource.
/// - Transitions the game state to `GameState::EnvironmentPostLoad`.
///
/// Logging:
/// - Outputs how many scenes were found.
/// - Logs warnings if layers 1 or 2 are missing.
/// - Signals when environment loading is complete.
fn process_loaded_area(mut commands: Commands,
                       gltf_assets: Res<Assets<Gltf>>,
                       mut next_state: ResMut<NextState<GameState>>,
                       waiting: Option<Res<WaitingForAreaAssets>>,
) {
    if let Some(waiting) = waiting {
        if let Some(gltf) = gltf_assets.get(&waiting.0) {
            let found_scenes = gltf.scenes.len();
            info!("Found [{:?}] scenes", found_scenes);

            let layer_0 = gltf.scenes.get(0).cloned().expect("Scene 0 not found. This is Panic because we need minimum one scene!");
            let layer_1 = gltf.scenes.get(1).cloned();
            let layer_2 = gltf.scenes.get(2).cloned();

            if found_scenes > 3 {
                info!("Battle Scenes was found!");
            }

            let mut map = HashMap::new();
            map.insert(String::from("layer_0"), layer_0.clone());
            if let Some(scene) = layer_1 {
                map.insert(String::from("layer_1"), scene.clone());
            } else {
                warn!("No Layer for Scene 1 found!");
            }

            if let Some(scene) = layer_2 {
                map.insert(String::from("layer_2"), scene.clone());
            } else {
                warn!("No Layer for Scene 2 found!");
            }

            commands.insert_resource(CurrentAreaScenes(map));
            commands.remove_resource::<WaitingForAreaAssets>();

            next_state.set(GameState::EnvironmentPostLoad);
            info!("Finished loading environments");
        }
    }
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
    let first_layer = current_area_scenes.0.get(&String::from("layer_0")).cloned();
    let second_layer = current_area_scenes.0.get(&String::from("layer_1")).cloned();
    let last_layer = current_area_scenes.0.get(&String::from("layer_2")).cloned();

    if let Some(first_layer) = first_layer {
        commands.spawn(SceneRoot(first_layer.clone()))
            .insert(Name::new("Area First Layer"))
            .insert(EnvironmentScene)
            .insert(NoFrustumCulling)
            .insert(RigidBody::Fixed)
            .insert(AsyncSceneCollider {
                shape: Some(ComputedColliderShape::TriMesh(TriMeshFlags::MERGE_DUPLICATE_VERTICES)),
                ..default()
            });
    }

    if let Some(second_layer) = second_layer {
        commands.spawn(SceneRoot(second_layer.clone()))
            .insert(Name::new("Area Second Layer"))
            .insert(NoFrustumCulling)
            .insert(EnvironmentScene);
    }

    if let Some(last_layer) = last_layer {
        commands.spawn(SceneRoot(last_layer.clone()))
            .insert(Name::new("Area Last Layer"))
            .insert(EnvironmentScene)
            .insert(RigidBody::Fixed)
            .insert(NoFrustumCulling)
            .insert(AsyncSceneCollider {
                shape: Some(ComputedColliderShape::TriMesh(TriMeshFlags::MERGE_DUPLICATE_VERTICES)),
                ..default()
            });
    }

    next_state.set(GameState::InGame(InGameState::Main));
}