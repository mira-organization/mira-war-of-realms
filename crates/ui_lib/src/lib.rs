mod screens;
mod elements;
pub mod colors;

use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use system::states::GameState;
use crate::elements::ElementPlugin;
use crate::screens::UIScreenPlugin;

#[derive(Reflect, Default, Clone, PartialEq, Debug)]
pub struct Radius {
    pub top_left: Val,
    pub top_right: Val,
    pub bottom_left: Val,
    pub bottom_right: Val,
}

impl Radius {
    pub fn all(val: Val) -> Self {
        Self {
            top_left: val,
            top_right: val,
            bottom_left: val,
            bottom_right: val
        }
    }
}

/// The `UiPlugin` struct implements the `Plugin` trait and is responsible for
/// setting up the UI-related components in the game.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    /// Builds the UI plugin by adding necessary UI components and systems.
    ///
    /// - Registers the `UiLunexPlugins`.
    /// - Optionally adds `UiLunexDebugPlugin` if debug mode is enabled.
    /// - Adds systems for entering and exiting the `InGameState::Main` state.
    ///
    /// # Parameters
    /// - `app`: Mutable reference to the `App` where systems and plugins are added.
    fn build(&self, app: &mut App) {
        app.add_plugins((ElementPlugin, UIScreenPlugin));

        app.add_systems(
            OnEnter(GameState::PreLoad),
            setup_ui_camera,
        );
    }
}

/// Spawns the UI camera responsible for rendering the UI elements.
///
/// - The camera uses HDR and a specific render order.
/// - It applies a bloom effect for a more stylized appearance.
/// - It is positioned far in the Z direction to ensure proper rendering.
///
/// # Parameters
/// - `commands`: Command buffer for spawning entities.
fn setup_ui_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("UI Camera"),
        Camera2d,
        Camera {
            hdr: true,
            order: 2,
            ..default()
        },
        RenderLayers::from_layers(&[1, 2]),
        Transform::from_translation(Vec3::Z * 1000.0),
    ));
}