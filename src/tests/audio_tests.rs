#[cfg(test)]
mod tests {
    use bevy::input::ButtonInput;
    use bevy::prelude::*;
    use bevy_kira_audio::{AudioSource, DynamicAudioChannels};
    use audio_lib::audio::{battle_music, change_volume, setup};
    use audio_lib::audio_control::AudioOption;
    use audio_lib::{load_up_audio_config, AudioManager, AudioType};
    use system::config::ConfigService;

    #[test]
    fn test_change_volume_master() {
        let mut app = App::new();
        app.init_resource::<AudioOption>()
            .insert_resource(AudioManager::new())
            .init_resource::<DynamicAudioChannels>()
            .insert_resource(ButtonInput::<KeyCode>::default());

        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::ArrowUp);
        app.update();

        app.add_systems(Update, change_volume);
        app.update();

        let audio_option = app.world().get_resource::<AudioOption>().unwrap();
        assert_eq!(audio_option.master_volume, 1.0, "The master volume should be 1.0");

        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().clear();
        app.update();

        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::ArrowDown);
        app.update();

        app.add_systems(Update, change_volume);
        app.update();

        let audio_option = app.world().get_resource::<AudioOption>().unwrap();
        assert_eq!(audio_option.master_volume, 0.85, "The master volume should be 0.85");
    }

    #[test]
    fn test_change_volume_category() {
        let mut app = App::new();
        app.init_resource::<AudioOption>()
            .insert_resource(AudioManager::new())
            .init_resource::<DynamicAudioChannels>()
            .init_resource::<ConfigService>()
            .insert_resource(ButtonInput::<KeyCode>::default());

        app.add_systems(Startup, load_up_audio_config);
        app.update();

        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::NumpadAdd);
        app.update();

        app.add_systems(Update, change_volume);
        app.update();

        let audio_option = app.world().get_resource::<AudioOption>().unwrap();
        assert_eq!(audio_option.volumes.get("environment").unwrap_or(&0.0), &1.0, "The environment volume should be 1.0");

        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().clear();
        app.update();

        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::NumpadSubtract);
        app.update();

        app.add_systems(Update, change_volume);
        app.update();

        let audio_option = app.world().get_resource::<AudioOption>().unwrap();
        assert_eq!(audio_option.volumes.get("environment").unwrap_or(&0.0), &0.85, "The environment volume should be 0.85");
    }

    #[test]
    fn test_setup_environment_audio() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()));
        let _ = app.world_mut().resource::<AssetServer>();
        // Setup mock resources

        app.init_asset::<AudioSource>();
        app.insert_resource(AudioManager::new());
        app.insert_resource(DynamicAudioChannels::default());
        app.insert_resource(AudioOption::default());

        // Insert fake asset server (we don't load real files in unit tests)

        // Add and run system
        app.add_systems(Startup, setup);
        app.update();

        let audio_manager = app.world().resource::<AudioManager>();

        // Check: environment_test should now exist
        assert_eq!(audio_manager.contains_channel("environment_test"), true);

        // Check: battle_ch should not exist
        assert_eq!(!audio_manager.contains_channel("battle_ch"), true);
    }

    #[test]
    fn test_battle_music_switches_audio() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()));
        let asset_server = app.world_mut().resource::<AssetServer>().clone();
        app.init_asset::<AudioSource>();

        // Prepare resources
        let mut audio_manager = AudioManager::new();
        let mut audio_channels = DynamicAudioChannels::default();

        audio_manager.add_audio(
            "environment_test",
            AudioType::Environment,
            "audio/ambient.ogg",
            &mut audio_channels,
            &asset_server,
            &AudioOption::default()
        );

        // Insert into world
        app.insert_resource(audio_manager);
        app.insert_resource(audio_channels);
        app.insert_resource(AudioOption::default());

        // Add and run system
        app.add_systems(Update, battle_music);
        app.update();

        let audio_manager = app.world().resource::<AudioManager>();

        // Check: environment_test is still registered
        assert!(audio_manager.contains_channel("environment_test"));

        // Check: battle_ch is added and playing
        assert!(audio_manager.contains_channel("battle_ch"));
    }
}