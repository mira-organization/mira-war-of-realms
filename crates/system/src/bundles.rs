use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy_rapier3d::prelude::*;
use crate::commons::*;

#[derive(Bundle)]
pub struct WorldPlayerBundle {
    pub name: Name,
    pub no_frustum_culling: NoFrustumCulling,
    pub animated_player: AnimatedPlayer,
    pub transform: Transform,
    pub world_player: WorldPlayer,
    pub living_entity: LivingEntity,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub gravity_scale: GravityScale,
    pub damping: Damping,
    pub locked_axes: LockedAxes,
    pub collider: Collider,
    pub kinematic_controller: KinematicCharacterController,
    pub attack_box_settings: AttackBoxSettings,
}

impl Default for WorldPlayerBundle {

    fn default() -> Self {
        Self {
            name: Name::new("WorldPlayer"),
            no_frustum_culling: NoFrustumCulling,
            animated_player: AnimatedPlayer,
            transform: Transform::from_xyz(40.0, 12.0, 40.0),
            world_player: WorldPlayer::default(),
            living_entity: LivingEntity,
            rigid_body: RigidBody::Dynamic,
            velocity: Velocity::default(),
            gravity_scale: GravityScale(2.5),
            damping: Damping {
                angular_damping: 2.0,
                linear_damping: 2.0,
            },
            locked_axes: LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
            collider: Collider::capsule(Vec3::new(0.0, 0.2, 0.0), Vec3::new(0.0, 1.6, 0.0), 0.2),
            kinematic_controller: KinematicCharacterController {
                max_slope_climb_angle: 45_f32.to_radians(),
                min_slope_slide_angle: 35_f32.to_radians(),
                autostep: Some(CharacterAutostep {
                    include_dynamic_bodies: true,
                    min_width: CharacterLength::Absolute(0.05),
                    max_height: CharacterLength::Absolute(0.55),
                }),
                snap_to_ground: Some(CharacterLength::Absolute(0.075)),
                ..default()
            },
            attack_box_settings: AttackBoxSettings::default(),
        }
    }
}