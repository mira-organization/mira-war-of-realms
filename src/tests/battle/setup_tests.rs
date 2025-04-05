#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use bevy::utils::HashMap;
    use battle_lib::setup::{setup_battle_entities, spawn_entities, update_enemy_position};
    use system::battle_commons::{BattleCurrentEntities, BattleMember, InBattle};
    use system::characters::CharacterParty;
    use system::commons::{Character, WorldPlayer};

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
        let character1 = app.world_mut().spawn(Transform::from_xyz(0.0, 0.0, 5.0)).id();
        app.world_mut().insert_resource(BattleCurrentEntities {
            need_patch: true,
            characters: HashMap::from([(1, character1)]),
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

        assert_eq!(transform1.translation.x, -10.0);
        assert_eq!(transform2.translation.x, -7.5); // Expecting it to be slightly offset
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

    #[test]
    fn test_spawn_entities() {
        // Setup minimal Bevy app
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()));
        // Add required resources and components
        let _ = app.world_mut().resource::<AssetServer>();
        app.init_asset::<Scene>();

        // Create dummy characters
        let character_main = Character {
            name: "Hero".to_string(),
            ..Default::default()
        };

        let character_other = Character {
            name: "Ally".to_string(),
            ..Default::default()
        };

        let mut party = HashMap::new();
        party.insert(0, character_main.clone());
        party.insert(1, character_other.clone());

        app.insert_resource(CharacterParty {
            team_leader: character_main.clone(),
            members: party
        });

        // Spawn the player entity
        let player_entity = app.world_mut().spawn((
            InBattle,
            WorldPlayer {
                displayed_character: character_main.clone(),
                ..Default::default()
            },
            Transform::default(),
        )).id();

        app.add_systems(Startup, spawn_entities);
        app.update();

        // Assert the player now has the BattleMember component
        let has_battle_member = app.world().get::<BattleMember>(player_entity).is_some();
        assert!(has_battle_member, "Player should have BattleMember component");

        // Assert party member was spawned
        let party_members: Vec<_> = app.world_mut()
            .query_filtered::<&Name, With<Transform>>()
            .iter(&app.world())
            .filter(|n| n.as_str() == "Ally")
            .collect();
        assert_eq!(party_members.len(), 1, "One additional party member should be spawned");

        // Assert enemies were spawned
        let enemies: Vec<_> = app.world_mut()
            .query_filtered::<&Name, With<Transform>>()
            .iter(&app.world())
            .filter(|n| n.as_str().starts_with("Enemy"))
            .collect();
        assert_eq!(enemies.len(), 4, "Four enemies should be spawned");
    }
}