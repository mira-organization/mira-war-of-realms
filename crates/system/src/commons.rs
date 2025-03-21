use bevy::asset::Handle;
use bevy::prelude::*;
use bevy_mod_outline::{AsyncSceneInheritOutline, OutlineMode, OutlineVolume};
use crate::battle_commons::BattleEnemy;

/// The `AttackHitBox` component represents the hit_box for an attack, which is used to detect collisions during combat.
///
/// It includes a timer that manages the duration of the hit_box being active, ensuring it only persists for a short amount of time.
///
/// # Fields
/// - `timer`: The timer that controls the duration for which the attack hit_box is active.
#[derive(Component, Reflect, Debug, Clone)]
pub struct AttackHitBox {
    /// The timer that controls the attack hit_box's duration.
    pub timer: Timer,
}

impl Default for AttackHitBox {
    /// Creates a new `AttackHitBox` component with a default timer duration.
    ///
    /// # Returns
    /// A new `AttackHitBox` component with a timer set to 0.05 seconds in `Once` mode.
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.05, TimerMode::Once),
        }
    }
}

/// The `LivingEntity` component is used to mark entities as living, such as characters, NPCs, or monsters.
///
/// This component is primarily used for categorization, and it doesn't carry any additional data on its own.
/// It is used to identify entities that are alive in the game world.
#[derive(Component, Debug, Clone)]
pub struct LivingEntity;

/// Represents an account-level player with information such as account level,
/// name, email, and a unique identifier.
#[derive(Component, Resource, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct AccountPlayer {
    /// The level of the player's account.
    pub account_level: usize,
    /// The name of the player.
    pub name: String,
    /// The email address associated with the player.
    pub email: String,
    /// The unique identifier for the player.
    pub uid: usize,
}

/// Represents a world-level player with attributes like action points
/// and movement speeds (walking and sprinting).
#[derive(Component, Resource, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct WorldPlayer {
    /// The number of action points available to the player.
    pub actions_points: usize,
    /// The player's walking speed.
    pub walk_speed: f32,
    /// The player's sprinting speed.
    pub sprinting_speed: f32,
    /// The player's step height which is allowed.
    pub max_step_height: f32,
    /// The in world state for handle animations.
    pub state: WorldPlayerState,
    /// The attack box for hit detection.
    pub attack_hit_box: AttackHitBox,

    pub displayed_character: Character,
}

impl Default for WorldPlayer {
    /// Provides default values for a `WorldPlayer`.
    /// - `actions_points`: 3
    /// - `walk_speed`: 3.0
    /// - `sprinting_speed`: 4.5
    fn default() -> Self {
        Self {
            actions_points: 3,
            walk_speed: 4.85,
            sprinting_speed: 7.5,
            max_step_height: 1.0,
            state: WorldPlayerState::default(),
            attack_hit_box: AttackHitBox::default(),
            displayed_character: Character::default()
        }
    }
}

/// The `WorldPlayerState` enum represents the different possible states of a player in the world.
///
/// This enum is used to track and manage the state of a player, such as whether the player is idle, walking, or sprinting.
/// It is particularly useful for controlling player movement and behavior within the game world.
///
/// # Variants
/// - `Idle`: The player is not moving and is in a resting state.
/// - `Walking`: The player is walking at a normal speed.
/// - `Sprinting`: The player is moving at an increased speed (sprinting).
#[derive(Component, Resource, Reflect, Default, Debug, Clone, PartialEq, Eq)]
#[reflect(Component)]
pub enum WorldPlayerState {
    /// The default state, representing when the player is idle and not moving.
    #[default]
    Idle,

    /// The state when the player is walking at normal speed.
    Walking,

    /// The state when the player is sprinting and moving at a faster speed.
    Sprinting,
}


/// Represents a character with base, extra, and damage-related attributes.
#[derive(Component, Resource, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct Character {
    /// The character's name
    pub name: String,
    /// The character's current stats
    pub current_stats: CharacterCurrentStats,
    /// The character's base attributes.
    pub base_attributes: CharacterBaseAttributes,
    /// The character's extra attributes.
    pub extra_attributes: CharacterExtraAttributes,
    /// The character's damage-related attributes.
    pub damage_attributes: CharacterDamageAttributes,
}

impl Default for Character {
    /// Provides default values for a `Character`.
    fn default() -> Self {
        Self {
            name: String::from("ignara"),
            current_stats: CharacterCurrentStats::default(),
            base_attributes: CharacterBaseAttributes::default(),
            extra_attributes: CharacterExtraAttributes::default(),
            damage_attributes: CharacterDamageAttributes::default(),
        }
    }
}

#[derive(Component, Reflect, Debug, Clone)]
pub struct CharacterAbilitySet(pub Vec<CharacterAbility>);

impl Default for CharacterAbilitySet {
    fn default() -> Self {
        Self(vec![
            CharacterAbility {
                name: String::from("Debug Attack"),
                family: AbilityType::Attack,
                selection_type: SelectionType::Single,
                target_type: TargetType::Enemy,
                scaling_type: ScalingType::Attack,
                scaling: 0.5,
                base_value: 5.0,
            },
            CharacterAbility {
                name: String::from("Debug Ability"),
                family: AbilityType::Ability,
                selection_type: SelectionType::Expansion(3),
                target_type: TargetType::Enemy,
                scaling_type: ScalingType::Attack,
                scaling: 0.9,
                base_value: 20.0,
            },
            CharacterAbility {
                name: String::from("Debug Ultimate"),
                family: AbilityType::Ultimate,
                selection_type: SelectionType::Aoe,
                target_type: TargetType::Enemy,
                scaling_type: ScalingType::Attack,
                scaling: 1.2,
                base_value: 35.0,
            },
        ])
    }
}

#[derive(Component, Reflect, Debug, Clone)]
pub struct CharacterAbility {
    pub name: String,
    pub family: AbilityType,
    pub selection_type: SelectionType,
    pub target_type: TargetType,
    pub scaling_type: ScalingType,
    pub scaling: f64,
    pub base_value: f64,
}

#[derive(Component, Resource, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct CharacterCurrentStats {
    /// The character's current hp value
    pub hp: f64,
    /// The character's current super armor value
    pub super_armor: f64,
    /// The character's current attack value
    pub attack: f64,
    /// The character's current defense value
    pub defense: f64,
    /// The character's current speed value
    pub speed: f64,
}

impl Default for CharacterCurrentStats {
    fn default() -> Self {
        Self {
            hp: 280.0,
            super_armor: 100.0,
            attack: 60.0,
            defense: 45.0,
            speed: 50.0
        }
    }
}

/// Represents the base attributes of a character, such as health, attack,
/// defense, and speed.
#[derive(Component, Resource, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct CharacterBaseAttributes {
    /// The character's health points.
    pub hp: f64,
    /// The maximum super armor points of the character.
    pub max_super_armor: f64,
    /// The character's attack value.
    pub attack: f64,
    /// The character's defense value.
    pub defense: f64,
    /// The character's speed value.
    pub speed: f64,
}

impl Default for CharacterBaseAttributes {
    /// Provides default values for `CharacterBaseAttributes`.
    /// - `hp`: 280
    /// - `max_super_armor`: 100
    /// - `attack`: 60
    /// - `defense`: 45
    /// - `speed`: 50
    fn default() -> Self {
        Self {
            hp: 280.0,
            max_super_armor: 100.0,
            attack: 60.0,
            defense: 45.0,
            speed: 50.0,
        }
    }
}

/// Represents additional attributes for a character, such as critical hit
/// chance, energy-related values, and bonus effects.
#[derive(Component, Resource, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct CharacterExtraAttributes {
    /// The chance of landing a critical hit (percentage).
    pub crit_chance: f32,
    /// The damage multiplier for critical hits (percentage).
    pub crit_damage: f32,
    /// The maximum energy the character can store.
    pub max_energy: usize,
    /// The rate at which the character regenerates energy (per second).
    pub energy_charge_rate: f32,
    /// The rate of damage dealt to an opponent's super armor (percentage).
    pub super_armor_damage_rate: f32,
    /// The bonus healing applied to the character.
    pub bonus_heal: f32,
    /// The character's effectiveness in applying status effects.
    pub effect_hit_rate: f32,
    /// The character's resistance to status effects (percentage).
    pub effect_wds: f32,
}

impl Default for CharacterExtraAttributes {
    /// Provides default values for `CharacterExtraAttributes`.
    fn default() -> Self {
        Self {
            crit_chance: 5.0,
            crit_damage: 50.0,
            max_energy: 160,
            energy_charge_rate: 100.0,
            super_armor_damage_rate: 5.0,
            bonus_heal: 0.0,
            effect_hit_rate: 0.0,
            effect_wds: 5.0,
        }
    }
}

/// Represents the elemental and type-specific damage attributes of a character.
#[derive(Component, Resource, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct CharacterDamageAttributes {
    /// The character's fire damage.
    pub fire_damage: f32,
    /// The character's fire resistance (percentage).
    pub fire_wds: f32,
    /// The character's water damage.
    pub water_damage: f32,
    /// The character's water resistance (percentage).
    pub water_wds: f32,
    /// The character's air damage.
    pub air_damage: f32,
    /// The character's air resistance (percentage).
    pub air_wds: f32,
    /// The character's geo (earth) damage.
    pub geo_damage: f32,
    /// The character's geo resistance (percentage).
    pub geo_wds: f32,
    /// The character's lightning damage.
    pub lightning_damage: f32,
    /// The character's lightning resistance (percentage).
    pub lightning_wds: f32,
    /// The character's ice damage.
    pub ice_damage: f32,
    /// The character's ice resistance (percentage).
    pub ice_wds: f32,
    /// The character's holy damage.
    pub holy_damage: f32,
    /// The character's holy resistance (percentage).
    pub holy_wds: f32,
    /// The character's dark damage.
    pub dark_damage: f32,
    /// The character's dark resistance (percentage).
    pub dark_wds: f32,
}

impl Default for CharacterDamageAttributes {
    /// Provides default values for `CharacterDamageAttributes`.
    fn default() -> Self {
        Self {
            fire_damage: 0.0,
            fire_wds: 10.0,
            water_damage: 0.0,
            water_wds: 10.0,
            air_damage: 0.0,
            air_wds: 10.0,
            geo_damage: 0.0,
            geo_wds: 10.0,
            lightning_damage: 0.0,
            lightning_wds: 10.0,
            ice_damage: 0.0,
            ice_wds: 10.0,
            holy_damage: 0.0,
            holy_wds: 10.0,
            dark_damage: 0.0,
            dark_wds: 10.0,
        }
    }
}

/// Represents an enemy entity that exists in the over-world.
///
/// A `WorldEnemy` has a location, a list of possible battle configurations,
/// weaknesses to certain elements, and a current state.
#[derive(Component, Resource, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct WorldEnemy {
    /// The location of the enemy in the game world.
    /// If `None`, the enemy's position has not been initialized yet.
    pub location: Option<Vec3>,

    /// A list of `BattleEnemy` entities that represent the different forms or configurations
    /// this enemy can take in a battle scenario.
    pub enemy_list: Vec<BattleEnemy>,

    /// A collection of elements that this enemy is weak against, such as fire or water.
    pub weakness_elements: Vec<Elements>,

    /// The current state of the enemy, representing its behavior or activity level.
    pub state: EnemyState,

    /// The enemy attack hit box.
    pub attack_hit_box: AttackHitBox
}

impl Default for WorldEnemy {
    /// Creates a default instance of `WorldEnemy` with:
    /// - No initial location.
    /// - An empty list of battle enemies.
    /// - No elemental weaknesses.
    /// - A default state of `EnemyState::Idling`.
    fn default() -> Self {
        Self {
            location: None,
            enemy_list: Vec::new(),
            weakness_elements: Vec::new(),
            state: Default::default(),
            attack_hit_box: AttackHitBox::default(),
        }
    }
}

/// Represents the state or behavior of an enemy.
///
/// The `EnemyState` enum captures the different states an enemy can be in,
/// such as idle, alert, or attacking.
#[derive(Component, Resource, Reflect, Default, Debug, Clone)]
#[reflect(Component)]
pub enum EnemyState {
    /// The enemy is idle and not actively engaged with the player.
    #[default]
    Idling,

    /// The enemy is observing its surroundings, possibly detecting the player's presence.
    Observing,

    /// The enemy is walking or patrolling in the game world.
    Walking,

    /// The enemy is on high alert, likely aware of the player's presence.
    Alert,

    /// The enemy is actively attacking the player or another target.
    Attacking,
}

#[derive(Component, Resource, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct Enemy {
    pub name: String,
    pub family: String,
    pub current_stats: EnemyCurrentStats,
    pub base_stats: EnemyBaseStats,
    pub special_attributes: EnemySpecialAttributes,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            name: String::from("placeholder"),
            family: String::from("test_enemy"),
            current_stats: EnemyCurrentStats::default(),
            base_stats: EnemyBaseStats::default(),
            special_attributes: EnemySpecialAttributes::default(),
        }
    }
}

#[derive(Component, Resource, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct EnemyCurrentStats {
    pub hp: f64,
    pub super_armor: f64,
    pub attack: f64,
    pub defense: f64,
    pub speed: f64
}

impl Default for EnemyCurrentStats {
    fn default() -> Self {
        Self {
            hp: 120.0,
            super_armor: 75.0,
            attack: 12.0,
            defense: 20.0,
            speed: 35.0
        }
    }
}

#[derive(Component, Resource, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct EnemyBaseStats {
    pub hp: f64,
    pub max_super_armor: f64,
    pub attack: f64,
    pub defense: f64,
    pub speed: f64
}

impl Default for EnemyBaseStats {
    fn default() -> Self {
        Self {
            hp: 120.0,
            max_super_armor: 75.0,
            attack: 12.0,
            defense: 20.0,
            speed: 35.0
        }
    }
}

#[derive(Component, Resource, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct EnemySpecialAttributes {

}

impl Default for EnemySpecialAttributes {
    fn default() -> Self {
        Self {}
    }
}

/// Represents a collection of animations and their associated animation graph for an entity.
///
/// This component is used to define and manage animations for entities, such as enemies or characters,
/// by linking a series of animation nodes and an animation graph that dictates how these animations
/// transition and interact with one another.
#[derive(Component, Resource, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct Animations {
    /// A list of animation node indices that represent individual animations
    /// or states within the animation graph.
    ///
    /// These nodes can correspond to specific animation clips or poses, which are
    /// dynamically accessed and updated during gameplay.
    pub animations: Vec<AnimationNodeIndex>,

    /// A handle to the animation graph resource.
    ///
    /// The animation graph defines how animations transition between different states,
    /// such as walking, running, or attacking. This graph is typically loaded as an external
    /// asset and used by the entity to determine its current animation state.
    pub graph: Handle<AnimationGraph>,
}

/// The `AnimatedPlayer` component is used to mark an entity as an animated player character.
///
/// This component is used for entities that represent the player in the game and have animations associated with them.
/// It does not carry any data but is used to identify player entities for animation purposes, such as handling character movement or combat animations.
#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct AnimatedPlayer;

/// The `AnimatedMob` component is used to mark an entity as an animated mob (e.g., enemy or NPC).
///
/// This component is used for entities that represent mobs or non-player characters (NPCs) that have animations.
/// It helps to distinguish these entities from others in the game world, and can be used to control their animation behavior, such as attack or movement animations.
#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct AnimatedMob;

/// The `Elements` enum represents different elemental types that can be associated with entities or abilities.
///
/// This enum is used for categorizing elements like fire, water, earth, etc., and can be applied to characters, abilities, or environmental effects.
/// Each element type represents a different force or attribute, often influencing gameplay through various effects or interactions.
///
/// # Variants
/// - `Fire`: Represents the fire element, often associated with heat or damage over time.
/// - `Water`: Represents the water element, associated with fluidity or healing.
/// - `Earth`: Represents the earth element, often associated with stability or physical attacks.
/// - `Air`: Represents the air element, associated with movement or agility.
/// - `Lightning`: Represents the lightning element, often associated with speed or electrical damage.
/// - `Ice`: Represents the ice element, associated with freezing or slowing effects.
/// - `Dark`: Represents the dark element, often associated with shadow or de-buffs.
/// - `Light`: Represents the light element, often associated with healing or buffing effects.
#[derive(Component, Resource, Reflect, Debug, Clone)]
#[reflect(Component)]
pub enum Elements {
    Fire,
    Water,
    Earth,
    Air,
    Lightning,
    Ice,
    Dark,
    Light,
}

/// A bundle of components that apply an outline effect to an entity.
///
/// The `OutlineTargetBundle` includes components to manage the visual appearance and behavior of an outline
/// around an entity. This can be used for target highlighting and other visual effects that require outlines.
///
/// # Fields
/// - `volume`: Defines the visibility, width, and color of the outline.
/// - `mode`: Determines the style of the outline (e.g., flat or smooth).
/// - `async_outline`: Manages the asynchronous inheritance of outline effects for the entity.
///
/// # Default Behavior
/// - **volume**: Visible outline with a width of `3.0` and a red color (`Color::srgb(1.0, 0.0, 0.0)`).
/// - **mode**: `FloodFlat` outline mode.
#[derive(Bundle)]
pub struct OutlineTargetBundle {
    pub volume: OutlineVolume,
    pub mode: OutlineMode,
    pub async_outline: AsyncSceneInheritOutline,
}

impl Default for OutlineTargetBundle {
    /// Creates a default `OutlineTargetBundle` with predefined settings.
    ///
    /// # Default Values:
    /// - **volume**: A red outline with width `3.0`, visible.
    /// - **mode**: `FloodFlat`.
    /// - **async_outline**: Default value of `AsyncSceneInheritOutline`.
    fn default() -> Self {
        Self {
            volume: OutlineVolume {
                visible: true,
                width: 3.0,
                colour: Color::srgb(1.0, 0.0, 0.0),
            },
            mode: OutlineMode::FloodFlat,
            async_outline: AsyncSceneInheritOutline::default(),
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct BeforeBattleLocation(pub Vec3);

#[derive(Component)]
pub struct ToRemoveAfterBattle;

#[derive(Resource, Debug, Clone)]
pub struct TurnOrder {
    pub order: Vec<Entity>,
    pub current_index: usize,
    pub next: bool
}

impl Default for TurnOrder {
    fn default() -> Self {
        Self {
            order: Vec::new(),
            current_index: 0,
            next: true
        }
    }
}

#[derive(Component, Reflect, Debug, Clone, PartialEq, Eq)]
#[reflect(Component)]
pub enum SelectionType {
    Single,
    Aoe,
    Expansion(usize)
}

#[derive(Component, Reflect, Debug, Clone, PartialEq, Eq)]
#[reflect(Component)]
pub enum AbilityType {
    Attack,
    Ability,
    Ultimate
}

#[derive(Component, Reflect, Debug, Clone, PartialEq, Eq)]
#[reflect(Component)]
pub enum TargetType {
    Allay,
    Enemy,
    All
}

#[derive(Component, Reflect, Debug, Clone, PartialEq, Eq)]
#[reflect(Component)]
pub enum ScalingType {
    Hp,
    Defense,
    Attack,
    Speed
}
