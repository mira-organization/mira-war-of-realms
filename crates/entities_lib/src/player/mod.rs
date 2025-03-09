mod input;
mod animation;

use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy_atmosphere::prelude::AtmosphereCamera;
use bevy_rapier3d::prelude::*;
use system::commons::{AnimatedPlayer, Animations, LivingEntity, WorldPlayer};
use system::states::GameState;
use crate::camera::{CameraController, GameCameraPlugin, PlayerWorldCamera};
use crate::player::animation::PlayerAnimationPlugin;
use crate::player::input::PlayerInputPlugin;

/// A plugin for managing the player's systems, including input, animations,
/// and spawning the player entity and camera in the game world.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    /// Configures the application to add player-related plugins and systems.
    ///
    /// - Adds the `PlayerInputPlugin` and `PlayerAnimationPlugin`.
    /// - Registers systems for creating the player entity and player camera
    ///   when entering the `GameState::InGame` state.
    fn build(&self, app: &mut App) {
        app.insert_resource(LastStableGround(Vec3::ZERO));
        app.add_plugins((GameCameraPlugin, PlayerInputPlugin, PlayerAnimationPlugin));
        app.add_systems(OnEnter(GameState::EnvironmentPostLoad), create_world_player);
        app.add_systems(OnEnter(GameState::EnvironmentPostLoad), create_player_camera.after(create_world_player));
    }
}

#[derive(Resource, Debug, Default)]
pub struct LastStableGround(pub Vec3);

/// Spawns the player entity in the game world with its associated components.
///
/// The player is initialized with animations, physics properties, and other game-relevant
/// components. This function also creates and assigns an animation graph to the player.
///
/// # Parameters
/// - `commands`: Provides access to entity creation and command buffers.
/// - `graphs`: A mutable resource containing all loaded `AnimationGraph` assets.
/// - `asset_server`: Used to load assets such as animations and 3D models.
pub fn create_world_player(
    mut commands: Commands,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    asset_server: Res<AssetServer>,
) {
    let mut graph = AnimationGraph::new();
    let animations = graph
        .add_clips(
            [
                GltfAssetLabel::Animation(0).from_asset("entities/characters/ignara.glb"),
                GltfAssetLabel::Animation(1).from_asset("entities/characters/ignara.glb"),
                GltfAssetLabel::Animation(2).from_asset("entities/characters/ignara.glb"),
                GltfAssetLabel::Animation(3).from_asset("entities/characters/ignara.glb"),
            ].into_iter().map(|path| asset_server.load(path)),
        1.0, graph.root).collect();
    let graph = graphs.add(graph);
    commands.insert_resource(Animations {
        animations,
        graph: graph.clone()
    });

    commands.spawn(SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("entities/characters/ignara.glb"))))
        .insert(Name::new("WorldPlayer"))
        .insert(NoFrustumCulling)
        .insert(AnimatedPlayer)
        .insert(Transform::from_xyz(40.0, 12.0, 40.0))
        .insert(WorldPlayer::default())
        .insert(LivingEntity)
        .insert(RigidBody::Dynamic)
        .insert(Velocity::default())
        .insert(GravityScale(2.5))
        .insert(Damping {
            angular_damping: 2.0,
            linear_damping: 2.0
        })
        .insert(LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z)
        .insert(Collider::capsule(Vec3::new(0.0, 0.2, 0.0), Vec3::new(0.0, 1.6, 0.0), 0.2))
        .insert(KinematicCharacterController {
            max_slope_climb_angle: 45_f32.to_radians(),
            min_slope_slide_angle: 35_f32.to_radians(),
            autostep: Some(CharacterAutostep {
                include_dynamic_bodies: true,
                min_width: CharacterLength::Absolute(0.05),
                max_height: CharacterLength::Absolute(0.55)
            }),
            snap_to_ground: Some(CharacterLength::Absolute(0.075)),
            ..default()
        });
}

/// Spawns a new player camera entity with the necessary components.
///
/// This function spawns a 3D camera that follows the player, along with a camera controller
/// and additional components for world and atmosphere-related camera behavior.
///
/// # Parameters
/// - `commands`: The `Commands` struct used to spawn the camera entity.
fn create_player_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        CameraController::default(),
        PlayerWorldCamera,
        AtmosphereCamera::default()
    ));
}
