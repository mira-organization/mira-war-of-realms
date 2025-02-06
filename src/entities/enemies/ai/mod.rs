mod base_ai;

use bevy::prelude::*;
use rand::Rng;
use crate::entities::enemies::ai::base_ai::BaseAI;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BaseAI);
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum AiState {
    #[default]
    Idle,
    Walking,
    Observing,
    Alert,
    Aggressive,
    Attacking
}

#[derive(Component, Debug, Clone)]
pub struct AiSetup {
    pub state: AiState,
    pub path: Vec<Vec3>,
    pub current_path_index: usize,
    pub idle_timer: f32,
}

impl Default for AiSetup {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            state: AiState::default(),
            path: Vec::new(),
            current_path_index: 0,
            idle_timer: rng.gen_range(2.0..6.0)
        }
    }
}