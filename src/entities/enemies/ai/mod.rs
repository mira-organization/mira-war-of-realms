mod base_ai;

use bevy::prelude::*;
use rand::Rng;
use crate::entities::enemies::ai::base_ai::BaseAI;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AiSetup>();
        app.add_plugins(BaseAI);
    }
}

#[derive(Reflect, Debug, Clone, Default, PartialEq, Eq)]
pub enum AiState {
    #[default]
    Idle,
    Walking,
    Observing,
    Alert,
    Aggressive,
    Attacking
}

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct AiSetup {
    pub state: AiState,
    pub path: Vec<Vec3>,
    pub current_path_index: usize,
    pub idle_timer: f32,
    pub alert_timer: f32,
    pub observing_timer: f32,
    pub detection_cone_angle: f32,
    pub detection_range: f32,
}

impl Default for AiSetup {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            state: AiState::default(),
            path: Vec::new(),
            current_path_index: 0,
            idle_timer: rng.gen_range(2.0..5.0),
            alert_timer: 0.0,
            observing_timer: 5.0,
            detection_cone_angle: 45.0_f32.to_radians(),
            detection_range: 10.0,
        }
    }
}