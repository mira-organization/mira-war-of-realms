use bevy::prelude::*;
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
    pub sprite: Sprite
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
        }
    }
}
