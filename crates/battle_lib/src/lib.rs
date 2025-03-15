mod logic;
mod observes;
mod setup;

use bevy::prelude::*;
use system::battle_commons::{ActiveCharacterOption, BattleSelectedStatus};
use crate::logic::BattleLogicPlugin;
use crate::setup::BattleSetupPlugin;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActiveCharacterOption>();
        app.init_resource::<BattleSelectedStatus>();
        app.add_plugins(MeshPickingPlugin);
        app.add_plugins((BattleSetupPlugin, BattleLogicPlugin));
    }
}