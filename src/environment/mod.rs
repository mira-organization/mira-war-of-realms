use std::f32::consts::PI;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use bevy_rapier3d::geometry::ComputedColliderShape;
use bevy_rapier3d::prelude::{AsyncSceneCollider, RigidBody, TriMeshFlags};
use crate::manager::GameState;

pub struct EnvironmentPlugin;

/// The `EnvironmentPlugin` plugin sets up the environment for the game, including the game floor and lighting.
///
/// This plugin is responsible for creating the floor of the game world and adding a directional light.
/// It runs when the game enters the `InGame` state.
///
/// # Example
/// This plugin is used to initialize the game world with a basic environment setup, such as the game floor and lighting.
impl Plugin for EnvironmentPlugin {
    /// Adds the system to spawn the game floor when the game enters the `InGame` state.
    ///
    /// # Arguments
    /// * `app` - The Bevy app to which the system is added.
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), create_game_floor);
    }
}

/// The `Environment` component is used to mark entities that are part of the environment, such as the game floor.
///
/// This component is added to the game floor entity to indicate it is part of the environment and should be handled accordingly.
/// It does not carry any additional data but helps to organize entities in the world.
#[derive(Component, Debug, Clone)]
pub struct Environment;

/// Spawns the game floor and sets up the directional light when the game enters the `InGame` state.
///
/// # Arguments
/// * `commands` - The Bevy commands used to spawn the entities.
/// * `asset_server` - The Bevy asset server used to load the floor model.
fn create_game_floor(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the game floor entity, loading the floor model from the asset server
    commands.spawn(SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("world/test-room.glb"))))
        .insert(Name::new("Floor"))
        .insert(RigidBody::Fixed)  // The floor is fixed and won't move
        .insert(Environment)  // Mark this entity as part of the environment
        .insert(AsyncSceneCollider {
            shape: Some(ComputedColliderShape::TriMesh(TriMeshFlags::MERGE_DUPLICATE_VERTICES)),
            ..default()
        });

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