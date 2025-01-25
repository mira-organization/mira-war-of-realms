use bevy::prelude::*;
use bevy_rapier3d::prelude::{KinematicCharacterController};
use crate::entities::player::PlayerWorldCamera;
use crate::entities::{WorldPlayer, WorldPlayerState};
use crate::events::player_events::PlayerActionEvent;
use crate::manager::GameState;

/// A plugin that handles player input and movement behavior.
///
/// This plugin captures keyboard inputs and translates them into movement actions,
/// which are then used to update the player's state and movement in the game world.
pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    /// Configures the application to add systems for handling player input and movement,
    /// which are only active during the `GameState::InGame` state.
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (fetch_keyboard_input, update_movement).run_if(in_state(GameState::InGame)));
    }
}

/// Captures keyboard inputs and generates `PlayerActionEvent` events to represent the player's actions.
///
/// This system determines the player's movement direction and state (e.g., walking, sprinting, or idle)
/// based on the keys being pressed.
///
/// # Parameters
/// - `input_event_writer`: Used to emit player action events.
/// - `keyboard`: Access to keyboard input states.
/// - `camera_query`: Used to get the transform of the camera for directional movement.
fn fetch_keyboard_input(
    mut input_event_writer: EventWriter<PlayerActionEvent>,
    keyboard: Res<ButtonInput<KeyCode>>,
    camera_query: Query<&Transform, With<PlayerWorldCamera>>,
) {
    if let Ok(camera_transform) = camera_query.get_single() {
        let forward_key = KeyCode::KeyW;
        let backward_key = KeyCode::KeyS;
        let left_key = KeyCode::KeyA;
        let right_key = KeyCode::KeyD;

        let sprint_key = KeyCode::ShiftLeft;

        let mut direction = Vec3::ZERO;
        if keyboard.pressed(forward_key) {
            direction += Vec3::new(camera_transform.forward().x, direction.y, camera_transform.forward().z);
        }

        if keyboard.pressed(backward_key) {
            direction += Vec3::new(camera_transform.back().x, direction.y, camera_transform.back().z);
        }

        if keyboard.pressed(left_key) {
            direction += camera_transform.left().as_vec3();
        }

        if keyboard.pressed(right_key) {
            direction += camera_transform.right().as_vec3();
        }

        if direction.length_squared() > 0.0 {
            let normalized_direction = direction.normalize();
            if keyboard.pressed(forward_key) || keyboard.pressed(backward_key)
                || keyboard.pressed(left_key) || keyboard.pressed(right_key) {
                input_event_writer.send(PlayerActionEvent::Move(normalized_direction));
            } else {
                input_event_writer.send(PlayerActionEvent::Idle);
            }
        } else {
            input_event_writer.send(PlayerActionEvent::Idle);
        }

        if keyboard.pressed(sprint_key) {
            input_event_writer.send(PlayerActionEvent::Sprinting(direction.normalize()));
        }
    }
}

/// Updates the player's movement and state based on the received `PlayerActionEvent` events.
///
/// This system handles movement translation, rotation, and state transitions
/// for the player entity.
///
/// # Parameters
/// - `time`: Provides the delta time for frame-based updates.
/// - `controllers`: Query to access player controllers, transforms, and world player components.
/// - `input_event_reader`: Reads the player action events.
fn update_movement(
    time: Res<Time>,
    mut controllers: Query<(&mut KinematicCharacterController, &mut Transform, &mut WorldPlayer), With<WorldPlayer>>,
    mut input_event_reader: EventReader<PlayerActionEvent>,
) {
    for event in input_event_reader.read() {
        for (mut controller, mut transform, mut world_player) in controllers.iter_mut() {
            match event {
                PlayerActionEvent::Move(direction) => {
                    if direction.length_squared() > 0.0 {
                        let flat_direction = Vec3::new(direction.x, 0.0, direction.z).normalize();
                        let target_rotation = Quat::from_rotation_arc(-Vec3::Z, flat_direction);
                        transform.rotation = transform.rotation.slerp(target_rotation, 0.1);
                        controller.translation = Some((direction * world_player.walk_speed) * time.delta_secs());
                        world_player.state = WorldPlayerState::Walking;
                    }
                }

                PlayerActionEvent::Sprinting(direction) => {
                    if direction.length_squared() > 0.0 {
                        let flat_direction = Vec3::new(direction.x, 0.0, direction.z).normalize();
                        let target_rotation = Quat::from_rotation_arc(-Vec3::Z, flat_direction);
                        transform.rotation = transform.rotation.slerp(target_rotation, 0.1);
                        controller.translation = Some((direction * world_player.sprinting_speed) * time.delta_secs());
                        world_player.state = WorldPlayerState::Sprinting;
                    }
                }

                PlayerActionEvent::Idle => {
                    controller.translation = None;
                    world_player.state = WorldPlayerState::Idle;
                }
            }
        }
    }
}