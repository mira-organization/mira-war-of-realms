mod audio;

use std::time::Duration;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_kira_audio::*;
use crate::audio::audio::AudioHandlerPlugin;

pub struct AudioStorePlugin;

impl Plugin for AudioStorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AudioManager::new());
        app.add_plugins(AudioHandlerPlugin);
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
#[allow(dead_code)]
pub enum AudioType {
    Environment,
    Battle,
    Sfx,
    Ui,
    Character,
    #[default]
    Unknown
}

#[derive(Resource)]
#[allow(dead_code)]
pub struct AudioManager {
    pub master_volume: f32,
    pub ui_volume: f32,
    pub sfx_volume: f32,
    pub character_voice_volume: f32,
    pub environment_volume: f32,
    pub muted: bool,
    pub audio: HashMap<String, AudioType>
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            master_volume: 1.0,
            ui_volume: 0.2,
            sfx_volume: 0.1,
            character_voice_volume: 0.1,
            environment_volume: 0.05,
            muted: false,
            audio: HashMap::new()
        }
    }

    pub fn add_audio(&mut self,
                     channel_name: &str,
                     audio_type: AudioType,
                     track_path: &str,
                     audio_kira_handle: &mut DynamicAudioChannels,
                     asset_server: &AssetServer
    ) {
        if self.audio.contains_key(channel_name) {
            warn!("Audio already in use: {}", channel_name);
            return;
        }

        let looped = self.looped_time(audio_type.clone());
        let mut binding = audio_kira_handle.create_channel(channel_name)
            .play(asset_server.load(track_path));
        let build = binding
            .fade_in(AudioTween::new(Duration::from_secs(2), AudioEasing::OutPowi(2)))
            .with_volume(0.05);
        if looped { build.looped(); }

        self.audio.insert(channel_name.to_string(), audio_type);
        debug!("AudioManager added to {}", channel_name);
    }

    pub fn remove_audio(&mut self, channel_name: &str, audio_kira_handle: &mut DynamicAudioChannels) {
        if !self.audio.contains_key(channel_name) {
            warn!("Audio not in use: {}", channel_name);
            return;
        }

        self.stop_channel(channel_name, audio_kira_handle);
        audio_kira_handle.remove_channel(channel_name);
        self.audio.remove(channel_name);
        debug!("Audio removed: {}", channel_name);
    }

    pub fn stop_channel(&mut self, channel_name: &str, audio_kira_handle: &mut DynamicAudioChannels) {
        if !self.audio.contains_key(channel_name) {
            warn!("Audio not in use: {}", channel_name);
            return;
        }

        audio_kira_handle.channel(channel_name).stop().fade_out(AudioTween::new(Duration::from_secs(2), AudioEasing::InPowi(2)));
        debug!("Stopped audio {}", channel_name);
    }

    fn looped_time(&self, audio_type: AudioType) -> bool {
        let looped;
        match audio_type {
            AudioType::Environment | AudioType::Battle => { looped = true; }
            _ => { looped = false; }
        }
        looped
    }
}

#[derive(Component, Resource, Debug)]
pub struct EnvironmentAudio {
    pub base_track: String,
    pub battle_track: String,
}

