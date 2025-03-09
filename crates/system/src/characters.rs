use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::commons::Character;

#[derive(Resource, Debug, Clone)]
pub struct CharacterParty {
    pub world_active: Character,
    pub members: HashMap<usize, Character>,
}

