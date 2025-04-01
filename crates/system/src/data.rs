use std::fs;
use bevy::prelude::{Entity, Resource};
use serde::Deserialize;
use crate::commons::Character;

/// Represents a character loaded from a JSON file.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct JSONCharacter {
    /// Character's first name.
    pub name: String,
    /// Character's last name.
    pub lastname: String,
    /// Character model file name.
    pub model: String,
    /// Attack range in the world.
    pub world_attack_range: f32,
    /// List of animations associated with the character.
    pub animations: Vec<CharacterAnimation>,
    /// List of character abilities.
    pub abilities: Vec<JSONCharacterAbility>,
}

impl JSONCharacter {
    /// Loads a character from a JSON file based on their name.
    ///
    /// # Arguments
    /// - `character_name` - The name of the character.
    ///
    /// # Returns
    /// - `Ok(JSONCharacter)` if successfully loaded.
    /// - `Err(String)` if the file cannot be read or parsed.
    pub fn fetch(character_name: &str) -> Result<Self, String> {
        let path = format!("assets/entities/characters/data/{}.json", character_name);
        let file_content = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read file {}: {}", path, e))?;

        serde_json::from_str(&file_content)
            .map_err(|e| format!("Failed to parse JSON in {}: {}", path, e))
    }

    /// Retrieves an animation by its name.
    ///
    /// # Arguments
    /// - `name` - The key of the animation.
    ///
    /// # Returns
    /// - `Some(&CharacterAnimation)` if found.
    /// - `None` if no matching animation exists.
    pub fn get_animation_by_name(&self, name: &str) -> Option<&CharacterAnimation> {
        self.animations.iter().find(|anim| anim.key == name)
    }
}

/// Represents a character ability.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct JSONCharacterAbility {
    pub name: String,
    pub family: EAbilityType,
    pub selection_type: ESelectionType,
    pub target_type: ETargetType,
    pub scaling_type: EScalingType,
    pub scaling: f64,
    pub base_value: f64,
}

/// Represents an animation associated with a character.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct CharacterAnimation {
    /// The animation key (e.g., "idle", "walk").
    pub key: String,
    /// The index of the animation in the `.glb` file.
    pub index: u32,
}

/// Defines how an ability selects its target.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum ESelectionType {
    Single,
    Aoe,
    Expansion(usize),
}

/// Defines the type of ability (e.g., attack, ultimate).
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum EAbilityType {
    Attack,
    Ability,
    Ultimate,
}

/// Defines the type of targets an ability can affect.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum ETargetType {
    Allay,
    Enemy,
    All,
}

/// Defines the scaling type of ability (e.g., attack-based, HP-based).
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum EScalingType {
    Hp,
    Defense,
    Attack,
    Speed,
}

/// A resource indicating whether the player wants to switch characters.
#[derive(Resource, Default, Clone, Debug)]
pub struct ChangeCharacter(pub bool);

/// A resource storing the currently active world character.
#[derive(Resource, Default, Clone, Debug)]
pub struct CurrentWorldCharacter(pub Option<(Entity, Character)>);