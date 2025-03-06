use std::f32::consts::PI;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier3d::prelude::{DefaultRapierContext, QueryFilter, RapierContextColliders, RapierQueryPipeline, RapierRigidBodySet};
use system::commons::WorldPlayer;
use crate::camera::CameraController;
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
    mut camera_query: Query<(&CameraController, &mut Transform), With<CameraController>>,
    player_query: Query<(Entity, &Transform), (With<WorldPlayer>, Without<CameraController>)>,
    rapier_query: Query<(&RapierQueryPipeline, &RapierContextColliders, &RapierRigidBodySet), With<DefaultRapierContext>>,
    mut mouse_events: EventReader<MouseMotion>,
) {
    let mut rotation = Vec2::ZERO;
    for event in mouse_events.read() {
        rotation = event.delta;
    }

    let Ok((camera, mut camera_transform)) = camera_query.get_single_mut() else { return; };
    let Ok((player_entity, player_transform)) = player_query.get_single() else { return; };
    let Ok((rapier_context, colliders, body_set)) = rapier_query.get_single() else { return; };

    rotation *= camera.sensitivity;
    let window = window_query.get_single().unwrap();

    let delta_x = (rotation.x / window.width()) * PI * camera.sensitivity.x;
    let delta_y = (rotation.y / window.height()) * PI * camera.sensitivity.y;

    let (mut yaw, mut pitch, _) = camera_transform.rotation.to_euler(EulerRot::YXZ);

    yaw -= delta_x;

    let max_pitch = PI / 3.6;
    let min_pitch = -PI / 2.5;
    pitch = (pitch - delta_y).clamp(min_pitch, max_pitch);

    camera_transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);

    let rotation_matrix = Mat3::from_quat(camera_transform.rotation);
    let offset = rotation_matrix.mul_vec3(Vec3::new(camera.offset.offset.0, camera.offset.offset.1, 0.0));

    let desired_translation = rotation_matrix.mul_vec3(Vec3::new(0.0, 0.0, camera.zoom.radius)) + offset;
    let player_position = player_transform.translation + Vec3::Y * 0.8;
    let mut final_translation = player_position + desired_translation;

    let max_distance = camera.zoom.max;
    let min_distance = camera.zoom.min;

    let mut target_distance = camera.zoom.radius;

    if pitch <= min_pitch + 0.1 {
        let zoom_factor = ((min_pitch - pitch) / (min_pitch - (-max_pitch))).clamp(0.0, 1.0);
        target_distance = camera.zoom.min + (camera.zoom.radius - camera.zoom.min) * (1.0 - zoom_factor);
    }

    if let Some((_hit_entity, hit)) = rapier_context.cast_ray_and_get_normal(
        colliders,
        body_set,
        player_position,
        (final_translation - player_position).normalize(),
        max_distance,
        true,
        QueryFilter::default().exclude_collider(player_entity),
    ) {
        let hit_distance = hit.time_of_impact as f32;
        if hit_distance > 0.1 {
            target_distance = hit_distance - 0.2;
        }
    }

    final_translation = player_position + camera_transform.forward() * -target_distance;

    let floor_check_origin = player_position + Vec3::Y * 0.5;
    let floor_check_dir = Vec3::NEG_Y;
    let floor_check_distance = 1.5;

    if let Some((_hit_entity, floor_hit)) = rapier_context.cast_ray_and_get_normal(
        colliders,
        body_set,
        floor_check_origin,
        floor_check_dir,
        floor_check_distance,
        true,
        QueryFilter::default().exclude_collider(player_entity),
    ) {
        final_translation.y = final_translation.y.max(floor_hit.point.y + 0.5);
    }

    if target_distance <= min_distance + 0.2 {
        let test_rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch + 0.02, 0.0);
        let test_translation = player_position + test_rotation.mul_vec3(Vec3::new(0.0, 0.0, -target_distance));

        let mut can_rotate = true;
        if let Some((_hit_entity, _)) = rapier_context.cast_ray(
            colliders,
            body_set,
            test_translation,
            (player_position - test_translation).normalize(),
            (player_position - test_translation).length(),
            true,
            QueryFilter::default().exclude_collider(player_entity),
        ) {
            can_rotate = false;
        }

        if can_rotate {
            camera_transform.rotation = test_rotation;
        }
    }

    let wall_check_direction = (final_translation - player_position).normalize();
    let max_wall_check_distance = target_distance + 1.0;
    let collision_check_distance = target_distance - 0.1;

    if let Some((_hit_entity, _)) = rapier_context.cast_ray(
        colliders,
        body_set,
        player_position,
        wall_check_direction,
        max_wall_check_distance,
        true,
        QueryFilter::default().exclude_collider(player_entity),
    ) {
        final_translation = player_position + wall_check_direction * collision_check_distance;
    }

    camera_transform.translation = final_translation;
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

        if camera.zoom.radius <= camera.zoom.offset_swap {
            camera.offset.offset.1 = 0.9;
        } else {
            camera.offset.offset.1 = 0.6;
        }
    }
}


