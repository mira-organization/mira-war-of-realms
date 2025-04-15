mod screens;
mod elements;
pub mod colors;

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use system::states::GameState;
use crate::elements::ElementPlugin;
use crate::screens::UIScreenPlugin;

static UI_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

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

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct UiElementState {
    pub hovered: bool,
    pub selected: bool,
    pub enabled: bool
}

impl Default for UiElementState {
    fn default() -> Self {
        Self {
            hovered: false,
            selected: false,
            enabled: true,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct UiGenID(usize);

impl Default for UiGenID {
    fn default() -> Self {
        Self(UI_ID_COUNTER.fetch_add(1, Relaxed))
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
        app.register_type::<UiElementState>();
        app.add_plugins((ElementPlugin, UIScreenPlugin));

        app.add_systems(
            OnEnter(GameState::PreLoad),
            setup_ui_camera,
        );

        app.add_systems(Update, handle_input_focus);
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

fn handle_input_focus(
    mut query: Query<(&mut UiElementState, &UiGenID, Entity)>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let mut sorted_ui_elements: Vec<_> = query.iter_mut().collect();
    sorted_ui_elements.sort_by_key(|(_, id, _)| id.0);

    let mut any_focused = false;

    // TAB Focus
    if keyboard.just_pressed(KeyCode::Tab) {
        let len = sorted_ui_elements.len();
        if len == 0 {
            return;
        }

        // Find the currently focused field and move the focus to the next one
        for i in 0..len {
            if sorted_ui_elements[i].0.selected {
                sorted_ui_elements[i].0.selected = false;

                let next = (i + 1) % len;  // Get the next field in the list

                if let Some(&mut (_, _, _)) = sorted_ui_elements.get_mut(next) { // Set the focused color
                    sorted_ui_elements[next].0.selected = true; // Set focus to the next field
                    any_focused = true;
                }

                break;  // Exit the loop after setting the next focus
            }
        }

        // If no focus was found, set the first field as focused
        if !any_focused && len > 0 {
            sorted_ui_elements[0].0.selected = true;
        }
    }
}
