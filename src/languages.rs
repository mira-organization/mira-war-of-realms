use bevy::prelude::*;
use bevy_fluent::prelude::*;

pub struct LanguagesPlugin;

impl Plugin for LanguagesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FluentPlugin);
    }
}

