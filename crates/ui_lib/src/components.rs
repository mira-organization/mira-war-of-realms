use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::window::SystemCursorIcon;
use bevy_lunex::{Ab, OnHoverSetCursor, Rl, UiBase, UiColor, UiHover, UiLayout, UiLayoutRoot, UiStateTrait};

/// Marker component for the main HUD.
/// Used to identify and manage the HUD entity.
#[derive(Component)]
pub struct MainHudMarker;

/// Bundle representing the main HUD layout.
///
/// This bundle contains:
/// - A `Name` component for identifying the HUD.
/// - A `UiLayoutRoot` for defining the root layout of the UI.
#[derive(Bundle)]
pub struct HudBundle {
    pub name: Name,
    pub root_layout: UiLayoutRoot,
}

impl Default for HudBundle {
    /// Creates a default `HudBundle` with a root layout in 2D space.
    ///
    /// # Returns
    /// A new instance of `HudBundle`.
    fn default() -> Self {
        Self {
            name: Name::new("Main Hud"),
            root_layout: UiLayoutRoot::new_2d(),
        }
    }
}

/// Bundle representing a toolbar UI element.
///
/// This bundle includes:
/// - A `Name` component for identification.
/// - A `UiLayout` component defining its position.
/// - A `UiColor` component for appearance.
/// - A `Sprite` component for rendering.
/// - A `RenderLayers` component to control rendering order.
#[derive(Bundle)]
pub struct ToolbarBundle {
    pub name: Name,
    pub ui_layout: UiLayout,
    pub ui_color: UiColor,
    pub sprite: Sprite,
    pub layer: RenderLayers,
}

impl Default for ToolbarBundle {
    /// Creates a default `ToolbarBundle` with a black transparent background
    /// and positioned at the bottom center of the screen.
    ///
    /// # Returns
    /// A new instance of `ToolbarBundle`.
    fn default() -> Self {
        Self {
            name: Name::new("Toolbar"),
            ui_layout: UiLayout::boundary()
                .pos1((Rl(100.0) - Ab(320.0), Ab(20.0)))
                .pos2((Rl(100.0) - Ab(20.0), Ab(70.0)))
                .pack(),
            ui_color: UiColor::from(Color::srgb(0.0, 0.0, 0.0).with_alpha(0.0)),
            sprite: Sprite::default(),
            layer: RenderLayers::layer(2),
        }
    }
}

/// Bundle representing an interactive UI icon.
///
/// This bundle includes:
/// - A `Name` for identification.
/// - A `UiLayout` defining position and size.
/// - A `UiColor` defining color states.
/// - A `Sprite` representing the icon image.
/// - A `RenderLayers` component for rendering order.
/// - An `OnHoverSetCursor` component to change the cursor when hovered.
#[derive(Bundle)]
pub struct IconBundle {
    pub name: Name,
    pub ui_layout: UiLayout,
    pub ui_color: UiColor,
    pub sprite: Sprite,
    pub layer: RenderLayers,
    pub on_hover_set_cursor: OnHoverSetCursor,
}

impl IconBundle {
    /// Creates a new `IconBundle` with the given parameters.
    ///
    /// # Parameters
    /// - `index`: The index of the icon in the toolbar.
    /// - `asset`: Handle to the image used as the icon.
    /// - `space`: The spacing between icons.
    /// - `size`: The size of the icon.
    /// - `toolbar_width`: The width of the toolbar.
    /// - `toolbar_height`: The height of the toolbar.
    ///
    /// # Returns
    /// A new instance of `IconBundle` with computed positions.
    pub fn new(index: usize, asset: Handle<Image>, space: f32, size: f32, toolbar_width: f32, toolbar_height: f32) -> Self {
        let total_width = 3.0 * space;
        let start_x = (toolbar_width - total_width) / 2.0;

        let x_offset = start_x + (index as f32 * space);
        let y_offset = (toolbar_height - size) / 2.0;

        Self {
            name: Name::new(format!("Icon-{}", index)),
            ui_layout: UiLayout::boundary()
                .pos1((Rl(100.0) - Ab(300.0) + Ab(x_offset), Ab(0.0) + Ab(y_offset)))
                .pos2((Rl(100.0) - Ab(300.0) + Ab(x_offset + size), Ab(y_offset + size)))
                .pack(),
            ui_color: UiColor::new(vec![
                (UiBase::id(), Color::srgb(1.0, 1.0, 1.0).with_alpha(1.0)),
                (UiHover::id(), Color::srgb(0.8, 0.8, 0.8).with_alpha(1.0)),
            ]),
            sprite: Sprite::from(asset),
            layer: RenderLayers::layer(2),
            on_hover_set_cursor: OnHoverSetCursor::new(SystemCursorIcon::Pointer),
        }
    }
}




