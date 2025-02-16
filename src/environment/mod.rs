mod env_init;
mod env_swap_system;

use std::f32::consts::PI;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_rapier3d::geometry::ComputedColliderShape;
use bevy_rapier3d::prelude::{AsyncSceneCollider, RigidBody, TriMeshFlags};
use fluent_bundle::types::AnyEq;
use crate::environment::env_init::{EnvInitPlugin};
use crate::environment::env_swap_system::EnvSwapSystemPlugin;
use crate::manager::{GameState, InGameState};

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnvironmentListResource>();
        app.add_plugins((EnvInitPlugin, EnvSwapSystemPlugin));
        app.add_systems(OnEnter(GameState::InGame(InGameState::Main)), (create_game_floor, create_light));
    }
}

#[derive(Resource, Debug)]
pub struct EnvironmentListResource(pub HashMap<String, Environment>);

#[derive(Resource, Debug)]
pub struct CurrentEnvironment(pub Environment);

impl Default for EnvironmentListResource {
    fn default() -> Self {
        Self {
            0: HashMap::new(),
        }
    }
}

#[derive(Component, Reflect, Debug, Clone)]
pub struct Environment {
    pub name: String,
    pub loaded: bool,
    pub areas: HashMap<String, Area>,
    pub state: EnvironmentState
}

#[derive(Reflect, Debug, Clone)]
pub struct Area {
    pub name: String,
    pub index: usize,
    pub player_in_bound: bool,
    pub battle_scenes: HashMap<String, BattleScene>
}

#[derive(Reflect, Debug, Clone, PartialEq)]
pub enum EnvironmentState {
    Exploring,
    Battle,
    Boss
}

#[derive(Component, Reflect, Debug, Clone)]
pub struct BattleScene {
    pub name: String,
    pub battle_music: HashMap<String, String>,
}

#[derive(Component, Debug, Clone)]
pub struct EnvironmentScene;

fn create_game_floor(mut commands: Commands, asset_server: Res<AssetServer>, environment: Res<EnvironmentListResource>) {
    let map = environment.0.clone();
    if map.is_empty() {
        error!("No environment found!");
        return;
    }

    let mut need_load = "";
    let player_safe_data_env = "tutorial";
    let player_safe_data_env_area = 2;
    let mut current_env: Option<Environment> = None;

    for (key, value) in map.iter() {
        if key.equals(&player_safe_data_env.to_string()) {
            for (_a_key, area) in value.areas.iter() {
                if area.index == player_safe_data_env_area {
                    need_load = &*area.name;
                }
            }
            current_env = Some(value.clone());
        }
    }

    let path = format!("environments/tutorial/{}", need_load);
    if let Some(env) = current_env {
        commands.insert_resource(CurrentEnvironment(env));
    }

    // Spawn the game floor entity, loading the floor model from the asset server
    commands.spawn(SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(path.clone()))))
        .insert(Name::new("Floor"))
        .insert(EnvironmentScene)
        .insert(RigidBody::Fixed)  // The floor is fixed and won't move // Mark this entity as part of the environment
        .insert(AsyncSceneCollider {
            shape: Some(ComputedColliderShape::TriMesh(TriMeshFlags::MERGE_DUPLICATE_VERTICES)),
            ..default()
        });
}

fn create_light(mut commands: Commands) {
    // Spawn the directional light entity
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,  // Set light intensity to an overcast day level
            shadows_enabled: true,  // Enable shadows for the light
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 200.0, 0.0),  // Position the light above the floor
            rotation: Quat::from_rotation_x(-PI / 4.0),  // Rotate the light to cast shadows at an angle
            ..default()
        },
        CascadeShadowConfigBuilder {
            num_cascades: 4,  // Set up 4 cascades for better shadow quality
            first_cascade_far_bound: 10.0,  // Set the distance for the first shadow cascade
            minimum_distance: 0.5,  // Minimum distance for shadow rendering
            maximum_distance: 200.0,  // Maximum distance for shadow rendering
            overlap_proportion: 0.2  // Set the overlap proportion for shadow cascades
        }
            .build(),
    ));
}