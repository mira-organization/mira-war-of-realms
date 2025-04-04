#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use bevy::utils::HashMap;
    use battle_lib::setup::{setup_battle_entities, update_enemy_position};
    use system::battle_commons::{BattleCurrentEntities, BattleMember};
    use system::commons::Character;

    #[test]
    fn test_update_enemy_position() {
        let mut app = App::new();

        // Setup world with BattleCurrentEntities and Enemies
        app.insert_resource(BattleCurrentEntities {
            need_patch: true,
            characters: HashMap::new(),
            enemies: HashMap::new()
        });

        // Spawn mock enemies with positions
        let enemy1 = app.world_mut().spawn(Transform::from_xyz(0.0, 0.0, 0.0)).id();
        let enemy2 = app.world_mut().spawn(Transform::from_xyz(5.0, 0.0, 0.0)).id();
        app.world_mut().insert_resource(BattleCurrentEntities {
            need_patch: true,
            characters: HashMap::new(),
            enemies: HashMap::from([
                (1, enemy1),
                (2, enemy2),
            ]),
        });

        // Run the update system
        app.update();
        app.add_systems(Update, update_enemy_position);
        app.update();

        // Check if positions are updated
        let mut transforms = app.world_mut().query::<&Transform>();
        let transform1 = transforms.get(app.world(), enemy1).unwrap();
        let transform2 = transforms.get(app.world(), enemy2).unwrap();

        assert_eq!(transform1.translation.x, 0.0);
        assert_eq!(transform2.translation.x, 5.0); // Expecting it to be slightly offset
    }

    #[test]
    fn test_setup_battle_entities() {
        let mut app = App::new();

        // Setup entities
        let _character_entity = app.world_mut().spawn_empty().insert((Character::default(), BattleMember)).id();
        let _enemy_entity = app.world_mut().spawn_empty().insert(BattleMember).id();

        let battle_entities = BattleCurrentEntities {
            need_patch: false,
            characters: HashMap::new(),
            enemies: HashMap::new(),
        };

        // Add characters and enemies
        app.world_mut().insert_resource(battle_entities);

        // Run the setup system
        app.add_systems(Startup, setup_battle_entities);
        app.update();

        let battle_entities = app.world_mut().resource::<BattleCurrentEntities>();

        // Verify that entities were added correctly
        assert_eq!(battle_entities.characters.contains_key(&1), true);
        assert_eq!(battle_entities.enemies.contains_key(&1), true);
    }
}