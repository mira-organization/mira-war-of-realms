pub mod attack_service;
pub mod load_service;
pub mod character_service;

use bevy::prelude::*;
use crate::service::attack_service::AttackService;
use crate::service::character_service::CharacterService;
use crate::service::load_service::LoadService;

/// The `ServicePlugin` is responsible for managing and registering various game services.
/// It ensures that all core service systems are included in the application.
pub struct ServicePlugin;

impl Plugin for ServicePlugin {
    /// Registers service-related plugins.
    ///
    /// - `AttackService`: Handles attack mechanics and related logic.
    /// - `LoadService`: Manages loading processes for game assets or entities.
    ///
    /// # Parameters
    /// - `app`: The Bevy application instance where the services are added.
    fn build(&self, app: &mut App) {
        app.add_plugins((AttackService, LoadService, CharacterService));
    }
}
