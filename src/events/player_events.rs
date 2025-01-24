use bevy::prelude::*;

pub struct PlayerEvents;

impl Plugin for PlayerEvents {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerActionEvent>();
    }
}

#[derive(Event)]
pub enum PlayerActionEvent {
    Idle,
    Move(Vec3),
    Sprinting(Vec3)
}