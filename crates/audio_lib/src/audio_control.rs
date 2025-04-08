use std::time::Duration;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_kira_audio::*;
use system::config::ConfigService;
use crate::{AudioManager, AudioType};

/// Struct representing audio options for the game. It holds the master volume and the individual volumes for different audio categories.
/// The `master_volume` is applied to all audio types, and the `volumes` HashMap holds category-specific volume settings.
#[derive(Resource)]
pub struct AudioOption {
    pub master_volume: f64,
    pub volumes: HashMap<String, f64>
}

impl Default for AudioOption {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioOption {
    /// Creates a new instance of `AudioOption` with default volume values.
    ///
    /// # Returns:
    /// A new `AudioOption` with `master_volume` set to 1.0 and an empty `volumes` map.
    pub fn new() -> Self {
        Self {
            master_volume: 1.0,
            volumes: HashMap::new(),
        }
    }

    /// Initializes the `AudioOption` struct with values from a configuration service.
    /// The method sets the `master_volume` and individual category volumes based on the provided config.
    /// All values are clamped between 0.0 and 1.0 to ensure valid audio levels.
    ///
    /// # Parameters:
    /// - `config`: A reference to the `ConfigService` that provides audio configuration values.
    pub fn initialize(&mut self, config: &ConfigService) {
        self.master_volume = config.audio_config.master_volume.clamp(0.0, 1.0);

        self.volumes.insert("environment".to_string(), config.audio_config.environment_volume.clamp(0.0, 1.0));
        self.volumes.insert("character_voice".to_string(), config.audio_config.character_voice_volume.clamp(0.0, 1.0));
        self.volumes.insert("sfx".to_string(), config.audio_config.sfx_volume.clamp(0.0, 1.0));
        self.volumes.insert("ui".to_string(), config.audio_config.ui_volume.clamp(0.0, 1.0));
    }

    /// Sets the master volume and applies the updated volume settings to the audio channels.
    /// This method will ensure the master volume is applied across all audio categories.
    ///
    /// # Parameters:
    /// - `volume`: The new master volume value to set.
    /// - `audio_kira_handle`: A mutable reference to `DynamicAudioChannels` to manage dynamic audio channels.
    /// - `audio_manager`: A reference to the `AudioManager` that handles the audio channels.
    pub fn set_master_volume(&mut self, volume: f64, audio_kira_handle: &mut DynamicAudioChannels, audio_manager: &AudioManager) {
        self.master_volume = (volume.clamp(0.0, 1.0) * 100.0).round() / 100.0;
        self.apply_volumes(audio_kira_handle, audio_manager);
    }

    /// Sets the volume for a specific audio category and applies the updated volume settings.
    ///
    /// # Parameters:
    /// - `category`: The name of the audio category (e.g., "environment", "sfx", etc.).
    /// - `volume`: The volume to set for the given category.
    /// - `audio_kira_handle`: A mutable reference to `DynamicAudioChannels` to manage dynamic audio channels.
    /// - `audio_manager`: A reference to the `AudioManager` that handles the audio channels.
    pub fn set_category_volume(&mut self, category: &str, volume: f64, audio_kira_handle: &mut DynamicAudioChannels, audio_manager: &AudioManager) {
        if let Some(ch) = self.volumes.get_mut(category) {
            *ch = (volume.clamp(0.0, 1.0) * 100.0).round() / 100.0;
            self.apply_volumes(audio_kira_handle, audio_manager);
        }
    }

    /// Applies the volume settings to all audio channels by adjusting each channel's volume according to the individual category volumes.
    /// The volume for each channel is calculated by multiplying the category volume by the master volume.
    /// This method ensures all channels are updated with the new volume settings.
    ///
    /// # Parameters:
    /// - `audio_kira_handle`: A mutable reference to `DynamicAudioChannels` to manage dynamic audio channels.
    /// - `audio_manager`: A reference to the `AudioManager` that handles the audio channels.
    fn apply_volumes(&self, audio_kira_handle: &mut DynamicAudioChannels, audio_manager: &AudioManager) {
        for (audio_type, volume) in &self.volumes {
            for (channel, audio_type_enum) in audio_manager.audio.clone() {
                if AudioType::from_string(audio_type) == audio_type_enum {
                    if let Some(audio) = audio_kira_handle.get_channel(channel.as_str()) {
                        let insert = (volume * self.master_volume * 100.0).round() / 100.0;

                        info!("Value: {}, channel: {:?}", insert, channel);

                        audio.set_volume(insert)
                            .fade_in(AudioTween::new(Duration::from_secs(1), AudioEasing::Linear));
                    }
                }
            }
        }
    }
}


