use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::text::FontSmoothing;
use system::states::GameState;
use crate::colors::Colored;
use crate::elements::button::{ButtonStyle, UiButton};
use crate::elements::input::{InputStyle, InputType, TextField};

#[derive(Component)]
struct MainRoot;

#[derive(Component)]
struct MainBackground;

#[derive(Component)]
struct MainOverlay;

#[derive(Component)]
struct MainContent;

#[derive(Component)]
struct FadeText {
    timer: Timer,
    fading_out: bool,
}

#[derive(Resource)]
struct OverlayFadeTimer(Timer);

#[derive(States, Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum MainMenuState {
    #[default]
    TitleScreen,
    AccountScreen,
    MainMenu
}

pub struct MainScreen;

impl Plugin for MainScreen {
    fn build(&self, app: &mut App) {
        app.init_state::<MainMenuState>();
        app.insert_resource(OverlayFadeTimer(Timer::from_seconds(1.0, TimerMode::Once)));
        app.add_systems(OnEnter(GameState::MainMenu), (create_main_screen, setup_title_menu).chain());
        app.add_systems(Update, (fade_in_overlay, fade_title_text).run_if(in_state(GameState::MainMenu)));
        app.add_systems(OnEnter(MainMenuState::AccountScreen), setup_account_screen);
    }
}

fn create_main_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Root Node
    commands.spawn((
        Name::new("Main screen"),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            ..default()
        },
        ZIndex(0),
        MainRoot,
        RenderLayers::layer(1)
    ))
        .with_children(|ui| {
            ui.spawn((
                Name::new("Main Background"),
                ImageNode {
                    image: asset_server.load("images/backgrounds/main_background.jpg"),
                    ..default()
                },
                RenderLayers::layer(1),
                MainBackground,
            ));
        });

    commands.spawn((
        Name::new("Menu Overlay"),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            ..default()
        },

        BackgroundColor(Colored::main_gray()),
        ZIndex(2),
        RenderLayers::layer(1),
        MainOverlay
    )).with_children(|ui| {
        ui.spawn((
            Name::new("Menu Overlay"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            RenderLayers::layer(1),
            MainContent
        ));
    });
}

fn setup_title_menu(
    mut commands: Commands,
    root_query: Query<Entity, With<MainContent>>,
) {
    let root = root_query.single();
    commands.entity(root).with_children(|ui| {
        ui.spawn((
            Text::new("Mira"),
            TextFont {
                font_size: 56.0,
                ..default()
            },
            TextColor(Colored::white()),
            RenderLayers::layer(1),
            FadeText {
                timer: Timer::from_seconds(1.0, TimerMode::Once),
                fading_out: false,
            },
        ));
    });
}

fn setup_account_screen(
    mut commands: Commands,
    root_query: Query<Entity, With<MainContent>>,
    asset_server: Res<AssetServer>,
) {
    let root = root_query.single();
    commands.entity(root).with_children(|ui| {
        ui.spawn((
            Node {
                width: Val::Px(800.0),
                height: Val::Px(550.0),
                display: Display::Flex,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                border: UiRect::all(Val::Px(2.5)),
                padding: UiRect::all(Val::Px(10.)),
                ..default()
            },
            BackgroundColor(Colored::blue_white()),
            BorderRadius::all(Val::Px(10.0)),
            BorderColor(Colored::main_accent()),
            BoxShadow {
                color: Colored::black(),
                blur_radius: Val::Px(10.0),
                spread_radius: Val::Px(5.0),
                x_offset: Val::Px(4.0),
                y_offset: Val::Px(6.0),
                ..default()
            },
            RenderLayers::layer(1),
        )).with_children(|ui| {
            ui.spawn((
                Text::new("Sign In"),
                TextFont {
                    font_size: 32.0,
                    font_smoothing: FontSmoothing::AntiAliased,
                    ..default()
                },
                TextColor(Colored::font_black_100()),
                RenderLayers::layer(1),
            ));

            ui.spawn((
                TextField::new("Username", true),
                InputStyle {
                    width: Val::Px(450.),
                    height: Val::Px(50.),
                    font_size: 18.0,
                    ..default()
                },
                //InputType::Text,
                RenderLayers::layer(1),
            ));

            ui.spawn((
                TextField::new("Password", true),
                InputStyle {
                    width: Val::Px(450.),
                    height: Val::Px(50.),
                    font_size: 18.0,
                    ..default()
                },
                InputType::Password,
                RenderLayers::layer(1),
            ));

            ui.spawn((
                UiButton::default(),
                ButtonStyle {
                    width: Val::Px(450.),
                    height: Val::Px(50.),
                    font_size: 16.0,
                    image: Some(asset_server.load("images/icons/login.png")),
                    ..default()
                },
                RenderLayers::layer(1),
            ));
        });
    });
}

fn fade_title_text(
    mut commands: Commands,
    time: Res<Time>,
    mut texts: Query<(Entity, &mut TextColor, &mut FadeText)>,
    mut menu_state: ResMut<NextState<MainMenuState>>,
) {
    for (entity, mut color, mut fade) in &mut texts {
        fade.timer.tick(time.delta());

        let progress = fade.timer.elapsed_secs() / fade.timer.duration().as_secs_f32();

        let alpha = if !fade.fading_out {
            progress.clamp(0.0, 1.0)
        } else {
            (1.0 - progress).clamp(0.0, 1.0)
        };

        color.0.set_alpha(alpha);

        if fade.timer.finished() {
            if !fade.fading_out {
                // Start FadeOut
                fade.fading_out = true;
                fade.timer.reset();
            } else {
                commands.entity(entity).despawn_recursive();
                menu_state.set(MainMenuState::AccountScreen);
            }
        }
    }
}

fn fade_in_overlay(
    time: Res<Time>,
    mut timer: ResMut<OverlayFadeTimer>,
    mut overlay_query: Query<&mut BackgroundColor, With<MainOverlay>>,
    state: Res<State<MainMenuState>>,
) {
    if *state.get() != MainMenuState::TitleScreen {
        return;
    }

    timer.0.tick(time.delta());

    let progress = timer.0.elapsed_secs() / timer.0.duration().as_secs_f32();
    let start_alpha = 1.0;
    let target_alpha = 0.3;

    let current_alpha = start_alpha - (progress.powf(2.0) * (start_alpha - target_alpha)).clamp(0.0, 1.0);

    for mut bg in &mut overlay_query {
        let mut color = bg.0;
        color.set_alpha(current_alpha);
        bg.0 = color;
    }
}


