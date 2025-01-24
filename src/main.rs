mod game_client_window;
mod environment;
mod manager;
mod entities;
mod events;

use bevy::prelude::*;

fn main() -> AppExit {
    debug!("Game Client is starting...");
    let mut app = App::new();
    game_client_window::create(&mut app, "Game Client", 1270.0, 720.0).run()
}
