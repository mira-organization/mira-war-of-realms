use bevy::prelude::*;
use bevy_kira_audio::{DynamicAudioChannels};
use crate::audio::{AudioManager, AudioType};
use crate::manager::{GameState, InGameState};

pub struct AudioHandlerPlugin;

/// The `AudioHandlerPlugin` is responsible for setting up and starting audio when the game enters the "InGame" state.
/// It adds a system call that gets executed when the game state transitions to `InGame`.
impl Plugin for AudioHandlerPlugin {
    fn build(&self, app: &mut App) {
        // Adds a system that runs when entering the "InGame" state
        app.add_systems(OnEnter(GameState::InGame(InGameState::Main)), setup);
    }
}

/// The `setup` function is called when the game state switches to "InGame".
/// Here, the audio for environmental sounds is added and played.
fn setup(asset_server: Res<AssetServer>, mut audio: ResMut<DynamicAudioChannels>, mut audio_manager: ResMut<AudioManager>) {
    // Add the environmental audio with the name "environment_test" and type `AudioType::Environment`.
    // The track "audio/env_test.ogg" is loaded and played.
    audio_manager.add_audio("environment_test", AudioType::Environment, "audio/env_test.ogg", &mut audio, &asset_server);
}
