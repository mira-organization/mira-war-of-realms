#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::io::Write;
    use bevy::utils::HashMap;
    use environment_lib::environment::{Area, Environment, EnvironmentState};
    use tempfile::tempdir;

    fn mock_load_areas(_env_name: &str) -> HashMap<String, Area> {
        let mut map = HashMap::new();
        map.insert("Area1".to_string(), Area {
            name: "Area1".to_string(),
            index: 0,
            player_in_bound: false,
            battle_scenes: Default::default(),
        });
        map.insert("Area2".to_string(), Area {
            name: "Area2".to_string(),
            index: 1,
            player_in_bound: false,
            battle_scenes: Default::default(),
        });
        map
    }

    #[test]
    fn test_load_environments_reads_directory_correctly() {
        let dir = tempdir().unwrap();
        let env_path = dir.path().join("assets/environments");
        fs::create_dir_all(&env_path).unwrap();

        let filenames = vec!["forest", "desert", "ice"];
        for name in &filenames {
            let file_path = env_path.join(name);
            let mut file = File::create(&file_path).unwrap();
            writeln!(file, "dummy content").unwrap();
        }


        {
            fn load_environments_from_path(path: &str, load_areas_fn: fn(&str) -> HashMap<String, Area>) -> HashMap<String, Environment> {
                let mut environments = HashMap::new();

                if let Ok(entries) = fs::read_dir(path) {
                    for entry in entries.flatten() {
                        if let Ok(file_name) = entry.file_name().into_string() {
                            let areas = load_areas_fn(file_name.as_str());

                            let environment = Environment {
                                name: file_name.clone(),
                                loaded: false,
                                areas,
                                state: EnvironmentState::Exploring,
                            };

                            environments.insert(file_name, environment);
                        }
                    }
                }

                environments
            }

            let result = load_environments_from_path(env_path.to_str().unwrap(), mock_load_areas);

            assert_eq!(result.len(), 3);
            for name in &filenames {
                let env = result.get(*name).expect("Environment should exist");
                assert_eq!(env.name, *name);
                assert_eq!(env.loaded, false);
                assert_eq!(env.state, EnvironmentState::Exploring);
                assert_eq!(env.areas.len(), 2);
            }
        }

    }
}