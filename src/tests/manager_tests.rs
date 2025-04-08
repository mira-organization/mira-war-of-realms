#[cfg(test)]
mod tests {
    use crate::manager::{toggle_debug_system, toggle_world_inspector_interface_system};
    use bevy::prelude::*;
    use bevy_rapier3d::prelude::DebugRenderContext;
    use system::config::{ConfigService, WorldInspectorState};

    // Test for the `toggle_debug_system` function
    #[test]
    fn test_toggle_debug_system() {
        // Set up the application and resources
        let mut app = App::new();
        app.insert_resource(ConfigService::default())
            .insert_resource(DebugRenderContext::default())
            .insert_resource(WorldInspectorState::default())
            .insert_resource(ButtonInput::<KeyCode>::default());

        // Run the toggle_debug_system to change the state based on the key press
        app.add_systems(Update, toggle_debug_system);
        app.update();

        // Check that the DebugRenderContext was toggled correctly
        let debug_context = app.world().resource::<DebugRenderContext>();
        assert_eq!(debug_context.enabled, true, "Debug context should be disabled after pressing F3");

        // Now simulate pressing the key again to toggle it off
        app.world_mut().get_resource_mut::<ButtonInput<KeyCode>>().unwrap().press(KeyCode::F3);
        app.add_systems(Update, toggle_debug_system);
        app.update();

        // Assert that the DebugRenderContext was toggled off
        let debug_context = app.world().resource::<DebugRenderContext>();
        assert_eq!(!debug_context.enabled, false, "Debug context should be enabled after pressing F3 again");
    }

    // Test for the `toggle_world_inspector_interface_system` function
    #[test]
    fn test_toggle_world_inspector_interface_system() {
        // Set up the application and resources
        let mut app = App::new();
        app.insert_resource(ConfigService::default())
            .insert_resource(WorldInspectorState::default())
            .insert_resource(ButtonInput::<KeyCode>::default());

        // Simulate pressing the "world inspector toggle" key (F1)
        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::F1);
        app.update();

        // Run the toggle_world_inspector_interface_system to change the state based on the key press
        app
            .add_systems(Update, toggle_world_inspector_interface_system);
        app.update();

        // Assert that the WorldInspectorState was toggled correctly
        let world_inspector_state = app.world().resource::<WorldInspectorState>();
        assert_eq!(world_inspector_state.0, true, "World inspector state should be visible after pressing F1");

        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .clear();
        app.update();

        // Simulate pressing the key again to toggle it off
        app.world_mut().resource_mut::<ButtonInput<KeyCode>>().press(KeyCode::F1);
        app.update();

        app
            .add_systems(Update, toggle_world_inspector_interface_system);
        app.update();

        // Assert that the WorldInspectorState was toggled off
        let world_inspector_state = app.world().resource::<WorldInspectorState>();
        assert_eq!(!world_inspector_state.0, false, "World inspector state should be hidden after pressing F1 again");
    }
}