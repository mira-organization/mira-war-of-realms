use bevy::prelude::*;
use system::battle_commons::{BattleCurrentEntities};
use system::commons::{Character, Enemy};
use system::states::{GameState, InGameState};
use crate::camera::CameraController;

/// A plugin for the battle camera.
///
/// This plugin adds a system that updates the camera during battles.
pub struct BattleCameraPlugin;

impl Plugin for BattleCameraPlugin {
    /// Registers the `update_camera_movement` system when the game is in the `Battle` state.
    ///
    /// # Parameters
    /// - `app`: The Bevy app where the plugin is registered.
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_camera_movement.run_if(in_state(GameState::InGame(InGameState::Battle))));
    }
}

/// Updates the camera position and rotation during a battle.
///
/// The camera is set to a fixed position and slightly tilted downward to provide a clear battle overview.
///
/// # Parameters
/// - `camera_query`: Query for the `CameraController` component and its associated `Transform`.
fn update_camera_movement(
    mut camera_query: Query<(&CameraController, &mut Transform), (With<CameraController>, Without<Character>)>,
    character_query: Query<&Transform, (With<Character>, Without<CameraController>)>,
    battle_entities: Res<BattleCurrentEntities>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Character>, Without<CameraController>)>,
    time: Res<Time>
) {
    let Ok((_camera, mut camera_transform)) = camera_query.get_single_mut() else { return; };
    let Some((&_char_index, &active_entity)) = battle_entities
        .characters
        .iter()
        .find(|&(_, &entity)| character_query.contains(entity))
    else { return; };
    let Ok(player_transform) = character_query.get(active_entity) else { return; };

    let enemy_positions: Vec<Vec3> = enemy_query.iter().map(|t| t.translation).collect();
    if enemy_positions.is_empty() { return; }

    let enemy_center = enemy_positions.iter().sum::<Vec3>() / enemy_positions.len() as f32;

    let camera_offset = Vec3::new(-2.0, -1.3, -1.5);
    let target_position = player_transform.translation - camera_offset;

    let look_target = enemy_center + Vec3::new(0.0, 1.4, 0.0);
    let mut direction = (look_target - target_position).normalize();

    // Limit vertical camera tilt to avoid extreme angles
    direction.y = direction.y.clamp(-0.5, 0.5);

    // Base rotation to look at the target
    let mut target_rotation = Quat::from_rotation_arc(Vec3::NEG_Z, direction);

    // Apply a slight 33° yaw rotation to the left
    let yaw_offset = Quat::from_rotation_y(-15.5_f32.to_radians());
    target_rotation = yaw_offset * target_rotation;

    // Ensure the camera is not upside-down
    let up = target_rotation * Vec3::Y;
    let fixed_rotation = if up.y < 0.0 {
        target_rotation * Quat::from_rotation_y(std::f32::consts::PI)
    } else {
        target_rotation
    };

    // Smooth interpolation for camera movement and rotation
    let interpolation_speed = 5.0 * time.delta_secs();
    camera_transform.translation = camera_transform.translation.lerp(target_position, interpolation_speed);
    camera_transform.rotation = camera_transform.rotation.slerp(fixed_rotation, interpolation_speed);



/*    // Set the camera to a fixed position
    camera_transform.translation = player_transform.translation + Vec3::new(0.0, 1.0, 0.0);

    // Slightly tilt the camera downward
    let pitch = -10.0_f32.to_radians();
    let yaw = 10.0_f32.to_radians();

    camera_transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);*/
}


