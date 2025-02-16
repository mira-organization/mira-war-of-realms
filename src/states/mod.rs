mod pre_load_state;
mod splash_screen_state;
mod title_screen_state;
mod account_screen_state;

use bevy::prelude::*;
use crate::states::account_screen_state::AccountScreenState;
use crate::states::pre_load_state::PreLoadState;
use crate::states::splash_screen_state::SplashScreenState;
use crate::states::title_screen_state::TitleScreenState;

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PreLoadState,
            SplashScreenState,
            TitleScreenState,
            AccountScreenState,
        ));
    }
}