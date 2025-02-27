use std::time::Duration;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_kira_audio::*;
use system::config::ConfigService;
use crate::{AudioManager, AudioType};

#[derive(Resource)]
pub struct AudioOption {
    pub master_volume: f64,
    pub volumes: HashMap<String, f64>
}

impl AudioOption {
    pub fn new() -> Self {
        Self {
            master_volume: 1.0,
            volumes: HashMap::new(),
        }
    }

    pub fn initialize(&mut self, config: &ConfigService) {
        self.master_volume = config.audio_config.master_volume.clamp(0.0, 1.0);

        self.volumes.insert("environment".to_string(), config.audio_config.environment_volume.clamp(0.0, 1.0));
        self.volumes.insert("character_voice".to_string(), config.audio_config.character_voice_volume.clamp(0.0, 1.0));
        self.volumes.insert("sfx".to_string(), config.audio_config.sfx_volume.clamp(0.0, 1.0));
        self.volumes.insert("ui".to_string(), config.audio_config.ui_volume.clamp(0.0, 1.0));
    }

    pub fn set_master_volume(&mut self, volume: f64, audio_kira_handle: &mut DynamicAudioChannels, audio_manager: &AudioManager) {
        self.master_volume = volume.clamp(0.0, 1.0);
        self.apply_volumes(audio_kira_handle, audio_manager);
    }

    pub fn set_category_volume(&mut self, category: &str, volume: f64, audio_kira_handle: &mut DynamicAudioChannels, audio_manager: &AudioManager) {
        if let Some(ch) = self.volumes.get_mut(category) {
            *ch = volume.clamp(0.0, 1.0);
            self.apply_volumes(audio_kira_handle, audio_manager);
        }
    }

    fn apply_volumes(&self, audio_kira_handle: &mut DynamicAudioChannels, audio_manager: &AudioManager) {
        for (audio_type, volume) in &self.volumes {
            for (channel, audio_type_enum) in audio_manager.audio.clone() {
                if AudioType::from_string(audio_type) == audio_type_enum {
                    if let Some(audio) = audio_kira_handle.get_channel(channel.as_str()) {
                        let insert = volume * self.master_volume;
                        info!("Value: {}, channel: {:?}", insert, channel);
                        audio.set_volume(insert)
                            .fade_in(AudioTween::new(Duration::from_secs(1), AudioEasing::Linear));
                    }
                }
            }
        }
    }
}

