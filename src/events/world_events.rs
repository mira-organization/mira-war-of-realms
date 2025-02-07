use bevy::prelude::*;

pub struct WorldEvents;

impl Plugin for WorldEvents {
    fn build(&self, app: &mut App) {
        app.add_event::<WorldPlayerHitWorldEnemyEvent>();
    }
}

#[derive(Event)]
pub struct WorldPlayerHitWorldEnemyEvent {
    pub player: Entity,
    pub enemy: Entity,
    pub location: Vec3,
}