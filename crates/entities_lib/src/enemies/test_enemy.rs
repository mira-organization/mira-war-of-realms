use bevy::prelude::*;
use bevy_rapier3d::dynamics::{Damping, LockedAxes, RigidBody, Velocity};
use bevy_rapier3d::geometry::Collider;
use rand::Rng;
use system::commons::{AnimatedMob, LivingEntity};
use system::states::GameState;
use crate::enemies::ai::{AiSetup, AiState};
use crate::enemies::WorldEnemy;

/// A plugin for setting up a test enemy in the game world.
///
/// This plugin adds systems for spawning and initializing a placeholder enemy entity,
/// which can be used for testing purposes or as a default enemy template.
pub struct TestEnemy;

impl Plugin for TestEnemy {
    /// Configures the application to add a system for setting up a test enemy
    /// when entering the `GameState::InGame` state.
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::EnvironmentPostLoad), setup_enemy);
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
pub fn setup_enemy(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("entities/enemies/test_enemy/placeholder.glb"))
        ),
        Name::new("Enemy-Test"),
        AnimatedMob,
        Transform::from_xyz(-32.0, 15.0, 15.0),
        AiSetup {
            state: AiState::Idle,
            path: vec![
                Vec3::new(-32.0, 14.9, 15.0),
                Vec3::new(-32.0, 14.9, 40.0),
                Vec3::new(-12.0, 14.9, 38.0),
                Vec3::new(-12.0, 14.9, 20.0)
            ],
            current_path_index: 0,
            idle_timer: rand::rng().random_range(2.0..6.0),
            ..default()
        },
        WorldEnemy::default(),
        LivingEntity,
        RigidBody::Dynamic,
        Velocity::default(),
        Damping {
            angular_damping: 1.0,
            linear_damping: 1.0,
        },
        LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
        Collider::capsule(
            Vec3::new(0.0, 0.2, 0.0),
            Vec3::new(0.0, 1.6, 0.0),
            0.2,
        )
    ));
}
