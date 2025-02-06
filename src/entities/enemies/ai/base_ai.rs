use bevy::prelude::*;
use bevy_rapier3d::prelude::Velocity;
use rand::Rng;
use crate::entities::enemies::ai::{AiSetup, AiState};
use crate::entities::enemies::WorldEnemy;
use crate::entities::WorldPlayer;
use crate::manager::GameState;

pub struct BaseAI;

impl Plugin for BaseAI {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, logic_system.run_if(in_state(GameState::InGame)));
    }
}

fn logic_system(mut query: Query<(&mut AiSetup, &mut Velocity, &Transform), With<WorldEnemy>>,
                player_query: Query<&Transform, With<WorldPlayer>>,
                time: Res<Time>
) {
    for (mut setup, mut velocity, transform) in query.iter_mut() {
        if let Ok(player_transform) = player_query.get_single() {
            let direction_to_player = (player_transform.translation - transform.translation).normalize_or_zero();
            let forward = transform.forward();
            let angle = forward.dot(direction_to_player).acos();
            let distance = transform.translation.distance(player_transform.translation);
            let player_in_cone = angle < setup.detection_cone_angle && distance < setup.detection_range;

            match setup.state {
                AiState::Idle => {
                    setup.idle_timer -= time.delta_secs();
                    velocity.linvel = Vec3::ZERO;
                    if setup.idle_timer <= 0.0 {
                        setup.state = AiState::Walking;
                        setup.idle_timer = rand::thread_rng().gen_range(2.0..6.0);
                    }
                }
                AiState::Walking => {
                    if setup.path.is_empty() {
                        setup.state = AiState::Idle;
                        continue;
                    }

                    if player_in_cone {
                        setup.state = AiState::Alert;
                        setup.alert_timer = 0.0;
                        continue;
                    }

                    let target = setup.path[setup.current_path_index];
                    let direction = (target - transform.translation).normalize_or_zero();
                    let speed = 2.0;
                    velocity.linvel = direction * speed;
                    if transform.translation.distance(target) < 0.5 {
                        setup.current_path_index = (setup.current_path_index + 1) % setup.path.len();
                        setup.state = AiState::Idle;
                    }
                }
                AiState::Alert => {
                    if player_in_cone {
                        setup.alert_timer += time.delta_secs();
                        if setup.alert_timer > 3.0 {
                            setup.state = AiState::Aggressive;
                        }
                    } else {
                        setup.state = AiState::Observing;
                        setup.observing_timer = 5.0;
                    }
                }
                AiState::Observing => {
                    setup.observing_timer -= time.delta_secs();
                    if setup.observing_timer <= 0.0 {
                        setup.state = AiState::Walking;
                    }
                }
                _ => {}
            }
        }
    }
}