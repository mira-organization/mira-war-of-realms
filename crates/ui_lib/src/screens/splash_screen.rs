use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use system::states::GameState;
use crate::colors::{BLACK, WHITE};

pub struct UISplashScreen;

impl Plugin for UISplashScreen {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SplashScreen), create_screen);
    }
}

fn create_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Name::new("Splashscreen"),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            column_gap: Val::Px(10.0),
            ..default()
        },
        BackgroundColor(BLACK),
        RenderLayers::layer(1)
    )).with_children(|ui| {
        ui.spawn((
            Name::new("Bevy Text"),
            Text::new("Powered By:"),
            TextColor(WHITE),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            RenderLayers::layer(1)
        ));

        ui.spawn((
            Name::new("Bevy Logo"),
            ImageNode {
                image: asset_server.load("images/bevy/bevy_logo_dark.png"),
                ..default()
            },
            RenderLayers::layer(1)
        ));
    });
}