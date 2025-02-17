use std::fs;
use std::path::Path;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;
use bevy_rapier3d::prelude::{DebugRenderContext, NoUserData, PhysicsSet, RapierDebugRenderPlugin, RapierPhysicsPlugin};
use bevy_third_person_camera::{CameraSyncSet, ThirdPersonCameraPlugin};
use serde::Deserialize;
use crate::audio::AudioStorePlugin;
use crate::entities::EntitiesPlugin;
use crate::environment::EnvironmentPlugin;
use crate::events::EventManagerPlugin;
use crate::languages::LanguagesPlugin;
use crate::service::ServicePlugin;
use crate::states::StatesPlugin;
use crate::ui::UiPlugin;
use crate::utils::key_code::convert;

pub const PLAYER_VOID_THRESHOLD: f32 = -5.0;

/// The main plugin responsible for initializing game states, resources, and sub-plugins.
pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        // Initialize the game state
        app.init_state::<GameState>();

        // Insert global configuration resource
        app.insert_resource(ConfigService::new());
        app.insert_resource(WorldInspectorState::default());
        app.insert_resource(DummySaveData::default());

        // Add various game-related plugins
        app.add_plugins(LanguagesPlugin);
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
        app.add_plugins(RapierDebugRenderPlugin {
            enabled: false,
            ..default()
        });
        app.add_plugins(WorldInspectorPlugin::default().run_if(check_world_inspector_state));
        app.add_plugins(ThirdPersonCameraPlugin);
        app.add_plugins(AudioPlugin);
        app.add_plugins(AudioStorePlugin);
        app.add_plugins((StatesPlugin, EventManagerPlugin, EntitiesPlugin,
                         EnvironmentPlugin, ServicePlugin, UiPlugin));
        app.configure_sets(PostUpdate, CameraSyncSet.after(PhysicsSet::StepSimulation));
        app.add_systems(Update, (toggle_debug_system, toggle_world_inspector_interface_system));
    }
}

/// Represents the state of the World Inspector UI.
///
/// This resource holds a single boolean value indicating whether the World Inspector UI
/// is currently visible or hidden. The state can be toggled by user input (e.g., a key press),
/// and this struct is used to track the visibility of the World Inspector in the application.
///
/// The `WorldInspectorState` is initialized to `false` (hidden) by default.
///
/// # Fields
///
/// * `0`: A boolean value that represents the visibility of the World Inspector UI.
///   - `true`: The World Inspector is visible.
///   - `false`: The World Inspector is hidden.
#[derive(Resource, Default, Debug)]
pub struct WorldInspectorState(pub bool);

/// Configuration for general game settings.
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct GameConfig {
    pub(crate) bevy_backend: String,
    pub(crate) lang_text: String,
    pub(crate) lang_voice: String,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            bevy_backend: String::from("PRIMARY"),
            lang_text: String::from("en-US"),
            lang_voice: String::from("en-US"),
        }
    }
}

/// Configuration for graphics settings such as resolution and fullscreen mode.
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct GraphicsConfig {
    pub(crate) resolution: String,
    pub(crate) fullscreen: bool,
}

impl Default for GraphicsConfig {
    fn default() -> Self {
        Self {
            resolution: String::from("1270x720"),
            fullscreen: false,
        }
    }
}

/// Configuration for input mappings and camera sensitivity.
#[derive(Deserialize, Debug)]
pub struct InputConfig {
    pub(crate) player_up: String,
    pub(crate) player_down: String,
    pub(crate) player_left: String,
    pub(crate) player_right: String,
    pub(crate) player_sprint: String,
    pub(crate) debug_change: String,
    pub(crate) world_inspector_ui: String,
    pub(crate) camera_vertical_sensitivity: f32,
    pub(crate) camera_horizontal_sensitivity: f32,
    pub(crate) camera_zoom_in: f32,
    pub(crate) camera_zoom_out: f32,
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            player_up: String::from("W"),
            player_down: String::from("S"),
            player_left: String::from("A"),
            player_right: String::from("D"),
            player_sprint: String::from("ShiftLeft"),
            debug_change: String::from("F3"),
            world_inspector_ui: String::from("F1"),
            camera_horizontal_sensitivity: 1.0,
            camera_vertical_sensitivity: 1.0,
            camera_zoom_in: 3.5,
            camera_zoom_out: 10.0
        }
    }
}

/// Enum representing different game states.
#[derive(Component, States, Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum GameState {
    #[default]
    PreLoad,
    SplashScreen,
    TitleScreen,
    AccountScreen,
    EnvironmentPreLoad,
    EnvironmentLoad,
    EnvironmentPostLoad,
    InGame(InGameState),
    InUi(UiState)
}

#[derive(States, Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum InGameState {
    #[default]
    Main,
    Battle,
    BattleEnd,
    Dialogue
}

#[derive(States, Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum UiState {
    Loading,
    #[default]
    Main,
    Settings,
    Shop,
    Wish
}

#[derive(Resource, Debug, PartialEq)]
pub struct DummySaveData {
    pub current_environment: String,
    pub current_area: usize,
}

impl Default for DummySaveData {
    fn default() -> Self {
        Self {
            current_environment: String::from("tutorial"),
            current_area: 3,
        }
    }
}

/// A service that loads and stores game configuration settings.
#[derive(Resource, Debug, Deserialize)]
#[allow(dead_code)]
pub struct ConfigService {
    pub game_config: GameConfig,
    pub graphics_config: GraphicsConfig,
    pub input_config: InputConfig,
}

impl ConfigService {
    /// Loads a configuration file and deserializes it into the specified type.
    ///
    /// # Arguments
    /// * `path` - A string slice representing the file path.
    ///
    /// # Panics
    /// This function will panic if the file cannot be read or parsed.
    fn load<T: for<'de> Deserialize<'de>>(path: &str) -> T {
        let content = fs::read_to_string(Path::new(path)).expect("Failed to read config file");
        toml::from_str(&content).expect("Failed to parse toml file")
    }

    /// Creates a new `ConfigService` instance and loads all configuration files.
    fn new() -> Self {
        Self {
            game_config: Self::load("conf/gameConfig.toml"),
            graphics_config: Self::load("conf/graphicsConfig.toml"),
            input_config: Self::load("conf/gameInput.toml"),
        }
    }
}

/// Checks if the current game state is an instance of `GameState::InGame`.
///
/// # Arguments
/// * `game_state` - A reference to the current state of the game.
///
/// # Returns
/// * `true` if the game is in an `InGame` state, otherwise `false`.
pub fn in_game_states(game_state: Res<State<GameState>>) -> bool {
    matches!(*game_state.get(), GameState::InGame(_))
}

/// Toggles the debug rendering system on or off based on a configured key input.
///
/// # Arguments
/// * `debug_context` - A mutable reference to the debug rendering context.
/// * `keyboard` - A resource representing the current state of keyboard inputs.
/// * `general_config` - A resource containing the game's configuration settings.
fn toggle_debug_system(
    mut debug_context: ResMut<DebugRenderContext>,
    keyboard: ResMut<ButtonInput<KeyCode>>,
    general_config: Res<ConfigService>,
) {
    let key = convert(general_config.input_config.debug_change.as_str())
        .expect("Fetch key for (debug change) was failed!");
    if keyboard.just_pressed(key) {
        debug_context.enabled = !debug_context.enabled
    }
}

/// Toggles the state of the World Inspector UI based on a key press.
///
/// This system checks if the configured key (from `ConfigService`) is pressed.
/// If the key is pressed, it inverts the current state of the `WorldInspectorState`
/// (i.e., toggles whether the World Inspector is visible or not).
///
/// # Arguments
///
/// * `keyboard`: A resource containing the current button input state for key presses.
/// * `general_config`: A resource containing the configuration settings, including the key for toggling the world inspector UI.
/// * `world_inspector_state`: A mutable reference to the state of the world inspector UI, which will be toggled.
///
/// # Panics
///
/// This function will panic if the key from `ConfigService` cannot be converted into a valid `KeyCode`.
fn toggle_world_inspector_interface_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    general_config: Res<ConfigService>,
    mut world_inspector_state: ResMut<WorldInspectorState>,
) {
    let key = convert(general_config.input_config.world_inspector_ui.as_str())
        .expect("Fetch key for (world inspector ui) was failed!");

    if keyboard.just_pressed(key) {
        world_inspector_state.0 = !world_inspector_state.0;
    }
}

/// Checks whether the World Inspector UI is currently enabled or not.
///
/// This function simply checks the state of the `WorldInspectorState`
/// and returns a boolean indicating whether the World Inspector UI is visible.
///
/// # Arguments
///
/// * `world_inspector_state`: A reference to the state of the world inspector UI.
///
/// # Returns
///
/// * `true` if the World Inspector UI is visible (enabled).
/// * `false` if the World Inspector UI is not visible (disabled).
fn check_world_inspector_state(
    world_inspector_state: Res<WorldInspectorState>,
) -> bool {
    world_inspector_state.0
}