pub mod player_events;
pub mod world_events;

use bevy::prelude::*;
use crate::events::player_events::PlayerEvents;
use crate::events::world_events::WorldEvents;

/// The `EventManagerPlugin` is responsible for managing and registering event-related plugins.
/// It ensures that all necessary event systems are included in the application.
pub struct EventManagerPlugin;

impl Plugin for EventManagerPlugin {
    /// Registers event-related plugins.
    ///
    /// - `PlayerEvents`: Handles player-specific events.
    /// - `WorldEvents`: Manages world-related events.
    ///
    /// # Parameters
    /// - `app`: The Bevy application instance where the plugins are added.
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerEvents, WorldEvents));
    }
}
