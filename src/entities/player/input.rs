use bevy::prelude::*;
use bevy_rapier3d::prelude::{KinematicCharacterController};
use crate::entities::player::PlayerWorldCamera;
use crate::entities::WorldPlayer;
use crate::events::player_events::PlayerActionEvent;
use crate::manager::GameState;

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (fetch_keyboard_input, update_movement).run_if(in_state(GameState::InGame)));
    }
}

fn fetch_keyboard_input(mut input_event_writer: EventWriter<PlayerActionEvent>,
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

fn update_movement(time: Res<Time>,
                   mut controllers: Query<(&mut KinematicCharacterController, &mut Transform, &WorldPlayer), With<WorldPlayer>>,
                   mut input_event_reader: EventReader<PlayerActionEvent>) {
    for event in input_event_reader.read() {
        for (mut controller, mut transform, world_player) in controllers.iter_mut() {
            match event {
                PlayerActionEvent::Move(direction) => {
                    if direction.length_squared() > 0.0 {
                        let flat_direction = Vec3::new(direction.x, 0.0, direction.z).normalize();
                        let target_rotation = Quat::from_rotation_arc(-Vec3::Z, flat_direction);
                        transform.rotation = transform.rotation.slerp(target_rotation, 0.1);
                        controller.translation = Some((direction * world_player.walk_speed) * time.delta_secs());
                    }
                }

                PlayerActionEvent::Sprinting(direction) => {
                    if direction.length_squared() > 0.0 {
                        let flat_direction = Vec3::new(direction.x, 0.0, direction.z).normalize();
                        let target_rotation = Quat::from_rotation_arc(-Vec3::Z, flat_direction);
                        transform.rotation = transform.rotation.slerp(target_rotation, 0.1);
                        controller.translation = Some((direction * world_player.sprinting_speed) * time.delta_secs());
                    }
                }

                PlayerActionEvent::Idle => {
                    controller.translation = None;
                }
            }
        }
    }
}