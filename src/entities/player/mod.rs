mod input;
mod animation;

use bevy::core_pipeline::bloom::Bloom;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::{Offset, ThirdPersonCamera, ThirdPersonCameraTarget, Zoom};
use crate::entities::player::input::PlayerInputPlugin;
use crate::entities::{AnimatedPlayer, Animations, LivingEntity, WorldPlayer};
use crate::entities::player::animation::PlayerAnimationPlugin;
use crate::manager::{ConfigService, GameState};

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
        app.add_plugins((PlayerInputPlugin, PlayerAnimationPlugin));
        app.add_systems(OnEnter(GameState::EnvironmentPostLoad), create_world_player);
        app.add_systems(OnEnter(GameState::EnvironmentPostLoad), create_player_camera);
    }
}

/// A marker component for the player's world camera.
///
/// This component is used to identify the camera entity associated with the player.
#[derive(Component, Reflect, Debug, Clone)]
pub struct PlayerWorldCamera;

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
                GltfAssetLabel::Animation(0).from_asset("entities/player/player_idle.glb"),
                GltfAssetLabel::Animation(1).from_asset("entities/player/player_idle_2.glb"),
                GltfAssetLabel::Animation(0).from_asset("entities/player/player_slow_run.glb"),
                GltfAssetLabel::Animation(1).from_asset("entities/player/player_fast_run.glb")
            ].into_iter().map(|path| asset_server.load(path)),
        1.0, graph.root).collect();
    let graph = graphs.add(graph);
    commands.insert_resource(Animations {
        animations,
        graph: graph.clone()
    });

    commands.spawn(SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("entities/player/player_idle.glb"))))
        .insert(Name::new("WorldPlayer"))
        .insert(AnimatedPlayer)
        .insert(Transform::from_xyz(40.0, 0.249, 40.0))
        .insert(ThirdPersonCameraTarget)
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

/// Spawns the camera entity associated with the player.
///
/// The camera is positioned behind the player with a third-person perspective
/// and includes various visual effects such as bloom and distance fog.
///
/// # Parameters
/// - `commands`: Provides access to entity creation and command buffers.
fn create_player_camera(mut commands: Commands, general_config: Res<ConfigService>) {
    commands.spawn((
        Name::new("PlayerCamera"),
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0),
        GlobalTransform::default(),
        PlayerWorldCamera,
        ThirdPersonCamera {
            sensitivity: Vec2::new(general_config.input_config.camera_horizontal_sensitivity,
                                   general_config.input_config.camera_vertical_sensitivity),
            zoom: Zoom::new(general_config.input_config.camera_zoom_in, general_config.input_config.camera_zoom_out),
            cursor_lock_key: KeyCode::Escape,
            offset: Offset::new(0.0, 0.8),
            offset_enabled: true,
            offset_toggle_enabled: false,
            ..default()
        },
        Bloom::default(),
        DistanceFog {
            color: Color::srgb(0.3, 0.3, 0.32),
            falloff: FogFalloff::Linear {
                start: 500.0,
                end: 600.0
            },
            ..default()
        },
        Projection::Perspective(PerspectiveProjection {
            near: 0.1,
            far: 1000.0,
            fov: std::f32::consts::FRAC_PI_4,
            ..default()
        })
    ));
}
