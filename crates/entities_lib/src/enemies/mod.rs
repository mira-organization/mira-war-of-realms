mod test_enemy;
pub mod ai;

use bevy::prelude::*;
use system::commons::*;
use crate::enemies::test_enemy::TestEnemy;

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
        app.register_type::<EnemyState>();
        app.add_plugins(TestEnemy);
    }
}
