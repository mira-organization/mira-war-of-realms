use bevy::prelude::*;
use system::data::{AssetsToLoad, ChangeCharacter};
use system::states::{GameState, InGameState};

pub struct FixAssetWaitingState;

impl Plugin for FixAssetWaitingState {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, wait_for_assets.run_if(in_state(GameState::EnvironmentPostLoad)));
    }
}

fn wait_for_assets(
    mut next_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    mut assets_to_load: ResMut<AssetsToLoad>,
    mut change_character: ResMut<ChangeCharacter>
) {
    let mut to_remove = Vec::new();
    info!("Waiting for fix assets to load [{}]", assets_to_load.0.len());
    for (index, id) in assets_to_load.0.clone().iter().enumerate() {
        if let Some(handle) = asset_server.get_load_state(*id) {
            if handle.is_loaded() {
                to_remove.push(index);
            }
        }
    }

    for index in to_remove {
        assets_to_load.0.remove(index);
    }

    if assets_to_load.0.is_empty() {
        info!("No assets to load anymore!");
        change_character.0 = true;
        next_state.set(GameState::InGame(InGameState::Main));
    }
}