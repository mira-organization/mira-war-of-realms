use bevy::prelude::{Component, Res, State, States};

/// Enum representing different game states.
///
/// This enum defines the various states the game can be in, such as loading, splash screen, title screen,
/// and different states for in-game or UI-related activities.
#[derive(Component, States, Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum GameState {
    #[default]
    /// The game is in the preload phase, before any assets are loaded.
    PreLoad,

    /// The splash screen is displayed at the start of the game.
    SplashScreen,

    /// The main menu before start the game loop
    MainMenu,

    /// The environment is being preloaded (before it's fully loaded).
    EnvironmentPreLoad,

    /// The environment is being loaded into the game.
    EnvironmentLoad,

    /// The environment has been loaded and post-processing is being done.
    EnvironmentPostLoad,

    /// The game is in an in-game state.
    InGame(InGameState),

    /// The game is in a UI state.
    InUi(UiState)
}

#[derive(States, Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum InGameState {
    #[default]
    /// The game is in the main in-game state.
    Main,

    /// The game is in a battle state.
    Battle,

    /// The battle has ended, and the game is transitioning out of battle.
    BattleEnd,

    /// The game is displaying a dialogue, likely a conversation or cutscene.
    Dialogue
}

#[derive(States, Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum UiState {
    /// The UI is in a loading state, possibly showing a loading screen or animation.
    Loading,

    #[default]
    /// The main UI state where the player interacts with the main menu or game HUD.
    Main,

    /// The UI is in the settings screen where the player can adjust preferences.
    Settings,

    /// The UI is in the shop screen where the player can purchase items or services.
    Shop,

    /// The UI is in the wish screen, likely a screen for wishes or rewards.
    Wish
}

/// Checks if the current game state is an instance of `GameState::InGame`.
///
/// This function checks if the current game state is one of the `InGame` states,
/// such as `Main`, `Battle`, `BattleEnd`, or `Dialogue`.
///
/// # Arguments
/// * `game_state` - A reference to the current state of the game.
///
/// # Returns
/// * `true` if the game is in an `InGame` state, otherwise `false`.
pub fn in_game_states(game_state: Res<State<GameState>>) -> bool {
    matches!(*game_state.get(), GameState::InGame(_))
}
