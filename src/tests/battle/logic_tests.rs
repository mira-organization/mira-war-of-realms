#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use bevy::utils::HashMap;
    use battle_lib::logic::{detect_current_character_operation, select_encounter_target, set_observe_entities};
    use system::battle_commons::{BattleCurrentEntities, BattleSelectedStatus, ObserveAble, SelectMarker, TurnCurrentMemberInfo};
    use system::commons::{AbilityType, CharacterAbility, ScalingType, SelectionType, TargetType};

    #[test]
    fn test_select_encounter_target_sets_first_enemy() {
        let mut app = App::new();

        // Insert required resources
        let mut enemies = HashMap::new();
        let enemy_entity = app.world_mut().spawn_empty().id();
        enemies.insert(1, enemy_entity);
        app.world_mut().insert_resource(BattleCurrentEntities {
            need_patch: false,
            characters: HashMap::new(),
            enemies,
        });

        app.world_mut().insert_resource(BattleSelectedStatus {
            selected: Some((1, enemy_entity)),
            ..default()
        });

        // Run system
        app.update(); // Required to prepare world for systems
        app.add_systems(Startup, select_encounter_target);
        app.update();

        // Check if the enemy was selected and has the outline component
        let selected = app.world().resource::<BattleSelectedStatus>();
        assert_eq!(selected.selected.unwrap().1, enemy_entity, "Selected the first enemy");
    }

    #[test]
    fn test_set_observe_entities_spawns_child_marker() {
        let mut app = App::new();

        // Add mesh assets
        app.init_resource::<Assets<Mesh>>();

        // Spawn a mock observable entity
        let observe_entity = app.world_mut().spawn((ObserveAble,)).id();

        // Run system
        app.add_systems(Startup, set_observe_entities);
        app.update();

        // Check if the entity has children
        let children = app.world().get::<Children>(observe_entity);
        assert!(children.is_some(), "ObserveAble entity should have children");

        let marker_found = children.unwrap().iter().any(|child| {
            let entity = app.world().entity(*child);
            entity.contains::<SelectMarker>()
        });

        assert!(marker_found, "One child should have SelectMarker component");
    }

    #[test]
    fn test_detect_current_character_operation_with_single_target() {
        let mut app = App::new();

        // Setup world: selected operation is Single target
        let enemy_entity = app.world_mut().spawn_empty().id();

        let mut enemies = HashMap::new();
        enemies.insert(1, enemy_entity);

        app.world_mut().insert_resource(BattleCurrentEntities {
            need_patch: false,
            characters: HashMap::new(),
            enemies,
        });

        app.world_mut().insert_resource(BattleSelectedStatus::default());

        app.world_mut().insert_resource(TurnCurrentMemberInfo {
            character: None,
            selected_operation: None,
            pre_operation: Some(CharacterAbility {
                name: "Test".to_string(),
                family: AbilityType::Attack,
                selection_type: SelectionType::Single,
                target_type: TargetType::Enemy,
                scaling_type: ScalingType::Attack,
                scaling: 1.0,
                base_value: 5.0,
            }),
        });

        app.update();

        // Run system
        app.add_systems(Update, detect_current_character_operation);
        app.update();

        // Check if the selected was modified
        let selected = app.world().resource::<BattleSelectedStatus>();
        assert!(
            selected.selected.is_none(),
            "Expected none because no entity is currently selected"
        );
    }
}