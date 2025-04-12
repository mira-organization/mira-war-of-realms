use bevy::prelude::*;

pub struct Colored;

impl Colored {
    pub fn black() -> Color {
        Color::BLACK
    }

    pub fn white() -> Color {
        Color::WHITE
    }

    pub fn main_gray() -> Color {
        Color::srgb_u8(35, 34, 46)
    }

    pub fn transparent() -> Color {
        Color::srgba(0.0, 0.0, 0.0, 1.0)
    }
}