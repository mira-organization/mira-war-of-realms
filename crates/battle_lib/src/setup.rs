use std::f32::consts::PI;
use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy::utils::HashMap;
use bevy_rapier3d::prelude::*;
use system::battle_commons::{BattleCurrentEntities, BattleMember, CharacterOperation, InBattle, ObserveAble, Slot};
use system::characters::{CharacterBundle, CharacterParty};
use system::commons::{Character, CharacterAbilitySet, Enemy, LivingEntity, WorldPlayer};
use system::states::{GameState, InGameState};

/// A plugin responsible for setting up the battle state in the game.
///
/// This plugin adds systems that are triggered when the game enters the `Battle` state.
/// It ensures that battle entities are spawned and set up correctly when the game transitions into combat.
pub struct BattleSetupPlugin;

impl Plugin for BattleSetupPlugin {
    /// Builds the plugin and adds systems to the app.
    ///
    /// This method registers two systems to run when the game enters the `Battle` state:
    /// 1. `spawn_entities`: A system that spawns the necessary entities for the battle.
    /// 2. `setup_battle_entities`: A system that further configures the battle entities by organizing them into characters and enemies.
    ///
    /// Both systems are added to run when the `GameState::InGame(InGameState::Battle)` state is entered.
    #[cfg_attr(coverage, exclude)]
    #[inline(never)]
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame(InGameState::Battle)), spawn_entities);
        app.add_systems(OnEnter(GameState::InGame(InGameState::Battle)), setup_battle_entities.after(spawn_entities));
        app.add_systems(Update, update_enemy_position
            .run_if(in_state(GameState::InGame(InGameState::Battle))));
    }
}

/// Updates the positions of the enemies in the battle and reassigns them new slots.
///
/// This function checks if the list of enemies and characters is not empty and if a patch
/// (state update) is needed. It then sorts the enemies by their slot, reassigns them new positions
/// based on a defined starting point, and updates the transformation for each enemy entity.
///
/// # Parameters
/// - `battle_entities`: A mutable reference to the `BattleCurrentEntities` resource, which holds the list of enemies
///   and characters in the current battle.
/// - `transforms`: A query to get the mutable `Transform` components of entities, which are updated to set the new positions.
///
/// # Behavior
/// - If the `battle_entities.enemies` or `battle_entities.characters` is empty, the function exits early.
/// - If `battle_entities.need_patch` is `true`, the enemy positions are updated by sorting them by their current slot,
///   assigning new positions starting from `(-10.0, 51.0, 15.0)` and incrementing the x-coordinate for each enemy.
/// - The enemy entities are then assigned to new slots in the `battle_entities.enemies` map.
/// - After the positions are updated, the `battle_entities.need_patch` flag is set to `false` to indicate no further patch is needed.
pub fn update_enemy_position(
    mut battle_entities: ResMut<BattleCurrentEntities>,
    mut transforms: Query<&mut Transform>,
) {
    // Early exit if there are no enemies or characters in the battle
    if battle_entities.enemies.is_empty() || battle_entities.characters.is_empty() {
        return;
    }

    // Proceed only if a patch is needed
    if battle_entities.need_patch {
        // Sort enemies by their current slot (index)
        let mut sorted_enemies: Vec<_> = battle_entities.enemies.iter().collect();
        sorted_enemies.sort_by_key(|(slot, _)| **slot);

        // Initialize variables to assign new positions
        let mut new_enemies = HashMap::new();
        let mut location = Transform::from_xyz(-10.0, 51.0, 15.0).translation;
        let mut new_slot = 1;

        // Update the position of each enemy and assign them new slots
        for (_, &entity) in sorted_enemies {
            if let Ok(mut transform) = transforms.get_mut(entity) {
                transform.translation = location;  // Set new position
                new_enemies.insert(new_slot, entity);  // Reassign enemy to new slot
                new_slot += 1;  // Increment the slot index
                location.x += 2.5;  // Move next enemy slightly to the right
            }
        }

        // Update the battle_entities with the new slot assignments
        battle_entities.enemies = new_enemies;
        battle_entities.need_patch = false;  // Indicate that the patch is complete
    }
}


/// A system that sets up the battle entities by categorizing them into characters and enemies.
///
/// This system runs when the game enters the battle state and organizes the entities involved in the battle.
/// It separates the entities into characters and enemies and stores them in a resource for later use in the battle.
///
/// # Parameters
/// - `commands`: The `Commands` object for modifying the world and adding resources.
/// - `query`: A query that retrieves entities with the `BattleMember` component (which includes both characters and enemies).
/// - `character_query`: A query to check if an entity is a character, based on the `Character` component.
///
/// # Resource
/// This system creates and inserts a `BattleCurrentEntities` resource, which contains separate mappings for characters and enemies.
pub fn setup_battle_entities(
    query: Query<Entity, With<BattleMember>>,
    character_query: Query<&Character>,
    mut battle_entities: ResMut<BattleCurrentEntities>,
) {
    let mut c_index = 0;
    let mut e_index = 0;

    // Iterate over all entities involved in the battle
    for entity in query.iter() {
        if let Ok(_) = character_query.get(entity) {
            // If the entity is a character, increment the character index and add it to the resource
            c_index += 1;
            battle_entities.characters.insert(c_index, entity);
        } else {
            // If the entity is not a character, increment the enemy index and add it to the resource
            e_index += 1;
            battle_entities.enemies.insert(e_index, entity);
        }
    }

    // Log the battle characters and enemies
    info!("battle characters: {:?}", battle_entities.characters);
    info!("battle enemies: {:?}", battle_entities.enemies);
}


/// Spawns player characters and enemies when entering a battle.
///
/// The system places the player's party members and enemies at predefined positions.
///
/// # Parameters
/// - `commands`: Command buffer for spawning entities.
/// - `asset_server`: Asset server for loading character and enemy models.
/// - `players`: Query for retrieving the current player entity in battle.
/// - `character_party`: Resource containing the player's party members.
pub fn spawn_entities(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut players: Query<(Entity, &mut Transform, &WorldPlayer), (With<InBattle>, With<WorldPlayer>)>,
    character_party: Res<CharacterParty>,
) {
    let (world_entity, mut transform, world_player) = match players.get_single_mut() {
        Ok(data) => data,
        Err(_) => return,
    };

    let mut location = Transform::from_xyz(-10.0, 51.0, 25.0).translation;
    let party_members = character_party.clone().members;

    for (_slot, member) in party_members {
        if member.name == world_player.displayed_character.name {
            transform.translation = location;
            transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
            commands.entity(world_entity)
                .insert(BattleMember)
                .insert(CharacterAbilitySet::default());
        } else {
            generate_character(&mut commands, &asset_server, location, &member);
        }
        location.x += 2.5;
    }

    let count: usize = 4;
    let mut location = Transform::from_xyz(-10.0, 51.0, 18.0).translation;

    for index in 1..=count {
        generate_enemies(&mut commands, &asset_server, location, index);
        location.x += 2.5;
    }
}

/// Spawns a character entity for the player's party.
///
/// The character is instantiated at the given location with default physics properties.
///
/// # Parameters
/// - `commands`: Command buffer for spawning entities.
/// - `asset_server`: Asset server for loading character models.
/// - `location`: Spawn position of the character.
/// - `character`: Reference to the character data.
fn generate_character(
    commands: &mut Commands,
    asset_server: &AssetServer,
    location: Vec3,
    character: &Character,
) {
    commands.spawn((CharacterBundle {
        scene: SceneRoot(asset_server.load(GltfAssetLabel::Scene(0)
            .from_asset(format!("entities/characters/model/{}.glb", character.name.to_lowercase())))),
        name: Name::new(character.name.to_string()),
        culling: NoFrustumCulling,
        transform: Transform {
            translation: location,
            rotation: Quat::from_rotation_y(PI),
            ..default()
        },
        character: character.clone(),
        rigid_body: RigidBody::Dynamic,
        velocity: Velocity::default(),
        damping: Damping {
            angular_damping: 2.0,
            linear_damping: 2.0,
        },
        locked_axes: LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
        collider: Collider::capsule(Vec3::new(0.0, 0.2, 0.0), Vec3::new(0.0, 1.6, 0.0), 0.2),
        character_operation: CharacterOperation::default(),
        character_ability_set: CharacterAbilitySet::default()
    }, BattleMember));
}

/// Spawns an enemy entity at the given location.
///
/// Each enemy is assigned a unique index and has basic physics properties.
///
/// # Parameters
/// - `commands`: Command buffer for spawning entities.
/// - `asset_server`: Asset server for loading enemy models.
/// - `location`: Spawn position of the enemy.
/// - `index`: A unique index to differentiate enemies.
fn generate_enemies(
    commands: &mut Commands,
    asset_server: &AssetServer,
    location: Vec3,
    index: usize,
) {
    let enemy = Enemy::default();

    commands.spawn((
        SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset(format!("entities/enemies/{}/{}.glb", enemy.family, enemy.name)))
        ),
        Name::new(format!("Enemy-0{}", index)),
        Transform {
            translation: location,
            rotation: Quat::from_rotation_y(0.0),
            ..default()
        },
        LivingEntity,
        ObserveAble,
        BattleMember,
        Slot(index),
        enemy.clone(),
        RigidBody::Dynamic,
        Velocity::default(),
        Damping {
            angular_damping: 1.0,
            linear_damping: 1.0,
        },
        LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
        Collider::capsule(
            Vec3::new(0.0, 0.2, 0.0),
            Vec3::new(0.0, 1.6, 0.0),
            0.2,
        )
    ));
}

