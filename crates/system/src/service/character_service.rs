use bevy::prelude::*;
use crate::battle_commons::TurnCurrentMemberInfo;
use crate::bundles::WorldPlayerBundle;
use crate::characters::CharacterParty;
use crate::commons::{Animations, AttackBoxSettings, Character, WorldPlayer};
use crate::config::{ConfigService, DummySaveData};
use crate::data::{ChangeCharacter, CurrentWorldCharacter, JSONCharacter};
use crate::states::{GameState, InGameState};
use crate::utils::key_code::convert;

/// The `CharacterService` plugin manages character switching in the game.
pub struct CharacterService;

impl Plugin for CharacterService {
    /// Registers the character switching systems into the Bevy application.
    ///
    /// - `trigger_switch_character`: Listens for key presses to initiate a character switch.
    /// - `switch_character`: Handles the actual process of changing the in-game character.
    fn build(&self, app: &mut App) {
        app.add_systems(Update, trigger_switch_character.run_if(in_state(GameState::InGame(InGameState::Main))));
        app.add_systems(Update, switch_character.run_if(resource_changed::<ChangeCharacter>));
    }
}

/// Detects character switch input and sets the `ChangeCharacter` resource accordingly.
///
/// This function listens for key presses mapped to character selection and updates
/// the `change_character` flag if a valid input is detected.
///
/// # Parameters
/// - `dummy_save_data`: Stores the currently active character.
/// - `keyboard`: Handles keyboard input.
/// - `general_config`: Stores key bindings for character selection.
/// - `change_character`: A flag that determines if a character switch should occur.
pub fn trigger_switch_character(
    mut dummy_save_data: ResMut<DummySaveData>,
    keyboard: Res<ButtonInput<KeyCode>>,
    general_config: Res<ConfigService>,
    mut change_character: ResMut<ChangeCharacter>
) {
    let character_01 = convert(general_config.input_config.character_01.as_str()).expect("Fetch key for (character 01) failed!");
    let character_02 = convert(general_config.input_config.character_02.as_str()).expect("Fetch key for (character 02) failed!");
    let character_03 = convert(general_config.input_config.character_03.as_str()).expect("Fetch key for (character 03) failed!");
    let character_04 = convert(general_config.input_config.character_04.as_str()).expect("Fetch key for (character 04) failed!");

    let mut name = "";

    if keyboard.just_pressed(character_01) && !change_character.0 {
        name = "ignara";
        change_character.0 = true;
    }

    if keyboard.just_pressed(character_02) && !change_character.0 {
        name = "lira";
        change_character.0 = true;
    }

    if keyboard.just_pressed(character_03) && !change_character.0 {
        name = "ignara";
    }

    if keyboard.just_pressed(character_04) && !change_character.0 {
        name = "ignara";
    }

    if change_character.0 {
        let result = match JSONCharacter::fetch(name) {
            Ok(result) => result,
            Err(err) => {
                error!(err);
                dummy_save_data.current_char.clone().unwrap()
            }
        };

        if let Some(current) = dummy_save_data.current_char.clone() {
            if current.name != name {
                dummy_save_data.current_char = Some(result);
            }
        } else {
            dummy_save_data.current_char = Some(result);
        }
    }
}

/// Handles the actual character switching process by updating game state.
///
/// This function:
/// - De-spawns the current character if necessary.
/// - Loads the new character model and animations.
/// - Updates the `CharacterParty` and `CurrentWorldCharacter` resources.
/// - Spawns the new character entity into the world.
///
/// # Parameters
/// - `commands`: Used to modify the entity world (spawn/de-spawn entities).
/// - `change_character`: Tracks whether a character change should happen.
/// - `dummy_save_data`: Stores the active character data.
/// - `asset_server`: Loads assets such as character models and animations.
/// - `graphs`: Stores animation graphs for character animations.
/// - `character_party`: Manages the list of available characters.
/// - `current_world_character`: Stores the currently active world character.
pub fn switch_character(
    mut commands: Commands,
    mut change_character: ResMut<ChangeCharacter>,
    dummy_save_data: Res<DummySaveData>,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut character_party: ResMut<CharacterParty>,
    mut current_world_character: ResMut<CurrentWorldCharacter>,
    query_transform: Query<&Transform, With<Character>>,
) {
    if change_character.0 {
        let mut graph = AnimationGraph::new();

        if let Some(data) = dummy_save_data.current_char.clone() {
            let mut transform = Transform::from_xyz(40.0, 14.0, 40.0);
            if let Some((entity, current_character)) = current_world_character.0.clone() {
                // Prevents unnecessary character switch if it's already the active one
                if current_character.name == data.name {
                    change_character.0 = false;
                    return;
                }

                transform = match query_transform.get(entity) {
                    Ok(transform) => *transform,
                    Err(_) => return,
                };
                // De-spawn the old character
                commands.entity(entity).despawn_recursive();
                current_world_character.0 = None;
            }

            // Load the new character model
            let model_path = format!("entities/characters/{}.glb", data.model.to_lowercase());
            let character = Character::load_from_json(data.clone());

            // Load animations for the character
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

            // Update animation resource
            commands.insert_resource(Animations {
                animations,
                graph: graph.clone()
            });

            // Update character party
            commands.insert_resource(TurnCurrentMemberInfo {
                character: Some(character.clone()),
                pre_operation: None,
                selected_operation: None,
            });

            character_party.members.insert(1, character.clone());
            character_party.members.insert(2, Character {
                name: String::from("placeholder"),
                ..default()
            });

            // Spawn new character entity into the world
            let entity = commands.spawn((
                SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(
                    model_path.clone(),
                ))),
                WorldPlayerBundle {
                    attack_box_settings: AttackBoxSettings {
                        max_range: data.world_attack_range
                    },
                    world_player: WorldPlayer {
                        displayed_character: character.clone(),
                        ..default()
                    },
                    transform,
                    ..default()
                },
                character.clone()
            )).id();

            // Set the active character reference
            current_world_character.0 = Some((entity, character.clone()));

            info!("Loading Character: {:?}", data.name);
        }
        change_character.0 = false;
    }
}
