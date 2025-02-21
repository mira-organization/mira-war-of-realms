use bevy::prelude::*;
use system::service::load_service::PipelinesReady;
use system::states::GameState;

pub struct PreLoadState;

impl Plugin for PreLoadState {
    fn build(&self, app: &mut App) {
        // Adds the `load_file_assets` system to the Update phase.
        // It only runs if the game is in the `PreLoad` state
        // and if the `PipelinesReady` resource has changed.
        app.add_systems(
            Update,
            load_file_assets
                .run_if(in_state(GameState::PreLoad))
                .run_if(resource_changed::<PipelinesReady>),
        );
    }
}


/// Transitions the game state to `GameState::InGame(InGameState::Main)`
/// once the loading process is completed.
///
/// # Arguments
/// * `ready` - A resource indicating the number of completed loading pipelines.
/// * `next_state` - A mutable reference to `NextState<GameState>` to modify the game state.
fn load_file_assets(ready: Res<PipelinesReady>, mut next_state: ResMut<NextState<GameState>>) {
    info!("transitioning state {:?}", ready.get());
    if ready.get() >= 6 {
        info!("Finished Loading!");
        next_state.set(GameState::SplashScreen);
    }
}