mod base_ai;

use bevy::prelude::*;
use rand::Rng;
use crate::enemies::ai::base_ai::BaseAI;

pub struct AiPlugin;

/// The `AiPlugin` is responsible for adding AI functionality to the game.
impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        // Registers the `AiSetup` component type to make it available in the world
        app.register_type::<AiSetup>();

        // Adds the `BaseAI` plugin, which contains the AI logic system
        app.add_plugins(BaseAI);
    }
}

/// The `AiState` enum defines the various states the AI can be in.
/// Each state represents a different phase of AI behavior.
#[derive(Reflect, Debug, Clone, Default, PartialEq, Eq)]
pub enum AiState {
    #[default]
    Idle,      // The AI is idle, not interacting with the player or moving
    Walking,   // The AI is walking to a specific point or following a path
    Observing, // The AI is observing its surroundings
    Alert,     // The AI is aware of the player's presence
    Aggressive,// The AI actively seeks to engage or attack the player
    Attacking  // The AI performs an attack action
}

/// The `AiSetup` component holds data that controls AI behavior and state transitions.
/// It tracks the AI's current state, path, timers, and detection capabilities.
#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct AiSetup {
    pub state: AiState,                 // Current state of the AI (Idle, Walking, etc.)
    pub path: Vec<Vec3>,                // Path the AI is following (if in Walking state)
    pub current_path_index: usize,     // The index of the current target point in the path
    pub idle_timer: f32,               // Timer for how long the AI stays idle
    pub alert_timer: f32,              // Timer to track how long the AI remains alert
    pub observing_timer: f32,          // Timer for how long the AI observes its surroundings
    pub detection_cone_angle: f32,     // Angle of the AI's detection cone (in radians)
    pub detection_range: f32,          // Maximum distance for the AI to detect the player
    pub aggression_range: f32,         // Distance at which the AI becomes aggressive toward the player
}

impl Default for AiSetup {
    fn default() -> Self {
        let mut rng = rand::rng();
        Self {
            state: AiState::default(),    // Start with the default state (Idle)
            path: Vec::new(),             // No path defined initially
            current_path_index: 0,       // Start at the first path index
            idle_timer: rng.random_range(2.0..5.0), // Random idle time between 2 and 5 seconds
            alert_timer: 0.0,            // Alert timer starts at 0
            observing_timer: 5.0,        // Observing timer starts at 5 seconds
            detection_cone_angle: 45.0_f32.to_radians(), // Default detection cone angle (45°)
            detection_range: 10.0,       // Default detection range of 10 units
            aggression_range: 15.0       // Default aggression range of 15 units
        }
    }
}