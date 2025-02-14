use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::environment::{Area, Environment, EnvironmentListResource, EnvironmentState};

pub struct EnvInitPlugin;

impl Plugin for EnvInitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_environment_system);
    }
}

pub fn setup_environment_system(mut commands: Commands) {
    let environments = load_environments();
    commands.insert_resource(EnvironmentListResource(environments));
}

fn load_environments() -> HashMap<String, Environment> {
    let mut environments = HashMap::new();
    if let Ok(entries) = std::fs::read_dir("assets/environments") {
        for entry in entries.flatten() {
            if let Ok(file_name) = entry.file_name().into_string() {
                let areas = load_areas(file_name.as_str());
                let environment = Environment {
                    name: file_name.clone(),
                    loaded: false,
                    areas,
                    state: EnvironmentState::Exploring
                };
                environments.insert(file_name, environment);
            }
        }
    }
    environments
}

fn load_areas(folder: &str) -> HashMap<String, Area> {
    let mut areas = HashMap::new();
    if let Ok(contents) = std::fs::read_dir(format!("assets/environments/{}", folder)) {
        for (index, entry) in contents.flatten().enumerate() {
            if let Ok(file_name) = entry.file_name().into_string() {
                let area = Area {
                    name: file_name.clone(),
                    index,
                    player_in_bound: false,
                    battle_scenes: HashMap::new(),
                };
                areas.insert(file_name, area);
            }
        }
    }
    areas
}