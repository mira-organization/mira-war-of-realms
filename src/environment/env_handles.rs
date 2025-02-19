use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy_rapier3d::dynamics::RigidBody;
use bevy_rapier3d::geometry::{AsyncSceneCollider, ComputedColliderShape, TriMeshFlags};
use crate::entities::{InBattle, WorldPlayer};
use crate::environment::{CurrentAreaScenes, EnvironmentScene};
use crate::events::world_events::WorldEntityHitEntityEvent;
use crate::manager::{GameState, InGameState};

pub struct EnvSwapSystemPlugin;

impl Plugin for EnvSwapSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, pre_load_battle.run_if(in_state(GameState::InGame(InGameState::Main))));
        app.add_systems(OnEnter(GameState::InGame(InGameState::Battle)), load_battle_scene);
        app.add_systems(Update, temp_leave_battle.run_if(in_state(GameState::InGame(InGameState::Battle))));
    }
}

fn pre_load_battle(mut hit_event: EventReader<WorldEntityHitEntityEvent>,
                     area: Res<CurrentAreaScenes>,
                     mut commands: Commands,
                     world_player_query: Query<Entity, With<WorldPlayer>>,
                     mut next_state: ResMut<NextState<GameState>>,
) {
    for event in hit_event.read() {
        let sender_a_player = world_player_query.get(event.sender).is_ok();
        if area.0.len() > 3 {
            if sender_a_player {
                commands.entity(event.sender).insert(InBattle);
            } else {
                commands.entity(event.entity).insert(InBattle);
            }
            info!("Starting Battle [{:?}]", area.0.len());
            next_state.set(GameState::InGame(InGameState::Battle));
        } else {
            warn!("No Battle Scenes was found! Scenes [ {:?} ]", area.0.len());
        }
    }
}

fn load_battle_scene(area: Res<CurrentAreaScenes>,
                     mut commands: Commands,
                     mut players: Query<&mut Transform, With<InBattle>>,
) {
    let battle_scene = area.0.get("battle_1");

    if let Some(scene) = battle_scene {
        commands.spawn(SceneRoot(scene.clone()))
            .insert(Name::new("Battle Scene"))
            .insert(EnvironmentScene)
            .insert(NoFrustumCulling)
            .insert(RigidBody::Fixed)
            .insert(AsyncSceneCollider {
                shape: Some(ComputedColliderShape::TriMesh(TriMeshFlags::MERGE_DUPLICATE_VERTICES)),
                ..default()
            });
        for mut transform in players.iter_mut() {
            transform.translation = Vec3::new(-7.5, 20.0, 9.5);
        }
    }

    info!("Loading Battle Scenes");
}

fn temp_leave_battle(mut commands: Commands,
                     mut players: Query<(Entity, &mut Transform), With<InBattle>>,
                     keyboard: Res<ButtonInput<KeyCode>>,
                     mut next_state: ResMut<NextState<GameState>>
) {
    if keyboard.just_pressed(KeyCode::KeyL) {
        for (entity, mut transform) in players.iter_mut() {
            commands.entity(entity).remove::<InBattle>();
            transform.translation = Vec3::new(0.0, 1.0, 0.0);
        }

        next_state.set(GameState::InGame(InGameState::Main));
        info!("Leaving Battle Scenes");
    }
}