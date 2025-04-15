use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::text::FontSmoothing;
use system::states::GameState;
use crate::colors::Colored;
use crate::elements::button::{ButtonStyle, UiButton};
use crate::elements::check_box::CheckBox;
use crate::elements::choice_box::{ChoiceBox, ChoiceBoxStyle, ChoiceOption};
use crate::elements::input::{InputStyle, InputType, TextField};
use crate::elements::slider::Slider;
use crate::Radius;

#[derive(Component)]
struct MainRoot;

#[derive(Component)]
struct MainBackground;

#[derive(Component)]
struct MainOverlay;

#[derive(Component)]
struct MainContent;

#[derive(Component)]
struct AccountScreen;

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
        app.add_systems(OnEnter(MainMenuState::MainMenu), setup_main_menu);
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
                width: Val::Px(550.0),
                min_height: Val::Px(450.0),
                display: Display::Flex,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                border: UiRect::all(Val::Px(0.0)),
                padding: UiRect::axes(Val::Px(40.), Val::Px(50.)),
                ..default()
            },
            BackgroundColor(Colored::white()),
            BorderRadius::all(Val::Px(10.0)),
            BorderColor(Colored::main_accent()),
            BoxShadow {
                color: Colored::black(),
                blur_radius: Val::Px(10.0),
                spread_radius: Val::Px(10.0),
                x_offset: Val::Px(0.0),
                y_offset: Val::Px(0.0),
                ..default()
            },
            RenderLayers::layer(1),
            AccountScreen
        )).with_children(|ui| {
            ui.spawn((
                Text::new("Login to Vogeez"),
                TextFont {
                    font_size: 32.0,
                    font_smoothing: FontSmoothing::AntiAliased,
                    ..default()
                },
                TextColor(Colored::main_gray()),
                RenderLayers::layer(1),
            ));

            // Text Field
            ui.spawn((
                Name::new("Account User"),
                Node {
                    width: Val::Percent(100.),
                    margin: UiRect::top(Val::Px(40.)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    ..default()
                },
                RenderLayers::layer(1),
            )).with_children(|inner| {
                inner.spawn((
                    Text::new("Email / Username"),
                    TextFont {
                        font_size: 14.0,
                        font_smoothing: FontSmoothing::AntiAliased,
                        ..default()
                    },
                    TextColor(Color::srgba_u8(100, 100, 100, 255)),
                    RenderLayers::layer(1),
                ));

                inner.spawn((
                    TextField::new("Username", true),
                    InputStyle {
                        width: Val::Percent(100.),
                        height: Val::Px(55.),
                        font_size: 16.0,
                        border_radius: Radius::all(Val::Px(8.)),
                        background_color: Colored::blue_white(),
                        color: Color::srgba_u8(68, 70, 71, 255),
                        border_color: Color::srgba_u8(211, 218, 224, 255),
                        ..default()
                    },
                    RenderLayers::layer(1),
                ));
            });

            // Password Field
            ui.spawn((
                Name::new("Account Pass"),
                Node {
                    width: Val::Percent(100.),
                    margin: UiRect::top(Val::Px(30.)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    ..default()
                },
                RenderLayers::layer(1),
            )).with_children(|inner| {
                inner.spawn((
                    Text::new("Password"),
                    TextFont {
                        font_size: 14.0,
                        font_smoothing: FontSmoothing::AntiAliased,
                        ..default()
                    },
                    TextColor(Color::srgba_u8(100, 100, 100, 255)),
                    RenderLayers::layer(1),
                ));

                inner.spawn((
                    TextField::new("Password", true),
                    InputStyle {
                        width: Val::Percent(100.),
                        height: Val::Px(55.),
                        font_size: 16.0,
                        border_radius: Radius::all(Val::Px(8.)),
                        background_color: Colored::blue_white(),
                        color: Color::srgba_u8(68, 70, 71, 255),
                        border_color: Color::srgba_u8(211, 218, 224, 255),
                        ..default()
                    },
                    InputType::Password,
                    RenderLayers::layer(1),
                ));
            });

            // Remember me
            ui.spawn((
                CheckBox {
                    label: String::from("Remember me"),
                    ..default()
                },
                RenderLayers::layer(1),
            ));

            // Account Login Button
            ui.spawn((
                Name::new("Account Login"),
                Node {
                    width: Val::Percent(100.),
                    margin: UiRect::top(Val::Px(30.)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    ..default()
                },
                RenderLayers::layer(1),
            )).with_children(|inner| {
                inner.spawn((
                    UiButton("Login".to_string()),
                    ButtonStyle {
                        width: Val::Percent(100.),
                        height: Val::Px(55.),
                        border_radius: Radius::all(Val::Px(8.)),
                        color: Colored::blue_white(),
                        background_color: Colored::main_accent(),
                        border_color: Colored::main_accent(),
                        font_size: 16.0,
                        ..default()
                    },
                    RenderLayers::layer(1),));
            });

            // Debug Login Button
            ui.spawn((
                Name::new("Debug Mode"),
                Node {
                    width: Val::Percent(100.),
                    margin: UiRect::top(Val::Px(30.)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    ..default()
                },
                RenderLayers::layer(1),
            ))
                .observe(|_event: Trigger<Pointer<Click>>, mut next_state: ResMut<NextState<MainMenuState>>| {
                    debug!("Click detected");
                    next_state.set(MainMenuState::MainMenu);
                })
                .with_children(|inner| {
                inner.spawn((
                    UiButton("Test Login".to_string()),
                    ButtonStyle {
                        width: Val::Percent(100.),
                        height: Val::Px(55.),
                        border_radius: Radius::all(Val::Px(8.)),
                        color: Colored::blue_white(),
                        background_color: Color::srgba_u8(140, 128, 15, 255),
                        border_color: Color::srgba_u8(140, 128, 15, 255),
                        hover_color: Color::srgba_u8(150, 138, 25, 255),
                        font_size: 16.0,
                        ..default()
                    },
                    RenderLayers::layer(1),));
            });

            // Test Slider
            ui.spawn((Slider::default(), RenderLayers::layer(1)));

            let default_op = ChoiceOption {
                selected: true,
                label: "Google".to_string(),
                icon: Some(asset_server.load("images/icons/google-icon.png")),
                ..default()
            };

            let list = vec![
                default_op.clone(),
                ChoiceOption {
                    label: "GitHub".to_string(),
                    icon: Some(asset_server.load("images/icons/github-icon.png")),
                    ..default()
                },
                ChoiceOption {
                    label: "Discord".to_string(),
                    icon: Some(asset_server.load("images/icons/discord-icon.png")),
                    ..default()
                },
                ChoiceOption {
                    label: "Youtube".to_string(),
                    icon: Some(asset_server.load("images/icons/youtube-icon.png")),
                    ..default()
                },
            ];

            // Test Choice
            ui.spawn((
                ChoiceBox {
                    value: default_op,
                    options: list,
                },
                ChoiceBoxStyle {
                    drop_icon: Some(asset_server.load("images/icons/drop-down.png")),
                    ..default()
                },
                RenderLayers::layer(1)
            ));
        });
    });
}

fn setup_main_menu(
    mut commands: Commands,
    query: Query<Entity, With<AccountScreen>>,
    root_query: Query<Entity, With<MainContent>>,
) {
    let root = root_query.single();
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    commands.entity(root).insert((
        UiButton("Game Start".to_string()),
        RenderLayers::layer(1)
    ));
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


