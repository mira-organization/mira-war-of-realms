#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use bevy::render::settings::{Backends, WgpuFeatures};
    use bevy::window::WindowResolution;
    use crate::game_window::create_gpu_settings;

    #[test]
    fn test_window_settings_values() {
        let title = "Test Window";
        let width = 800.0;
        let height = 600.0;

        let win = Window {
            title: title.to_string(),
            resolution: WindowResolution::new(width, height),
            ..default()
        };

        assert_eq!(win.title, title);
        assert_eq!(win.resolution.width(), width);
        assert_eq!(win.resolution.height(), height);
    }

    #[test]
    fn test_gpu_settings_values() {
        let settings = create_gpu_settings();
        assert_eq!(settings.features, WgpuFeatures::POLYGON_MODE_LINE);
        assert_eq!(settings.backends, Some(Backends::PRIMARY));
    }
}