#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use bevy::scene::ScenePlugin;
    use bevy::utils::HashMap;
    use system::characters::CharacterParty;
    use system::commons::Character;
    use system::config::{ConfigService, DummySaveData};
    use system::data::{ChangeCharacter, CharacterAnimation, CurrentWorldCharacter, JSONCharacter};
    use system::service::character_service::{switch_character, trigger_switch_character};

    #[test]
    fn test_trigger_switch_character() {
        let mut app = App::new();

        app.insert_resource(DummySaveData { current_environment: "".to_string(), current_area: 0, current_char: None });
        app.insert_resource(ChangeCharacter(false));
        app.insert_resource(ButtonInput::<KeyCode>::default());

        // Simulating key press configurations.
        app.insert_resource(ConfigService::default());

        // Simulate pressing the key to switch to "lira"
        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::Digit2);
        app.update();

        app.add_systems(Update, trigger_switch_character);

        // Run the system to simulate the character switch.
        app.update();

        // Check if the character changed correctly
        let dummy_save_data = app.world().resource::<DummySaveData>();
        assert_eq!(dummy_save_data.current_char.as_ref().unwrap().name, "Lira");
    }

    #[test]
    fn test_switch_character() {
        let mut app = App::new();

        app.add_plugins((MinimalPlugins, AssetPlugin::default(),
                         AnimationPlugin::default(), ScenePlugin::default()));

        // Insert necessary resources
        app.insert_resource(DummySaveData {
            current_environment: "".to_string(),
            current_area: 0,
            current_char: Some(mock_character_data("lira", "model/placeholder")),
        });

        app.insert_resource(ChangeCharacter(true));
        let _ = app.world_mut().resource::<AssetServer>();

        app.insert_resource(CharacterParty {
            team_leader: Default::default(),
            members: HashMap::new(),
        });

        app.insert_resource(CurrentWorldCharacter(None));

        // Mock the query transform to return a fixed transform for the old character
        app.world_mut().spawn((Character::default(), Transform::from_xyz(40.0, 14.0, 40.0)));

        // Run the system
        app.add_systems(Update, switch_character);
        app.update();

        // Check if the character was switched correctly
        let character_party = app.world().resource::<CharacterParty>();
        assert_eq!(character_party.members.len(), 2);
        assert_eq!(character_party.members.get(&1).unwrap().name, "lira");

        let current_world_character = app.world().resource::<CurrentWorldCharacter>();
        assert!(current_world_character.0.is_some());
    }

    fn mock_character_data(name: &str, model: &str) -> JSONCharacter {
        JSONCharacter {
            name: name.to_string(),
            lastname: "".to_string(),
            model: model.to_string(),
            world_attack_range: 5.0,
            animations: vec![
                CharacterAnimation {
                    key: String::from("idle"),
                    index: 0,
                },
                CharacterAnimation {
                    key: String::from("walk"),
                    index: 1,
                },
                CharacterAnimation {
                    key: String::from("sprint"),
                    index: 2,
                },
                CharacterAnimation {
                    key: String::from("idle-02"),
                    index: 3,
                }
            ],
            abilities: vec![],
        }
    }

}