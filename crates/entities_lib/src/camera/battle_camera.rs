use bevy::prelude::*;
use system::battle_commons::{BattleSelectedStatus, TurnCurrentMemberInfo};
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
pub fn update_camera_movement(
    mut camera_query: Query<(&CameraController, &mut Transform), (With<CameraController>, Without<Character>)>,
    character_query: Query<(Entity, &Character, &Transform), Without<CameraController>>,
    enemy_query: Query<(Entity, &Transform), (With<Enemy>, Without<Character>, Without<CameraController>)>,
    turn_info: Res<TurnCurrentMemberInfo>,
    selected_status: Res<BattleSelectedStatus>,
    time: Res<Time>,
) {
    let Ok((_camera, mut camera_transform)) = camera_query.get_single_mut() else { return; };

    let Some(active_character) = turn_info.character.as_ref() else { return; };

    let Some((_active_entity, _, player_transform)) = character_query
        .iter()
        .find(|(_, character, _)| character.name == active_character.name && character.lastname == active_character.lastname)
    else { return; };

    // Collect enemy positions and optionally get selected enemy position
    let mut enemy_positions: Vec<Vec3> = Vec::new();
    let mut selected_enemy_pos: Option<Vec3> = None;

    for (entity, transform) in enemy_query.iter() {
        if let Some((_, selected_entity)) = selected_status.selected {
            if entity == selected_entity {
                selected_enemy_pos = Some(transform.translation);
            }
        }
        enemy_positions.push(transform.translation);
    }

    if enemy_positions.is_empty() { return; }

    // Calculate the center of all enemies
    let enemy_center = enemy_positions.iter().sum::<Vec3>() / enemy_positions.len() as f32;

    // Calculate base camera offset
    let base_offset = Vec3::new(0.15, -1.8, -1.5);
    let distance_factor = 0.95 + (enemy_positions.len() as f32 * 0.15);
    let camera_offset = base_offset * distance_factor;

    // Camera position is always based on player position
    let target_position = player_transform.translation - camera_offset;

    // Determine where the camera should look at
    let look_target = if let Some(selected_pos) = selected_enemy_pos {
        // If a specific enemy is selected, look more toward them
        player_transform.translation.lerp(selected_pos + Vec3::Y * 1.5, 0.75)
    } else {
        // Otherwise look toward the general center of enemies
        player_transform.translation.lerp(enemy_center + Vec3::Y * 1.5, 0.65)
    };

    // Direction the camera should face
    let mut direction = (look_target - target_position).normalize();

    // Clamp vertical tilt
    direction.y = direction.y.clamp(-0.3, 0.5);

    // Create target rotation
    let mut target_rotation = Quat::from_rotation_arc(Vec3::NEG_Z, direction);

    // Add slight yaw offset for style
    let yaw_offset = Quat::from_rotation_y(12.0_f32.to_radians());
    target_rotation = yaw_offset * target_rotation;

    // Fix camera flip if upside down
    let up = target_rotation * Vec3::Y;
    let fixed_rotation = if up.y < 0.0 {
        target_rotation * Quat::from_rotation_y(std::f32::consts::PI)
    } else {
        target_rotation
    };

    // Interpolate position and rotation smoothly
    let interpolation_speed = 5.5 * time.delta_secs();
    camera_transform.translation = camera_transform.translation.lerp(target_position, interpolation_speed);
    camera_transform.rotation = camera_transform.rotation.slerp(fixed_rotation, interpolation_speed);
}




