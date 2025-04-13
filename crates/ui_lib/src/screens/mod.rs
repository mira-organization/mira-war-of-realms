mod splash_screen;
mod main_screen;

use bevy::prelude::*;
use crate::screens::splash_screen::UISplashScreen;
use crate::screens::main_screen::MainScreen;

pub struct UIScreenPlugin;

impl Plugin for UIScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UISplashScreen, MainScreen));
    }
}