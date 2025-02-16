use bevy::prelude::*;
use crate::manager::GameState;

pub struct SplashScreenState;

impl Plugin for SplashScreenState {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SplashScreen), not_implemented_yet);
    }
}

fn not_implemented_yet(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::TitleScreen);
}