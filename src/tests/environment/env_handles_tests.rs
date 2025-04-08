#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use bevy::render::view::NoFrustumCulling;
    use bevy::scene::ScenePlugin;
    use bevy::utils::HashMap;
    use bevy_rapier3d::prelude::{AsyncSceneCollider, RigidBody};
    use environment_lib::environment::{BattleEnvironment, CurrentAreaScenes, EnvironmentScene};
    use environment_lib::environment::env_handles::{load_battle_scene, pre_load_battle, temp_swap_to_main};
    use system::battle_commons::{BattleMember, InBattle};
    use system::commons::{BeforeBattleLocation, ToRemoveAfterBattle, WorldPlayer};
    use system::events::world_events::WorldEntityHitEntityEvent;
    use system::states::GameState;

    #[test]
    fn test_pre_load_battle_triggers_correctly() {
        let mut app = App::new();

        app.add_plugins(MinimalPlugins);

        // Add needed event and state resources
        app.add_event::<WorldEntityHitEntityEvent>();
        app.insert_resource(NextState::<GameState>::default());

        // Create dummy area scenes with >3 entries
        let mut dummy_scenes = HashMap::new();
        dummy_scenes.insert("layer_0".into(), Handle::<Scene>::weak_from_u128(1));
        dummy_scenes.insert("layer_1".into(), Handle::<Scene>::weak_from_u128(2));
        dummy_scenes.insert("layer_2".into(), Handle::<Scene>::weak_from_u128(3));
        dummy_scenes.insert("layer_3".into(), Handle::<Scene>::weak_from_u128(4));
        app.insert_resource(CurrentAreaScenes(dummy_scenes));

        // Spawn dummy player and enemy
        let player = app.world_mut().spawn((WorldPlayer::default(), Transform::from_xyz(1.0, 2.0, 3.0))).id();
        let enemy = app.world_mut().spawn(Transform::default()).id();

        // Send a hit event
        app.world_mut().send_event(WorldEntityHitEntityEvent {
            sender: player,
            entity: enemy,
        });

        // Add system
        app.add_systems(Update, pre_load_battle);
        app.update();

        let world = app.world_mut();

        // Player should have InBattle, enemy should have ToRemoveAfterBattle
        assert!(world.get::<InBattle>(player).is_some());
        assert!(world.get::<ToRemoveAfterBattle>(enemy).is_some());

        // BeforeBattleLocation should be set to player position
        let loc = world.get_resource::<BeforeBattleLocation>().unwrap();
        assert_eq!(loc.0, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_pre_load_battle_does_nothing_when_not_enough_scenes() {
        let mut app = App::new();

        app.add_plugins(MinimalPlugins);
        app.add_event::<WorldEntityHitEntityEvent>();
        app.insert_resource(NextState::<GameState>::default());

        // Not enough scenes
        let mut dummy_scenes = HashMap::new();
        dummy_scenes.insert("layer_0".into(), Handle::<Scene>::weak_from_u128(1));
        dummy_scenes.insert("layer_1".into(), Handle::<Scene>::weak_from_u128(2));
        app.insert_resource(CurrentAreaScenes(dummy_scenes));

        // Spawn entities
        let player = app.world_mut().spawn((WorldPlayer::default(), Transform::from_xyz(1.0, 2.0, 3.0))).id();
        let enemy = app.world_mut().spawn(Transform::default()).id();

        app.world_mut().send_event(WorldEntityHitEntityEvent {
            sender: player,
            entity: enemy,
        });

        app.add_systems(Update, pre_load_battle);
        app.update();

        let world = app.world_mut();

        // Should not be inserted
        assert!(world.get::<InBattle>(player).is_none());
        assert!(world.get::<ToRemoveAfterBattle>(enemy).is_none());
    }

    #[test]
    fn test_load_battle_scene_spawns_correctly() {
        let mut app = App::new();

        app.add_plugins((MinimalPlugins, AssetPlugin::default(), ScenePlugin::default()));
        let _ = app.world_mut().resource::<AssetServer>();

        // Create dummy battle scene handle
        let battle_handle = Handle::<Scene>::weak_from_u128(999);
        let mut scenes = HashMap::new();
        scenes.insert("battle_1".to_string(), battle_handle.clone());

        app.insert_resource(CurrentAreaScenes(scenes));
        app.add_systems(Update, load_battle_scene);
        app.update();

        let world = app.world_mut();
        let mut query = world.query::<(
            &SceneRoot,
            &Name,
            &EnvironmentScene,
            &NoFrustumCulling,
            &RigidBody,
            &BattleEnvironment,
            &AsyncSceneCollider,
        )>();

        let mut found = false;

        for (scene_root, name, _, _, rb, _, _collider) in query.iter(world) {
            assert_eq!(scene_root.0, battle_handle);
            assert_eq!(name.as_str(), "Battle Scene");
            assert_eq!(*rb, RigidBody::Fixed);
            found = true;
        }

        assert!(found, "Battle scene was not spawned");
    }

    #[test]
    fn test_temp_swap_to_main_restores_player_and_cleans_battle() {

        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        let player_entity = app.world_mut().spawn((
            InBattle,
            WorldPlayer::default(),
            Transform::from_xyz(10.0, 0.0, 5.0),
        )).id();

        let enemy_entity = app.world_mut().spawn(ToRemoveAfterBattle).id();

        let battle_member = app.world_mut().spawn(BattleMember).id();

        let battle_env = app.world_mut().spawn(BattleEnvironment).id();

        let return_position = Vec3::new(1.0, 0.0, 1.0);
        app.insert_resource(BeforeBattleLocation(return_position));

        app.init_resource::<NextState<GameState>>();

        app.add_systems(Update, temp_swap_to_main);
        app.update();

        let world = app.world_mut();

        let transform = world.get::<Transform>(player_entity).unwrap();
        assert_eq!(transform.translation, return_position);
        assert!(world.get::<InBattle>(player_entity).is_none());

        assert!(!world.get_entity(enemy_entity).is_ok());
        assert!(!world.get_entity(battle_member).is_ok());
        assert!(!world.get_entity(battle_env).is_ok());

    }
}