use bevy::prelude::*;
use crate::battle_commons::TurnCurrentMemberInfo;
use crate::bundles::WorldPlayerBundle;
use crate::characters::CharacterParty;
use crate::commons::{Animations, AttackBoxSettings, Character, WorldPlayer};
use crate::config::{ConfigService, DummySaveData};
use crate::data::{ChangeCharacter, CurrentWorldCharacter, JSONCharacter};
use crate::states::{GameState, InGameState};
use crate::utils::key_code::convert;

pub struct CharacterService;

impl Plugin for CharacterService {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, trigger_switch_character.run_if(in_state(GameState::InGame(InGameState::Main))));
        app.add_systems(Update, switch_character.run_if(resource_changed::<ChangeCharacter>));
    }
}

fn trigger_switch_character(
    mut dummy_save_data: ResMut<DummySaveData>,
    keyboard: Res<ButtonInput<KeyCode>>,
    general_config: Res<ConfigService>,
    mut change_character: ResMut<ChangeCharacter>
) {
    let character_01 = convert(general_config.input_config.character_01.as_str()).expect("Fetch key for (character 01) was failed!");
    let character_02 = convert(general_config.input_config.character_02.as_str()).expect("Fetch key for (character 02) was failed!");
    let character_03 = convert(general_config.input_config.character_03.as_str()).expect("Fetch key for (character 03) was failed!");
    let character_04 = convert(general_config.input_config.character_04.as_str()).expect("Fetch key for (character 04) was failed!");

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

pub fn switch_character(
    mut commands: Commands,
    mut change_character: ResMut<ChangeCharacter>,
    dummy_save_data: Res<DummySaveData>,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut character_party: ResMut<CharacterParty>,
    mut current_world_character: ResMut<CurrentWorldCharacter>,
) {
    if change_character.0 {
        let mut graph = AnimationGraph::new();

        if let Some(data) = dummy_save_data.current_char.clone() {
            if let Some((entity, current_character)) = current_world_character.0.clone() {
                if current_character.name == data.name {
                    change_character.0 = false;
                    return;
                }

                commands.entity(entity).despawn_recursive();
                current_world_character.0 = None;
            }

            let model_path = format!("entities/characters/{}.glb", data.model.to_lowercase());
            let character = Character::load_from_json(data.clone());

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
                character: Some(character.clone()),
                pre_operation: None,
                selected_operation: None,
            });

            character_party.members.insert(1, character.clone());
            character_party.members.insert(2, Character {
                name: String::from("placeholder"),
                ..default()
            });

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
                    ..default()
                },
                character.clone()
            )).id();

            current_world_character.0 = Some((entity, character.clone()));

            info!("Loading Character: {:?}", data.name);
        }
        change_character.0 = false;
    }
}