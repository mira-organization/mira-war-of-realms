pub mod audio;
mod audio_control;

use std::time::Duration;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_kira_audio::*;
use system::config::ConfigService;
use crate::audio_control::AudioOption;

pub struct AudioStorePlugin;

/// The `AudioStorePlugin` manages audio resources and handles the setup of the `AudioManager`.
impl Plugin for AudioStorePlugin {
    fn build(&self, app: &mut App) {
        // Inserts the `AudioManager` resource into the app, which handles all audio functionality
        app.insert_resource(AudioOption::new());
        app.insert_resource(AudioManager::new());
        app.add_plugins(AudioPlugin);
        app.add_systems(Startup, load_up_audio_config);
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
    pub muted: bool,                      // A flag indicating whether audio is muted
    pub audio: HashMap<String, AudioType>,  // A map of active audio channels by name and type
    pub audio_handle: HashMap<String, Handle<AudioSource>>,
}

#[allow(dead_code)]
impl AudioManager {
    pub fn new() -> Self {
        Self {
            muted: false,
            audio: HashMap::new(),
            audio_handle: HashMap::new()
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
                     asset_server: &AssetServer,
                     option: &AudioOption
    ) {
        // Check if the audio is already in use
        if self.audio.contains_key(channel_name) {
            warn!("Audio already in use: {}", channel_name);
            return;
        }

        // Check if the audio should be looped based on its type
        let looped = self.looped_time(audio_type.clone());
        let volume = self.load_correct_volume(audio_type.clone(), option);

        let handle = asset_server.load::<AudioSource>(track_path);

        // Create a new audio channel and play the track
        let mut binding = audio_kira_handle.create_channel(channel_name)
            .play(handle.clone());

        // Apply fade-in effect and set initial volume
        let build = binding
            .fade_in(AudioTween::new(Duration::from_secs(2), AudioEasing::Linear))
            .with_volume(volume);

        // If the audio should loop, apply the loop effect
        if looped { build.looped(); }

        // Insert the audio into the manager's map
        self.audio.insert(channel_name.to_string(), audio_type);
        self.audio_handle.insert(channel_name.to_string(), handle.clone());
        info!("AudioManager added to {}", channel_name);
    }

    /// Removes an audio track from the audio manager and stops it.
    /// If the track is not currently in use, a warning is displayed.
    pub fn remove_audio(&mut self, channel_name: &str, audio_kira_handle: &mut DynamicAudioChannels) {
        if !self.audio.contains_key(channel_name) {
            warn!("Audio not in use: {}", channel_name);
            return;
        }

        audio_kira_handle.remove_channel(channel_name);
        self.audio_handle.remove(channel_name);
        self.audio.remove(channel_name);
        info!("Audio removed: {}", channel_name);
    }

    /// Stops the audio track associated with the given channel name.
    /// A fade-out effect is applied before the track is stopped.
    pub fn stop_channel(&mut self, channel_name: &str, audio_kira_handle: &mut DynamicAudioChannels) {
        if !self.audio.contains_key(channel_name) || audio_kira_handle.get_channel(channel_name).is_none() {
            warn!("Audio not in use: {}", channel_name);
            return;
        }

        // Apply fade-out effect before stopping the audio
        audio_kira_handle.channel(channel_name).stop().fade_out(AudioTween::new(Duration::from_secs(2), AudioEasing::Linear));
        info!("Stopped audio {}", channel_name);
    }

    pub fn pause_channel(&mut self, channel_name: &str, audio_kira_handle: &mut DynamicAudioChannels) {
        if !self.audio.contains_key(channel_name) || audio_kira_handle.get_channel(channel_name).is_none() {
            warn!("Audio not in use: {}", channel_name);
            return;
        }

        // Apply fade-out effect before stopping the audio
        audio_kira_handle.channel(channel_name).pause().fade_out(AudioTween::new(Duration::from_secs(2), AudioEasing::Linear));
        info!("Paused audio {}", channel_name);
    }

    /// Resumes playback of an audio track associated with the given channel name.
    /// A fade-in effect is applied as the track resumes.
    pub fn play_channel(&mut self, channel_name: &str, audio_kira_handle: &mut DynamicAudioChannels, option: &AudioOption) {
        if !self.audio.contains_key(channel_name) {
            warn!("Audio not in use: {}", channel_name);
            return;
        }

        if let Some(handle) = self.audio_handle.get(channel_name) {
            let audio_type = self.audio.get(channel_name).unwrap();
            let looped = self.looped_time(audio_type.clone());
            let volume = self.load_correct_volume(audio_type.clone(), option);

            let mut binding = audio_kira_handle.create_channel(channel_name)
                .play(handle.clone());

            // Apply fade-in effect and set initial volume
            let build = binding
                .fade_in(AudioTween::new(Duration::from_secs(2), AudioEasing::Linear))
                .with_volume(volume);

            // If the audio should loop, apply the loop effect
            if looped { build.looped(); }

            info!("Play audio {}", channel_name);
        }
    }

    pub fn resume_channel(&mut self, channel_name: &str, audio_kira_handle: &mut DynamicAudioChannels) {
        if !self.audio.contains_key(channel_name) {
            warn!("Audio not in use: {}", channel_name);
            return;
        }

        // Resume the audio with a fade-in effect
        audio_kira_handle.channel(channel_name).resume().fade_in(AudioTween::new(Duration::from_secs(2), AudioEasing::Linear));
        info!("Resume audio {}", channel_name);
    }

    /// Determines if the audio type should be looped based on its category (e.g., Environment, Battle).
    fn looped_time(&self, audio_type: AudioType) -> bool {
        match audio_type {
            AudioType::Environment | AudioType::Battle => true,  // Loop environmental and battle tracks
            _ => false,  // Other types of audio (e.g., SFX, UI) do not loop
        }
    }

    fn load_correct_volume(&self, audio_type: AudioType, audio_option: &AudioOption) -> f64 {
        match audio_type {
            AudioType::Environment | AudioType::Battle => *audio_option.volumes.get("environment").unwrap_or(&1.0),
            AudioType::Sfx => *audio_option.volumes.get("sfx").unwrap_or(&1.0),
            AudioType::Ui => *audio_option.volumes.get("ui").unwrap_or(&1.0),
            AudioType::Character => *audio_option.volumes.get("character").unwrap_or(&1.0),
            AudioType::Unknown => 0.1,
        }
    }

    fn contains_channel(&self, channel_name: &str) -> bool {
        self.audio.contains_key(channel_name)
    }
}

fn load_up_audio_config(config: Res<ConfigService>, mut audio_option: ResMut<AudioOption>) {
    audio_option.initialize(&config);
}