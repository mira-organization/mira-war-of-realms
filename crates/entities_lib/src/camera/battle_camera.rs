use bevy::prelude::*;
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
    mut camera_query: Query<(&CameraController, &mut Transform), With<CameraController>>,
) {
    let Ok((_camera, mut camera_transform)) = camera_query.get_single_mut() else { return; };

    // Set the camera to a fixed position
    camera_transform.translation = Vec3::new(-7.5, 55.0, 30.0);

    // Slightly tilt the camera downward
    let pitch = -20.0_f32.to_radians();
    camera_transform.rotation = Quat::from_rotation_x(pitch);
}


