use bevy::core_pipeline::bloom::Bloom;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::window::SystemCursorIcon;
use bevy_lunex::*;
use system::states::{GameState, InGameState};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UiLunexPlugins, UiLunexDebugPlugin::<1, 2>));
        app.add_systems(OnEnter(GameState::InGame(InGameState::Main)), (setup_ui_camera, setup_debug_hud));
    }
}

fn setup_ui_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("UI Camera"),
        Camera2d,
        Camera {
            hdr: true,
            order: 1,
            ..default()
        },
        Bloom::OLD_SCHOOL,
        RenderLayers::from_layers(&[0, 1]),
        Transform::from_translation(Vec3::Z * 1000.0),
        UiSourceCamera::<0>
    ));
}

fn setup_debug_hud(mut commands: Commands) {
    commands.spawn((
        Name::new("Debug Hud"),
        UiLayoutRoot::new_2d(),
        UiFetchFromCamera::<0>,
    )).with_children(|ui| {

        ui.spawn((
            Name::new("My Sprite"),
            // Give it some solid aspect ratio
            UiLayout::window().pos(Ab(20.0)).size(Ab(256.0))
                .pack(),

            UiColor::new(vec![
                (UiBase::id(), Color::srgb(1.0, 0.0, 0.0).with_alpha(0.8)),
                (UiHover::id(), Color::srgb(0.0, 0.0, 1.0))
            ]),
            // Give it some material
            //Sprite::from_image(asset_server.load("images/test.jpg")),
            Sprite::default(),
            // On hover change the cursor to this
            OnHoverSetCursor::new(SystemCursorIcon::Crosshair),
        ));

    })
        .observe(hover_set::<Pointer<Over>, true>)
        .observe(hover_set::<Pointer<Out>, false>)
        .observe(|_: Trigger<Pointer<Click>>| {
            info!("Clicked pointer!");
        });
}