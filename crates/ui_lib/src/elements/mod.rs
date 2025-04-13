use bevy::prelude::*;
use crate::elements::input::InputUiPlugin;

pub mod input;

pub struct ElementPlugin;

impl Plugin for ElementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputUiPlugin);
    }
}