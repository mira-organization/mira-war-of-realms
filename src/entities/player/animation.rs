use std::time::Duration;
use bevy::prelude::*;
use crate::entities::{Animations, WorldPlayer, WorldPlayerState};
use crate::entities::player::create_world_player;
use crate::manager::GameState;

/// A plugin responsible for managing player animations.
///
/// This plugin sets up the animation transitions for the player and updates
/// them based on the player's current state.
pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    /// Configures the application to add the setup and update systems
    /// for player animations, which are only active in the `GameState::InGame`.
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (setup, update).run_if(in_state(GameState::InGame)).after(create_world_player));
    }
}

/// Sets up the player's animation system by initializing the animation graph and transitions.
///
/// This system runs once when a new `AnimationPlayer` component is added to the player entity.
///
/// # Parameters
/// - `commands`: Provides access to entity commands for adding components.
/// - `animations`: The resource containing animation data and the graph.
/// - `players`: Query to access entities with a newly added `AnimationPlayer`.
fn setup(
    mut commands: Commands,
    animations: Res<Animations>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, mut animation_player) in players.iter_mut() {
        let mut animation_transitions = AnimationTransitions::new();
        animation_transitions.play(&mut animation_player, animations.animations[0], Duration::ZERO).repeat();
        commands.entity(entity).insert(AnimationGraphHandle(animations.graph.clone())).insert(animation_transitions);
    }
}


/// Updates the player's animations based on the player's state.
///
/// This system ensures that the correct animation is played based on the `WorldPlayerState`.
///
/// # Parameters
/// - `players`: Query to access player entities and their states.
/// - `animations`: The resource containing animation data.
/// - `animation_players`: Query to access animation players and their transitions.
fn update(
    time: Res<Time>,
    mut players: Query<&mut WorldPlayer>,
    animations: Res<Animations>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    mut timers: Local<Vec<Timer>>
) {
    if timers.len() < players.iter().len() {
        timers.resize_with(players.iter().len(), || Timer::new(Duration::from_secs(20), TimerMode::Repeating));
    }

    for (i, player) in players.iter_mut().enumerate() {
        for (mut animation_player, mut animation_transitions) in &mut animation_players {
            let timer = &mut timers[i];
            timer.tick(time.delta());

            match player.state {
                WorldPlayerState::Idle => {
                    if timer.finished() {
                        let idle_animation_entries = [1, 1, 1];
                        let random_index = rand::random::<usize>() % idle_animation_entries.len();
                        let random_idle = animations.animations[idle_animation_entries[random_index]];

                        animation_transitions.play(&mut animation_player, random_idle, Duration::from_millis(425));
                        timer.reset();
                    } else {
                        if !animation_player.is_playing_animation(animations.animations[0]) {
                            for (current_index, active_animation) in animation_player.playing_animations_mut() {
                                if !active_animation.is_finished() {
                                    if current_index.index() == 2 {
                                        return;
                                    }
                                }
                            }
                            animation_transitions.play(&mut animation_player, animations.animations[0], Duration::from_millis(425)).repeat();
                        }
                    }
                }

                WorldPlayerState::Walking => {
                    if !animation_player.is_playing_animation(animations.animations[2]) {
                        animation_transitions.play(&mut animation_player, animations.animations[2], Duration::from_millis(450)).repeat();
                    }
                    timer.reset();
                }

                WorldPlayerState::Sprinting => {
                    if !animation_player.is_playing_animation(animations.animations[3]) {
                        animation_transitions.play(&mut animation_player, animations.animations[3], Duration::from_millis(550)).repeat();
                    }
                    timer.reset();
                }
            }
        }
    }
}