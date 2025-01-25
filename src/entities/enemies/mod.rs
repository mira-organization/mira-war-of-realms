mod test_enemy;

use bevy::prelude::*;
use crate::entities::Elements;
use crate::entities::enemies::test_enemy::TestEnemy;

/// A plugin for managing enemy entities in the game.
///
/// This plugin is responsible for registering enemy-related types and adding necessary sub-plugins.
pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    /// Configures the application to include enemy-related systems and components.
    ///
    /// - Registers `WorldEnemy`, `BattleEnemy`, and `EnemyState` types for reflection.
    /// - Adds the `TestEnemy` plugin for enemy testing or behavior simulation.
    fn build(&self, app: &mut App) {
        app.register_type::<WorldEnemy>();
        app.register_type::<BattleEnemy>();
        app.register_type::<EnemyState>();
        app.add_plugins(TestEnemy);
    }
}

/// Represents an enemy entity that exists in the overworld.
///
/// A `WorldEnemy` has a location, a list of possible battle configurations,
/// weaknesses to certain elements, and a current state.
#[derive(Component, Resource, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct WorldEnemy {
    /// The location of the enemy in the game world.
    /// If `None`, the enemy's position has not been initialized yet.
    pub location: Option<Vec3>,

    /// A list of `BattleEnemy` entities that represent the different forms or configurations
    /// this enemy can take in a battle scenario.
    pub enemy_list: Vec<BattleEnemy>,

    /// A collection of elements that this enemy is weak against, such as fire or water.
    pub weakness_elements: Vec<Elements>,

    /// The current state of the enemy, representing its behavior or activity level.
    pub state: EnemyState,
}

impl Default for WorldEnemy {
    /// Creates a default instance of `WorldEnemy` with:
    /// - No initial location.
    /// - An empty list of battle enemies.
    /// - No elemental weaknesses.
    /// - A default state of `EnemyState::Idling`.
    fn default() -> Self {
        Self {
            location: None,
            enemy_list: Vec::new(),
            weakness_elements: Vec::new(),
            state: Default::default(),
        }
    }
}

/// Represents an enemy entity specifically in a battle scenario.
///
/// This struct is designed to store data relevant to an enemy's behavior and attributes during a battle.
#[derive(Component, Resource, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct BattleEnemy {
    // Fields for battle-specific attributes and logic can be added here.
    // For example, health, attack power, or battle-specific abilities.
}

/// Represents the state or behavior of an enemy.
///
/// The `EnemyState` enum captures the different states an enemy can be in,
/// such as idle, alert, or attacking.
#[derive(Component, Resource, Reflect, Default, Debug, Clone)]
#[reflect(Component)]
pub enum EnemyState {
    /// The enemy is idle and not actively engaged with the player.
    #[default]
    Idling,

    /// The enemy is observing its surroundings, possibly detecting the player's presence.
    Observing,

    /// The enemy is walking or patrolling in the game world.
    Walking,

    /// The enemy is on high alert, likely aware of the player's presence.
    Alert,

    /// The enemy is actively attacking the player or another target.
    Attacking,
}
