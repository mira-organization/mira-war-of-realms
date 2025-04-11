pub mod env_init;
pub mod env_handles;
pub mod ready_up_handles;

use std::f32::consts::PI;
use bevy::asset::RenderAssetUsages;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::Deserialize;
use system::shader::{ToonLight, ToonMaterial};
use system::states::GameState;
use crate::environment::env_handles::EnvSwapSystemPlugin;
use crate::environment::env_init::EnvInitPlugin;
use crate::environment::ready_up_handles::ReadyUpHandles;

pub struct EnvironmentPlugin;

/// The main plugin responsible for managing environments in the game.
///
/// This plugin initializes environment-related resources, adds sub-plugins,
/// and registers necessary systems related to environment management.
///
/// # Systems Added
/// - `create_light`: Handles the creation of lighting when an environment is loaded.
///
/// # Plugins Added
/// - `EnvInitPlugin`: Handles environment initialization.
/// - `ReadyUpHandles`: Manages loading environments and areas.
/// - `EnvSwapSystemPlugin`: Manages environment swapping.
impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnvironmentListResource>();
        app.add_plugins((EnvInitPlugin, ReadyUpHandles, EnvSwapSystemPlugin));
        app.add_systems(OnEnter(GameState::EnvironmentPostLoad), create_light);
    }
}

/// Stores a list of all available environments in the game.
///
/// The key is a `String` representing the environment name, and the value is
/// an `Environment` struct containing details about the environment.
///
/// This resource is initialized as an empty `HashMap` by default.
#[derive(Resource, Debug)]
pub struct EnvironmentListResource(pub HashMap<String, Environment>);

impl Default for EnvironmentListResource {
    fn default() -> Self {
        Self {
            0: HashMap::new(),
        }
    }
}

/// Stores information about the currently active environment and area.
///
/// This resource holds both the selected `Environment` and the specific `Area`
/// within it that the player is currently in.
#[derive(Resource, Debug)]
pub struct CurrentEnvironment {
    pub environment: Environment,
    pub area: Area,
}

/// Represents an environment in the game.
///
/// An environment consists of multiple `Area`s and has a `state`
/// that determines whether it's in an exploring, battle, or boss state.
///
/// # Fields
/// - `name`: The name of the environment.
/// - `loaded`: Whether the environment is currently loaded.
/// - `areas`: A map of areas within this environment.
/// - `state`: The current state of the environment.
#[derive(Component, Reflect, Debug, Clone)]
pub struct Environment {
    pub name: String,
    pub loaded: bool,
    pub areas: HashMap<String, Area>,
    pub state: EnvironmentState
}

/// Represents a specific area within an environment.
///
/// Each area has an index, a name, and may contain battle scenes.
/// The `player_in_bound` field indicates if the player is currently in this area.
///
/// # Fields
/// - `name`: The name of the area.
/// - `index`: The index of the area within the environment.
/// - `player_in_bound`: Whether the player is currently inside the area's boundaries.
/// - `battle_scenes`: A collection of battle scenes associated with this area.
#[derive(Reflect, Debug, Clone)]
pub struct Area {
    pub name: String,
    pub index: usize,
    pub player_in_bound: bool,
    pub battle_scenes: HashMap<String, BattleScene>
}

/// Defines the possible states of an environment.
///
/// # Variants
/// - `Exploring`: The player is freely exploring the environment.
/// - `Battle`: The player is currently in a battle.
/// - `Boss`: The player is engaged in a boss fight.
#[derive(Reflect, Debug, Clone, PartialEq)]
pub enum EnvironmentState {
    Exploring,
    Battle,
    Boss
}

/// Represents a battle scene in an area.
///
/// Each battle scene has a name and a set of associated battle music tracks.
///
/// # Fields
/// - `name`: The name of the battle scene.
/// - `battle_music`: A map of music tracks for the battle.
#[derive(Component, Reflect, Debug, Clone)]
pub struct BattleScene {
    pub name: String,
    pub battle_music: HashMap<String, String>,
}

/// A marker component for environment-related scenes.
///
/// This component is used to tag entities representing environment visuals in the game world.
#[derive(Component, Debug, Clone)]
pub struct EnvironmentScene;

#[derive(Component, Debug, Clone)]
pub struct BattleEnvironment;

/// Stores the currently loaded area scenes as handles.
///
/// This resource maps scene layer names to their corresponding `Handle<Scene>` objects.
/// The layers include collision, environment visuals, and objects.
///
/// # Layers
/// - `"first_layer"`: Contains collision data.
/// - `"second_layer"`: Contains the visual environment.
/// - `"last_layer"`: Contains objects in the scene.
#[derive(Resource, Debug, Clone)]
pub struct CurrentAreaScenes(pub HashMap<String, Handle<Scene>>);

/// Stores a handle to a GLTF asset that is being loaded for an area.
/// Used to track the loading state of area assets.
#[derive(Resource, Debug, Clone)]
pub struct WaitingForAreaAssets(pub Handle<Gltf>);

/// Stores a handle to a GLTF asset that contains effect scene data.
/// Used for loading additional scene effects like lights or particle effects.
#[derive(Resource, Debug, Clone)]
pub struct EffectSceneAssets(pub Handle<Gltf>);

/// Represents light data extracted from GLTF extras.
/// The data is deserialized from JSON and contains parameters to configure different types of lights.
#[derive(Deserialize, Debug)]
pub struct LightData {
    /// Name of the light type (e.g., "point", "spot").
    pub name: String,
    /// Optional intensity of the light, defaults if not provided.
    pub intensity: Option<f32>,
    /// Optional range of the light, defining how far it affects its surroundings.
    pub range: Option<f32>,
    /// Optional Radius is used by Point lights only!
    pub radius: Option<f32>,
    /// RGB color values of the light, represented as an array of three floating-point numbers.
    pub color: [f32; 3],
    /// Option bool value for handle displaying character and object shadows
    pub shadows: Option<bool>,
    /// Optional inner cone angle for spotlights, defining the sharply lit area.
    pub inner_cone: Option<f32>,
    /// Optional outer cone angle for spotlights, defining the full spread of light.
    pub outer_cone: Option<f32>
}

/// Enum representing different types of lights that can be spawned in the game.
/// - `Point` for omnidirectional point lights.
/// - `Spot` for directional spotlights with a cone shape.
pub enum LightType {
    Point(PointLight),
    Spot(SpotLight),
}


fn create_light(mut commands: Commands, mut toon_materials: ResMut<Assets<ToonMaterial>>, mut meshes: ResMut<Assets<Mesh>>, mut images: ResMut<Assets<Image>>,) {
    // Spawn the directional light entity
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,  // Set light intensity to an overcast day level
            shadows_enabled: true,  // Enable shadows for the light
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 200.0, 0.0),  // Position the light above the floor
            rotation: Quat::from_rotation_x(-PI / 4.0),  // Rotate the light to cast shadows at an angle
            ..default()
        },
        CascadeShadowConfigBuilder {
            num_cascades: 4,  // Set up 4 cascades for better shadow quality
            first_cascade_far_bound: 10.0,  // Set the distance for the first shadow cascade
            minimum_distance: 0.5,  // Minimum distance for shadow rendering
            maximum_distance: 200.0,  // Maximum distance for shadow rendering
            overlap_proportion: 0.2  // Set the overlap proportion for shadow cascades
        }
            .build(),
        ToonLight,
    ));

    let toon_material = toon_materials.add(ToonMaterial {
        texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    commands.spawn((
        Mesh3d(meshes.add(Torus::new(1.0, 1.5))),
        MeshMaterial3d(toon_material),
        Transform::from_translation(Vec3::new(0.0, 16.0, 0.0)),
    ));
}

fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        bevy::render::render_resource::Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        &texture_data,
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    )
}