use bevy::prelude::*;

pub struct WorldEvents;

impl Plugin for WorldEvents {
    fn build(&self, app: &mut App) {
        app.add_event::<WorldEntityHitEntityEvent>();
    }
}

#[derive(Event)]
pub struct WorldEntityHitEntityEvent {
    pub sender: Entity,
    pub entity: Entity,
}