mod player;
pub mod enemies;

use bevy::prelude::*;
use crate::entities::enemies::ai::AiPlugin;
use crate::entities::enemies::EnemiesPlugin;
use crate::entities::player::PlayerPlugin;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AccountPlayer>();
        app.register_type::<WorldPlayer>();
        app.register_type::<Character>();
        app.register_type::<Elements>();
        app.add_plugins((PlayerPlugin, EnemiesPlugin, AiPlugin));
    }
}

#[derive(Component, Reflect, Debug, Clone)]
pub struct AttackHitBox {
    pub timer: Timer,
}

impl Default for AttackHitBox {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.05, TimerMode::Once),
        }
    }
}

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
    pub attack_hit_box: AttackHitBox
}

impl Default for WorldPlayer {
    /// Provides default values for a `WorldPlayer`.
    /// - `actions_points`: 3
    /// - `walk_speed`: 3.0
    /// - `sprinting_speed`: 4.5
    fn default() -> Self {
        Self {
            actions_points: 3,
            walk_speed: 4.65,
            sprinting_speed: 6.5,
            max_step_height: 1.0,
            state: WorldPlayerState::default(),
            attack_hit_box: AttackHitBox::default()
        }
    }
}

#[derive(Component, Resource, Reflect, Default, Debug, Clone)]
#[reflect(Component)]
pub enum WorldPlayerState {
    #[default]
    Idle,
    Walking,
    Sprinting
}

/// Represents a character with base, extra, and damage-related attributes.
#[derive(Component, Resource, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct Character {
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
            base_attributes: CharacterBaseAttributes::default(),
            extra_attributes: CharacterExtraAttributes::default(),
            damage_attributes: CharacterDamageAttributes::default(),
        }
    }
}

/// Represents the base attributes of a character, such as health, attack,
/// defense, and speed.
#[derive(Component, Resource, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct CharacterBaseAttributes {
    /// The character's health points.
    pub hp: usize,
    /// The maximum super armor points of the character.
    pub max_super_armor: usize,
    /// The character's attack value.
    pub attack: usize,
    /// The character's defense value.
    pub defense: usize,
    /// The character's speed value.
    pub speed: usize,
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
            hp: 280,
            max_super_armor: 100,
            attack: 60,
            defense: 45,
            speed: 50,
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
    pub(crate) animations: Vec<AnimationNodeIndex>,

    /// A handle to the animation graph resource.
    ///
    /// The animation graph defines how animations transition between different states,
    /// such as walking, running, or attacking. This graph is typically loaded as an external
    /// asset and used by the entity to determine its current animation state.
    pub graph: Handle<AnimationGraph>,
}

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct AnimatedPlayer;

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct AnimatedMob;

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
    Light
}