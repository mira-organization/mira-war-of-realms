#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use entities_lib::player::animation::{setup, update};
    use system::commons::{Animations, WorldPlayer, WorldPlayerState};

    #[test]
    fn test_setup_assigns_animation_graph_and_transitions() {
        let mut app = App::new();

        // Dummy Graph & Index
        let dummy_graph = Handle::weak_from_u128(456);
        let dummy_index: AnimationNodeIndex = 0.into();

        let animations = Animations {
            animations: vec![dummy_index],
            graph: dummy_graph.clone(),
        };
        app.insert_resource(animations.clone());

        // Parent (WorldPlayer)
        let world_player = app.world_mut().spawn((WorldPlayer::default(), Transform::default(), GlobalTransform::default())).id();

        // Child mit AnimationPlayer
        let child = app
            .world_mut()
            .spawn((
                Name::new("AnimatedEntity"),
                AnimationPlayer::default(),
                Transform::default(),
                GlobalTransform::default(),
            ))
            .id();

        app.world_mut().entity_mut(world_player).add_children(&[child]);

        app.add_systems(Update, setup);
        app.update();

        let entity = app.world().entity(child);

        assert!(
            entity.get::<AnimationGraphHandle>().is_some(),
            "Expected AnimationGraphHandle to be inserted"
        );

        assert!(
            entity.get::<AnimationTransitions>().is_some(),
            "Expected AnimationTransitions to be inserted"
        );
    }

    #[test]
    fn test_animation_update_system() {
        let mut app = App::new();

        let graph_handle = Handle::weak_from_u128(999);

        app.init_resource::<Time>();
        app.insert_resource(Animations {
            animations: vec![
                AnimationNodeIndex::new(0),  // Idle 1
                AnimationNodeIndex::new(1),  // Idle 2
                AnimationNodeIndex::new(2),  // Walking
                AnimationNodeIndex::new(3),  // Sprinting
            ],
            graph: graph_handle,
        });

        // Create player and animation player entities
        let player_entity = app.world_mut().spawn((
            WorldPlayer {
                walk_speed: 1.0,
                sprinting_speed: 2.0,
                state: WorldPlayerState::Idle,
                ..default()
            },
        )).id();

        let animation_entity = app.world_mut().spawn((
            AnimationPlayer::default(),
            AnimationTransitions::new(),
        )).id();

        // Run the update once to initialize timers
        app.update();

        // Add the animation system
        app.add_systems(Update, update);

        // Change player state to Walking and update
        app.world_mut().entity_mut(player_entity).get_mut::<WorldPlayer>().unwrap().state = WorldPlayerState::Walking;
        app.update();

        // Access the AnimationPlayer and AnimationTransitions components separately
        if let Some(animation_player) = app.world_mut().entity_mut(animation_entity).get_mut::<AnimationPlayer>() {
            // Check if the walking animation is playing
            let is_walking = animation_player.playing_animations().any(|(idx, _)| idx.index() == 2);  // Check if walking animation (index 2) is playing
            assert!(is_walking, "Expected walking animation to be playing");
        } else {
            panic!("Expected AnimationPlayer component");
        }

        // Change player state to Sprinting and update
        app.world_mut().entity_mut(player_entity).get_mut::<WorldPlayer>().unwrap().state = WorldPlayerState::Sprinting;
        app.update();

        if let Some(animation_player) = app.world_mut().entity_mut(animation_entity).get_mut::<AnimationPlayer>() {
            // Check if the sprinting animation is playing
            let is_sprinting = animation_player.playing_animations().any(|(idx, _)| idx.index() == 3);  // Check if sprinting animation (index 3) is playing
            assert!(is_sprinting, "Expected sprinting animation to be playing");
        } else {
            panic!("Expected AnimationPlayer component");
        }
    }


}