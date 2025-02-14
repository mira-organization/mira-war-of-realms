use std::fs;
use std::path::Path;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;
use bevy_rapier3d::prelude::{NoUserData, PhysicsSet, RapierDebugRenderPlugin, RapierPhysicsPlugin};
use bevy_third_person_camera::{CameraSyncSet, ThirdPersonCameraPlugin};
use serde::Deserialize;
use crate::audio::AudioStorePlugin;
use crate::entities::EntitiesPlugin;
use crate::environment::EnvironmentPlugin;
use crate::events::EventManagerPlugin;
use crate::languages::LanguagesPlugin;
use crate::service::load_service::PipelinesReady;
use crate::service::ServicePlugin;

pub const PLAYER_VOID_THRESHOLD: f32 = -5.0;

/// The main plugin responsible for initializing game states, resources, and sub-plugins.
pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        // Initialize the game state
        app.init_state::<GameState>();

        // Insert global configuration resource
        app.insert_resource(ConfigService::new());

        // Add various game-related plugins
        app.add_plugins(LanguagesPlugin);
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
        app.add_plugins(RapierDebugRenderPlugin::default());
        app.add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F3)));
        app.add_plugins(ThirdPersonCameraPlugin);
        app.add_plugins(AudioPlugin);
        app.add_plugins(AudioStorePlugin);
        app.add_plugins((EventManagerPlugin, EntitiesPlugin, EnvironmentPlugin, ServicePlugin));
        app.configure_sets(PostUpdate, CameraSyncSet.after(PhysicsSet::StepSimulation));
        app.add_systems(
            Update,
            transition
                .run_if(in_state(GameState::PreLoad))
                .run_if(resource_changed::<PipelinesReady>),
        );
    }
}

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
    InGame(InGameState),
}

#[derive(States, Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InGameState {
    #[default]
    Main,
    Battle
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

pub fn in_game_states(game_state: Res<State<GameState>>) -> bool {
    matches!(*game_state.get(), GameState::InGame(_))
}

fn transition(ready: Res<PipelinesReady>, mut next_state: ResMut<NextState<GameState>>) {
    info!("transitioning state {:?}", ready.get());
    if ready.get() >= 6 {
        info!("Finished Loading!");
        next_state.set(GameState::InGame(InGameState::Main));
    }
}