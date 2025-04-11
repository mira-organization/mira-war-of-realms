use bevy::prelude::*;
use bevy_kira_audio::DynamicAudioChannels;
use system::states::GameState;
use crate::audio_control::AudioOption;
use crate::{AudioManager, AudioType};

pub struct TitleMusic;

impl Plugin for TitleMusic {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::TitleScreen), play_title_music);
        app.add_systems(OnEnter(GameState::EnvironmentLoad), stop_title_music);
    }
}

pub fn play_title_music(asset_server: Res<AssetServer>,
                        mut audio: ResMut<DynamicAudioChannels>,
                        mut audio_manager: ResMut<AudioManager>,
                        option: Res<AudioOption>
) {
    audio_manager.add_audio("title_music", AudioType::Environment, "audio/title.ogg", &mut audio, &asset_server, &option);
}

pub fn stop_title_music(mut audio: ResMut<DynamicAudioChannels>,
                        mut audio_manager: ResMut<AudioManager>
) {
    if audio_manager.contains_channel("title_music") {
        audio_manager.stop_channel("title_music", &mut audio);
    }
}