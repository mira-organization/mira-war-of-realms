use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy::utils::HashMap;
use bevy_rapier3d::prelude::{Collider, Damping, LockedAxes, RigidBody, Velocity};
use crate::battle_commons::CharacterOperation;
use crate::commons::Character;

/// Represents a character party, including the team leader and other party members.
///
/// The `CharacterParty` struct holds the information for a group of characters, with one team leader and multiple party members.
/// The leader has special importance, and the members are stored in a `HashMap` for efficient access.
#[derive(Component, Resource, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct CharacterParty {
    /// The character leading the party.
    /// This character is typically the central figure in the group with unique responsibilities.
    pub team_leader: Character,

    /// A map of party members, where the key is a unique identifier (usize), and the value is the `Character` struct.
    /// This allows for dynamic membership and easy access to individual characters within the party.
    pub members: HashMap<usize, Character>,
}

/// A bundle containing all the necessary components to define a character in the game.
///
/// This struct is used to group multiple components that are required to represent a character in the game world.
/// It includes essential components like scene data, character properties, physics attributes, and operations.
#[derive(Bundle)]
pub struct CharacterBundle {
    /// The root scene component for the character.
    /// This is responsible for representing the character's placement in the game world.
    pub scene: SceneRoot,

    /// The character's name.
    /// This is typically used for displaying the character's name or referencing them in the game.
    pub name: Name,

    /// A component that disables frustum culling for this character.
    /// This ensures the character is always rendered, regardless of whether they are within the camera's view.
    pub culling: NoFrustumCulling,

    /// The transformation (position, rotation, and scale) of the character in the game world.
    /// This is used to place the character at the correct location and orientation.
    pub transform: Transform,

    /// The `Character` component representing the character itself.
    /// This holds the character's attributes, abilities, and other gameplay-related information.
    pub character: Character,

    /// The physics body that controls the character's physical behavior in the world.
    /// This is used for handling interactions with the game world through physics simulation.
    pub rigid_body: RigidBody,

    /// The velocity of the character.
    /// This indicates the character's current movement speed and direction.
    pub velocity: Velocity,

    /// The damping applied to the character's velocity.
    /// This is used to simulate resistance or friction that slows the character down over time.
    pub damping: Damping,

    /// The locked axes that restrict the character's movement along certain axes.
    /// This is useful for controlling the character's allowed movement space (e.g., locking vertical movement).
    pub locked_axes: LockedAxes,

    /// The collision shape used for detecting interactions with other objects in the game world.
    /// This determines the character's hit-box and how they collide with the environment and other entities.
    pub collider: Collider,

    /// The operations available to the character, such as attacks, abilities, and ultimate moves.
    /// This holds the list of actions that the character can perform in combat.
    pub character_operation: CharacterOperation,
}


