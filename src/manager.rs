use std::fs;
use std::path::Path;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;
use bevy_rapier3d::prelude::{NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin};
use bevy_third_person_camera::ThirdPersonCameraPlugin;
use serde::Deserialize;
use crate::audio::AudioStorePlugin;
use crate::entities::EntitiesPlugin;
use crate::environment::EnvironmentPlugin;
use crate::events::EventManagerPlugin;
use crate::languages::LanguagesPlugin;

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.insert_resource(ConfigService::new());
        app.add_plugins(LanguagesPlugin);
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
        app.add_plugins(RapierDebugRenderPlugin::default());
        app.add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F3)));
        app.add_plugins(ThirdPersonCameraPlugin);
        app.add_plugins(AudioPlugin);
        app.add_plugins(AudioStorePlugin);
        app.add_plugins((EventManagerPlugin, EntitiesPlugin, EnvironmentPlugin));
    }
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct InputConfig {
    pub(crate) player_up: String,
    pub(crate) player_down: String,
    pub(crate) player_left: String,
    pub(crate) player_right: String,
    pub(crate) player_sprint: String,
    pub(crate) camera_vertical_sensitivity: f32,
    pub(crate) camera_horizontal_sensitivity: f32,
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
        }
    }
}

#[derive(Component, States, Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum GameState {
    SplashScreen,
    TitleScreen,
    AccountScreen,
    #[default]
    InGame
}

#[derive(Resource, Debug, Deserialize)]
pub struct ConfigService {
    pub game_config: GameConfig,
    pub graphics_config: GraphicsConfig,
    pub input_config: InputConfig,
}

impl ConfigService {
    fn load<T: for<'de> Deserialize<'de>>(path: &str) -> T {
        let content = fs::read_to_string(Path::new(path)).expect("Failed to read config file");
        toml::from_str(&content).expect("Failed to parse toml file")
    }

    fn new() -> Self {
        Self {
            game_config: Self::load("conf/gameConfig.toml"),
            graphics_config: Self::load("conf/graphicsConfig.toml"),
            input_config: Self::load("conf/gameInput.toml"),
        }
    }
}