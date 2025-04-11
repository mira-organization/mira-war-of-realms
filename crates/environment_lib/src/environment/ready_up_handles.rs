use bevy::gltf::GltfNode;
use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy::utils::HashMap;
use bevy_rapier3d::prelude::{AsyncSceneCollider, ComputedColliderShape, RigidBody, TriMeshFlags};
use serde_json::Value;
use system::config::DummySaveData;
use system::data::AssetsToLoad;
use system::shader::ToonMarker;
use system::states::{GameState, InGameState};
use crate::environment::*;

pub struct ReadyUpHandles;

impl Plugin for ReadyUpHandles {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::EnvironmentPreLoad), pre_load_environments);
        app.add_systems(OnEnter(GameState::EnvironmentLoad), (pre_load_area, pre_load_gltf_assets));
        app.add_systems(Update, process_loaded_area.run_if(in_state(GameState::EnvironmentLoad)));
        app.add_systems(Update, load_active_area_lights.run_if(in_state(GameState::EnvironmentPostLoad)));
        app.add_systems(OnEnter(GameState::EnvironmentPostLoad), load_active_area);
        app.add_systems(Update, apply_toon_shader_to_scene_meshes
            .run_if(in_state(GameState::InGame(InGameState::Main)))
            .after(load_active_area));
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
pub fn pre_load_environments(mut commands: Commands,
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
        if key.eq(&dummy_save_data.current_environment) {
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
pub fn pre_load_area(mut commands: Commands,
                 asset_server: Res<AssetServer>,
                 environment: Res<CurrentEnvironment>, mut assets_to_load: ResMut<AssetsToLoad>
) {
    let path = format!("environments/{}/{}", environment.environment.name, environment.area.name);
    let glb_handle = asset_server.load::<Gltf>(path.as_str());
    commands.insert_resource(WaitingForAreaAssets(glb_handle.clone()));
    assets_to_load.0.push(glb_handle.untyped().id());
    info!("Pre Loading glb [{:?}]", path);
}

pub fn pre_load_gltf_assets(mut commands: Commands, asset_server: Res<AssetServer>, environment: Res<CurrentEnvironment>) {
    let path = format!("environments/{}/{}", environment.environment.name, environment.area.name);
    let gltf_handle = asset_server.load::<Gltf>(path.as_str());

    commands.insert_resource(EffectSceneAssets(gltf_handle.clone()));
    info!("Pre Loading gltf for extras [{:?}]", path);
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
pub fn process_loaded_area(mut commands: Commands,
                       gltf_assets: Res<Assets<Gltf>>,
                       mut next_state: ResMut<NextState<GameState>>,
                       waiting: Option<Res<WaitingForAreaAssets>>,
) {
    if let Some(waiting) = waiting {
        if let Some(gltf) = gltf_assets.get(&waiting.0) {
            let mut map = HashMap::new();
            let found_scenes = gltf.scenes.len();
            info!("Found [{:?}] scenes", found_scenes);

            let layer_0 = gltf.scenes.get(0).cloned().expect("Scene 0 not found. This is Panic because we need minimum one scene!");
            let layer_1 = gltf.scenes.get(1).cloned();
            let layer_2 = gltf.scenes.get(2).cloned();

            if found_scenes > 4 {
                info!("Battle Scenes was found!");
                let mut count = 1;
                for (index, scene) in gltf.scenes.iter().enumerate() {
                    if index > 3 {
                        map.insert(format!("battle_{}", count), scene.clone());
                        count += 1;
                    }
                }
            }

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
pub fn load_active_area(mut commands: Commands,
                    current_area_scenes: Res<CurrentAreaScenes>
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
            .insert(ToonMarker)
            .insert(AsyncSceneCollider {
                shape: Some(ComputedColliderShape::TriMesh(TriMeshFlags::MERGE_DUPLICATE_VERTICES)),
                ..default()
            });
    }

    if let Some(second_layer) = second_layer {
        commands.spawn(SceneRoot(second_layer.clone()))
            .insert(Name::new("Area Second Layer"))
            .insert(NoFrustumCulling)
            .insert(ToonMarker)
            .insert(EnvironmentScene);
    }

    if let Some(last_layer) = last_layer {
        commands.spawn(SceneRoot(last_layer.clone()))
            .insert(Name::new("Area Last Layer"))
            .insert(EnvironmentScene)
            .insert(RigidBody::Fixed)
            .insert(NoFrustumCulling)
            .insert(ToonMarker)
            .insert(AsyncSceneCollider {
                shape: Some(ComputedColliderShape::TriMesh(TriMeshFlags::MERGE_DUPLICATE_VERTICES)),
                ..default()
            });
    }

}

/// Loads active area lights from GLTF extra scene assets and spawns them into the world.
///
/// # Parameters
/// - `commands`: Commands for spawning entities.
/// - `next_state`: The next game state to transition to after loading lights.
/// - `gltf_assets`: GLTF asset resources.
/// - `gltf_nodes`: GLTF node resources containing extra metadata.
/// - `extra_scene_assets`: Optional extra scene assets that may contain light data.
pub fn load_active_area_lights(
    mut commands: Commands,
    gltf_assets: Res<Assets<Gltf>>,
    gltf_nodes: Res<Assets<GltfNode>>,
    extra_scene_assets: Option<Res<EffectSceneAssets>>,
) {
    if let Some(layer_lights) = extra_scene_assets {
        if let Some(gltf) = gltf_assets.get(&layer_lights.0) {
            process_gltf_lights(&mut commands, &gltf, &gltf_nodes);
        }
    }
}

/// Processes GLTF nodes to extract light information and spawn them into the world.
///
/// # Parameters
/// - `commands`: Mutable reference to commands for spawning entities.
/// - `gltf`: Reference to the loaded GLTF asset.
/// - `gltf_nodes`: Reference to the GLTF node assets.
fn process_gltf_lights(
    commands: &mut Commands,
    gltf: &Gltf,
    gltf_nodes: &Assets<GltfNode>,
) {
    for node_handle in &gltf.nodes {
        if let Some(node) = gltf_nodes.get(node_handle) {
            if let Some(extras) = &node.extras {
                info!("Value: {:?}", &extras.value);
                if let Ok(parsed) = serde_json::from_str::<Value>(&extras.value) {
                    if let Some(bevy_json) = parsed.get("bevy_value").and_then(|v| v.as_str()) {
                        debug!("Json: {:?}", bevy_json);
                        if let Ok(light_data) = serde_json::from_str::<LightData>(bevy_json) {
                            spawn_light(commands, node, light_data);
                        }
                    }
                }
            }
        }
    }
}
/// Spawns a light entity based on the extracted light data.
///
/// # Parameters
/// - `commands`: Mutable reference to commands for spawning entities.
/// - `node`: Reference to the GLTF node containing transformation data.
/// - `light_data`: The extracted light data to configure the light entity.
fn spawn_light(commands: &mut Commands, node: &GltfNode, light_data: LightData) {
    debug!("Spawning light: {:?}", light_data);
    let light = match light_data.name.as_str() {
        "point" => LightType::Point(PointLight {
            intensity: light_data.intensity.unwrap_or(1000.0),
            range: light_data.range.unwrap_or(2.0),
            color: Color::srgb(light_data.color[0], light_data.color[1], light_data.color[2]),
            radius: light_data.radius.unwrap_or(1.5),
            shadows_enabled: light_data.shadows.unwrap_or(true),
            ..Default::default()
        }),
        "spot" => LightType::Spot(SpotLight {
            intensity: light_data.intensity.unwrap_or(1000.0),
            color: Color::srgb(light_data.color[0], light_data.color[1], light_data.color[2]),
            inner_angle: light_data.inner_cone.unwrap_or(PI / 4.0 * 0.85),
            outer_angle: light_data.outer_cone.unwrap_or(PI / 4.0),
            shadows_enabled: light_data.shadows.unwrap_or(true),
            radius: light_data.radius.unwrap_or(1.0),
            range: light_data.range.unwrap_or(2.0),
            ..Default::default()
        }),
        _ => return,
    };

    let transform = Transform {
        translation: node.transform.translation,
        rotation: node.transform.rotation,
        scale: node.transform.scale,
    };
    match light {
        LightType::Point(point_light) => commands.spawn((point_light, transform)),
        LightType::Spot(spot_light) => commands.spawn((spot_light, transform)),
    };
}

pub fn apply_toon_shader_to_scene_meshes(
    mut commands: Commands,
    mut toon_materials: ResMut<Assets<ToonMaterial>>,
    standard_materials: Res<Assets<StandardMaterial>>,
    mut mesh_standard: Query<(Entity, &MeshMaterial3d<StandardMaterial>)>,
    scene_roots: Query<(Entity, &Children), With<ToonMarker>>,
    children_query: Query<&Children>,
) {
    fn traverse(
        entity: Entity,
        children_query: &Query<&Children>,
        mesh_standard: &mut Query<(Entity, &MeshMaterial3d<StandardMaterial>)>,
        standard_materials: &Res<Assets<StandardMaterial>>,
        toon_materials: &mut ResMut<Assets<ToonMaterial>>,
        commands: &mut Commands,
    ) {
        if let Ok((entity, mesh_mat)) = mesh_standard.get_mut(entity) {
            if let Some(std_mat) = standard_materials.get(&mesh_mat.0) {
                let toon_handle = toon_materials.add(ToonMaterial {
                    texture: std_mat.base_color_texture.clone(),
                    base_color: std_mat.base_color.to_linear(),
                    light_direction: Vec3::new(-0.3, -1.0, -0.3).normalize(),
                    light_color: Color::srgba(1.0, 0.9, 0.85, 1.0).to_linear(),
                    ambient_color: Color::srgba(0.02, 0.02, 0.02, 1.0).to_linear(),
                    rim_amount: 1.2,
                    rim_color: Color::srgba(1.0, 0.9, 0.85, 1.0).to_linear(),
                    rim_threshold: 0.01,
                    band_count: 6,
                    ..default()
                });

                commands
                    .entity(entity)
                    .remove::<MeshMaterial3d<StandardMaterial>>()
                    .insert(MeshMaterial3d::<ToonMaterial>(toon_handle));
            }
        }

        if let Ok(children) = children_query.get(entity) {
            for &child in children {
                traverse(child, children_query, mesh_standard, standard_materials, toon_materials, commands);
            }
        }
    }

    for (root_entity, children) in &scene_roots {
        for &child in children {
            traverse(child, &children_query, &mut mesh_standard, &standard_materials, &mut toon_materials, &mut commands);
        }

        commands.entity(root_entity).remove::<ToonMarker>();
    }
}