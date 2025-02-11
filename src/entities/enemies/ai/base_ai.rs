use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, Velocity};
use rand::Rng;
use crate::entities::enemies::ai::{AiSetup, AiState};
use crate::entities::enemies::WorldEnemy;
use crate::entities::WorldPlayer;
use crate::manager::GameState;
use crate::service::attack_service::spawn_attack_hit_box;

pub struct BaseAI;

/// Plugin that manages the base AI behavior for enemies in the game.
/// It adds the logic system that runs on each fixed update during the `InGame` state.
impl Plugin for BaseAI {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, logic_system.run_if(in_state(GameState::InGame)));
    }
}

/// System that handles the AI behavior for enemies. The AI follows different states
/// like Idle, Walking, Alert, Observing, Aggressive, and Attacking, based on the player’s position
/// and other conditions. The AI will move towards the player if detected and perform attacks when in range.
///
/// # Arguments
/// - `commands`: A mutable reference to `Commands` to spawn entities like attack hit_boxes.
/// - `query`: A set of queries to get enemy data (transform, setup, velocity) and player position.
/// - `time`: A resource containing the delta time (used for timers and movement speed).
fn logic_system(
    mut commands: Commands,
    mut query: ParamSet<(
        Query<(Entity, &mut Transform, &mut AiSetup, &mut Velocity), With<WorldEnemy>>,
        Query<&Transform, With<WorldPlayer>>)>,
    time: Res<Time>
) {
    let player_transform = query.p1().get_single().ok().map(|t| t.translation);

    // Iterate through each enemy entity and apply AI logic
    for (entity, mut transform, mut setup, mut velocity) in query.p0().iter_mut() {
        if let Some(player_pos) = player_transform {
            // Calculate direction and distance to the player
            let direction_to_player = (player_pos - transform.translation).normalize_or_zero();
            let forward = transform.forward();
            let angle = forward.dot(direction_to_player).acos();
            let distance = transform.translation.distance(player_pos);
            let player_in_cone = angle < setup.detection_cone_angle && distance < setup.detection_range;

            match setup.state {
                AiState::Idle => {
                    if player_in_cone {
                        setup.state = AiState::Alert;
                        setup.alert_timer = 0.0;
                        continue;
                    }

                    setup.idle_timer -= time.delta_secs();
                    velocity.linvel = Vec3::new(0.0, velocity.linvel.y, 0.0);
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

                    // Move towards the next path point
                    let target = setup.path[setup.current_path_index];
                    let direction = (target - transform.translation).normalize_or_zero();
                    let speed = 2.0;
                    velocity.linvel = Vec3::new(direction.x * speed, velocity.linvel.y, direction.z * speed);
                    if transform.translation.distance(target) < 0.5 {
                        setup.current_path_index = (setup.current_path_index + 1) % setup.path.len();
                        setup.state = AiState::Idle;
                    }

                    if !setup.path.is_empty() {
                        let target = setup.path[setup.current_path_index];
                        let direction = (target - transform.translation).normalize_or_zero();
                        transform.look_to(direction, Vec3::Y);
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
                AiState::Aggressive => {
                    if distance > setup.aggression_range {
                        setup.state = AiState::Observing;
                        setup.observing_timer = 5.0;
                        setup.alert_timer = 0.0;
                    } else {
                        if transform.translation.distance(player_pos) > 2.0 {
                            velocity.linvel = Vec3::new(direction_to_player.x * 4.0, velocity.linvel.y, direction_to_player.z * 4.0);
                            transform.look_to(direction_to_player, Vec3::Y);
                            continue;
                        }
                        setup.state = AiState::Attacking;
                    }
                }
                AiState::Attacking => {
                    // Spawn an attack hit_box when the AI is attacking
                    spawn_attack_hit_box(
                        &mut commands,
                        entity,
                        Collider::ball(2.0),
                        Transform::from_translation(Vec3::ZERO),
                        Some(Color::srgb_u8(255, 0, 155)),
                        0.05
                    );
                    setup.state = AiState::Observing;
                }
            }
        }
    }
}