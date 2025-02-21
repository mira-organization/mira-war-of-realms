pub mod audio;

use std::time::Duration;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_kira_audio::*;

pub struct AudioStorePlugin;

/// The `AudioStorePlugin` manages audio resources and handles the setup of the `AudioManager`.
impl Plugin for AudioStorePlugin {
    fn build(&self, app: &mut App) {
        // Inserts the `AudioManager` resource into the app, which handles all audio functionality
        app.insert_resource(AudioManager::new());
        app.add_plugins(AudioPlugin);
        info!("Crate audio was loaded successfully!");
    }
}

/// The `AudioType` enum defines the different categories of audio in the game.
/// It is used to differentiate between various types of audio resources such as environment, battle, and UI sounds.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
#[allow(dead_code)]
pub enum AudioType {
    Environment,  // For background environmental sounds
    Battle,       // For sounds related to battle sequences
    Sfx,          // For sound effects (e.g., explosions, footsteps)
    Ui,           // For UI sounds (e.g., button clicks, notifications)
    Character,    // For character-specific voice lines or sounds
    #[default]
    Unknown,      // Default audio type when the type is unknown
}

/// The `AudioManager` resource holds all audio-related settings and tracks that are currently playing.
/// It includes volume controls for various audio categories and manages the active audio channels.
#[derive(Resource)]
#[allow(dead_code)]
pub struct AudioManager {
    pub master_volume: f32,               // The overall master volume for the game
    pub ui_volume: f32,                   // The volume for UI-related sounds
    pub sfx_volume: f32,                  // The volume for sound effects
    pub character_voice_volume: f32,      // The volume for character voice lines
    pub environment_volume: f32,          // The volume for environmental sounds
    pub muted: bool,                      // A flag indicating whether audio is muted
    pub audio: HashMap<String, AudioType>  // A map of active audio channels by name and type
}

#[allow(dead_code)]
impl AudioManager {
    /// Creates a new instance of `AudioManager` with default values.
    /// The volumes for different audio categories are set to predefined levels, and no audio is initially playing.
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

    /// Adds a new audio track to the audio manager, associating it with a channel name and audio type.
    /// If the track is already in use, it is not added again.
    /// The track is played with a fade-in effect, and certain audio types may loop.
    pub fn add_audio(&mut self,
                     channel_name: &str,
                     audio_type: AudioType,
                     track_path: &str,
                     audio_kira_handle: &mut DynamicAudioChannels,
                     asset_server: &AssetServer
    ) {
        // Check if the audio is already in use
        if self.audio.contains_key(channel_name) {
            warn!("Audio already in use: {}", channel_name);
            return;
        }

        // Check if the audio should be looped based on its type
        let looped = self.looped_time(audio_type.clone());

        // Create a new audio channel and play the track
        let mut binding = audio_kira_handle.create_channel(channel_name)
            .play(asset_server.load(track_path));

        // Apply fade-in effect and set initial volume
        let build = binding
            .fade_in(AudioTween::new(Duration::from_secs(2), AudioEasing::OutPowi(2)))
            .with_volume(0.05);

        // If the audio should loop, apply the loop effect
        if looped { build.looped(); }

        // Insert the audio into the manager's map
        self.audio.insert(channel_name.to_string(), audio_type);
        debug!("AudioManager added to {}", channel_name);
    }

    /// Removes an audio track from the audio manager and stops it.
    /// If the track is not currently in use, a warning is displayed.
    pub fn remove_audio(&mut self, channel_name: &str, audio_kira_handle: &mut DynamicAudioChannels) {
        if !self.audio.contains_key(channel_name) {
            warn!("Audio not in use: {}", channel_name);
            return;
        }

        // Stop the audio and remove it from the channel list
        self.stop_channel(channel_name, audio_kira_handle);
        audio_kira_handle.remove_channel(channel_name);
        self.audio.remove(channel_name);
        debug!("Audio removed: {}", channel_name);
    }

    /// Stops the audio track associated with the given channel name.
    /// A fade-out effect is applied before the track is stopped.
    pub fn stop_channel(&mut self, channel_name: &str, audio_kira_handle: &mut DynamicAudioChannels) {
        if !self.audio.contains_key(channel_name) {
            warn!("Audio not in use: {}", channel_name);
            return;
        }

        // Apply fade-out effect before stopping the audio
        audio_kira_handle.channel(channel_name).stop().fade_out(AudioTween::new(Duration::from_secs(2), AudioEasing::InPowi(2)));
        debug!("Stopped audio {}", channel_name);
    }

    /// Resumes playback of an audio track associated with the given channel name.
    /// A fade-in effect is applied as the track resumes.
    pub fn play_channel(&mut self, channel_name: &str, audio_kira_handle: &mut DynamicAudioChannels) {
        if !self.audio.contains_key(channel_name) {
            warn!("Audio not in use: {}", channel_name);
            return;
        }

        // Resume the audio with a fade-in effect
        audio_kira_handle.channel(channel_name).resume().fade_in(AudioTween::new(Duration::from_secs(2), AudioEasing::InPowi(2)));
        debug!("Resume audio {}", channel_name);
    }

    /// Determines if the audio type should be looped based on its category (e.g., Environment, Battle).
    fn looped_time(&self, audio_type: AudioType) -> bool {
        match audio_type {
            AudioType::Environment | AudioType::Battle => true,  // Loop environmental and battle tracks
            _ => false,  // Other types of audio (e.g., SFX, UI) do not loop
        }
    }
}

/// The `EnvironmentAudio` component stores information about the environment audio tracks.
/// It includes the base track and the battle track for the environment, which are used for different gameplay situations.
#[derive(Component, Resource, Debug)]
#[allow(dead_code)]
pub struct EnvironmentAudio {
    pub base_track: String,  // The default ambient track for the environment
    pub battle_track: String,  // The track played during battle or combat situations
}