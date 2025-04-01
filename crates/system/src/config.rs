use std::fs;
use std::path::Path;
use bevy::log::error;
use bevy::prelude::Resource;
use serde::Deserialize;
use crate::data::JSONCharacter;

/// Configuration for general game settings such as backend and language preferences.
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct GameConfig {
    /// The backend used by Bevy for rendering.
    pub(crate) bevy_backend: String,

    /// The language used for text in the game.
    pub(crate) lang_text: String,

    /// The language used for voice in the game.
    pub(crate) lang_voice: String,
}

impl Default for GameConfig {
    /// Returns the default game configuration.
    ///
    /// # Returns
    /// - `GameConfig`: A default configuration with `PRIMARY` for `bevy_backend` and `en-US` for both text and voice languages.
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
    /// The resolution of the game display.
    pub(crate) resolution: String,

    /// Whether fullscreen mode is enabled or not.
    pub(crate) fullscreen: bool,
}

impl Default for GraphicsConfig {
    /// Returns the default graphics configuration.
    ///
    /// # Returns
    /// - `GraphicsConfig`: A default configuration with `1270x720` for resolution and fullscreen set to `false`.
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
    /// The key used to move the player up.
    pub player_up: String,

    /// The key used to move the player down.
    pub player_down: String,

    /// The key used to move the player left.
    pub player_left: String,

    /// The key used to move the player right.
    pub player_right: String,

    /// The key used to make the player sprint.
    pub player_sprint: String,

    /// The key used to change to attack at battle
    pub battle_attack_0: String,

    /// The key used to change to spell at battle
    pub battle_spell_0: String,

    /// The key used to change to ultimate at battle
    pub battle_ultimate: String,

    /// The key used to change to character 01 in your party
    pub character_01: String,

    /// The key used to change to character 02 in your party
    pub character_02: String,

    /// The key used to change to character 03 in your party
    pub character_03: String,

    /// The key used to change to character 04 in your party
    pub character_04: String,

    /// The key used to toggle debug change.
    pub debug_change: String,

    /// The key used to open the world inspector UI.
    pub world_inspector_ui: String,

    /// The key used to lock the cursor.
    pub cursor_lock_button: String,

    /// The vertical sensitivity of the camera.
    pub camera_vertical_sensitivity: f32,

    /// The horizontal sensitivity of the camera.
    pub camera_horizontal_sensitivity: f32,

    /// The zoom-in sensitivity of the camera.
    pub camera_zoom_in: f32,

    /// The zoom-out sensitivity of the camera.
    pub camera_zoom_out: f32,
}

impl Default for InputConfig {
    /// Returns the default input configuration.
    ///
    /// # Returns
    /// - `InputConfig`: A default configuration with standard keys for movement and camera sensitivity.
    fn default() -> Self {
        Self {
            player_up: String::from("W"),
            player_down: String::from("S"),
            player_left: String::from("A"),
            player_right: String::from("D"),
            player_sprint: String::from("ShiftLeft"),
            battle_attack_0: String::from("Q"),
            battle_spell_0: String::from("E"),
            battle_ultimate: String::from("Space"),
            character_01: String::from("1"),
            character_02: String::from("2"),
            character_03: String::from("3"),
            character_04: String::from("4"),
            debug_change: String::from("F3"),
            world_inspector_ui: String::from("F1"),
            cursor_lock_button: String::from("Escape"),
            camera_horizontal_sensitivity: 1.0,
            camera_vertical_sensitivity: 1.0,
            camera_zoom_in: 3.5,
            camera_zoom_out: 10.0,
        }
    }
}

/// Configuration for audio settings such as volume levels for various game sounds.
#[derive(Deserialize, Debug)]
pub struct AudioConfig {
    /// The master volume level of the game.
    pub master_volume: f64,

    /// The volume level for environmental sounds.
    pub environment_volume: f64,

    /// The volume level for character voices.
    pub character_voice_volume: f64,

    /// The volume level for sound effects.
    pub sfx_volume: f64,

    /// The volume level for UI sounds.
    pub ui_volume: f64,
}

impl Default for AudioConfig {
    /// Returns the default audio configuration.
    ///
    /// # Returns
    /// - `AudioConfig`: A default configuration with all volume levels set to `1.0` (maximum).
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            environment_volume: 1.0,
            character_voice_volume: 1.0,
            sfx_volume: 1.0,
            ui_volume: 1.0,
        }
    }
}

/// A service that loads and stores game configuration settings for various aspects of the game.
#[derive(Resource, Debug, Deserialize)]
#[allow(dead_code)]
pub struct ConfigService {
    /// Stores the game-related configurations.
    pub game_config: GameConfig,

    /// Stores the graphics-related configurations.
    pub graphics_config: GraphicsConfig,

    /// Stores the input-related configurations.
    pub input_config: InputConfig,

    /// Stores the audio-related configurations.
    pub audio_config: AudioConfig,
}

impl ConfigService {
    /// Loads a configuration file and deserializes it into the specified type.
    ///
    /// # Arguments
    /// - `path`: The file path of the configuration file to load.
    ///
    /// # Panics
    /// This function will panic if the file cannot be read or parsed correctly.
    ///
    /// # Returns
    /// - `T`: The deserialized configuration data.
    pub fn load<T: for<'de> Deserialize<'de>>(path: &str) -> T {
        let content = fs::read_to_string(Path::new(path)).expect("Failed to read config file");
        toml::from_str(&content).expect("Failed to parse toml file")
    }

    /// Creates a new `ConfigService` instance and loads all configuration files.
    ///
    /// # Returns
    /// - `ConfigService`: A new instance with loaded configurations for game, graphics, input, and audio.
    pub fn new() -> Self {
        Self {
            game_config: Self::load("conf/gameConfig.toml"),
            graphics_config: Self::load("conf/graphicsConfig.toml"),
            input_config: Self::load("conf/gameInput.toml"),
            audio_config: Self::load("conf/gameAudio.toml"),
        }
    }
}

/// A dummy structure used for save data in the game world.
#[derive(Resource, Debug, PartialEq)]
pub struct DummySaveData {
    /// The current environment being used in the game.
    pub current_environment: String,

    /// The current area in the game world.
    pub current_area: usize,

    pub current_char: Option<JSONCharacter>
}

impl Default for DummySaveData {
    /// Returns the default save data.
    ///
    /// # Returns
    /// - `DummySaveData`: A default save data instance with `tutorial` for environment and `3` for area.
    fn default() -> Self {
        match JSONCharacter::fetch("ignara") {
            Ok(character) => {
                Self {
                    current_environment: String::from("tutorial"),
                    current_area: 3,
                    current_char: Some(character)
                }
            },
            Err(err) => {
                error!(err);
                Self {
                    current_environment: String::from("tutorial"),
                    current_area: 3,
                    current_char: None
                }
            }
        }
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
