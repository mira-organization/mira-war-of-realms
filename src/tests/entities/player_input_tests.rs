#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use entities_lib::player::input::battle_input_system;
    use system::battle_commons::TurnCurrentMemberInfo;
    use system::commons::{AbilityType, Character, CharacterAbility, CharacterAbilitySet, ScalingType, SelectionType, TargetType, TurnOrder};
    use system::config::{ConfigService, InputConfig};

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.init_resource::<TurnOrder>();
        app.init_resource::<TurnCurrentMemberInfo>();
        app.init_resource::<ConfigService>();
        app.init_resource::<ButtonInput<KeyCode>>();
        app
    }

    fn make_dummy_character(name: String, mut app: App) -> Entity {
        let character = Character {
            name,
            ..Default::default()
        };

        let ability_set = CharacterAbilitySet(vec![
            make_dummy_operation(AbilityType::Attack, "Slash"),
            make_dummy_operation(AbilityType::Ability, "Fireball"),
            make_dummy_operation(AbilityType::Ultimate, "Meteor Shower"),
        ]);

        let entity = app.world_mut().spawn((
            character,
            ability_set,
        )).id();

        entity
    }

    fn setup_config(app: &mut App) {
        let config = ConfigService::default();
        app.insert_resource(config);
    }

    fn make_dummy_operation(family: AbilityType, name: &str) -> CharacterAbility {
        CharacterAbility {
            name: name.to_string(),
            family,
            target_type: TargetType::Enemy,
            scaling_type: ScalingType::Attack,
            base_value: 5.0,
            scaling: 1.0,
            selection_type: SelectionType::Single
        }
    }

    #[test]
    fn test_fetch_battle_operation_basic_flow() {
        let mut app = setup_app();

        // Insert ConfigService with Key Mappings
        app.insert_resource(ConfigService {
            input_config: InputConfig {
                battle_attack_0: "Q".to_string(),
                ..Default::default()
            },
            ..Default::default()
        });

        // Simulate Key Press
        let mut input = ButtonInput::<KeyCode>::default();
        input.press(KeyCode::KeyQ);
        app.insert_resource(input);

        // Setup character and abilities
        let entity = app.world_mut().spawn((
            Character {
                name: "Tester".to_string(),
                ..Default::default()
            },
            CharacterAbilitySet(vec![
                make_dummy_operation(AbilityType::Attack, "Slash"),
                make_dummy_operation(AbilityType::Attack, "Block"),
            ]),
        )).id();

        // Setup TurnOrder
        {
            let mut order = app.world_mut().resource_mut::<TurnOrder>();
            order.order = vec![entity];
            order.current_index = 1;
        }

        // Clear TurnCurrentMemberInfo
        app.world_mut().resource_mut::<TurnCurrentMemberInfo>().pre_operation = None;
        app.world_mut().resource_mut::<TurnCurrentMemberInfo>().character = None;
        app.world_mut().resource_mut::<TurnCurrentMemberInfo>().selected_operation = None;

        // Run system
        app.add_systems(Update, battle_input_system);
        app.update();

        // Assert pre_operation is set
        let turn_info = app.world().resource::<TurnCurrentMemberInfo>();
        assert_eq!(turn_info.character.as_ref().unwrap().name, "Tester");
        assert_eq!(turn_info.pre_operation.as_ref().unwrap().name, "Slash");
        assert!(turn_info.selected_operation.is_none());

        // Reuse existing operation: Simulate another update tick with existing pre_operation
        let mut turn_info = app.world_mut().resource_mut::<TurnCurrentMemberInfo>();
        turn_info.selected_operation = Some(turn_info.pre_operation.clone().unwrap());

        // Run system again without new key input
        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().clear();
        app.update();

        // Assert selected_operation is still "Slash"
        let turn_info = app.world().resource::<TurnCurrentMemberInfo>();
        assert_eq!(turn_info.selected_operation.as_ref().unwrap().name, "Slash");
    }

    #[test]
    fn test_fetch_battle_operation_spell() {
        let mut app = setup_app();

        // Insert ConfigService with Key Mappings
        app.insert_resource(ConfigService {
            input_config: InputConfig {
                battle_spell_0: "E".to_string(),
                ..Default::default()
            },
            ..Default::default()
        });

        // Simulate Key Press
        let mut input = ButtonInput::<KeyCode>::default();
        input.press(KeyCode::KeyE);
        app.insert_resource(input);

        // Setup character and abilities
        let entity = app.world_mut().spawn((
            Character {
                name: "Tester".to_string(),
                ..Default::default()
            },
            CharacterAbilitySet(vec![
                make_dummy_operation(AbilityType::Ability, "Slash"),
                make_dummy_operation(AbilityType::Attack, "Block"),
            ]),
        )).id();

        // Setup TurnOrder
        {
            let mut order = app.world_mut().resource_mut::<TurnOrder>();
            order.order = vec![entity];
            order.current_index = 1;
        }

        // Clear TurnCurrentMemberInfo
        app.world_mut().resource_mut::<TurnCurrentMemberInfo>().pre_operation = None;
        app.world_mut().resource_mut::<TurnCurrentMemberInfo>().character = None;
        app.world_mut().resource_mut::<TurnCurrentMemberInfo>().selected_operation = None;

        // Run system
        app.add_systems(Update, battle_input_system);
        app.update();

        // Assert pre_operation is set
        let turn_info = app.world().resource::<TurnCurrentMemberInfo>();
        assert_eq!(turn_info.character.as_ref().unwrap().name, "Tester");
        assert_eq!(turn_info.pre_operation.as_ref().unwrap().name, "Slash");
        assert!(turn_info.selected_operation.is_none());

        // Reuse existing operation: Simulate another update tick with existing pre_operation
        let mut turn_info = app.world_mut().resource_mut::<TurnCurrentMemberInfo>();
        turn_info.selected_operation = Some(turn_info.pre_operation.clone().unwrap());

        // Run system again without new key input
        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().clear();
        app.update();

        // Assert selected_operation is still "Slash"
        let turn_info = app.world().resource::<TurnCurrentMemberInfo>();
        assert_eq!(turn_info.selected_operation.as_ref().unwrap().name, "Slash");
    }

    #[test]
    fn test_fetch_battle_operation_ultimate() {
        let mut app = setup_app();

        // Insert ConfigService with Key Mappings
        app.insert_resource(ConfigService {
            input_config: InputConfig {
                battle_ultimate: "Space".to_string(),
                ..Default::default()
            },
            ..Default::default()
        });

        // Simulate Key Press
        let mut input = ButtonInput::<KeyCode>::default();
        input.press(KeyCode::Space);
        app.insert_resource(input);

        // Setup character and abilities
        let entity = app.world_mut().spawn((
            Character {
                name: "Tester".to_string(),
                ..Default::default()
            },
            CharacterAbilitySet(vec![
                make_dummy_operation(AbilityType::Ultimate, "Slash"),
                make_dummy_operation(AbilityType::Attack, "Block"),
            ]),
        )).id();

        // Setup TurnOrder
        {
            let mut order = app.world_mut().resource_mut::<TurnOrder>();
            order.order = vec![entity];
            order.current_index = 1;
        }

        // Clear TurnCurrentMemberInfo
        app.world_mut().resource_mut::<TurnCurrentMemberInfo>().pre_operation = None;
        app.world_mut().resource_mut::<TurnCurrentMemberInfo>().character = None;
        app.world_mut().resource_mut::<TurnCurrentMemberInfo>().selected_operation = None;

        // Run system
        app.add_systems(Update, battle_input_system);
        app.update();

        // Assert pre_operation is set
        let turn_info = app.world().resource::<TurnCurrentMemberInfo>();
        assert_eq!(turn_info.character.as_ref().unwrap().name, "Tester");
        assert_eq!(turn_info.pre_operation.as_ref().unwrap().name, "Slash");
        assert!(turn_info.selected_operation.is_none());

        // Reuse existing operation: Simulate another update tick with existing pre_operation
        let mut turn_info = app.world_mut().resource_mut::<TurnCurrentMemberInfo>();
        turn_info.selected_operation = Some(turn_info.pre_operation.clone().unwrap());

        // Run system again without new key input
        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().clear();
        app.update();

        // Assert selected_operation is still "Slash"
        let turn_info = app.world().resource::<TurnCurrentMemberInfo>();
        assert_eq!(turn_info.selected_operation.as_ref().unwrap().name, "Slash");
    }


}