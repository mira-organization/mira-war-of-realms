use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_lunex::{Ab, Rl, UiColor, UiLayout, UiLayoutRoot};

#[derive(Component)]
pub struct MainHudMarker;

#[derive(Bundle)]
pub struct HudBundle {
    pub name: Name,
    pub root_layout: UiLayoutRoot,
}

impl Default for HudBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Main Hud"),
            root_layout: UiLayoutRoot::new_2d()
        }
    }
}

#[derive(Bundle)]
pub struct ToolbarBundle {
    pub name: Name,
    pub ui_layout: UiLayout,
    pub ui_color: UiColor,
    pub sprite: Sprite,
    pub layer: RenderLayers
}

impl Default for ToolbarBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Toolbar"),
            ui_layout: UiLayout::boundary()
                .pos1((Rl(100.0) - Ab(320.0), Ab(20.0)))
                .pos2((Rl(100.0) - Ab(20.0), Ab(70.0)))
                .pack(),
            ui_color: UiColor::from(Color::srgb(0.0, 0.0, 0.0).with_alpha(0.0)),
            sprite: Sprite::default(),
            layer: RenderLayers::layer(2)
        }
    }
}

#[derive(Bundle)]
pub struct IconBundle {
    pub name: Name,
    pub ui_layout: UiLayout,
    pub ui_color: UiColor,
    pub sprite: Sprite,
    pub layer: RenderLayers
}

impl IconBundle {
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
            ui_color: UiColor::from(Color::srgb(1.0, 1.0, 1.0).with_alpha(1.0)),
            sprite: Sprite::from(asset),
            layer: RenderLayers::layer(2),
        }
    }
}



