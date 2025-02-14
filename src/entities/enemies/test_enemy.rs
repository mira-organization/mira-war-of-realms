use bevy::prelude::*;
use bevy_rapier3d::dynamics::{Damping, LockedAxes, RigidBody, Velocity};
use bevy_rapier3d::geometry::Collider;
use rand::Rng;
use crate::entities::{AnimatedMob, LivingEntity};
use crate::entities::enemies::ai::{AiSetup, AiState};
use crate::entities::enemies::WorldEnemy;
use crate::manager::{GameState, InGameState};

/// A plugin for setting up a test enemy in the game world.
///
/// This plugin adds systems for spawning and initializing a placeholder enemy entity,
/// which can be used for testing purposes or as a default enemy template.
pub struct TestEnemy;

impl Plugin for TestEnemy {
    /// Configures the application to add a system for setting up a test enemy
    /// when entering the `GameState::InGame` state.
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame(InGameState::Main)), setup_enemy);
    }
}

/// A system for spawning and initializing a test enemy entity.
///
/// This system is executed when the game transitions into the `GameState::InGame` state.
/// It spawns a placeholder enemy with a variety of components, including its visual representation,
/// physics properties, and game-specific data.
///
/// # Parameters
/// - `commands`: Provides access to entity creation and command buffers.
/// - `asset_server`: Used to load assets such as the enemy's 3D model.
fn setup_enemy(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("entities/enemies/test_enemy/placeholder.glb"))
        ))
        .insert(Name::new("Enemy-Test"))
        .insert(AnimatedMob)
        .insert(Transform::from_xyz(4.0, 0.0, 2.0))
        .insert(AiSetup {
            state: AiState::Idle,
            path: vec![
                Vec3::new(16.0, 0.0, 2.0),
                Vec3::new(8.0, 0.0, 8.0),
                Vec3::new(1.5, 0.0, -9.0),
            ],
            current_path_index: 0,
            idle_timer: rand::rng().random_range(2.0..6.0),
            ..default()
        })
        .insert(WorldEnemy::default())
        .insert(LivingEntity)
        .insert(RigidBody::Dynamic)
        .insert(Velocity::default())
        .insert(Damping {
            angular_damping: 1.0,
            linear_damping: 1.0,
        })
        .insert(LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z)
        .insert(Collider::capsule(
            Vec3::new(0.0, 0.2, 0.0),
            Vec3::new(0.0, 1.6, 0.0),
            0.2,
        ));
}
