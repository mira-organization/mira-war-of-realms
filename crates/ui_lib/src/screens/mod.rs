mod splash_screen;
mod title_screen;
mod main_menu;

use bevy::prelude::*;
use crate::screens::splash_screen::UISplashScreen;
use crate::screens::title_screen::TitleScreen;

pub struct UIScreenPlugin;

impl Plugin for UIScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UISplashScreen, TitleScreen));
    }
}