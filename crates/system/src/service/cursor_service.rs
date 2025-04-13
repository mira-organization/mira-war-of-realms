use bevy::prelude::*;
use bevy::window::SystemCursorIcon;
use crate::data::CursorIcons;

pub struct CursorService;

impl Plugin for CursorService {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_cursor_icons);
    }
}

pub fn init_cursor_icons(
    mut commands: Commands
) {
    commands.insert_resource(CursorIcons(vec![
        SystemCursorIcon::Default.into(),
        SystemCursorIcon::Crosshair.into(),
        SystemCursorIcon::Wait .into(),
        SystemCursorIcon::Text.into(),
        SystemCursorIcon::Pointer.into(),
    ]));
}