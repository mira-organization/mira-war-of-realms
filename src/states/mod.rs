mod pre_load_state;
mod title_screen_state;
mod account_screen_state;
mod fix_asset_waiting;

use bevy::prelude::*;
use crate::states::account_screen_state::AccountScreenState;
use crate::states::fix_asset_waiting::FixAssetWaitingState;
use crate::states::pre_load_state::PreLoadState;
use crate::states::title_screen_state::TitleScreenState;

/// A plugin that adds different game states related to the main flow of the game.
///
/// This plugin adds various state systems to handle the transitions between
/// different parts of the game, such as preloading, splash screen, title screen,
/// and account screen states.
pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    /// Builds the plugin and adds the state-related systems to the app.
    ///
    /// This function registers the different game states with the application. These states are used
    /// to manage the game's flow and user interactions in various stages, including:
    /// - PreLoadState: Preloading resources or assets.
    /// - SplashScreenState: Displaying the splash screen at the start.
    /// - TitleScreenState: Displaying the main menu or title screen.
    /// - AccountScreenState: Managing the account-related interactions.
    ///
    /// # Arguments
    /// * `app` - A mutable reference to the `App` struct, which allows you to add resources, systems, and plugins.
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PreLoadState,
            TitleScreenState,
            AccountScreenState,
            FixAssetWaitingState,
        ));
    }
}
