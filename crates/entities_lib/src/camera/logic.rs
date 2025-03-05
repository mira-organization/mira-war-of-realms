use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use system::config::ConfigService;
use system::utils::key_code::convert;
use crate::camera::CameraController;

pub struct CameraLogicPlugin;

impl Plugin for CameraLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_cursor);
    }
}

fn toggle_cursor(mut camera_query: Query<&mut CameraController>,
                 keys: Res<ButtonInput<KeyCode>>,
                 mut window_query: Query<&mut Window, With<PrimaryWindow>>,
                 general_config: Res<ConfigService>
) {
    let Ok(mut camera) = camera_query.get_single_mut() else { return; };
    let lock_key = convert(general_config.input_config.cursor_lock_button.as_str()).expect("Fetch key for (cursor lock) was failed!");

    if keys.just_pressed(lock_key) {
        camera.lock_active = !camera.lock_active;
    }

    if let Ok(mut window) = window_query.get_single_mut() {
        if camera.lock_active {
            window.cursor_options.grab_mode = CursorGrabMode::Confined;
            window.cursor_options.visible = false;
        } else {
            window.cursor_options.grab_mode = CursorGrabMode::None;
            window.cursor_options.visible = true;
        }
    }
}

pub fn cursor_lock_condition(camera: Query<&CameraController>) -> bool {
    let Ok(camera) = camera.get_single() else { return true };
    camera.lock_active
}