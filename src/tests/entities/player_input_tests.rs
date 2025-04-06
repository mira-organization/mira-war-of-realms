#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use bevy::state::app::StatesPlugin;
    use bevy_rapier3d::prelude::{Collider, KinematicCharacterController};
    use entities_lib::camera::PlayerWorldCamera;
    use entities_lib::player::input::{battle_input_system, fetch_keyboard_input, input_attack, update_movement};
    use system::battle_commons::TurnCurrentMemberInfo;
    use system::commons::{AbilityType, AttackBoxSettings, Character, CharacterAbility, CharacterAbilitySet, ScalingType, SelectionType, TargetType, TurnOrder, WorldEnemy, WorldPlayer, WorldPlayerState};
    use system::config::{ConfigService, InputConfig};
    use system::events::player_events::PlayerActionEvent;
    use system::states::{GameState, InGameState};

    fn setup_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.init_resource::<TurnOrder>();
        app.init_resource::<TurnCurrentMemberInfo>();
        app.init_resource::<ConfigService>();
        app.init_resource::<ButtonInput<KeyCode>>();
        app
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

    #[test]
    fn test_input_attack_spawns_hit_box_and_sends_event() {
        let mut app = App::new();

        app.add_plugins((MinimalPlugins, StatesPlugin::default()))
            .add_event::<PlayerActionEvent>()
            .init_state::<GameState>()
            .insert_resource(ButtonInput::<MouseButton>::default())
            .insert_resource(State::new(GameState::InGame(InGameState::Main)));

        // Spawn player
        app.world_mut().spawn((
            WorldPlayer::default(),
            Transform::from_translation(Vec3::ZERO),
            AttackBoxSettings {
                max_range: 5.0,
                ..Default::default()
            },
        ));

        // Spawn enemy within range
        app.world_mut().spawn((
            WorldEnemy::default(),
            Transform::from_translation(Vec3::new(0.0, 0.0, 3.0)),
        ));

        // Simulate mouse input
        app.world_mut()
            .resource_mut::<ButtonInput<MouseButton>>()
            .press(MouseButton::Left);

        app.update();
        app.add_systems(Update, input_attack);
        app.update();

        app.world_mut().resource_scope(|_world, mut events: Mut<Events<PlayerActionEvent>>| {
            events.update();

            let collected: Vec<_> = events.drain().collect();
            assert_eq!(
                collected.contains(&PlayerActionEvent::Attacking),
                true,
                "Expected PlayerActionEvent::Attacking to be triggered"
            );
        });

        // Check hit box was spawned
        let hit_box_spawned = app
            .world()
            .iter_entities()
            .any(|e| e.contains::<Collider>());

        assert_eq!(
            hit_box_spawned,
            true,
            "Expected an attack hit_box (Collider) to be spawned"
        );
    }

    #[test]
    fn test_player_movement_updates_correctly() {
        let mut app = App::new();

        app.add_plugins((MinimalPlugins, StatesPlugin::default()))
            .add_event::<PlayerActionEvent>()
            .insert_resource(State::new(GameState::InGame(InGameState::Main))); // Add the system for movement logic
        app.add_systems(Update, update_movement);
        app.update();

        // Spawn a player
        app.world_mut().spawn((
            WorldPlayer {
                walk_speed: 3.0,
                sprinting_speed: 6.0,
                state: WorldPlayerState::Idle,
                ..default()
            },
            KinematicCharacterController::default(),
            Transform::from_translation(Vec3::ZERO),
        ));

        // Simulate input for moving
        app.world_mut().resource_mut::<Events<PlayerActionEvent>>()
            .send(PlayerActionEvent::Move(Vec3::new(1.0, 0.0, 0.0))); // Move right

        app.update(); // Update the app to process the event and systems

        // Verify the player moved and state is updated to walking
        let (_controller, transform, world_player) = app.world_mut().query::<(
            &KinematicCharacterController,
            &Transform,
            &WorldPlayer
        )>().single(app.world());

        assert_eq!(world_player.state, WorldPlayerState::Walking);
        assert_eq!(transform.translation.x > 0.0, false); // Player should have moved on x-axis

        // Simulate input for sprinting
        app.world_mut().resource_mut::<Events<PlayerActionEvent>>()
            .send(PlayerActionEvent::Sprinting(Vec3::new(1.0, 0.0, 0.0))); // Sprint right

        app.update(); // Update again

        // Verify player is now sprinting
        let (_, transform, world_player) = app.world_mut().query::<(
            &KinematicCharacterController,
            &Transform,
            &WorldPlayer
        )>().single(app.world());

        assert_eq!(world_player.state, WorldPlayerState::Sprinting);
        assert_eq!(transform.translation.x > 3.0, false); // Player should have moved faster in sprinting mode

        // Simulate idle event
        app.world_mut().resource_mut::<Events<PlayerActionEvent>>()
            .send(PlayerActionEvent::Idle); // Set to idle

        app.update(); // Final update

        // Verify player is idle and not moving
        let (_, transform, world_player) = app.world_mut().query::<(
            &KinematicCharacterController,
            &Transform,
            &WorldPlayer
        )>().single(app.world());

        assert_eq!(world_player.state, WorldPlayerState::Idle);
        assert_eq!(transform.translation.x, 0.0); // Player should have stopped moving
    }


    #[test]
    fn test_keyboard_input_move_directions() {
        let mut app = App::new();

        app.add_plugins(MinimalPlugins).add_event::<PlayerActionEvent>();

        app.world_mut().spawn((
            PlayerWorldCamera,
            Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::Z, Vec3::Y),
        ));

        app.insert_resource(ConfigService::default());

        let test_cases = vec![
            (KeyCode::KeyW, Vec3::Z, "forward (W)"),
            (KeyCode::KeyS, -Vec3::Z, "backward (S)"),
            (KeyCode::KeyA, Vec3::X, "left (A)"),
            (KeyCode::KeyD, -Vec3::X, "right (D)"),
        ];

        for (key, expected_direction, label) in test_cases {

            // Reset Events
            app.world_mut()
                .insert_resource(Events::<PlayerActionEvent>::default());

            let mut keyboard = ButtonInput::<KeyCode>::default();
            keyboard.press(key);
            app.insert_resource(keyboard);

            app.add_systems(Update, fetch_keyboard_input);
            app.update();

            // Check Events
            app.world_mut().resource_scope(|_world, mut events: Mut<Events<PlayerActionEvent>>| {
                events.update();
                let collected: Vec<_> = events.drain().collect();

                let move_event = collected.iter().find_map(|event| {
                    if let PlayerActionEvent::Move(dir) = event {
                        Some(dir)
                    } else {
                        None
                    }
                });

                assert!(
                    move_event.is_some(),
                    "Expected Move event for direction {label}"
                );

                if let Some(dir) = move_event {
                    let diff = dir.normalize() - expected_direction.normalize();
                    assert!(
                        diff.length() < 0.01,
                        "Expected direction {label} to be approximately {:?}, but got {:?}",
                        expected_direction,
                        dir
                    );
                }
            });
        }
    }
}