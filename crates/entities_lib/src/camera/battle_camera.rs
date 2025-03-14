use bevy::prelude::*;
use system::states::{GameState, InGameState};
use crate::camera::CameraController;

pub struct BattleCameraPlugin;

impl Plugin for BattleCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_camera_movement.run_if(in_state(GameState::InGame(InGameState::Battle))));
    }
}

fn update_camera_movement(
    mut camera_query: Query<(&CameraController, &mut Transform), With<CameraController>>,
) {
    let Ok((_camera, mut camera_transform)) = camera_query.get_single_mut() else { return; };
    camera_transform.translation = Vec3::new(-7.5, 55.0, 30.0);
    let pitch = -20.0_f32.to_radians();
    camera_transform.rotation = Quat::from_rotation_x(pitch);
}

