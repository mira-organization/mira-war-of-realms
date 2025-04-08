use bevy::prelude::*;

pub struct PlayerEvents;

/// The `PlayerEvents` plugin adds custom events related to player actions, such as movement, sprinting, and attacking.
///
/// This plugin registers the `PlayerActionEvent` enum, which will be used to track different types of player actions
/// in the game. The events can be listened to and handled by other systems in the game.
///
/// # Example
/// The event can be triggered when the player performs an action, such as moving or attacking.
impl Plugin for PlayerEvents {
    /// Registers the `PlayerActionEvent` as a custom event in the Bevy app.
    ///
    /// # Arguments
    /// * `app` - The Bevy app to which the event is added.
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerActionEvent>();
    }
}

/// Represents the different actions a player can perform.
///
/// This enum defines various states and actions for the player, including idle, movement, sprinting, and attacking.
///
/// # Variants
/// - `Idle`: Represents the player being idle (not moving or performing any actions).
/// - `Move(Vec3)`: Represents the player moving in 3D space. The `Vec3` is the direction and distance of movement.
/// - `Sprinting(Vec3)`: Represents the player sprinting. The `Vec3` is the direction and speed of sprinting.
/// - `Attacking`: Represents the player performing an attack action.
#[derive(Event, Debug, PartialEq)]
pub enum PlayerActionEvent {
    /// The player is idle and not performing any action.
    Idle,

    /// The player is moving. The `Vec3` represents the movement vector (direction and distance).
    Move(Vec3),

    /// The player is sprinting. The `Vec3` represents the direction and speed of the sprint.
    Sprinting(Vec3),

    /// The player is attacking.
    Attacking,
}
