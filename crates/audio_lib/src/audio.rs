use bevy::prelude::*;
use bevy_kira_audio::DynamicAudioChannels;
use system::states::{in_game_states, GameState, InGameState};
use crate::{AudioManager, AudioType};
use crate::audio_control::AudioOption;

pub struct AudioHandlerPlugin;

/// The `AudioHandlerPlugin` is responsible for setting up and starting audio when the game enters the "InGame" state.
/// It adds a system call that gets executed when the game state transitions to `InGame`.
impl Plugin for AudioHandlerPlugin {
    fn build(&self, app: &mut App) {
        // Adds a system that runs when entering the "InGame" state
        app.add_systems(OnEnter(GameState::EnvironmentPostLoad), setup);
        app.add_systems(OnEnter(GameState::InGame(InGameState::Battle)), battle_music);
        app.add_systems(OnEnter(GameState::InGame(InGameState::BattleEnd)), setup);
        app.add_systems(Update, change_volume.run_if(in_game_states));
    }
}

/// The `setup` function is called when the game state switches to "InGame".
/// Here, the audio for environmental sounds is added and played.
fn setup(asset_server: Res<AssetServer>,
         mut audio: ResMut<DynamicAudioChannels>,
         mut audio_manager: ResMut<AudioManager>,
         option: Res<AudioOption>
) {
    if audio_manager.contains_channel("battle_ch") {
        audio_manager.stop_channel("battle_ch", &mut audio);
    }
    // Add the environmental audio with the name "environment_test" and type `AudioType::Environment`.
    // The track "audio/ambient.ogg" is loaded and played.
    if !audio_manager.contains_channel("environment_test") {
        audio_manager.add_audio("environment_test", AudioType::Environment, "audio/ambient.ogg", &mut audio, &asset_server, &option);
    } else {
        audio_manager.resume_channel("environment_test", &mut audio);
    }
}

fn battle_music(asset_server: Res<AssetServer>,
                mut audio: ResMut<DynamicAudioChannels>,
                mut audio_manager: ResMut<AudioManager>,
                option: Res<AudioOption>
) {
    if audio_manager.contains_channel("environment_test") {
        audio_manager.pause_channel("environment_test", &mut audio);
    }

    if !audio_manager.contains_channel("battle_ch") {
        audio_manager.add_audio("battle_ch", AudioType::Environment, "audio/battle.ogg", &mut audio, &asset_server, &option);
    } else {
        audio_manager.play_channel("battle_ch", &mut audio, &option);
    }
}

/// This is only temporary, after finishing settings ui we can delete this code!
pub fn change_volume(
    mut config: ResMut<AudioOption>,
    input: Res<ButtonInput<KeyCode>>,
    mut audio: ResMut<DynamicAudioChannels>,
    audio_manager: Res<AudioManager>,
) {
    if input.just_pressed(KeyCode::NumpadAdd) {
        let current_volume = *config.volumes.get("environment").unwrap_or(&0.5);
        config.set_category_volume("environment", current_volume + 0.05, &mut audio, &audio_manager);

    } else if input.just_pressed(KeyCode::NumpadSubtract) {
        let current_volume = *config.volumes.get("environment").unwrap_or(&0.5);
        config.set_category_volume("environment", current_volume - 0.05, &mut audio, &audio_manager);
    }

    if input.just_pressed(KeyCode::ArrowUp) {
        let current_volume = config.master_volume;
        config.set_master_volume(current_volume + 0.05, &mut audio, &audio_manager);
    }

    if input.just_pressed(KeyCode::ArrowDown) {
        let current_volume = config.master_volume;
        config.set_master_volume(current_volume - 0.05, &mut audio, &audio_manager);
    }
}