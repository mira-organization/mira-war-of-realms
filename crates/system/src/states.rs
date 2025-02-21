use bevy::prelude::{Component, Res, State, States};

/// Enum representing different game states.
#[derive(Component, States, Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum GameState {
    #[default]
    PreLoad,
    SplashScreen,
    TitleScreen,
    AccountScreen,
    EnvironmentPreLoad,
    EnvironmentLoad,
    EnvironmentPostLoad,
    InGame(InGameState),
    InUi(UiState)
}

#[derive(States, Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum InGameState {
    #[default]
    Main,
    Battle,
    BattleEnd,
    Dialogue
}

#[derive(States, Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum UiState {
    Loading,
    #[default]
    Main,
    Settings,
    Shop,
    Wish
}

/// Checks if the current game state is an instance of `GameState::InGame`.
///
/// # Arguments
/// * `game_state` - A reference to the current state of the game.
///
/// # Returns
/// * `true` if the game is in an `InGame` state, otherwise `false`.
pub fn in_game_states(game_state: Res<State<GameState>>) -> bool {
    matches!(*game_state.get(), GameState::InGame(_))
}