use std::f32::consts::PI;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy_rapier3d::pipeline::QueryFilter;
use bevy_rapier3d::plugin::DefaultRapierContext;
use bevy_rapier3d::prelude::{RapierContextColliders, RapierQueryPipeline, RapierRigidBodySet};
use system::commons::WorldPlayer;
use system::config::ConfigService;
use system::states::{GameState, InGameState};
use system::utils::key_code::convert;
use crate::camera::CameraController;

/// Plugin for handling player camera logic including mouse rotation, zoom, and cursor locking.
pub struct CameraLogicPlugin;

impl Plugin for CameraLogicPlugin {
    fn build(&self, app: &mut App) {
        // Add systems for camera rotation, zoom, and cursor toggle, with conditions based on cursor lock state.
        app.add_systems(PreUpdate, rotation_mouse.run_if(cursor_lock_condition));
        app.add_systems(Update, zoom_mouse.run_if(cursor_lock_condition));
        app.add_systems(Update, toggle_cursor);
    }
}

/// System to handle camera rotation based on mouse input.
///
/// # Parameters
/// - `window_query`: Query to access the main window for window size.
/// - `camera_query`: Query to access the camera and its controller for rotation adjustments.
/// - `player_query`: Query to access the player entity and transform for position reference.
/// - `rapier_query`: Query to access the physics context to detect obstacles for camera collision detection.
/// - `mouse_events`: EventReader for mouse motion events to determine mouse movement.
fn rotation_mouse(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<(&CameraController, &mut Transform), With<CameraController>>,
    player_query: Query<(Entity, &Transform), (With<WorldPlayer>, Without<CameraController>)>,
    rapier_query: Query<(&RapierQueryPipeline, &RapierContextColliders, &RapierRigidBodySet), With<DefaultRapierContext>>,
    mut mouse_events: EventReader<MouseMotion>,
    current_state: Res<State<GameState>>,
) {
    if current_state.eq(&GameState::InGame(InGameState::Battle)) {
        return;
    }

    // Calculate rotation based on mouse movement.
    let mut rotation = Vec2::ZERO;
    for event in mouse_events.read() {
        rotation += event.delta;
    }

    // Fetch camera, player, and rapier context components.
    let Ok((camera, mut camera_transform)) = camera_query.get_single_mut() else { return; };
    let Ok((player_entity, player_transform)) = player_query.get_single() else { return; };
    let Ok((rapier_context, colliders, body_set)) = rapier_query.get_single() else { return; };
    let window = window_query.get_single().unwrap();

    // Adjust rotation based on mouse movement and camera sensitivity.
    rotation *= camera.sensitivity;
    let delta_x = (rotation.x / window.width()) * PI * camera.sensitivity.x;
    let delta_y = (rotation.y / window.height()) * PI * camera.sensitivity.y;

    // Apply yaw and pitch changes based on mouse delta.
    let (mut yaw, mut pitch, _) = camera_transform.rotation.to_euler(EulerRot::YXZ);
    yaw -= delta_x;
    pitch = (pitch - delta_y).clamp(-PI / 2.5, PI / 3.6);

    // Set the new rotation for the camera.
    camera_transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);

    // Calculate offset and desired translation for the camera.
    let rotation_matrix = Mat3::from_quat(camera_transform.rotation);
    let offset = rotation_matrix.mul_vec3(Vec3::new(camera.offset.offset.0, camera.offset.offset.1, 0.0));
    let player_position = player_transform.translation + Vec3::Y * 0.8;
    let desired_translation = player_position + rotation_matrix.mul_vec3(Vec3::new(0.0, 0.0, camera.zoom.radius)) + offset;
    let mut final_translation = desired_translation;

    // Adjust target distance for collision detection.
    let mut target_distance = camera.zoom.radius;
    if pitch <= -PI / 2.4 {
        target_distance *= 0.8;
    }

    // Check for collisions between the camera and the world (to avoid clipping through objects).
    if let Some((_hit_entity, hit)) = rapier_context.cast_ray_and_get_normal(
        colliders,
        body_set,
        player_position,
        (final_translation - player_position).normalize(),
        camera.zoom.max,
        true,
        QueryFilter::default().exclude_collider(player_entity),
    ) {
        target_distance = hit.time_of_impact as f32 - 0.2;
    }

    // Adjust final camera position based on collision checks.
    final_translation = player_position + camera_transform.forward() * -target_distance;

    // Ensure the camera stays above the ground.
    if let Some((_hit_entity, floor_hit)) = rapier_context.cast_ray_and_get_normal(
        colliders,
        body_set,
        player_position + Vec3::Y * 0.5,
        Vec3::NEG_Y,
        1.75,
        true,
        QueryFilter::default().exclude_collider(player_entity),
    ) {
        final_translation.y = final_translation.y.max(floor_hit.point.y + 0.35);
    }

    // Ensure the camera doesn't go too close to the player.
    if target_distance <= camera.zoom.min + 0.1 && final_translation.y < player_position.y - 0.4 {
        let test_rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch + 0.02, 0.0);
        let test_translation = player_position + test_rotation.mul_vec3(Vec3::new(0.0, 0.0, -target_distance));

        if rapier_context.cast_ray(
            colliders,
            body_set,
            test_translation,
            (player_position - test_translation).normalize(),
            (player_position - test_translation).length() - 1.5,
            true,
            QueryFilter::default().exclude_collider(player_entity),
        ).is_none() {
            camera_transform.rotation = test_rotation;
        }
    }

    // Adjust vertical camera position if too close to the player.
    let distance_to_player = (final_translation - player_position).length();
    if distance_to_player < camera.zoom.offset_swap && final_translation.y > player_position.y - 0.4 {
        final_translation.y += 0.6;
    }

    // Final collision check to ensure camera doesn't pass through walls.
    if let Some((_hit_entity, _)) = rapier_context.cast_ray(
        colliders,
        body_set,
        player_position,
        (final_translation - player_position).normalize(),
        target_distance + 2.25,
        true,
        QueryFilter::default().exclude_collider(player_entity),
    ) {
        final_translation = player_position + (final_translation - player_position).normalize() * (target_distance - 0.1);
    }

    // Set final camera position.
    camera_transform.translation = final_translation;
}

/// System to handle zoom functionality based on mouse scroll input.
///
/// # Parameters
/// - `scroll_event`: EventReader for mouse wheel scroll events to adjust zoom.
/// - `camera_query`: Query to access the camera controller for zoom adjustments.
fn zoom_mouse(
    mut scroll_event: EventReader<MouseWheel>,
    mut camera_query: Query<&mut CameraController>
) {
    // Calculate scroll input.
    let mut scroll = 0.0;
    for event in scroll_event.read() {
        scroll += event.y;
    }

    // Adjust camera zoom based on scroll input.
    if let Ok(mut camera) = camera_query.get_single_mut() {
        if scroll.abs() > 0.0 {
            let target_radius = (camera.zoom.target_radius
                - scroll * camera.zoom.target_radius * 0.1 * camera.zoom.zoom_sensitivity)
                .clamp(camera.zoom.min, camera.zoom.max);

            camera.zoom.target_radius = target_radius;
        }

        // Smoothly transition to the new zoom radius.
        camera.zoom.radius += (camera.zoom.target_radius - camera.zoom.radius) * 0.15;
    }
}

/// System to toggle cursor lock when a specific key is pressed.
///
/// # Parameters
/// - `camera_query`: Query to access the camera controller for lock state.
/// - `keys`: Resource for button input to detect key presses.
/// - `window_query`: Query to access the window for cursor visibility and grab mode.
/// - `general_config`: Resource for configuration settings including cursor lock button.
fn toggle_cursor(
    mut camera_query: Query<&mut CameraController>,
    keys: Res<ButtonInput<KeyCode>>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    general_config: Res<ConfigService>,
    current_state: Res<State<GameState>>,
) {
    // Fetch the camera controller and the key to toggle cursor lock.
    let Ok(mut camera) = camera_query.get_single_mut() else { return; };
    let lock_key = convert(general_config.input_config.cursor_lock_button.as_str()).expect("Fetch key for (cursor lock) was failed!");

    // Toggle the lock state on key press.
    if keys.just_pressed(lock_key) {
        camera.lock_active = !camera.lock_active;
    }

    // Update window settings based on cursor lock state.
    if let Ok(mut window) = window_query.get_single_mut() {
        if current_state.eq(&GameState::InGame(InGameState::Battle)) {
            window.cursor_options.visible = true;
            return;
        }

        if camera.lock_active {
            window.cursor_options.grab_mode = CursorGrabMode::Locked;
            window.cursor_options.visible = false;
        } else {
            window.cursor_options.grab_mode = CursorGrabMode::None;
            window.cursor_options.visible = true;
        }
    }
}

/// Condition to check if the camera cursor is locked.
///
/// # Parameters
/// - `camera`: Query to access the camera controller for the lock state.
pub fn cursor_lock_condition(camera: Query<&CameraController>) -> bool {
    let Ok(camera) = camera.get_single() else { return true };
    camera.lock_active
}
