use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::ui::widget::NodeImageMode;
use system::states::GameState;
use crate::colors::{BLACK, WHITE};

#[derive(Component)]
struct TitleScreenRoot;

#[derive(Component)]
struct PressAnyKeyText;

#[derive(Resource)]
struct PressAnyKeyPulseTimer(Timer);

#[derive(Component)]
struct FadeOverlay;

pub struct TitleScreen;

impl Plugin for  TitleScreen {
    fn build(&self, app: &mut App) {
        app.insert_resource(PressAnyKeyPulseTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));
        app.add_systems(OnEnter(GameState::TitleScreen), spawn_title_screen_ui);
        app.add_systems(Update, (animate_press_any_key_text, detect_button_hit).run_if(in_state(GameState::TitleScreen)));
    }
}

fn spawn_title_screen_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Root node
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            ..default()
        },
        BackgroundColor(BLACK),
        ZIndex(1),
        RenderLayers::layer(1),
        TitleScreenRoot,
    )).with_children(|parent| {
        // Background image
        parent.spawn((
                ImageNode {
                    image: asset_server.load("images/backgrounds/title_background.png"),
                    image_mode: NodeImageMode::Auto,
                    ..default()
                },
                RenderLayers::layer(1),
                ZIndex(2),
        ));

    });

    // Dark overlay
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.945)),
        ZIndex(3),
        RenderLayers::layer(1),
        FadeOverlay,
    )).with_children(|parent| {
        // Center content
        parent.spawn((
            Node {
                width: Val::Px(900.0),
                height: Val::Px(500.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(64.0),
                ..default()
            },
            ZIndex(3),
            RenderLayers::layer(1),
        )).with_children(|center| {
            // Title Text
            center.spawn((
                Text::new("Mira Title"),
                TextColor(WHITE),
                TextFont {
                    font_size: 56.0,
                    ..default()
                },
                RenderLayers::layer(1),
            ));

            // Press Any Key Text
            center.spawn((
                Text::new("Press any key to start"),
                TextColor(WHITE),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                RenderLayers::layer(1),
                PressAnyKeyText,
            ));
        });
    });
}

fn animate_press_any_key_text(
    time: Res<Time>,
    mut timer: ResMut<PressAnyKeyPulseTimer>,
    mut query: Query<&mut TextColor, With<PressAnyKeyText>>,
) {
    timer.0.tick(time.delta());

    let t = (timer.0.elapsed_secs() * std::f32::consts::PI).sin() * 0.5 + 0.5;

    let brightness = 0.5 + 0.5 * t;
    let alpha = 0.5 + 0.5 * t;

    for mut text_color in &mut query {
        text_color.0 = Color::srgba(brightness, brightness, brightness, alpha).into();
    }
}


fn detect_button_hit(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    fade_query: Query<Entity, With<FadeOverlay>>,
    title_root: Query<Entity, With<TitleScreenRoot>>,
) {
    if keyboard.just_pressed(KeyCode::Space) || keyboard.just_released(KeyCode::Enter)
        || mouse.just_pressed(MouseButton::Left) || mouse.just_released(MouseButton::Right) {
        if let Ok(entity) = title_root.get_single() {
            commands.entity(entity).despawn_recursive();
        }

        if let Ok(entity) = fade_query.get_single() {
            commands.entity(entity).despawn_recursive();
        }
        next_state.set(GameState::AccountScreen);
    }
}
