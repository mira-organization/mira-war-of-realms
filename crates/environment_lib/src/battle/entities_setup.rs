use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy_rapier3d::dynamics::{Damping, LockedAxes, RigidBody, Velocity};
use bevy_rapier3d::geometry::Collider;
use system::characters::{CharacterBundle, CharacterParty};
use system::commons::{Character, InBattle, LivingEntity, WorldPlayer};
use system::states::{GameState, InGameState};
use crate::battle::{on_mouse_click, on_mouse_enter, on_mouse_leave};

/// A plugin responsible for spawning battle entities, such as player characters and enemies.
pub struct BattleEntitiesPlugin;

impl Plugin for BattleEntitiesPlugin {
    /// Registers the `spawn_entities` system, which runs when entering the `Battle` state.
    ///
    /// # Parameters
    /// - `app`: The Bevy app where the plugin is registered.
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame(InGameState::Battle)), spawn_entities);
    }
}

/// Spawns player characters and enemies when entering a battle.
///
/// The system places the player's party members and enemies at predefined positions.
///
/// # Parameters
/// - `commands`: Command buffer for spawning entities.
/// - `asset_server`: Asset server for loading character and enemy models.
/// - `players`: Query for retrieving the current player entity in battle.
/// - `character_party`: Resource containing the player's party members.
/// - `meshes`: Mutable resource for storing generated meshes.
fn spawn_entities(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut players: Query<(&mut Transform, &WorldPlayer), (With<InBattle>, With<WorldPlayer>)>,
    character_party: Res<CharacterParty>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let (mut transform, world_player) = match players.get_single_mut() {
        Ok(data) => data,
        Err(_) => return,
    };


    let mut location = Transform::from_xyz(-10.0, 51.0, 25.0).translation;
    let party_members = character_party.clone().members;

    for (_slot, member) in party_members {
        if member.name == world_player.displayed_character.name {
            transform.translation = location;
            transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
        } else {
            generate_character(&mut commands, &asset_server, location, &member);
        }
        location.x += 2.5;
    }

    let count: u32 = 4;
    let mut location = Transform::from_xyz(-10.0, 51.0, 15.0).translation;

    for index in 0..count {
        generate_enemies(&mut commands, &asset_server, &mut meshes, location, index);
        location.x += 2.5;
    }
}

/// Spawns a character entity for the player's party.
///
/// The character is instantiated at the given location with default physics properties.
///
/// # Parameters
/// - `commands`: Command buffer for spawning entities.
/// - `asset_server`: Asset server for loading character models.
/// - `location`: Spawn position of the character.
/// - `character`: Reference to the character data.
fn generate_character(
    commands: &mut Commands,
    asset_server: &AssetServer,
    location: Vec3,
    character: &Character,
) {
    commands.spawn(CharacterBundle {
        scene: SceneRoot(asset_server.load(GltfAssetLabel::Scene(0)
            .from_asset(format!("entities/characters/{}.glb", character.name)))),
        name: Name::new(character.name.to_string()),
        culling: NoFrustumCulling,
        transform: Transform {
            translation: location,
            rotation: Quat::from_rotation_y(0.0),
            ..default()
        },
        character: character.clone(),
        rigid_body: RigidBody::Dynamic,
        velocity: Velocity::default(),
        damping: Damping {
            angular_damping: 2.0,
            linear_damping: 2.0,
        },
        locked_axes: LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
        collider: Collider::capsule(Vec3::new(0.0, 0.2, 0.0), Vec3::new(0.0, 1.6, 0.0), 0.2),
    });
}

/// Spawns an enemy entity at the given location.
///
/// Each enemy is assigned a unique index and has basic physics properties.
///
/// # Parameters
/// - `commands`: Command buffer for spawning entities.
/// - `asset_server`: Asset server for loading enemy models.
/// - `meshes`: Mutable resource for adding generated meshes.
/// - `location`: Spawn position of the enemy.
/// - `index`: A unique index to differentiate enemies.
fn generate_enemies(
    commands: &mut Commands,
    asset_server: &AssetServer,
    meshes: &mut ResMut<Assets<Mesh>>,
    location: Vec3,
    index: u32,
) {
    commands.spawn((
        SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("entities/enemies/test_enemy/placeholder.glb"))
        ),
        Name::new(format!("Enemy0{}", index)),
        Transform {
            translation: location,
            rotation: Quat::from_rotation_y(std::f32::consts::PI),
            ..default()
        },
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
    )).with_children(|children| {
        children.spawn((
            Transform::from_xyz(0.0, 0.8, 0.0),
            Mesh3d(meshes.add(Capsule3d::new(0.2, 1.4)))
        )).observe(on_mouse_click).observe(on_mouse_enter).observe(on_mouse_leave);
    });
}
