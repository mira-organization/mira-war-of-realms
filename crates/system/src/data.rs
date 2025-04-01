use std::fs;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct JSONCharacter {
    pub name: String,
    pub lastname: String,
    pub model: String,
    pub world_attack_range: f32,
    pub animations: Vec<CharacterAnimation>,
    pub abilities: Vec<JSONCharacterAbility>
}

impl JSONCharacter {
    pub fn fetch(character_name: &str) -> Result<Self, String> {
        let path = format!("assets/entities/characters/data/{}.json", character_name);
        let file_content = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read file {}: {}", path, e))?;

        serde_json::from_str(&file_content)
            .map_err(|e| format!("Failed to parse JSON in {}: {}", path, e))
    }

    pub fn get_animation_by_name(&self, name: &str) -> Option<&CharacterAnimation> {
        let mut result = None;

        for animation in &self.animations {
            if animation.key.eq(name) {
                result = Some(animation);
            }
        }

        result
    }
}

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

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct CharacterAnimation {
    pub key: String,
    pub index: u32,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum ESelectionType {
    Single,
    Aoe,
    Expansion(usize)
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum EAbilityType {
    Attack,
    Ability,
    Ultimate
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum ETargetType {
    Allay,
    Enemy,
    All
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum EScalingType {
    Hp,
    Defense,
    Attack,
    Speed
}