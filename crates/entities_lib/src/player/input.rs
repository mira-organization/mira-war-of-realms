use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use system::battle_commons::{TurnCurrentMemberInfo};
use system::commons::{AbilityType, AttackBoxSettings, Character, CharacterAbilitySet, TurnOrder, WorldEnemy, WorldPlayer, WorldPlayerState};
use system::config::ConfigService;
use system::events::player_events::PlayerActionEvent;
use system::PLAYER_VOID_THRESHOLD;
use system::service::attack_service::spawn_attack_hit_box;
use system::states::{in_game_states, GameState, InGameState};
use system::utils::key_code::convert;
use crate::player::{LastStableGround, PlayerWorldCamera};

/// A plugin that handles player input and movement behavior.
///
/// This plugin captures keyboard inputs and translates them into movement actions,
/// which are then used to update the player's state and movement in the game world.
pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    /// Configures the application to add systems for handling player input and movement,
    /// which are only active during the `GameState::InGame` state.
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (fetch_keyboard_input,
                                 update_movement,
                                 input_attack,
                                 track_stable_ground,
                                 check_void_fall
        ).run_if(in_game_states));
        app.add_systems(Update, battle_input_system.run_if(in_state(GameState::InGame(InGameState::Battle))));
    }
}

/// Captures keyboard inputs and generates `PlayerActionEvent` events to represent the player's actions.
///
/// This system determines the player's movement direction and state (e.g., walking, sprinting, or idle)
/// based on the keys being pressed.
///
/// # Parameters
/// - `input_event_writer`: Used to emit player action events.
/// - `keyboard`: Access to keyboard input states.
/// - `camera_query`: Used to get the transform of the camera for directional movement.
fn fetch_keyboard_input(
    mut input_event_writer: EventWriter<PlayerActionEvent>,
    keyboard: Res<ButtonInput<KeyCode>>,
    camera_query: Query<&Transform, With<PlayerWorldCamera>>,
    general_config: Res<ConfigService>,
) {

    if let Ok(camera_transform) = camera_query.get_single() {
        let forward_key = convert(general_config.input_config.player_up.as_str()).expect("Fetch key for (forward) was failed!");
        let backward_key = convert(general_config.input_config.player_down.as_str()).expect("Fetch key for (backward) was failed!");
        let left_key = convert(general_config.input_config.player_left.as_str()).expect("Fetch key for (left) was failed!");
        let right_key = convert(general_config.input_config.player_right.as_str()).expect("Fetch key for (right) was failed!");

        let sprint_key = convert(general_config.input_config.player_sprint.as_str()).expect("Fetch key for (sprinting) was failed!");

        let mut direction = Vec3::ZERO;
        if keyboard.pressed(forward_key) {
            direction += Vec3::new(camera_transform.forward().x, direction.y, camera_transform.forward().z);
        }

        if keyboard.pressed(backward_key) {
            direction += Vec3::new(camera_transform.back().x, direction.y, camera_transform.back().z);
        }

        if keyboard.pressed(left_key) {
            direction += camera_transform.left().as_vec3();
        }

        if keyboard.pressed(right_key) {
            direction += camera_transform.right().as_vec3();
        }

        if direction.length_squared() > 0.0 {
            let normalized_direction = direction.normalize();
            if keyboard.pressed(forward_key) || keyboard.pressed(backward_key)
                || keyboard.pressed(left_key) || keyboard.pressed(right_key) {
                input_event_writer.send(PlayerActionEvent::Move(normalized_direction));
            } else {
                input_event_writer.send(PlayerActionEvent::Idle);
            }
        } else {
            input_event_writer.send(PlayerActionEvent::Idle);
        }

        if keyboard.pressed(sprint_key) {
            input_event_writer.send(PlayerActionEvent::Sprinting(direction.normalize()));
        }
    }
}

/// Updates the player's movement and state based on the received `PlayerActionEvent` events.
///
/// This system handles movement translation, rotation, and state transitions
/// for the player entity.
///
/// # Parameters
/// - `time`: Provides the delta time for frame-based updates.
/// - `controllers`: Query to access player controllers, transforms, and world player components.
/// - `input_event_reader`: Reads the player action events.
pub fn update_movement(
    time: Res<Time>,
    mut controllers: Query<(&mut KinematicCharacterController, &mut Transform, &mut WorldPlayer), With<WorldPlayer>>,
    mut input_event_reader: EventReader<PlayerActionEvent>,
    current_state: Res<State<GameState>>,
) {
    if current_state.eq(&GameState::InGame(InGameState::Battle)) {
        return;
    }
        for event in input_event_reader.read() {
            for (mut controller, mut transform, mut world_player) in controllers.iter_mut() {
                match event {
                    PlayerActionEvent::Move(direction) => {
                        if direction.length_squared() > 0.0 {
                            let flat_direction = Vec3::new(direction.x, 0.0, direction.z).normalize();
                            let target_rotation = Quat::from_rotation_arc(Vec3::Z, flat_direction);
                            transform.rotation = transform.rotation.slerp(target_rotation, 0.25);
                            controller.translation = Some((direction * world_player.walk_speed) * time.delta_secs());
                            world_player.state = WorldPlayerState::Walking;
                        }
                    }

                    PlayerActionEvent::Sprinting(direction) => {
                        if direction.length_squared() > 0.0 {
                            let flat_direction = Vec3::new(direction.x, 0.0, direction.z).normalize();
                            let target_rotation = Quat::from_rotation_arc(Vec3::Z, flat_direction);
                            transform.rotation = transform.rotation.slerp(target_rotation, 0.25);
                            controller.translation = Some((direction * world_player.sprinting_speed) * time.delta_secs());
                            world_player.state = WorldPlayerState::Sprinting;
                        }
                    }

                    PlayerActionEvent::Idle | PlayerActionEvent::Attacking => {
                        controller.translation = None;
                        world_player.state = WorldPlayerState::Idle;
                    }
                }
            }
        }
}

/// Handles the player input for attacking, including detecting when the left mouse button is pressed.
///
/// When the left mouse button is pressed, this function triggers an attack event and spawns an attack hit_box
/// at the player's current location to register the attack's collision. It also sends a `PlayerActionEvent::Attacking`
/// event to notify other systems of the player's attack action.
///
/// # Arguments
/// - `mouse_input`: A resource to check the input state of the mouse buttons.
/// - `commands`: A mutable reference to the Bevy `Commands` to spawn entities.
/// - `query`: A query to get the player entity in the game world.
/// - `input_event_writer`: An event writer to send the player's action event (Attacking).
pub fn input_attack(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &AttackBoxSettings), With<WorldPlayer>>,
    enemy_query: Query<&Transform, (With<WorldEnemy>, Without<WorldPlayer>)>,
    mut input_event_writer: EventWriter<PlayerActionEvent>,
    current_state: Res<State<GameState>>,
) {
    if current_state.eq(&GameState::InGame(InGameState::Battle)) {
        return;
    }

    if mouse_input.just_pressed(MouseButton::Left) {
        // Trigger the player's attack event
        input_event_writer.send(PlayerActionEvent::Attacking);

        // Spawn the attack hit_box if the player exists in the query
        if let Ok((player, mut player_transform, config)) = query.get_single_mut() {
            let closest_enemy = enemy_query
                .iter()
                .filter(|enemy_transform| {
                    player_transform.translation.distance(enemy_transform.translation) <= config.max_range
                })
                .min_by(|a, b| {
                    player_transform
                        .translation
                        .distance(a.translation)
                        .partial_cmp(&player_transform.translation.distance(b.translation))
                        .unwrap()
                });

            let (hit_box_position, rotation, enemy_translation) = if let Some(enemy_transform) = closest_enemy {
                let direction = (enemy_transform.translation - player_transform.translation).normalize();
                let distance_to_enemy = player_transform.translation.distance(enemy_transform.translation);
                let attack_range = distance_to_enemy.min(config.max_range);
                let attack_offset = Vec3::new(0.0, 0.0, attack_range);

                let look_rotation = Quat::from_rotation_arc(Vec3::Z, direction);

                (attack_offset + Vec3::Y * 0.8, look_rotation, enemy_transform.translation)
            } else {
                let default_offset = Vec3::new(0.0, 0.8, 3.0);
                (default_offset, player_transform.rotation, Vec3::new(0.0, 0.0, 0.0))
            };

            player_transform.rotation = rotation;

            debug!("Player: {:?}, Enemy: {:?}, Hit-Box Position: {:?}", player_transform.translation, enemy_translation, hit_box_position);

            spawn_attack_hit_box(
                &mut commands,
                player,
                Collider::ball(0.75),
                Transform {
                    translation: hit_box_position,
                    ..Default::default()
                },
                Some(Color::srgb_u8(0, 255, 155)),
                0.001
            );
        }
    }
}

/// A system that handles player input for selecting battle operations (e.g., attack, spell, ultimate).
///
/// This system listens for specific key presses based on the player's configured input settings
/// and updates the currently selected battle operation accordingly. The battle operation can be
/// one of three types: Attack, Ability, or Ultimate.
pub fn battle_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    turn_current_member_info: ResMut<TurnCurrentMemberInfo>,
    general_config: Res<ConfigService>,
    character_query: Query<(&Character, &CharacterAbilitySet)>,
    turn_order: Res<TurnOrder>
) {
    // Fetch the keys mapped to specific actions from the configuration service
    let attack_key = convert(general_config.input_config.battle_attack_0.as_str()).expect("Fetch key for (attack 0) was failed!");
    let spell_key = convert(general_config.input_config.battle_spell_0.as_str()).expect("Fetch key for (spell 0) was failed!");
    let ultimate_key = convert(general_config.input_config.battle_ultimate.as_str()).expect("Fetch key for (ultimate) was failed!");

    // Update the selected operation based on the key press
    if keyboard.just_pressed(attack_key) {

        fetch_battle_operation(turn_order, character_query, turn_current_member_info, AbilityType::Attack);

    } else if keyboard.just_pressed(spell_key) {

        fetch_battle_operation(turn_order, character_query, turn_current_member_info, AbilityType::Ability);

    } else if keyboard.just_pressed(ultimate_key) {

        fetch_battle_operation(turn_order, character_query, turn_current_member_info, AbilityType::Ultimate);

    }
}

/// Fetches the battle operation for the current character's turn based on the provided ability family.
///
/// This function checks the current entity's abilities and selects an operation for the turn. It will
/// choose an ability that matches the specified `AbilityType` (family). If the entity has already
/// selected an operation for this turn, it will be reused. Otherwise, the function searches through
/// the character's available abilities and assigns the first one that matches the `AbilityType`.
///
/// # Parameters
/// - `turn_order`: A resource that holds the current turn order and the index of the current entity.
/// - `query`: A query to get both the `Character` and their `CharacterAbilitySet` components for the current entity.
/// - `turn_current_member_info`: A mutable resource holding the information about the current turn's member, including
///   the character and their selected operation.
/// - `family`: The `AbilityType` that is used to filter the character's abilities for the operation. This helps
///   to select the relevant ability based on the turn's context (e.g., attack, defense, etc.).
///
/// # Behavior
/// - If it's not the current entity's turn or the entity has no ability matching the `AbilityType`, it returns early.
/// - If the entity has already selected an operation of the given `AbilityType`, it is reused.
/// - If no operation is selected, it will search the character's abilities and choose the first one matching the `AbilityType`.
///
/// # Logs
/// - Logs the selected operation's name when found.
fn fetch_battle_operation(
    turn_order: Res<TurnOrder>,
    query: Query<(&Character, &CharacterAbilitySet)>,
    mut turn_current_member_info: ResMut<TurnCurrentMemberInfo>,
    family: AbilityType,
) {
    // Get the entity currently in turn
    debug!("Op index: {:?}", turn_order.current_index);
    let mut offset = 0;
    if turn_order.current_index > 0 {
        offset = 1;
    }

    if let Some(current_entity) = turn_order.order.get(turn_order.current_index - offset) {
        debug!("entity: {:?}", current_entity);

        // Attempt to fetch character and ability set for the entity
        let (character, ability_set) = match query.get(*current_entity) {
            Ok((character, ability_set)) => (character, ability_set),
            Err(_) => {
                warn!("This is not your turn!");
                return;
            }
        };

        // If a previous operation of the same family exists, reuse it
        if let Some(current_operation) = turn_current_member_info.pre_operation.clone() {
            if current_operation.family.eq(&family) {
                turn_current_member_info.selected_operation = Some(current_operation);
                return;
            }
        }

        // Search for the first ability matching the specified family
        let mut operation = None;
        for abilities in ability_set.0.clone() {
            if abilities.family.eq(&family) {
                operation = Some(abilities);
                break;
            }
        }

        // Update the turn information with the selected operation
        turn_current_member_info.character = Some(character.clone());
        turn_current_member_info.pre_operation = operation;

        // Log the selected battle operation
        info!("Battle operation: {:?}", turn_current_member_info.clone().pre_operation.unwrap().name);
    }
}



/// Tracks the last stable ground position of the player, updating it when the player is above a threshold height.
///
/// This system monitors the player's position and updates the stored "last stable ground" position if the player is above
/// a certain height threshold (indicating they are standing on stable ground).
///
/// # Arguments
/// - `players`: A query to access the player’s position in the world.
/// - `last_stable_ground`: A resource to store the last known stable ground position.
fn track_stable_ground(mut players: Query<&Transform, With<WorldPlayer>>,
                       mut last_stable_ground: ResMut<LastStableGround>)
{
    for transform in players.iter_mut() {
        if transform.translation.y >= -0.001 {
            last_stable_ground.0 = transform.translation;
        }
    }
}

/// Checks if the player has fallen below a certain threshold, and if so, resets the player’s position to the last stable ground.
///
/// This system prevents the player from falling into the void by checking the player's height. If the player’s position
/// falls below a predefined threshold (`PLAYER_VOID_THRESHOLD`), it resets the player's position to the stored last stable ground.
///
/// # Arguments
/// - `players`: A query to access the player’s position in the world.
/// - `last_stable_ground`: A resource containing the player’s last stable ground position.
fn check_void_fall(mut players: Query<&mut Transform, With<WorldPlayer>>,
                   last_stable_ground: ResMut<LastStableGround>)
{
    for mut transform in players.iter_mut() {
        if transform.translation.y < PLAYER_VOID_THRESHOLD {
            // Reset the player's position to the last stable ground
            transform.translation = last_stable_ground.0;
            info!("Player was sent to {:?}", last_stable_ground.0);
        }
    }
}
