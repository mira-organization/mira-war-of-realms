#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use battle_lib::turn_logic::{battle_order_system, setup_battle_order};
    use system::battle_commons::TurnCurrentMemberInfo;
    use system::commons::{AbilityType, Character, CharacterAbility, CharacterAbilitySet, CharacterCurrentStats, Enemy, EnemyCurrentStats, ScalingType, SelectionType, TargetType, TurnOrder};

    #[test]
    fn test_setup_battle_order() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Add the TurnOrder resource to the world
        app.insert_resource(TurnOrder::default());

        let character_1 = app.world_mut().spawn(Character {
            current_stats: CharacterCurrentStats {
                speed: 50.0,
                ..default()
            },
            ..default()
        }).id();
        let character_2 = app.world_mut().spawn(Character {
            current_stats: CharacterCurrentStats {
                speed: 70.0,
                ..default()
            },
            ..default()
        }).id();
        let enemy_1 = app.world_mut().spawn(Enemy {
            current_stats: EnemyCurrentStats {
                speed: 65.0,
                ..default()
            },
            ..default()
        }).id();

        // Run the update system and setup battle order
        app.add_systems(Startup, setup_battle_order);
        app.update(); // First update to initialize everything

        // Assert that the order is based on speed: character_2 > enemy_1 > character_1
        let turn_order = app.world().resource::<TurnOrder>();
        assert_eq!(turn_order.order.len(), 3);
        assert_eq!(turn_order.order[0], character_2);  // character_2 should be first
        assert_eq!(turn_order.order[1], enemy_1);     // enemy_1 should be second
        assert_eq!(turn_order.order[2], character_1); // character_1 should be last
    }

    #[test]
    fn test_battle_order_system_selects_alive_character() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Insert needed resources
        app.insert_resource(TurnOrder {
            order: vec![],
            current_index: 0,
            next: true,
        });

        app.insert_resource(TurnCurrentMemberInfo::default());

        // Spawn characters and enemies
        let (char_alive, char_dead, enemy_alive) = {
            let world = app.world_mut();

            // Alive character
            let id_alive = world.spawn((
                Character {
                    current_stats: CharacterCurrentStats {
                        hp: 100.0,
                        speed: 50.0,
                        ..default()
                    },
                    ..default()
                },
                Name::new("AliveCharacter"),
                CharacterAbilitySet(vec![
                    CharacterAbility {
                        name: "Test Char".to_string(),
                        family: AbilityType::Attack,
                        selection_type: SelectionType::Single,
                        target_type: TargetType::Enemy,
                        scaling_type: ScalingType::Attack,
                        scaling: 2.0,
                        base_value: 7.0,
                    }, // Dummy ability
                ]),
            )).id();

            // Dead character
            let id_dead = world.spawn((
                Character {
                    current_stats: CharacterCurrentStats {
                        hp: 0.0,
                        speed: 30.0,
                        ..default()
                    },
                    ..default()
                },
                Name::new("DeadCharacter"),
                CharacterAbilitySet(vec![
                    CharacterAbility {
                        name: "Test".to_string(),
                        family: AbilityType::Attack,
                        selection_type: SelectionType::Single,
                        target_type: TargetType::Enemy,
                        scaling_type: ScalingType::Attack,
                        scaling: 1.0,
                        base_value: 5.0,
                    }
                ]),
            )).id();

            // Alive enemy
            let id_enemy = world.spawn((
                Enemy {
                    current_stats: EnemyCurrentStats {
                        hp: 100.0,
                        speed: 60.0,
                        ..default()
                    },
                    ..default()
                },
                Name::new("EnemyAlive"),
            )).id();

            (id_alive, id_dead, id_enemy)
        };

        // Set initial turn order: dead character is first, should be skipped
        app.world_mut().resource_mut::<TurnOrder>().order = vec![
            char_dead,
            char_alive,
            enemy_alive,
        ];

        // Add the system manually (no startup stage)
        app.add_systems(Update, battle_order_system);

        // Run the update to trigger the logic
        app.update();

        // Fetch and verify updated TurnCurrentMemberInfo
        let turn_info = app.world().resource::<TurnCurrentMemberInfo>();
        let turn_order = app.world().resource::<TurnOrder>();

        // Should have skipped dead character and selected the alive one
        assert_eq!(
            turn_info.character.is_some(),
            true,
            "Expected character to be set"
        );
        assert_eq!(
            turn_info.character.as_ref().unwrap().current_stats.hp > 0.0,
            true,
            "Character should be alive"
        );
        assert_eq!(
            turn_order.current_index,
            1,
            "Turn index should move to next after processing"
        );
    }

}