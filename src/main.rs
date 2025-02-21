mod game_client_window;
mod manager;
mod languages;
mod states;

use bevy::prelude::*;

fn main() -> AppExit {
    debug!("Game Client is starting...");
    let mut app = App::new();
    game_client_window::create(&mut app, "Game Client", 1270.0, 720.0).run()
}