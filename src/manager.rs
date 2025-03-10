use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::{DebugRenderContext, NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin};
use audio_lib::audio::AudioHandlerPlugin;
use audio_lib::AudioStorePlugin;
use entities_lib::EntitiesPlugin;
use environment_lib::battle::BattleEnvironmentPlugin;
use system::config::{ConfigService, DummySaveData};
use system::states::GameState;
use environment_lib::environment::EnvironmentPlugin;
use system::characters::CharacterParty;
use system::commons::Character;
use system::events::EventManagerPlugin;
use system::service::ServicePlugin;
use crate::languages::LanguagesPlugin;
use crate::states::StatesPlugin;
use system::utils::key_code::convert;
use ui_lib::UiPlugin;

/// The main plugin responsible for initializing game states, resources, and sub-plugins.
pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        // Initialize the game state
        app.init_state::<GameState>();

        // Insert global configuration resource
        app.insert_resource(ConfigService::new());
        app.insert_resource(WorldInspectorState::default());
        app.insert_resource(DummySaveData::default());
        app.insert_resource(CharacterParty {
            team_leader: Character::default(),
            members: HashMap::new()
        });

        // Add various game-related plugins
        app.add_plugins(LanguagesPlugin);
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
        app.add_plugins(RapierDebugRenderPlugin {
            enabled: false,
            ..default()
        });
        app.add_plugins(WorldInspectorPlugin::default().run_if(check_world_inspector_state));
        app.add_plugins((AudioStorePlugin, AudioHandlerPlugin));
        app.add_plugins((StatesPlugin, EventManagerPlugin, EntitiesPlugin,
                         EnvironmentPlugin, BattleEnvironmentPlugin, ServicePlugin, UiPlugin));
        app.add_systems(Update, (toggle_debug_system, toggle_world_inspector_interface_system));
    }
}

/// Represents the state of the World Inspector UI.
///
/// This resource holds a single boolean value indicating whether the World Inspector UI
/// is currently visible or hidden. The state can be toggled by user input (e.g., a key press),
/// and this struct is used to track the visibility of the World Inspector in the application.
///
/// The `WorldInspectorState` is initialized to `false` (hidden) by default.
///
/// # Fields
///
/// * `0`: A boolean value that represents the visibility of the World Inspector UI.
///   - `true`: The World Inspector is visible.
///   - `false`: The World Inspector is hidden.
#[derive(Resource, Default, Debug)]
pub struct WorldInspectorState(pub bool);

/// Toggles the debug rendering system on or off based on a configured key input.
///
/// # Arguments
/// * `debug_context` - A mutable reference to the debug rendering context.
/// * `keyboard` - A resource representing the current state of keyboard inputs.
/// * `general_config` - A resource containing the game's configuration settings.
fn toggle_debug_system(
    mut debug_context: ResMut<DebugRenderContext>,
    keyboard: ResMut<ButtonInput<KeyCode>>,
    general_config: Res<ConfigService>,
) {
    let key = convert(general_config.input_config.debug_change.as_str())
        .expect("Fetch key for (debug change) was failed!");
    if keyboard.just_pressed(key) {
        debug_context.enabled = !debug_context.enabled
    }
}

/// Toggles the state of the World Inspector UI based on a key press.
///
/// This system checks if the configured key (from `ConfigService`) is pressed.
/// If the key is pressed, it inverts the current state of the `WorldInspectorState`
/// (i.e., toggles whether the World Inspector is visible or not).
///
/// # Arguments
///
/// * `keyboard`: A resource containing the current button input state for key presses.
/// * `general_config`: A resource containing the configuration settings, including the key for toggling the world inspector UI.
/// * `world_inspector_state`: A mutable reference to the state of the world inspector UI, which will be toggled.
///
/// # Panics
///
/// This function will panic if the key from `ConfigService` cannot be converted into a valid `KeyCode`.
fn toggle_world_inspector_interface_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    general_config: Res<ConfigService>,
    mut world_inspector_state: ResMut<WorldInspectorState>,
) {
    let key = convert(general_config.input_config.world_inspector_ui.as_str())
        .expect("Fetch key for (world inspector ui) was failed!");

    if keyboard.just_pressed(key) {
        world_inspector_state.0 = !world_inspector_state.0;
    }
}

/// Checks whether the World Inspector UI is currently enabled or not.
///
/// This function simply checks the state of the `WorldInspectorState`
/// and returns a boolean indicating whether the World Inspector UI is visible.
///
/// # Arguments
///
/// * `world_inspector_state`: A reference to the state of the world inspector UI.
///
/// # Returns
///
/// * `true` if the World Inspector UI is visible (enabled).
/// * `false` if the World Inspector UI is not visible (disabled).
fn check_world_inspector_state(
    world_inspector_state: Res<WorldInspectorState>,
) -> bool {
    world_inspector_state.0
}