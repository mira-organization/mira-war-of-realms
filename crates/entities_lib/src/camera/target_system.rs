use bevy::prelude::*;
use bevy_mod_outline::{AsyncSceneInheritOutline, OutlineMode, OutlineStencil, OutlineVolume};
use system::commons::WorldPlayer;
use system::states::{GameState, InGameState};
use crate::camera::CameraController;
use crate::enemies::WorldEnemy;

/// A plugin responsible for managing the targeting system in the game.
/// It enables enemy highlighting when they are within view and range.
pub struct TargetSystemPlugin;

impl Plugin for TargetSystemPlugin {
    /// Registers the necessary plugins and systems for the targeting system.
    ///
    /// - Adds the `OutlinePlugin` and `AutoGenerateOutlineNormalsPlugin` for visual outlines.
    /// - Registers the `find_nearest_target_in_view` system to run during the `InGame::Main` state.
    fn build(&self, app: &mut App) {
        app.add_systems(Update, find_nearest_target_in_view.run_if(in_state(GameState::InGame(InGameState::Main))).run_if(is_game_ready));
    }
}

/// Finds the nearest enemy within a specific range and highlights it if it's visible in the player's view.
///
/// - **Player**: Retrieves the player's position.
/// - **Enemies**: Iterates through all enemy entities to determine which is closest.
/// - **Camera**: Checks whether the enemy is within the camera's view.
/// - **Highlighting**: Applies an outline effect to the closest enemy while removing it from others.
///
/// The function performs the following steps:
/// 1. Iterates through all enemies and calculates their distance from the player.
/// 2. Ignores enemies beyond a set range (`6.0` units).
/// 3. Checks if the enemy is within the camera's view using normalized device coordinates (NDC).
/// 4. Highlights the closest enemy while removing the highlight from others.
fn find_nearest_target_in_view(
    player_query: Query<&Transform, With<WorldPlayer>>,
    mut enemies: Query<(Entity, &Transform, Option<&mut OutlineVolume>), With<WorldEnemy>>,
    camera_query: Query<(&GlobalTransform, &Camera), With<CameraController>>,
    mut commands: Commands,
) {
    let player_transform = match player_query.get_single() {
        Ok(transform) => transform,
        Err(_) => return
    };

    let (camera_transform, camera) = match camera_query.get_single() {
        Ok(data) => data,
        Err(_) => return
    };

    let mut closest_enemy: Option<(Entity, f32)> = None;

    for (enemy_entity, enemy_transform, _) in enemies.iter() {
        let enemy_pos = enemy_transform.translation;
        let distance = player_transform.translation.distance(enemy_pos);

        if distance > 6.0 {
            continue;
        }

        if let Some(view_pos) = camera.world_to_ndc(camera_transform, enemy_pos) {
            if view_pos.x.abs() <= 1.0 && view_pos.y.abs() <= 1.0 {
                if closest_enemy.map_or(true, |(_, d)| distance < d) {
                    closest_enemy = Some((enemy_entity, distance));
                }
            }
        }
    }

    let mut enemies_to_update = Vec::new();

    for (enemy_entity, _, outline) in enemies.iter_mut() {
        if let Some((target, _)) = closest_enemy {
            if target == enemy_entity {
                if outline.is_none() {
                    enemies_to_update.push((enemy_entity, true));
                }
                continue;
            }
        }
        enemies_to_update.push((enemy_entity, false));
    }

    for (enemy_entity, highlight) in enemies_to_update {
        let mut entity = commands.entity(enemy_entity);
        if highlight {
            entity.insert(OutlineVolume {
                visible: true,
                width: 3.0,
                colour: Color::srgb(1.0, 0.0, 0.0),
            })
                .insert(OutlineStencil {
                    enabled: true,
                    offset: 1.0,
                })
                .insert(OutlineMode::FloodFlat)
                .insert(AsyncSceneInheritOutline::default());
        } else {
            entity
                .remove::<OutlineVolume>()
                .remove::<OutlineStencil>()
                .remove::<OutlineMode>();
        }
    }
}


fn is_game_ready(
    player_query: Query<(), With<WorldPlayer>>,
    camera_query: Query<(), With<CameraController>>,
) -> bool {
    !player_query.is_empty() && !camera_query.is_empty()
}



