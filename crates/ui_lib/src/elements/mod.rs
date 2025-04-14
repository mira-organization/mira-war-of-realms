use bevy::prelude::*;
use crate::elements::button::ButtonUiPlugin;
use crate::elements::check_box::CheckBoxUiPPlugin;
use crate::elements::input::InputUiPlugin;

pub mod input;
pub(crate) mod button;
pub mod check_box;

pub struct ElementPlugin;

impl Plugin for ElementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((InputUiPlugin, ButtonUiPlugin, CheckBoxUiPPlugin));
    }
}