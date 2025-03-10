use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy::utils::HashMap;
use bevy_rapier3d::prelude::{Collider, Damping, LockedAxes, RigidBody, Velocity};
use crate::commons::Character;

#[derive(Component, Resource, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct CharacterParty {
    pub team_leader: Character,
    pub members: HashMap<usize, Character>,
}

#[derive(Bundle)]
pub struct CharacterBundle {
    pub scene: SceneRoot,
    pub name: Name,
    pub culling: NoFrustumCulling,
    pub transform: Transform,
    pub character: Character,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub damping: Damping,
    pub locked_axes: LockedAxes,
    pub collider: Collider,
}

