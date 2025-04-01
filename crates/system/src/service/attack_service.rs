use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_rapier3d::prelude::{ActiveCollisionTypes, Collider, ColliderDebugColor, DefaultRapierContext, RapierContextColliders, RapierContextSimulation, RigidBody, Sensor};
use crate::commons::{AttackHitBox, LivingEntity};
use crate::events::world_events::WorldEntityHitEntityEvent;
use crate::states::{GameState, InGameState};

pub struct AttackService;

/// The `AttackService` plugin adds systems for handling attacks in the game.
/// It includes systems for managing attack timers, detecting collisions with attack hit_boxes, and logging debug events.
impl Plugin for AttackService {
    /// Sets up the attack systems in the Bevy app, adding them to the update stage when the game is in the `InGame` state.
    ///
    /// # Arguments
    /// * `app` - The Bevy app where the systems are added.
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            attack_timer_system,
            attack_collision_system,
            debug_event_log
        ).run_if(in_state(GameState::InGame(InGameState::Main))));
    }
}

/// Spawns an attack hit_box as an entity and attaches it to the parent entity.
/// The hit_box is a collider that will be active for a specific duration and will be removed once that time has passed.
///
/// # Arguments
/// * `commands` - The Bevy commands to spawn and configure the entity.
/// * `parent` - The entity that the hit_box will be attached to.
/// * `shape` - The shape of the collider (e.g., a box, sphere).
/// * `transform` - The initial transform for the hit_box (position, rotation, scale).
/// * `color` - An optional color for the debug visualization of the hit_box. If not provided, defaults to white.
/// * `duration` - The duration for which the hit_box will be active, after which it will be de-spawned.
pub fn spawn_attack_hit_box(commands: &mut Commands,
                            parent: Entity,
                            shape: Collider,
                            transform: Transform,
                            color: Option<Color>,
                            duration: f32
) {
    let debug_color;
    if color.is_none() {
        debug_color = Color::WHITE;
    } else {
        debug_color = color.unwrap();
    }

    // Spawn the attack hit_box entity with its properties
    let collider_entity = commands.spawn((
        AttackHitBox {
            timer: Timer::from_seconds(duration, TimerMode::Once)
        },
        RigidBody::KinematicPositionBased,
        shape.clone(),
        ActiveCollisionTypes::default(),
        Sensor,
        Transform::from(transform),
        ColliderDebugColor(Hsla::from(debug_color))
    )).set_parent(parent).id();

    // Attach the hit_box entity as a child of the parent entity
    commands.entity(parent).add_child(collider_entity);
}

/// This system updates the timers for all attack hit_boxes and de-spawns them once their timer has expired.
/// It checks each hit_box for the expiration of its duration and removes the hit_box entity if the timer has finished.
///
/// # Arguments
/// * `commands` - The Bevy commands used to de-spawn the hit_box entities.
/// * `time` - The Bevy time resource, used to update the timers for each hit_box.
/// * `query` - A query to find entities with `AttackHitBox` components and their parents.
fn attack_timer_system(mut commands: Commands,
                       time: Res<Time>,
                       mut query: Query<(Entity, &Parent, &mut AttackHitBox)>
) {
    for (entity, parent, mut hit_box) in query.iter_mut() {
        hit_box.timer.tick(time.delta());
        if hit_box.timer.just_finished() {
            // Remove the hit_box entity after its timer expires
            commands.entity(parent.get()).remove_children(&[entity]);
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// This system checks for collisions between attack hit_boxes and living entities.
/// If a collision is detected, it triggers an event indicating that the entity was hit by an attack.
///
/// # Arguments
/// * `event_writer` - The event writer to send the collision events.
/// * `query` - A query to find attack hit_boxes and their parent entities.
/// * `collider_query` - A query to find all living entities in the game world.
/// * `rapier_context` - The context for checking intersections between colliders using the Rapier physics engine.
fn attack_collision_system(
    mut event_writer: EventWriter<WorldEntityHitEntityEvent>,
    query: Query<(Entity, &Parent), With<AttackHitBox>>,
    collider_query: Query<Entity, With<LivingEntity>>,
    rapier_context: Query<(&RapierContextSimulation, &RapierContextColliders), With<DefaultRapierContext>>,
    mut processed_collisions: Local<HashSet<(Entity, Entity)>>
) {
    processed_collisions.clear();

    for (attack_entity, parent) in &query {
        for collider_entity in collider_query.iter() {
            if parent.get() == collider_entity {
                continue; // Skip collision with the entity that owns the hit_box
            }

            for (context_func, context_colliders) in rapier_context.iter() {
                // Check if the attack hit_box intersects with a living entity
                if context_func.intersection_pair(context_colliders, attack_entity, collider_entity).is_some() {
                    let collision_pair = (parent.get(), collider_entity);

                    if processed_collisions.insert(collision_pair) {
                        event_writer.send(WorldEntityHitEntityEvent {
                            sender: parent.get(),
                            entity: collider_entity,
                        });
                    }
                }
            }
        }
    }
}

/// This system reads and logs events triggered when an entity is hit by an attack.
/// It logs the sender (the attacking entity) and the receiver (the entity that was hit).
///
/// # Arguments
/// * `event_reader` - The event reader to read the `WorldEntityHitEntityEvent` events.
fn debug_event_log(mut event_reader: EventReader<WorldEntityHitEntityEvent>) {
    for event in event_reader.read() {
        info!("Entity [ {:?} ] has hit Entity [ {:?} ]", event.sender, event.entity);
    }
}

