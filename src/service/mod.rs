pub mod attack_service;
pub mod load_service;

use bevy::prelude::*;
use crate::service::attack_service::AttackService;
use crate::service::load_service::LoadService;

pub struct ServicePlugin;

impl Plugin for ServicePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AttackService, LoadService));
    }
}