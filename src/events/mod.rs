pub mod player_events;
pub mod world_events;

use bevy::prelude::*;
use crate::events::player_events::PlayerEvents;
use crate::events::world_events::WorldEvents;

pub struct EventManagerPlugin;

impl Plugin for EventManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerEvents, WorldEvents));
    }
}