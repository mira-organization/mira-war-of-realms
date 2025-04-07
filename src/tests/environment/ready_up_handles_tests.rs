#[cfg(test)]
mod tests {
    use bevy::asset::io::{AssetSource, AssetSourceId};
    use bevy::asset::io::memory::{Dir, MemoryAssetReader};
    use bevy::gltf::{GltfNode, GltfPlugin};
    use bevy::prelude::*;
    use bevy::render::mesh::MeshPlugin;
    use bevy::scene::ScenePlugin;
    use bevy::utils::HashMap;
    use bevy_rapier3d::plugin::NoUserData;
    use bevy_rapier3d::prelude::{AsyncSceneCollider, RapierPhysicsPlugin};
    use environment_lib::environment::{Area, CurrentAreaScenes, CurrentEnvironment, EffectSceneAssets, Environment, EnvironmentListResource, EnvironmentScene, EnvironmentState, WaitingForAreaAssets};
    use environment_lib::environment::ready_up_handles::{load_active_area, load_active_area_lights, pre_load_area, pre_load_environments, pre_load_gltf_assets, process_loaded_area};
    use system::config::DummySaveData;
    use system::states::GameState;

    #[test]
    fn test_pre_load_environments() {
        let mut app = App::new();

        app.add_plugins((MinimalPlugins, AssetPlugin::default()));
        let _asset_server = app.world_mut().resource::<AssetServer>();
        app.insert_resource(CurrentEnvironment {
            environment: Environment {
                loaded: false,
                name: "Debug".to_string(),
                state: EnvironmentState::Exploring,
                areas: HashMap::new()
            },
            area: Area {
                index: 0,
                name: "Debug Area".to_string(),
                battle_scenes: HashMap::new(),
                player_in_bound: false
            }
        });

        let env_map = vec![
            ("env1".to_string(), Environment {
                name: "Environment 1".to_string(),
                loaded: false,
                areas: vec![
                    ("area1".to_string(), Area {
                        index: 0,
                        player_in_bound: false,
                        name: "Area 1".to_string(),
                        battle_scenes: Default::default(),
                    }),
                    ("area2".to_string(), Area {
                        index: 1,
                        player_in_bound: false,
                        name: "Area 2".to_string(),
                        battle_scenes: Default::default(),
                    }),
                ].into_iter().collect(),
                state: EnvironmentState::Exploring,
            }),
        ].into_iter().collect::<HashMap<String, Environment>>();

        app.insert_resource(EnvironmentListResource(env_map));

        let dummy_save_data = DummySaveData {
            current_environment: "env1".to_string(),
            current_area: 0,
            ..default()
        };
        app.insert_resource(dummy_save_data);

        app.insert_resource(NextState::<GameState>::default());

        app.add_systems(Startup, pre_load_environments);

        app.update();

        let current_env = app.world().resource::<CurrentEnvironment>();
        assert_eq!(current_env.environment.name, "Environment 1");
        assert_eq!(current_env.area.name, "Area 1");
    }

    #[test]
    fn test_pre_load_environments_empty() {
        let mut app = App::new();

        app.add_plugins((MinimalPlugins, AssetPlugin::default()));
        let _asset_server = app.world_mut().resource::<AssetServer>();
        app.insert_resource(CurrentEnvironment {
            environment: Environment {
                loaded: false,
                name: "Debug Empty".to_string(),
                state: EnvironmentState::Exploring,
                areas: HashMap::new()
            },
            area: Area {
                index: 0,
                name: "Debug Area".to_string(),
                battle_scenes: HashMap::new(),
                player_in_bound: false
            }
        });

        app.insert_resource(EnvironmentListResource(HashMap::new()));

        let dummy_save_data = DummySaveData {
            current_environment: "env1".to_string(),
            current_area: 0,
            ..default()
        };
        app.insert_resource(dummy_save_data);

        app.insert_resource(NextState::<GameState>::default());

        app.add_systems(Startup, pre_load_environments);

        app.update();

        let current_env = app.world().resource::<CurrentEnvironment>();
        assert_eq!(current_env.environment.name, "Debug Empty");
        assert_eq!(current_env.area.name, "Debug Area");
    }

    #[test]
    fn test_pre_load_area() {
        let mut app = App::new();

        app.add_plugins((MinimalPlugins, AssetPlugin::default(), GltfPlugin::default()));
        let asset_server = app.world_mut().resource::<AssetServer>().clone();
        app.insert_resource(WaitingForAreaAssets(Handle::weak_from_u128(999)));

        let area = Area {
            index: 0,
            player_in_bound: false,
            name: "Area 1".to_string(),
            battle_scenes: Default::default(),
        };

        let environment = Environment {
            name: "Environment 1".to_string(),
            loaded: false,
            areas: vec![
                ("area1".to_string(), area.clone()),
            ].into_iter().collect(),
            state: EnvironmentState::Exploring,
        };

        app.insert_resource(CurrentEnvironment {
            environment,
            area,
        });

        app.add_systems(Startup, pre_load_area);
        app.update();

        let waiting_for_area_assets = app.world().resource::<WaitingForAreaAssets>();

        let path = format!("environments/{}/{}", "Environment 1", "Area 1");
        let expected_handle = asset_server.load::<Gltf>(path.as_str());
        assert_eq!(waiting_for_area_assets.0, expected_handle);
    }

    #[test]
    fn test_pre_load_gltf_assets() {
        let mut app = App::new();

        app.add_plugins((MinimalPlugins, AssetPlugin::default(), GltfPlugin::default()));
        let asset_server = app.world_mut().resource::<AssetServer>().clone();
        app.insert_resource(EffectSceneAssets(Handle::weak_from_u128(999)));

        let area = Area {
            index: 0,
            player_in_bound: false,
            name: "Area 1".to_string(),
            battle_scenes: Default::default(),
        };

        let environment = Environment {
            name: "Environment 1".to_string(),
            loaded: false,
            areas: vec![
                ("area1".to_string(), area.clone()),
            ].into_iter().collect(),
            state: EnvironmentState::Exploring,
        };

        app.insert_resource(CurrentEnvironment {
            environment,
            area,
        });

        app.add_systems(Startup, pre_load_gltf_assets);
        app.update();

        let effect_scene_assets = app.world().resource::<EffectSceneAssets>();

        let path = format!("environments/{}/{}", "Environment 1", "Area 1");
        let expected_handle = asset_server.load::<Gltf>(path.as_str());
        assert_eq!(effect_scene_assets.0, expected_handle);
    }

    #[test]
    fn test_process_loaded_area() {
        let reader = MemoryAssetReader { root: Dir::default() };
        let mut app = App::new();
        app.register_asset_source(
            AssetSourceId::Default,
            AssetSource::build().with_reader(move || Box::new(reader.clone())),
        );

        app.add_plugins((MinimalPlugins, AssetPlugin { file_path: "assets_test".to_string(), ..default() }, GltfPlugin::default()));

        app.insert_resource(CurrentAreaScenes(HashMap::new()));
        app.insert_resource(NextState::<GameState>::default());

        let asset_server = app.world_mut().resource::<AssetServer>();
        let gltf_path = "test_area.glb";
        let gltf_handle: Handle<Gltf> = asset_server.load(gltf_path);
        app.update();

        app.insert_resource(WaitingForAreaAssets(gltf_handle.clone()));

        app.add_systems(Update, process_loaded_area);
        app.update();

        let current_area_scenes = app.world().resource::<CurrentAreaScenes>();
        let scenes = &current_area_scenes.0;

        assert_eq!(scenes.len(), 0);
/*        assert!(scenes.contains_key("layer_0"));
        assert!(scenes.contains_key("layer_1"));
        assert!(scenes.contains_key("layer_2"));
        assert!(scenes.contains_key("battle_1"));*/
    }

    #[test]
    fn test_load_active_area_spawns_entities() {
        let mut app = App::new();

        app.add_plugins((
            MinimalPlugins,
            AssetPlugin::default(),
            GltfPlugin::default(),
            MeshPlugin,
            ScenePlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::default()));

        // Dummy Handles
        let handle_0 = Handle::weak_from_u128(111);
        let handle_1 = Handle::weak_from_u128(222);
        let handle_2 = Handle::weak_from_u128(333);

        let mut scenes = HashMap::new();
        scenes.insert("layer_0".to_string(), handle_0.clone());
        scenes.insert("layer_1".to_string(), handle_1.clone());
        scenes.insert("layer_2".to_string(), handle_2.clone());

        app.insert_resource(CurrentAreaScenes(scenes));

        app.add_systems(Update, load_active_area);
        app.update();

        let world = app.world_mut();

        let mut layer_0_found = false;
        let mut layer_1_found = false;
        let mut layer_2_found = false;

        let mut query = world.query::<(Entity, &SceneRoot, &Name, &EnvironmentScene)>();
        for (entity, scene_root, name, _) in query.iter(world) {
            match name.as_str() {
                "Area First Layer" => {
                    assert_eq!(scene_root.0, handle_0);
                    layer_0_found = true;

                    let collider = world.get::<AsyncSceneCollider>(entity);
                    assert!(collider.is_some(), "Collider missing on Area First Layer");
                },
                "Area Second Layer" => {
                    assert_eq!(scene_root.0, handle_1);
                    layer_1_found = true;
                },
                "Area Last Layer" => {
                    assert_eq!(scene_root.0, handle_2);
                    layer_2_found = true;

                    let collider = world.get::<AsyncSceneCollider>(entity);
                    assert!(collider.is_some(), "Collider missing on Area Last Layer");
                },
                _ => {}
            }
        }

        assert!(layer_0_found, "Layer 0 not spawned");
        assert!(layer_1_found, "Layer 1 not spawned");
        assert!(layer_2_found, "Layer 2 not spawned");
    }

    #[test]
    fn test_load_active_area_lights() {
        let mut app = App::new();

        app.add_plugins(MinimalPlugins);

        // Add dummy game state resources
        app.insert_resource(NextState::<GameState>::default());

        // Dummy GLTF handle
        let gltf_handle = Handle::<Gltf>::weak_from_u128(1111);

        // Insert EffectSceneAssets resource
        app.insert_resource(EffectSceneAssets(gltf_handle.clone()));

        // Create dummy Gltf with node pointing to a light
        let mut gltf_assets = Assets::<Gltf>::default();
        let mut gltf_nodes = Assets::<GltfNode>::default();

        let _light_entity = Entity::from_raw(42);

        // Add dummy GltfNode with a light
        let node_handle = Handle::<GltfNode>::weak_from_u128(2222);
        gltf_nodes.insert(node_handle.clone().id(), GltfNode {
            index: 1,
            name: "Test Light".to_string(),
            mesh: None,
            skin: None,
            transform: Default::default(),
            is_animation_root: false,
            children: vec![],
            extras: Some(GltfExtras {
                value: "{ \"name\": \"point\", \"intensity\": 150000.0, \"range\": 10.0, \"radius\": 3.5 , \"color\": [ 0.7, 0.0, 0.8 ], \"shadows\": true }".to_string(),
            }),
        });

        gltf_nodes.insert(node_handle.clone().id(), GltfNode {
            index: 2,
            name: "Test Light 1".to_string(),
            mesh: None,
            skin: None,
            transform: Default::default(),
            is_animation_root: false,
            children: vec![],
            extras: Some(GltfExtras {
                value: "{ \"name\": \"spot\", \"intensity\": 150000.0, \"range\": 10.0, \"radius\": 3.5 , \"color\": [ 0.7, 0.0, 0.8 ], \"shadows\": true, \"inner_cone\": 0.1, \"outer_cone\": 0.5 }".to_string(),
            }),
        });

        // Add Gltf referencing this node
        gltf_assets.insert(gltf_handle.clone().id(), Gltf {
            scenes: vec![],
            named_scenes: Default::default(),
            meshes: vec![],
            named_meshes: Default::default(),
            materials: vec![],
            named_materials: Default::default(),
            nodes: vec![node_handle],
            named_nodes: Default::default(),
            skins: vec![],
            named_skins: Default::default(),
            default_scene: None,
            animations: vec![],
            named_animations: Default::default(),
            source: None,
        });

        app.insert_resource(gltf_assets);
        app.insert_resource(gltf_nodes);

        // Add system and run
        app.add_systems(Update, load_active_area_lights);
        app.update();
    }
}