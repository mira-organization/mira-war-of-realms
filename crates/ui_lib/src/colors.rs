use bevy::prelude::*;

pub struct Colored;

impl Colored {
    pub fn black() -> Color {
        Color::BLACK
    }

    pub fn font_black_100() -> Color {
        Color::srgba(0.1, 0.105, 0.12, 1.0)
    }

    pub fn placeholder_ui_color() -> Color {
        Color::srgba_u8(128, 129, 138, 255)
    }

    pub fn disable_ui_color() -> Color {
        Color::srgba_u8(185, 186, 199, 255)
    }

    pub fn white() -> Color {
        Color::WHITE
    }

    pub fn blue_white() -> Color {
        Color::srgba_u8(228, 229, 240, 255)
    }

    pub fn main_gray() -> Color {
        Color::srgba(0.145, 0.143, 0.19, 1.0)
    }

    pub fn main_accent() -> Color {
        Color::srgba_u8(181, 76, 137, 255)
    }

    pub fn transparent() -> Color {
        Color::srgba(0.0, 0.0, 0.0, 1.0)
    }

    pub fn green() -> Color {
        Color::srgba(0.0, 1.0, 0.0, 1.0)
    }
}