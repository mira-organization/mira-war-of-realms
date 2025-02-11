use bevy::prelude::*;

pub struct WorldEvents;

/// The `WorldEvents` plugin adds custom events related to interactions between entities in the world,
/// specifically when one entity hits another.
///
/// This plugin registers the `WorldEntityHitEntityEvent`, which will be used to track when one entity
/// collides or interacts with another entity in the game. The event can be listened to by other systems
/// to trigger appropriate responses such as damage or status effects.
///
/// # Example
/// This event is useful when checking for collisions between entities, such as an attack hitting a target.
impl Plugin for WorldEvents {
    /// Registers the `WorldEntityHitEntityEvent` as a custom event in the Bevy app.
    ///
    /// # Arguments
    /// * `app` - The Bevy app to which the event is added.
    fn build(&self, app: &mut App) {
        app.add_event::<WorldEntityHitEntityEvent>();
    }
}

/// Represents an event where one entity in the world hits another entity.
///
/// This event contains information about the entities involved in the interaction, including the entity
/// that caused the hit (`sender`) and the entity that was hit (`entity`).
///
/// # Fields
/// - `sender`: The entity that initiated the hit (e.g., the attacker).
/// - `entity`: The entity that was hit (e.g., the target).
#[derive(Event)]
pub struct WorldEntityHitEntityEvent {
    /// The entity that initiated the hit.
    pub sender: Entity,

    /// The entity that was hit.
    pub entity: Entity,
}
