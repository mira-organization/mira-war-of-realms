use bevy::prelude::*;
use system::states::GameState;

pub struct MainMenuScreen;

impl Plugin for MainMenuScreen {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_main_menu_screen);
    }
}

fn setup_main_menu_screen(mut commands: Commands) {

}