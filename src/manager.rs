use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::{NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin};
use bevy_third_person_camera::ThirdPersonCameraPlugin;
use crate::entities::EntitiesPlugin;
use crate::environment::EnvironmentPlugin;
use crate::events::EventManagerPlugin;

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
        app.add_plugins(RapierDebugRenderPlugin::default());
        app.add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F3)));
        app.add_plugins(ThirdPersonCameraPlugin);
        app.add_plugins((EventManagerPlugin, EntitiesPlugin, EnvironmentPlugin));
    }
}

#[derive(Component, States, Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum GameState {
    SplashScreen,
    TitleScreen,
    AccountScreen,
    #[default]
    InGame
}