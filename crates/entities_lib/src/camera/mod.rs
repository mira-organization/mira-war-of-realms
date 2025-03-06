mod logic;

use bevy::prelude::*;
use crate::camera::logic::CameraLogicPlugin;

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CameraLogicPlugin);
    }
}

#[derive(Component, Debug, Copy, Clone)]
pub struct CameraController {
    pub sensitivity: Vec2,
    pub lock_active: bool,
    pub zoom: Zoom,
    pub offset: Offset,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            sensitivity: Vec2::new(1.55, 1.55),
            lock_active: true,
            zoom: Zoom::new(1.0, 8.0),
            offset: Offset::new(0.0, 0.6),
        }
    }
}

/// A marker component for the player's world camera.
///
/// This component is used to identify the camera entity associated with the player.
#[derive(Component, Reflect, Debug, Clone)]
pub struct PlayerWorldCamera;

#[derive(Clone, Copy, Debug)]
pub struct Zoom {
    pub min: f32,
    pub max: f32,
    pub offset_swap: f32,
    pub zoom_sensitivity: f32,
    pub radius: f32,
    pub target_radius: f32,
}

impl Zoom {
    pub fn new(min: f32, max: f32) -> Self {
        let initial_radius = (min + max) / 2.0;
        Self {
            min,
            max,
            offset_swap: min + 1.25,
            zoom_sensitivity: 2.0,
            radius: initial_radius,
            target_radius: initial_radius,
        }
    }
}


#[derive(Clone, Copy, Debug)]
pub struct Offset {
    pub offset: (f32, f32),
}

impl Offset {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            offset: (x, y),
        }
    }
}