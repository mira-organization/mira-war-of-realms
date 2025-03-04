use std::fs;
use std::path::Path;
use bevy::prelude::Resource;
use serde::Deserialize;

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
    pub player_up: String,
    pub player_down: String,
    pub player_left: String,
    pub player_right: String,
    pub player_sprint: String,
    pub debug_change: String,
    pub world_inspector_ui: String,
    pub cursor_lock_button: String,
    pub camera_vertical_sensitivity: f32,
    pub camera_horizontal_sensitivity: f32,
    pub camera_zoom_in: f32,
    pub camera_zoom_out: f32,
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
            cursor_lock_button: String::from("Escape"),
            camera_horizontal_sensitivity: 1.0,
            camera_vertical_sensitivity: 1.0,
            camera_zoom_in: 3.5,
            camera_zoom_out: 10.0
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct AudioConfig {
    pub master_volume: f64,
    pub environment_volume: f64,
    pub character_voice_volume: f64,
    pub sfx_volume: f64,
    pub ui_volume: f64
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            environment_volume: 1.0,
            character_voice_volume: 1.0,
            sfx_volume: 1.0,
            ui_volume: 1.0
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
    pub audio_config: AudioConfig
}

impl ConfigService {
    /// Loads a configuration file and deserializes it into the specified type.
    ///
    /// # Arguments
    /// * `path` - A string slice representing the file path.
    ///
    /// # Panics
    /// This function will panic if the file cannot be read or parsed.
    pub fn load<T: for<'de> Deserialize<'de>>(path: &str) -> T {
        let content = fs::read_to_string(Path::new(path)).expect("Failed to read config file");
        toml::from_str(&content).expect("Failed to parse toml file")
    }

    /// Creates a new `ConfigService` instance and loads all configuration files.
    pub fn new() -> Self {
        Self {
            game_config: Self::load("conf/gameConfig.toml"),
            graphics_config: Self::load("conf/graphicsConfig.toml"),
            input_config: Self::load("conf/gameInput.toml"),
            audio_config: Self::load("conf/gameAudio.toml"),
        }
    }
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