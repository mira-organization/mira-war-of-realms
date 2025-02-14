use bevy::prelude::*;
use bevy_rapier3d::dynamics::RigidBody;
use bevy_rapier3d::geometry::{AsyncSceneCollider, ComputedColliderShape, TriMeshFlags};
use crate::environment::{CurrentEnvironment, EnvironmentScene, EnvironmentState};
use crate::manager::{GameState, InGameState};

pub struct BattleHandleService;

impl Plugin for BattleHandleService {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, enter_battle
            .run_if(in_state(GameState::InGame(InGameState::Main))));
    }
}

fn enter_battle(current_environment: Res<CurrentEnvironment>,
                mut next_state: ResMut<NextState<GameState>>,
                asset_server: Res<AssetServer>,
                query: Query<Entity, With<EnvironmentScene>>,
                mut commands: Commands,
) {
    if current_environment.0.state == EnvironmentState::Battle {
        info!("Battle is already running");
        next_state.set(GameState::InGame(InGameState::Battle));
        let path = "environments/tutorial/area_0002.glb";

        for env_scene in query.iter() {
            commands.entity(env_scene).despawn_recursive();
        }

        commands.spawn(SceneRoot(asset_server.load(GltfAssetLabel::Scene(1).from_asset(path))))
            .insert(Name::new("Floor"))
            .insert(EnvironmentScene)
            .insert(RigidBody::Fixed)
            .insert(AsyncSceneCollider {
                shape: Some(ComputedColliderShape::TriMesh(TriMeshFlags::MERGE_DUPLICATE_VERTICES)),
                ..default()
            });
    }
}