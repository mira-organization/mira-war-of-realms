use std::time::Duration;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_kira_audio::{AudioControl, AudioEasing, AudioInstance, AudioTween, DynamicAudioChannels};
use crate::manager::GameState;

pub struct AudioStorePlugin;

impl Plugin for AudioStorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup);
    }
}

#[derive(Resource, Debug)]
pub struct AudioManager {
    pub volume: f32,
    pub muted: bool,
    pub channels: HashMap<String, Handle<AudioInstance>>,
}

#[derive(Component, Resource, Debug)]
pub struct EnvironmentAudio {
    pub base_track: String,
    pub battle_track: String,
}

fn setup(asset_server: Res<AssetServer>, mut audio: ResMut<DynamicAudioChannels>) {
    let handle = asset_server.load("audio/env_test.ogg");
    let audio_handle = audio.create_channel("test")
        .play(handle)
        .fade_in(AudioTween::new(Duration::from_millis(2000), AudioEasing::OutPowi(2)))
        .with_volume(0.05)
        .looped().handle();
}

