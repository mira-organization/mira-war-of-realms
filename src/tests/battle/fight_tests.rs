#[cfg(test)]
mod tests {
    use bevy::ecs::system::SystemState;
    use bevy::prelude::*;
    use bevy::utils::HashMap;
    use battle_lib::fight::character_perform_attack;
    use system::battle_commons::{BattleCurrentEntities, BattleSelectedStatus, TurnCurrentMemberInfo};
    use system::commons::{AbilityType, Character, CharacterAbility, Enemy, ScalingType, SelectionType, TargetType, TurnOrder};
    use system::states::GameState;

    #[test]
    fn test_character_performs_attack_on_selected_enemy() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Setup world and resources
        let mut world = app.world_mut();
        let character = Character::default();
        let enemy_entity = world.spawn(Enemy::default()).id();
        let battle_members = BattleCurrentEntities::default();

        // Setup selected enemy
        let selected = BattleSelectedStatus {
            selected: Some((1, enemy_entity)),
            sub_selected: HashMap::new(),
        };

        // Insert resources
        world.insert_resource(TurnCurrentMemberInfo {
            character: Some(character.clone()),
            selected_operation: Some(CharacterAbility {
                name: "Test".to_string(),
                family: AbilityType::Attack,
                selection_type: SelectionType::Single,
                target_type: TargetType::Enemy,
                scaling_type: ScalingType::Attack,
                scaling: 1.0,
                base_value: 5.0,
            }),
            pre_operation: None,
        });
        world.insert_resource(battle_members);
        world.insert_resource(selected);
        world.insert_resource(TurnOrder::default());
        world.insert_resource(NextState::<GameState>::default());

        // Run system
        let mut system_state: SystemState<
            (
                Commands,
                Query<(Entity, &mut Enemy), (With<Enemy>, Without<Character>)>,
                ResMut<BattleCurrentEntities>,
                Res<BattleSelectedStatus>,
                ResMut<NextState<GameState>>,
                ResMut<TurnOrder>,
                ResMut<TurnCurrentMemberInfo>,
            ),
        > = SystemState::new(&mut world);
        let (commands, enemy_query, battle_members, selected, next_state, turn_order, turn_info) =
            system_state.get_mut(&mut world);

        character_perform_attack(
            commands,
            enemy_query,
            battle_members,
            selected,
            next_state,
            turn_order,
            turn_info,
        );

        // Check that the enemy took damage (10 * 0.5 = 5 damage)
        let enemy = world.get::<Enemy>(enemy_entity).unwrap();
        assert_eq!(enemy.current_stats
                       .hp, 94.0);
    }

    #[test]
    fn test_character_performs_attack_on_multiple_enemies() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Setup world and resources
        let mut world = app.world_mut();
        let character = Character::default();
        let enemy1 = world.spawn(Enemy::default()).id();
        let enemy2 = world.spawn(Enemy::default()).id();
        let battle_members = BattleCurrentEntities::default();

        // Setup selected and sub-selected enemies
        let mut sub_selected = HashMap::new();
        sub_selected.insert(2, enemy2);
        let selected = BattleSelectedStatus {
            selected: Some((1, enemy1)),
            sub_selected,
        };

        // Insert resources
        world.insert_resource(TurnCurrentMemberInfo {
            character: Some(character.clone()),
            selected_operation: Some(CharacterAbility {
                name: "Test 2".to_string(),
                family: AbilityType::Ability,
                selection_type: SelectionType::Expansion(3),
                target_type: TargetType::Enemy,
                scaling_type: ScalingType::Attack,
                scaling: 1.0,
                base_value: 7.0,
            }),
            pre_operation: None,
        });

        world.insert_resource(battle_members);
        world.insert_resource(selected);
        world.insert_resource(TurnOrder::default());
        world.insert_resource(NextState::<GameState>::default());

        // Run system
        let mut system_state: SystemState<
            (
                Commands,
                Query<(Entity, &mut Enemy), (With<Enemy>, Without<Character>)>,
                ResMut<BattleCurrentEntities>,
                Res<BattleSelectedStatus>,
                ResMut<NextState<GameState>>,
                ResMut<TurnOrder>,
                ResMut<TurnCurrentMemberInfo>,
            ),
        > = SystemState::new(&mut world);
        let (commands, enemy_query, battle_members, selected, next_state, turn_order, turn_info) =
            system_state.get_mut(&mut world);

        character_perform_attack(
            commands,
            enemy_query,
            battle_members,
            selected,
            next_state,
            turn_order,
            turn_info,
        );

        // Check that both enemies took damage (10 * 0.5 = 5 damage)
        let enemy1 = world.get::<Enemy>(enemy1).unwrap();
        let enemy2 = world.get::<Enemy>(enemy2).unwrap();
        assert_eq!(enemy1.current_stats.hp, 94.0);
        assert_eq!(enemy2.current_stats.hp, 94.0);
    }
}
