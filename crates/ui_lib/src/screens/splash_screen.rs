use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use system::states::GameState;
use crate::colors::Colored;

#[derive(Component)]
struct SplashRoot;

#[derive(Component)]
struct FadeOverlay;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

#[derive(Resource, Deref, DerefMut)]
struct SplashPhaseTimer(Timer);

#[derive(Resource)]
struct FadeTimer {
    timer: Timer,
    from: f32,
    to: f32,
}

#[derive(States, Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum SplashFadePhase {
    #[default]
    FadeInStudio,
    StudioDisplay,
    FadeOutStudio,
    FadeInPowered,
    PoweredDisplay,
    FadeOutPowered,
}

pub struct UISplashScreen;

impl Plugin for UISplashScreen {
    fn build(&self, app: &mut App) {
        app.init_state::<SplashFadePhase>();
        app.insert_resource(SplashTimer(Timer::from_seconds(2.0, TimerMode::Once)))
            .insert_resource(SplashPhaseTimer(Timer::from_seconds(2.0, TimerMode::Once)));
        app.insert_resource(FadeTimer {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
            from: 1.0,
            to: 0.0,
        });
        app.add_systems(OnEnter(GameState::SplashScreen), create_studio_screen);
        app.add_systems(Update, (splash_screen_update, fade_overlay_system).run_if(in_state(GameState::SplashScreen)));
    }
}

fn create_studio_screen(mut commands: Commands) {
    commands.spawn((
        Name::new("Splashscreen"),
        SplashRoot,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Colored::main_gray()),
        ZIndex(1),
        RenderLayers::layer(1)
    ))
        .with_children(|ui| {
            ui.spawn((
                Name::new("Vogeez Text"),
                Text::new("Vogeez Studio"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Colored::white()),
                RenderLayers::layer(1),
            ));
        });

    commands.spawn((
        FadeOverlay,
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Colored::main_gray()),
        ZIndex(10),
        RenderLayers::layer(1),
    ));
}

fn splash_screen_update(
    mut commands: Commands,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut splash_timer: ResMut<SplashTimer>,
    mut phase_timer: ResMut<SplashPhaseTimer>,
    mut fade_timer: ResMut<FadeTimer>,
    mut fade_phase: ResMut<NextState<SplashFadePhase>>,
    current_fade_phase: Res<State<SplashFadePhase>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    root_query: Query<Entity, With<SplashRoot>>,
    asset_server: Res<AssetServer>,
    mut children: Query<&mut Children>,
    fade_drop: Query<Entity, With<FadeOverlay>>
) {
    match current_fade_phase.get() {
        SplashFadePhase::FadeInStudio => {
            // handled by fade_overlay_system → transition to StudioDisplay
        }

        SplashFadePhase::StudioDisplay => {
            splash_timer.tick(time.delta());

            let proceed = splash_timer.finished()
                || input.any_pressed([KeyCode::Space, KeyCode::Escape])
                || mouse.any_pressed([MouseButton::Left, MouseButton::Right]);

            if proceed {
                fade_timer.timer.reset();
                fade_timer.from = 0.0;
                fade_timer.to = 1.0;
                fade_phase.set(SplashFadePhase::FadeOutStudio);
            }
        }

        SplashFadePhase::FadeOutStudio => {
            // handled by fade_overlay_system → transition to FadeInPowered
        }

        SplashFadePhase::FadeInPowered => {

            fade_timer.timer.reset();
            fade_timer.from = 1.0;
            fade_timer.to = 0.0;
            fade_phase.set(SplashFadePhase::PoweredDisplay);

            if let Ok(root) = root_query.get_single() {
                if let Ok(child_list) = children.get_mut(root) {
                    for &child in child_list.iter() {
                        commands.entity(child).despawn_recursive();
                    }

                    // Add Powered By + Logo
                    commands.entity(root).with_children(|ui| {
                        ui.spawn((
                            Name::new("Bevy Text"),
                            Text::new("Powered By:"),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(Colored::white()),
                            RenderLayers::layer(1),
                        ));

                        ui.spawn((
                            Name::new("Bevy Logo"),
                            ImageNode {
                                image: asset_server.load("images/bevy/bevy_logo_dark.png"),
                                ..default()
                            },
                            RenderLayers::layer(1),
                        ));
                    });
                }
            }
        }

        SplashFadePhase::PoweredDisplay => {
            phase_timer.tick(time.delta());

            let proceed = phase_timer.finished()
                || input.any_pressed([KeyCode::Space, KeyCode::Escape])
                || mouse.any_pressed([MouseButton::Left, MouseButton::Right]);

            if proceed {
                fade_timer.timer.reset();
                fade_timer.from = 0.0;
                fade_timer.to = 1.0;
                fade_phase.set(SplashFadePhase::FadeOutPowered);
            }
        }

        SplashFadePhase::FadeOutPowered => {
            if fade_timer
                .timer.finished() {
                if let Ok(root) = root_query.get_single() {
                    commands.entity(root).despawn_recursive();
                }

                if let Ok(fade_entity) = fade_drop.get_single() {
                    commands.entity(fade_entity).despawn_recursive();
                }

                next_game_state.set(GameState::MainMenu);
            }
        }
    }
}

fn fade_overlay_system(
    time: Res<Time>,
    mut overlay_query: Query<&mut BackgroundColor, With<FadeOverlay>>,
    mut fade_timer: ResMut<FadeTimer>,
    mut next_fade_phase: ResMut<NextState<SplashFadePhase>>,
    current_fade_phase: Res<State<SplashFadePhase>>,
) {
    fade_timer.timer.tick(time.delta());

    let t = (fade_timer.timer.elapsed_secs() / fade_timer.timer.duration().as_secs_f32()).clamp(0.0, 1.0);
    let alpha = fade_timer.from + (fade_timer.to - fade_timer.from) * t;

    // Update the background color alpha based on the fade progress
    for mut color in &mut overlay_query {
        color.0.set_alpha(alpha);
    }

    // Handle phase transitions once fade timer is finished
    if fade_timer.timer.finished() {
        match *current_fade_phase.get() {
            SplashFadePhase::FadeInStudio if t >= 1.0 => {
                next_fade_phase.set(SplashFadePhase::StudioDisplay);
            }
            SplashFadePhase::FadeOutStudio if t >= 1.0 => {
                next_fade_phase.set(SplashFadePhase::FadeInPowered);
            }
            SplashFadePhase::FadeInPowered if t >= 1.0 => {
                next_fade_phase.set(SplashFadePhase::PoweredDisplay);
            }
            SplashFadePhase::FadeOutPowered if t >= 1.0 => {
                // Transition will be handled in splash_screen_update
            }
            _ => {}
        }
    }
}