mod components;

use bevy::core_pipeline::bloom::Bloom;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_lunex::*;
use system::states::{GameState, InGameState};
use crate::components::{HudBundle, MainHudMarker, ToolbarBundle};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UiLunexPlugins, UiLunexDebugPlugin::<1, 1>));
        app.add_systems(OnEnter(GameState::InGame(InGameState::Main)), (setup_ui_camera, setup_main_hud));
    }
}

fn setup_ui_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("UI Camera"),
        Camera2d,
        Camera {
            hdr: true,
            order: 2,
            ..default()
        },
        Bloom::OLD_SCHOOL,
        RenderLayers::from_layers(&[1, 2]),
        Transform::from_translation(Vec3::Z * 1000.0),
        UiSourceCamera::<0>,
    ));
}

fn setup_main_hud(mut commands: Commands) {
    commands.spawn((
        HudBundle::default(),
        UiFetchFromCamera::<0>,
        MainHudMarker,
        PickingBehavior::IGNORE
    )).with_children(|ui| {

        ui.spawn((
            ToolbarBundle::default(),
            RenderLayers::layer(2)
        ));

    })
        .observe(|_: Trigger<Pointer<Click>>| {
            info!("Clicked pointer!");
        });
}