#[cfg(test)]
mod tests {

    use bevy::prelude::*;
    use bevy::scene::ScenePlugin;
    use bevy_rapier3d::prelude::*;
    use entities_lib::enemies::ai::{AiSetup, AiState};
    use entities_lib::enemies::test_enemy::setup_enemy;
    use system::commons::{AnimatedMob, LivingEntity, WorldEnemy};

    #[test]
    fn test_setup_enemy_spawns_correct_entity() {
        let mut app = App::new();

        // Add minimal plugins & required resources
        app.add_plugins((MinimalPlugins, AssetPlugin::default(), ScenePlugin));
        let _ = app.world_mut().resource::<AssetServer>();

        // Insert a mock AssetServer that returns dummy handles
        let _handle = Handle::<Scene>::weak_from_u128(123);

        app.add_systems(Startup, setup_enemy);

        // Run Startup system
        app.update();

        let mut found_enemy = false;

        for entity in app.world().iter_entities() {
            let world = &mut app.world();
            let e = world.entity(entity.id());

            if e.contains::<AnimatedMob>() &&
                e.contains::<AiSetup>() &&
                e.contains::<WorldEnemy>() &&
                e.contains::<LivingEntity>() &&
                e.contains::<RigidBody>() &&
                e.contains::<Velocity>() &&
                e.contains::<Damping>() &&
                e.contains::<LockedAxes>() &&
                e.contains::<Collider>() &&
                e.contains::<SceneRoot>() {
                found_enemy = true;

                // Optional: Validate AiSetup values
                let ai = e.get::<AiSetup>().unwrap();
                assert_eq!(ai.state, AiState::Idle);
                assert_eq!(ai.path.len(), 4);

                // Optional: Check Transform
                let transform = e.get::<Transform>().unwrap();
                assert_eq!(transform.translation, Vec3::new(-32.0, 15.0, 15.0));
            }
        }

        assert!(found_enemy, "Expected one enemy entity with full setup");
    }


}