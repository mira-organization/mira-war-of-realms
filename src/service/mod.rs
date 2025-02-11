pub mod attack_service;

use bevy::prelude::*;
use crate::service::attack_service::AttackService;

pub struct ServicePlugin;

impl Plugin for ServicePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AttackService);
    }
}