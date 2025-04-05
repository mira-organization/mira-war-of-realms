pub mod logic;
pub mod observes;
pub mod setup;
pub mod fight;
pub mod turn_logic;
mod character_operations;

use bevy::prelude::*;
use system::battle_commons::{TurnCurrentMemberInfo, BattleSelectedStatus, Slot};
use system::commons::TurnOrder;
use crate::fight::BattleFightPlugin;
use crate::logic::BattleLogicPlugin;
use crate::setup::BattleSetupPlugin;
use crate::turn_logic::BattleTurnLogicPlugin;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Slot>();
        app.init_resource::<TurnCurrentMemberInfo>();
        app.init_resource::<BattleSelectedStatus>();
        app.init_resource::<TurnOrder>();

        app.add_plugins(MeshPickingPlugin);
        app.add_plugins((BattleSetupPlugin, BattleLogicPlugin, BattleFightPlugin, BattleTurnLogicPlugin));
    }
}