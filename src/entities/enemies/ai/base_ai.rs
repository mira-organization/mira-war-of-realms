use bevy::prelude::*;
use bevy_rapier3d::prelude::Velocity;
use rand::Rng;
use crate::entities::enemies::ai::{AiSetup, AiState};
use crate::entities::enemies::WorldEnemy;
use crate::manager::GameState;

pub struct BaseAI;

impl Plugin for BaseAI {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, logic_system.run_if(in_state(GameState::InGame)));
    }
}

fn logic_system(mut query: Query<(&mut AiSetup, &mut Velocity, &Transform), With<WorldEnemy>>,
                time: Res<Time>
) {
    for (mut setup, mut velocity, transform) in query.iter_mut() {
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

                let target = setup.path[setup.current_path_index];
                let direction = (target - transform.translation).normalize_or_zero();
                let speed = 2.0;
                velocity.linvel = direction * speed;
                if transform.translation.distance(target) < 0.5 {
                    setup.current_path_index = (setup.current_path_index + 1) % setup.path.len();
                    setup.state = AiState::Idle;
                }
            }
            _ => {}
        }
    }
}