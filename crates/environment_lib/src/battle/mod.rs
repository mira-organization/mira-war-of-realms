use bevy::prelude::*;
use system::commons::InBattle;
use system::states::{GameState, InGameState};

pub struct BattleEnvironmentPlugin;

impl Plugin for BattleEnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame(InGameState::Battle)), spawn_player_characters);
    }
}

fn spawn_player_characters(mut players: Query<&mut Transform, With<InBattle>>) {
    for mut transform in players.iter_mut() {
        transform.translation = Vec3::new(-10.0, 51.0, 25.0);
    }
}