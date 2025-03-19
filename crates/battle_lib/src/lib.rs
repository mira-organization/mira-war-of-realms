mod logic;
mod observes;
mod setup;
mod fight;

use bevy::prelude::*;
use system::battle_commons::{ActiveCharacterOption, BattleSelectedStatus, CharacterTurnState};
use system::states::GameState;
use crate::fight::BattleFightPlugin;
use crate::logic::BattleLogicPlugin;
use crate::setup::BattleSetupPlugin;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActiveCharacterOption>();
        app.init_resource::<BattleSelectedStatus>();
        app.init_resource::<CharacterTurnState>();

        app.add_plugins(MeshPickingPlugin);
        app.add_plugins((BattleSetupPlugin, BattleLogicPlugin, BattleFightPlugin));
    }
}