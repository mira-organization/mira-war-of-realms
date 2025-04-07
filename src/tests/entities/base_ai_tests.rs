#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use bevy_rapier3d::prelude::Velocity;
    use entities_lib::enemies::ai::{AiSetup, AiState};
    use entities_lib::enemies::ai::base_ai::logic_system;
    use system::commons::{WorldEnemy, WorldPlayer};


    // Helper function to simulate time updates
    fn update_ai(app: &mut App, target_time: f32) {
        let mut current_time = 0.0;
        while current_time < target_time {
            let delta = app.world().get_resource::<Time>().unwrap().delta_secs();

            app.update();

            current_time += delta;
        }
    }

    #[test]
    fn test_enemy_ai_logic() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Init Time resource
        let mut time = Time::default();
        time.update_with_instant(std::time::Instant::now());
        app.insert_resource(time.clone());

        // Setup initial positions for player and enemy
        let player_position = Vec3::new(0.0, 0.0, 0.0);
        let enemy_position = Vec3::new(0.0, 0.0, 10.0);

        // Spawn player
        app.world_mut().spawn((
            WorldPlayer::default(),
            Transform::from_translation(player_position),
        ));

        // Spawn enemy with AI config
        let enemy_entity = app.world_mut().spawn((
            WorldEnemy::default(),
            Transform::from_translation(enemy_position),
            Velocity::default(),
            AiSetup {
                state: AiState::Idle,
                path: vec![
                    Vec3::new(0.0, 0.0, 10.0),
                    Vec3::new(0.0, 0.0, 0.0),
                ],
                current_path_index: 0,
                detection_cone_angle: std::f32::consts::PI,
                detection_range: 20.0,
                aggression_range: 11.0,
                ..default()
            },
        )).id();

        // Add AI system to app
        app.add_systems(Update, logic_system);
        app.update();

        // Step 1: Idle → Alert (Player within detection cone)
        update_ai(&mut app, 0.1);
        {
            let ai = app.world().get::<AiSetup>(enemy_entity).unwrap();
            assert_eq!(ai.state, AiState::Alert);
        }

        // Step 2: Alert continues
        update_ai(&mut app, 2.0);
        {
            let ai = app.world().get::<AiSetup>(enemy_entity).unwrap();
            assert_eq!(ai.state, AiState::Alert);
            assert!(ai.alert_timer >= 2.0);
        }

        // Step 3: Alert → Aggressive (Player stays in range)
        update_ai(&mut app, 2.0);
        {
            let ai = app.world().get::<AiSetup>(enemy_entity).unwrap();
            assert_eq!(ai.state, AiState::Aggressive);
        }

        // Step 4: Player moves out of range
        {
            let world_mut = app.world_mut();
            let mut query = world_mut.query::<&mut Transform>();
            let mut player = query.iter_mut(world_mut).next().unwrap();
            player.translation = Vec3::new(0.0, 0.0, 30.0);
        }

        // Step 5: Aggressive → Observing (Player out of aggression range)
        update_ai(&mut app, 0.1);
        {
            let ai = app.world().get::<AiSetup>(enemy_entity).unwrap();
            assert_eq!(ai.state, AiState::Observing);
        }

        // Step 6: Observing → Walking (Time passes)
        update_ai(&mut app, 5.0);
        {
            let ai = app.world().get::<AiSetup>(enemy_entity).unwrap();
            assert!(matches!(ai.state, AiState::Idle | AiState::Walking));
        }

        // Step 7: Walking continues
        update_ai(&mut app, 0.5);
        {
            let ai = app.world().get::<AiSetup>(enemy_entity).unwrap();
            assert!(matches!(ai.state, AiState::Idle | AiState::Walking));
        }

        // Step 8: Player moves back into attack range
        {
            let world_mut = app.world_mut();
            let mut query = world_mut.query::<&mut Transform>();
            let mut player = query.iter_mut(world_mut).next().unwrap();
            player.translation = Vec3::new(0.0, 0.0, 10.5);
        }

        // Step 9: Walking → Alert → Aggressive → Attacking → Observing -> Idle
        update_ai(&mut app, 0.1);  // Alert
        update_ai(&mut app, 3.1);  // Aggressive
        update_ai(&mut app, 0.1);  // Attacking

        {
            let ai = app.world().get::<AiSetup>(enemy_entity).unwrap();
            assert_eq!(matches!(ai.state, AiState::Idle | AiState::Walking | AiState::Alert | AiState::Observing), true);
        }
    }

}