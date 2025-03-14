mod entities_setup;

use bevy::prelude::*;
use crate::battle::entities_setup::BattleEntitiesPlugin;

/// A plugin that manages the battle environment.
///
/// This plugin adds essential sub-plugins for battle interactions.
pub struct BattleEnvironmentPlugin;

impl Plugin for BattleEnvironmentPlugin {
    /// Registers the required plugins for the battle environment.
    ///
    /// # Parameters
    /// - `app`: The Bevy app where the plugin is registered.
    fn build(&self, app: &mut App) {
        // Enables mesh-based interaction handling
        app.add_plugins(MeshPickingPlugin);

        // Adds battle-related entities
        app.add_plugins(BattleEntitiesPlugin);
    }
}

/// Handles mouse click events on interactive objects.
///
/// Logs the entity that was clicked.
///
/// # Parameters
/// - `event`: The event containing information about the clicked entity.
fn on_mouse_click(event: Trigger<Pointer<Click>>) {
    info!("Clicked on pointer {:?}", event.entity());
}

/// Handles mouse hover enter events on interactive objects.
///
/// Logs when the cursor enters an entity's interactive area.
///
/// # Parameters
/// - `event`: The event containing information about the entity being hovered over.
fn on_mouse_enter(event: Trigger<Pointer<Over>>) {
    info!("Entered pointer {:?}", event.entity());
}

/// Handles mouse hover leave events on interactive objects.
///
/// Logs when the cursor leaves an entity's interactive area.
///
/// # Parameters
/// - `event`: The event containing information about the entity being left.
fn on_mouse_leave(event: Trigger<Pointer<Out>>) {
    info!("Leaving pointer {:?}", event.entity());
}
