#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use bevy::window::SystemCursorIcon;
    use system::data::CursorIcons;
    use system::service::cursor_service::init_cursor_icons;

    #[test]
    fn test_init_cursor_icons() {
        let mut app = App::new();
        app.add_systems(Startup, init_cursor_icons);

        app.update();

        let icons = app.world().get_resource::<CursorIcons>().unwrap();

        let expected_icons = CursorIcons(vec![
            SystemCursorIcon::Default.into(),
            SystemCursorIcon::Crosshair.into(),
            SystemCursorIcon::Wait.into(),
            SystemCursorIcon::Text.into(),
            SystemCursorIcon::Pointer.into(),
        ]);

        assert_eq!(expected_icons.0.len(), icons.0.len());
    }

}