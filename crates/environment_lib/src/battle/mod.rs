use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy_rapier3d::dynamics::{Damping, LockedAxes};
use bevy_rapier3d::geometry::Collider;
use bevy_rapier3d::prelude::{RigidBody, Velocity};
use system::characters::{CharacterBundle, CharacterParty};
use system::commons::{Character, InBattle, WorldPlayer};
use system::states::{GameState, InGameState};

pub struct BattleEnvironmentPlugin;

impl Plugin for BattleEnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame(InGameState::Battle)), spawn_player_characters);
    }
}

fn spawn_player_characters(mut commands: Commands,
                           asset_server: Res<AssetServer>,
                           mut players: Query<(&mut Transform, &WorldPlayer), (With<InBattle>, With<WorldPlayer>)>,
                           character_party: Res<CharacterParty>) {

    let (mut transform, world_player) = match players.get_single_mut() {
        Ok(data) => data,
        Err(_) => return
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
}

fn generate_character(commands: &mut Commands,
                      asset_server: &AssetServer,
                      location: Vec3,
                      character: &Character) {
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