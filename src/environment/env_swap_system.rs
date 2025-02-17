use bevy::prelude::*;
use crate::environment::{CurrentEnvironment, EnvironmentState};
use crate::events::world_events::WorldEntityHitEntityEvent;
use crate::manager::{GameState, InGameState};

pub struct EnvSwapSystemPlugin;

impl Plugin for EnvSwapSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, swap_to_battle
            .run_if(in_state(GameState::InGame(InGameState::Main)))
            .run_if(on_event::<WorldEntityHitEntityEvent>));
    }
}

fn swap_to_battle(mut current_env: ResMut<CurrentEnvironment>,
                  mut event_reader: EventReader<WorldEntityHitEntityEvent>
) {
    if event_reader.read().next().is_some() {
        if current_env.environment.state == EnvironmentState::Battle {
            return;
        }
        info!("Trigger EnvSwapSystemPlugin");
        current_env.environment.state = EnvironmentState::Battle;
    }
}