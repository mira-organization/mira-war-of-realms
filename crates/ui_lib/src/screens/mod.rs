mod splash_screen;

use bevy::prelude::*;
use crate::screens::splash_screen::UISplashScreen;

pub struct UIScreenPlugin;

impl Plugin for UIScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UISplashScreen);
    }
}