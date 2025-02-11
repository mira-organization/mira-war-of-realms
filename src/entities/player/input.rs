use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::ThirdPersonCamera;
use crate::entities::player::{LastStableGround, PlayerWorldCamera};
use crate::entities::{WorldPlayer, WorldPlayerState};
use crate::events::player_events::PlayerActionEvent;
use crate::manager::{ConfigService, GameState, PLAYER_VOID_THRESHOLD};
use crate::service::attack_service::spawn_attack_hit_box;
use crate::utils::key_code::convert;

/// A plugin that handles player input and movement behavior.
///
/// This plugin captures keyboard inputs and translates them into movement actions,
/// which are then used to update the player's state and movement in the game world.
pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    /// Configures the application to add systems for handling player input and movement,
    /// which are only active during the `GameState::InGame` state.
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (fetch_keyboard_input,
                                 update_movement,
                                 input_attack,
                                 limit_camera_pitch,
                                 track_stable_ground,
                                 check_void_fall
        ).run_if(in_state(GameState::InGame)));
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
    general_config: Res<ConfigService>,
) {

    if let Ok(camera_transform) = camera_query.get_single() {
        let forward_key = convert(general_config.input_config.player_up.as_str()).expect("Fetch key for (forward) was failed!");
        let backward_key = convert(general_config.input_config.player_down.as_str()).expect("Fetch key for (backward) was failed!");
        let left_key = convert(general_config.input_config.player_left.as_str()).expect("Fetch key for (left) was failed!");
        let right_key = convert(general_config.input_config.player_right.as_str()).expect("Fetch key for (right) was failed!");

        let sprint_key = convert(general_config.input_config.player_sprint.as_str()).expect("Fetch key for (sprinting) was failed!");

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

                PlayerActionEvent::Idle | PlayerActionEvent::Attacking => {
                    controller.translation = None;
                    world_player.state = WorldPlayerState::Idle;
                }
            }
        }
    }
}

fn input_attack(mouse_input: Res<ButtonInput<MouseButton>>,
                commands: Commands,
                query: Query<Entity, With<WorldPlayer>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Ok(player) = query.get_single() {
            spawn_attack_hit_box(
                commands,
                player,
                Collider::ball(0.75),
                Transform::from_translation(Vec3::new(0.0, 0.8, 0.0)),
                0.05
            );
        }
    }
}

fn limit_camera_pitch(mut query: Query<&mut Transform, With<ThirdPersonCamera>>) {
    for mut transform in query.iter_mut() {
        let (yaw, mut pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);

        let min_pitch: f32 = -std::f32::consts::FRAC_PI_2;
        let max_pitch: f32 = 123.123;

        pitch = pitch.clamp(min_pitch, max_pitch);
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

fn track_stable_ground(mut players: Query<&Transform, With<WorldPlayer>>,
                       mut last_stable_ground: ResMut<LastStableGround>
) {
    for transform in players.iter_mut() {
        if transform.translation.y >= -0.001 {
            last_stable_ground.0 = transform.translation;
        }
    }
}

fn check_void_fall(mut players: Query<&mut Transform, With<WorldPlayer>>,
                   mut last_stable_ground: ResMut<LastStableGround>
) {
    for mut transform in players.iter_mut() {
        if transform.translation.y < PLAYER_VOID_THRESHOLD {
            transform.translation = last_stable_ground.0;
            info!("Player was send to {:?}", last_stable_ground.0);
        }
    }
}