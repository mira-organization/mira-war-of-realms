#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use battle_lib::turn_logic::setup_battle_order;
    use system::commons::{Character, CharacterCurrentStats, Enemy, EnemyCurrentStats, TurnOrder};

    #[test]
    fn test_setup_battle_order() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Add the TurnOrder resource to the world
        app.insert_resource(TurnOrder::default());

        let character_1 = app.world_mut().spawn(Character {
            current_stats: CharacterCurrentStats {
                speed: 50.0,
                ..default()
            },
            ..default()
        }).id();
        let character_2 = app.world_mut().spawn(Character {
            current_stats: CharacterCurrentStats {
                speed: 70.0,
                ..default()
            },
            ..default()
        }).id();
        let enemy_1 = app.world_mut().spawn(Enemy {
            current_stats: EnemyCurrentStats {
                speed: 65.0,
                ..default()
            },
            ..default()
        }).id();

        // Run the update system and setup battle order
        app.add_systems(Startup, setup_battle_order);
        app.update(); // First update to initialize everything

        // Assert that the order is based on speed: character_2 > enemy_1 > character_1
        let turn_order = app.world().resource::<TurnOrder>();
        assert_eq!(turn_order.order.len(), 3);
        assert_eq!(turn_order.order[0], character_2);  // character_2 should be first
        assert_eq!(turn_order.order[1], enemy_1);     // enemy_1 should be second
        assert_eq!(turn_order.order[2], character_1); // character_1 should be last
    }



}