mod input;
mod animation;

use bevy::prelude::*;
use system::battle_commons::TurnCurrentMemberInfo;
use system::bundles::WorldPlayerBundle;
use system::characters::CharacterParty;
use system::commons::{Animations, AttackBoxSettings, Character, WorldPlayer};
use system::config::DummySaveData;
use system::data::JSONCharacter;
use system::states::GameState;
use crate::camera::{GameCameraPlugin, PlayerWorldCamera};
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
    mut character_party: ResMut<CharacterParty>,
    dummy_save_data: Res<DummySaveData>,
) {
    let character = WorldPlayer::default();
    let mut graph = AnimationGraph::new();

    if let Some(data) = dummy_save_data.current_char.clone() {
        debug!("Loading Character: {:?}", data.name);
        let model_path = format!("entities/characters/{}.glb", data.model.to_lowercase());

        let animations = graph
            .add_clips(
                [
                    GltfAssetLabel::Animation(JSONCharacter::get_animation_by_name(&data, "idle").unwrap().index as usize).from_asset(model_path.clone()),
                    GltfAssetLabel::Animation(JSONCharacter::get_animation_by_name(&data, "walk").unwrap().index as usize).from_asset(model_path.clone()),
                    GltfAssetLabel::Animation(JSONCharacter::get_animation_by_name(&data, "sprint").unwrap().index as usize).from_asset(model_path.clone()),
                    GltfAssetLabel::Animation(JSONCharacter::get_animation_by_name(&data, "idle-02").unwrap().index as usize).from_asset(model_path.clone()),
                ].into_iter().map(|path| asset_server.load(path)),
                1.0, graph.root).collect();
        let graph = graphs.add(graph);
        commands.insert_resource(Animations {
            animations,
            graph: graph.clone()
        });

        commands.insert_resource(TurnCurrentMemberInfo {
            character: Some(character.displayed_character.clone()),
            pre_operation: None,
            selected_operation: None,
        });

        character_party.members.insert(1, character.displayed_character.clone());
        character_party.members.insert(2, Character {
            name: String::from("placeholder"),
            ..default()
        });

        commands.spawn((
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(
                model_path.clone(),
            ))),
            WorldPlayerBundle {
                attack_box_settings: AttackBoxSettings {
                    max_range: data.world_attack_range
                },
                ..default()
            },
            Character::default(),
        ));
    }
}
