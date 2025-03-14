use bevy::prelude::*;

/// Represents the status of a battle entity, including whether it is currently selected.
///
/// This component is used to track the selection state of characters and enemies in battle.
/// It supports reflection for debugging and UI tools.
#[derive(Component, Reflect, Clone, Debug)]
#[reflect(Component)]
pub struct BattleEntityStatus {
    /// Indicates whether the entity is currently selected.
    pub selected: bool,
}

impl Default for BattleEntityStatus {
    /// Provides a default instance where `selected` is set to `false`.
    ///
    /// # Returns
    /// A `BattleEntityStatus` instance with `selected` set to `false`.
    fn default() -> Self {
        Self {
            selected: false
        }
    }
}

/// Marker component indicating that an entity is currently in battle.
///
/// This component is used to filter entities that are active participants in a battle.
#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct InBattle;


/// Represents an enemy entity specifically in a battle scenario.
///
/// This struct is designed to store data relevant to an enemy's behavior and attributes during a battle.
#[derive(Component, Resource, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct BattleEnemy {
    // Fields for battle-specific attributes and logic can be added here.
    // For example, health, attack power, or battle-specific abilities.
}