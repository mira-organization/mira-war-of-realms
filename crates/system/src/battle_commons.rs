use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::commons::Character;

/// Represents the status of a battle entity, including whether it is currently selected.
///
/// This component is used to track the selection state of characters and enemies in battle.
/// It supports reflection for debugging and UI tools.
#[derive(Resource, Clone, Debug)]
pub struct BattleSelectedStatus {
    /// Indicates whether the entity is currently selected.
    pub selected: Option<(usize, Entity)>,
    pub sub_selected: HashMap<usize, Entity>,
}

impl Default for BattleSelectedStatus {
    /// Provides a default instance where `selected` is set to `false`.
    ///
    /// # Returns
    /// A `BattleEntityStatus` instance with `selected` set to `false`.
    fn default() -> Self {
        Self {
            selected: None,
            sub_selected: HashMap::new(),
        }
    }
}

#[derive(Resource, Debug, Clone, Default)]
pub struct BattleCurrentEntities {
    pub need_patch: bool,
    pub enemies: HashMap<usize, Entity>,
    pub characters: HashMap<usize, Entity>,
}

/// Marker component indicating that an entity is currently in battle.
///
/// This component is used to filter entities that are active participants in a battle.
#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct InBattle;

#[derive(Component, Debug, Clone)]
pub struct ObserveAble;

#[derive(Component, Debug, Clone)]
pub struct BattleMember;

/// Represents an enemy entity specifically in a battle scenario.
///
/// This struct is designed to store data relevant to an enemy's behavior and attributes during a battle.
#[derive(Component, Resource, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct BattleEnemy {
    // Fields for battle-specific attributes and logic can be added here.
    // For example, health, attack power, or battle-specific abilities.
}

/// Represents the currently active character and their selected operation.
///
/// This resource holds the information about the active character and the operation they are performing (e.g., attack, ability, ultimate).
/// It is used to track the character's state in combat and the operation being executed.
#[derive(Resource, Debug, Clone)]
pub struct ActiveCharacterOption {
    /// The active character in the game.
    /// This character will have attributes and abilities that influence the operations they can perform.
    pub character: Character,

    /// The operation selected by the active character.
    /// This could be an attack, ability, or ultimate action, with specific parameters depending on the operation type.
    pub selected_operation: AttackOperation
}

impl Default for ActiveCharacterOption {
    /// Provides a default `ActiveCharacterOption`.
    ///
    /// The default character is created using `Character::default()`, and the default operation is a basic attack with strength 1.
    fn default() -> Self {
        Self {
            character: Character::default(),
            selected_operation: AttackOperation::Attack(1)
        }
    }
}

/// Represents the possible operations a character can perform in combat.
///
/// This component holds a vector of different `AttackOperation` types that define what actions a character can take in battle.
#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct CharacterOperation(pub Vec<AttackOperation>);

impl Default for CharacterOperation {
    /// Provides a default `CharacterOperation` with a set of predefined operations.
    ///
    /// The default set includes:
    /// - A basic attack with strength 1 (`Attack(1)`),
    /// - An ability with strength 1 (`Ability(1)`),
    /// - The ultimate ability (`Ultimate`).
    fn default() -> Self {
        Self(vec![AttackOperation::Attack(1), AttackOperation::Ability(1), AttackOperation::Ultimate])
    }
}

/// Represents the different types of operations a character can perform in combat.
///
/// These operations define the character's actions during combat, such as performing an attack, using an ability, or activating an ultimate.
#[derive(Component, Reflect, Debug, Clone, Eq, PartialEq)]
#[reflect(Component)]
pub enum AttackOperation {
    /// Represents a basic attack.
    ///
    /// The parameter `u8` might represent the strength or level of the attack.
    Attack(u8),

    /// Represents a special ability.
    ///
    /// The parameter `u8` might represent the level or strength of the ability.
    Ability(u8),

    /// Represents the ultimate ability.
    ///
    /// The ultimate ability is typically a powerful action that does not require any additional parameters.
    Ultimate,
}

#[derive(Resource, Debug, Clone)]
pub struct CharacterTurnState {
    pub entity: Option<Character>,
    pub action: AttackOperation,
}

impl Default for CharacterTurnState {
    fn default() -> Self {
        Self {
            entity: None,
            action: AttackOperation::Ultimate,
        }
    }
}

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct Slot(pub usize);
