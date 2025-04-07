#[cfg(test)]
mod tests {
    use bevy::asset::io::{AssetSource, AssetSourceId};
    use bevy::asset::io::memory::{Dir, MemoryAssetReader};
    use bevy::gltf::GltfPlugin;
    use bevy::prelude::*;
    use bevy::utils::HashMap;
    use environment_lib::environment::{Area, CurrentAreaScenes, CurrentEnvironment, EffectSceneAssets, Environment, EnvironmentListResource, EnvironmentState, WaitingForAreaAssets};
    use environment_lib::environment::ready_up_handles::{pre_load_area, pre_load_environments, pre_load_gltf_assets, process_loaded_area};
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

        app.add_plugins((MinimalPlugins, AssetPlugin { file_path: "src/test/assets".to_string(), ..default() }, GltfPlugin::default()));

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
}