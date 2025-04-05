pub mod input;
mod animation;

use bevy::prelude::*;
use crate::camera::{GameCameraPlugin, PlayerWorldCamera};
use crate::player::animation::PlayerAnimationPlugin;
use crate::player::input::PlayerInputPlugin;

/// A plugin for managing the player's systems, including input, animations,
/// and spawning the player entity and camera in the game world.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    /// Configures the application to add player-related plugins and systems.
    ///
    /// - Adds the `PlayerInputPlugin` and `PlayerAnimationPlugin`.
    /// - Registers systems for creating the player entity and player camera
    ///   when entering the `GameState::InGame` state.
    fn build(&self, app: &mut App) {
        app.insert_resource(LastStableGround(Vec3::ZERO));
        app.add_plugins((GameCameraPlugin, PlayerInputPlugin, PlayerAnimationPlugin));
    }
}

#[derive(Resource, Debug, Default)]
pub struct LastStableGround(pub Vec3);