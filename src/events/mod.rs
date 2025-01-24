pub mod player_events;

use bevy::prelude::*;
use crate::events::player_events::PlayerEvents;

pub struct EventManagerPlugin;

impl Plugin for EventManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerEvents);
    }
}