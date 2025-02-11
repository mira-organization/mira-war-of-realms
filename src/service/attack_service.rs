use bevy::prelude::*;
use bevy_rapier3d::prelude::{ActiveCollisionTypes, Collider, ReadDefaultRapierContext, RigidBody, Sensor};
use bevy_rapier3d::rapier::prelude::RigidBodyType;
use crate::entities::{AttackHitBox, LivingEntity};
use crate::environment::Environment;
use crate::events::world_events::WorldEntityHitEntityEvent;
use crate::manager::GameState;

pub struct AttackService;

impl Plugin for AttackService {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (attack_timer_system, attack_collision_system).run_if(in_state(GameState::InGame)));
    }
}

pub fn spawn_attack_hit_box(mut commands: Commands,
                        parent: Entity,
                        shape: Collider,
                        transform: Transform,
                        duration: f32
) {
    let collider_entity = commands
        .spawn(
            AttackHitBox {
                timer: Timer::from_seconds(duration, TimerMode::Once)
            }
        ).insert(RigidBody::KinematicPositionBased)
        .insert(shape)
        .insert(ActiveCollisionTypes::default())
        .insert(Sensor)
        .insert(Transform::from(transform))
        .set_parent(parent)
        .id();

    commands.entity(parent).add_child(collider_entity);
}

fn attack_timer_system(mut commands: Commands,
                       time: Res<Time>,
                       mut query: Query<(Entity, &Parent, &mut AttackHitBox)>
) {
    for (entity, parent, mut hit_box) in query.iter_mut() {
        hit_box.timer.tick(time.delta());
        if hit_box.timer.just_finished() {
            commands.entity(parent.get()).remove_children(&[entity]);
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn attack_collision_system(
    mut event_writer: EventWriter<WorldEntityHitEntityEvent>,
    query: Query<(Entity, &Parent), With<AttackHitBox>>,
    collider_query: Query<Entity, With<LivingEntity>>, // Only check for LivingEntity
    rapier_context: ReadDefaultRapierContext,
) {
    for (attack_entity, parent) in &query {
        for collider_entity in collider_query.iter() {
            if rapier_context.intersection_pair(attack_entity, collider_entity).is_some() {
                if parent.get() == collider_entity {
                    return;
                }
                event_writer.send(WorldEntityHitEntityEvent {
                    sender: parent.get(),
                    entity: collider_entity,
                });
                info!("Detect: {:?} hit {:?}", attack_entity, collider_entity);
            }
        }
    }
}
