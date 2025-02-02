use bevy::prelude::*;
use bevy_kira_audio::{DynamicAudioChannels};
use crate::audio::{AudioManager, AudioType};
use crate::manager::GameState;

pub struct AudioHandlerPlugin;

impl Plugin for AudioHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup);
    }
}

fn setup(asset_server: Res<AssetServer>, mut audio: ResMut<DynamicAudioChannels>, mut audio_manager: ResMut<AudioManager>) {
    audio_manager.add_audio("environment_test", AudioType::Environment, "audio/env_test.ogg", &mut audio, &asset_server);
    audio_manager.add_audio("sfx_test", AudioType::Sfx, "audio/sfx_test.ogg", &mut audio, &asset_server);
}