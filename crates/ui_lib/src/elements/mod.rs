use bevy::prelude::*;
use crate::elements::button::ButtonUiPlugin;
use crate::elements::check_box::CheckBoxUiPPlugin;
use crate::elements::choice_box::ChoiceBoxUiPlugin;
use crate::elements::input::InputUiPlugin;
use crate::elements::slider::SliderUiPlugin;

pub mod input;
pub(crate) mod button;
pub mod check_box;
pub mod slider;
pub mod choice_box;

pub struct ElementPlugin;

impl Plugin for ElementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            InputUiPlugin,
            ButtonUiPlugin,
            CheckBoxUiPPlugin,
            SliderUiPlugin,
            ChoiceBoxUiPlugin,
        ));
    }
}