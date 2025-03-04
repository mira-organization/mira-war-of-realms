use std::f32::consts::PI;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::camera::{CameraController, PlayerWorldCamera};
use crate::camera::logic::cursor_lock_condition;

pub struct MouseCameraPlugin;

impl Plugin for MouseCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, rotation_mouse.run_if(cursor_lock_condition));
        app.add_systems(Update, zoom_mouse.run_if(cursor_lock_condition));
    }
}

fn rotation_mouse(window_query: Query<&Window, With<PrimaryWindow>>,
                      mut camera_query: Query<(&CameraController, &mut Transform), With<PlayerWorldCamera>>,
                      mut mouse_events: EventReader<MouseMotion>
) {
    let mut rotation = Vec2::ZERO;
    for event in mouse_events.read() {
        rotation = event.delta;
    }

    let Ok((camera, mut camera_transform)) = camera_query.get_single_mut() else {
        return;
    };

    rotation *= camera.sensitivity;
    if rotation.length_squared() > 0.0 {
        let window = window_query.get_single().unwrap();
        let delta_x = {
            let delta = rotation.x / window.width() * PI * camera.sensitivity.x;
            delta
        };

        let delta_y = rotation.y / window.height() * PI *  camera.sensitivity.y;
        let yaw = Quat::from_rotation_y(-delta_x);
        let pitch = Quat::from_rotation_x(-delta_y);
        camera_transform.rotation = yaw * camera_transform.rotation;

        let new_rotation = camera_transform.rotation * pitch;
        let up_vector = new_rotation * Vec3::Y;
        if up_vector.y > 0.0 {
            camera_transform.rotation = new_rotation;
        }
    }

    let rotation_matrix = Mat3::from_quat(camera_transform.rotation);
    camera_transform.translation = rotation_matrix.mul_vec3(Vec3::new(0.0, 0.0, camera.zoom.radius));
}

fn zoom_mouse(mut scroll_event: EventReader<MouseWheel>, mut camera: Query<&mut CameraController>) {
    let mut scroll = 0.0;
    for event in scroll_event.read() {
        scroll += event.y;
    }

    if let Ok(mut camera) = camera.get_single_mut() {
        if scroll.abs() > 0.0 {
            let new_radius =
                camera.zoom.radius - scroll * camera.zoom.radius * 0.1 * camera.zoom.zoom_sensitivity;
            camera.zoom.radius = new_radius.clamp(camera.zoom.min, camera.zoom.max);
        }
    }
}