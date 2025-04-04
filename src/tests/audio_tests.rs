#[cfg(test)]
mod tests {
    use bevy::input::ButtonInput;
    use bevy::prelude::*;
    use bevy_kira_audio::DynamicAudioChannels;
    use audio_lib::audio::change_volume;
    use audio_lib::audio_control::AudioOption;
    use audio_lib::{load_up_audio_config, AudioManager};
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
}