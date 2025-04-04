mod game_window;
mod manager;
mod languages;
mod states;
mod tests;

use bevy::prelude::*;

/// The entry point for the game client application.
///
/// This function initializes and runs the game client. It logs the start of the client,
/// creates a window for the game, and sets the size of the window. Finally, it runs the app
/// until it exits.
///
/// # Returns
/// * `AppExit`: The exit status for the application, indicating whether it exited successfully or with an error.
fn main() -> AppExit {
    // Log the start of the game client
    debug!("Game Client is starting...");

    // Create a new app instance
    let mut app = App::new();

    // Set up the game client window with specified dimensions and run the app
    game_window::create(&mut app, "Game Client", 1270.0, 720.0).run()
}
