use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use system::commons::WorldPlayer;
use crate::camera::CameraController;

pub struct CameraLogicPlugin;

impl Plugin for CameraLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, sync_entity_with_camera.before(TransformSystem::TransformPropagate));
        app.add_systems(Update, toggle_cursor);
    }
}

fn sync_entity_with_camera(mut camera_query: Query<(&mut CameraController, &mut Transform), With<CameraController>>,
                           player_query: Query<&Transform, (With<WorldPlayer>, Without<CameraController>)>,
) {
    let Ok(player_transform) = player_query.get_single() else { return; };
    let Ok((camera_controller, mut camera_transform)) = camera_query.get_single_mut() else { return; };

    let rotation_matrix = Mat3::from_quat(camera_transform.rotation);

    let offset =
        rotation_matrix.mul_vec3(Vec3::new(camera_controller.offset.offset.0, camera_controller.offset.offset.1, 0.0));

    let desired_translation = rotation_matrix.mul_vec3(Vec3::new(0.0, 0.0, camera_controller.zoom.radius)) + offset;
    camera_transform.translation = desired_translation + player_transform.translation;
}

fn toggle_cursor(mut camera_query: Query<&mut CameraController>,
                 keys: Res<ButtonInput<KeyCode>>,
                 mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let Ok(mut camera) = camera_query.get_single_mut() else { return; };

    if keys.just_pressed(KeyCode::Escape) {
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