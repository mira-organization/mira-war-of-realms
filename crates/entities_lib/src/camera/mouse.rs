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

fn rotation_mouse(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<(&CameraController, &mut Transform), With<PlayerWorldCamera>>,
    mut mouse_events: EventReader<MouseMotion>
) {
    let mut rotation = Vec2::ZERO;
    for event in mouse_events.read() {
        rotation += event.delta;
    }

    let Ok((camera, mut camera_transform)) = camera_query.get_single_mut() else {
        return;
    };

    rotation *= camera.sensitivity;
    if rotation.length_squared() > 0.0 {
        let window = window_query.get_single().unwrap();
        let delta_x = (rotation.x / window.width()) * PI * camera.sensitivity.x;
        let delta_y = (rotation.y / window.height()) * PI * camera.sensitivity.y;

        let (mut yaw, mut pitch, _) = camera_transform.rotation.to_euler(EulerRot::YXZ);

        yaw -= delta_x;
        pitch = (pitch - delta_y).clamp(-PI / 2.5, PI / 2.5);

        camera_transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
    }

    let rotation_matrix = Mat3::from_quat(camera_transform.rotation);
    camera_transform.translation = rotation_matrix.mul_vec3(Vec3::new(0.0, 0.0, camera.zoom.radius));
}



fn zoom_mouse(
    mut scroll_event: EventReader<MouseWheel>,
    mut camera_query: Query<&mut CameraController>
) {
    let mut scroll = 0.0;
    for event in scroll_event.read() {
        scroll += event.y;
    }

    if let Ok(mut camera) = camera_query.get_single_mut() {
        if scroll.abs() > 0.0 {
            let target_radius = (camera.zoom.target_radius
                - scroll * camera.zoom.target_radius * 0.1 * camera.zoom.zoom_sensitivity)
                .clamp(camera.zoom.min, camera.zoom.max);

            camera.zoom.target_radius = target_radius;
        }

        camera.zoom.radius += (camera.zoom.target_radius - camera.zoom.radius) * 0.15;
    }
}


