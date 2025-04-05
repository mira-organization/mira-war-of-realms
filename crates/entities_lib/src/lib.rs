pub mod player;
pub mod enemies;
mod camera;

use bevy::prelude::*;
use bevy_atmosphere::prelude::AtmospherePlugin;
use bevy_mod_outline::{AutoGenerateOutlineNormalsPlugin, OutlinePlugin};
use system::characters::CharacterParty;
use system::commons::{AccountPlayer, Character, Elements, Enemy, WorldPlayer};
use crate::enemies::ai::AiPlugin;
use crate::enemies::EnemiesPlugin;
use crate::player::PlayerPlugin;

/// The `EntitiesPlugin` plugin is responsible for registering and adding various components and plugins related to entities in the game.
///
/// This plugin registers several types for reflection and adds multiple plugins, such as the `PlayerPlugin`, `EnemiesPlugin`, and `AiPlugin`.
/// It is responsible for setting up and managing entity-related systems, such as player and enemy entities.
///
/// # Example
/// This plugin is used to handle entity creation and registration of player, enemy, and other related components in the game.
pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    /// Registers types for reflection and adds necessary plugins for managing entities.
    ///
    /// # Arguments
    /// * `app` - The Bevy app to which the types and plugins are added.
    fn build(&self, app: &mut App) {
        // Registering types for reflection, which enables dynamic access to components
        app.register_type::<AccountPlayer>();
        app.register_type::<WorldPlayer>();
        app.register_type::<Character>();
        app.register_type::<Enemy>();
        app.register_type::<CharacterParty>();
        app.register_type::<Elements>();
        app.add_plugins((AtmospherePlugin, OutlinePlugin, AutoGenerateOutlineNormalsPlugin::default()));

        // Adding additional plugins for player, enemies, and AI management
        app.add_plugins((PlayerPlugin, EnemiesPlugin, AiPlugin));
    }
}