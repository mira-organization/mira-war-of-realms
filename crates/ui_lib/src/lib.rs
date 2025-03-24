mod components;

use bevy::core_pipeline::bloom::Bloom;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_lunex::*;
use bevy_rapier3d::prelude::DebugRenderContext;
use system::config::WorldInspectorState;
use system::states::{GameState, InGameState};
use crate::components::{HudBundle, IconBundle, MainHudMarker, ToolbarBundle};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiLunexPlugins);
        let debug_mode = false;
        if debug_mode {
            app.add_plugins(UiLunexDebugPlugin::<1, 2>);
        }
        app.add_systems(OnEnter(GameState::InGame(InGameState::Main)),
                        (setup_ui_camera, setup_main_hud));
        app.add_systems(OnExit(GameState::InGame(InGameState::Main)), remove_main_hud);
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

fn setup_main_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let images = vec![
        asset_server.load::<Image>("images/inspector.png"),
        asset_server.load::<Image>("images/dev_overlay.png"),
        asset_server.load::<Image>("images/gizmos.png"),
    ];

    commands.spawn((
        HudBundle::default(),
        UiFetchFromCamera::<0>,
        MainHudMarker,
        PickingBehavior::IGNORE,
    )).with_children(|ui| {
        ui.spawn(ToolbarBundle::default())
            .with_children(|ui| {
                for (i, image) in images.iter().enumerate() {
                    ui.spawn(IconBundle::new(i, image.clone(), 68.0, 36.0, 300.0, 50.0))
                        .observe(move |_:
                                       Trigger<Pointer<Click>>,
                                       mut world_inspector_state: ResMut<WorldInspectorState>,
                                       mut debug_context: ResMut<DebugRenderContext>,
                        | {
                            info!("Clicked pointer {}!", i);
                            if i == 0 {
                                world_inspector_state.0 = !world_inspector_state.0;
                            } else if i == 2 {
                                debug_context.enabled = !debug_context.enabled;
                            }
                        })
                        .observe(move |_: Trigger<Pointer<Over>> | {
                            info!("Enter pointer {}!", i);
                        })
                        .observe(move |_: Trigger<Pointer<Out>> | {
                            info!("Leave pointer {}!", i);
                        });
                }
            });
    });
}

fn remove_main_hud(
    mut commands: Commands,
    query: Query<Entity, With<MainHudMarker>>,
    cam_query: Query<Entity, With<Camera2d>>,
) {
    if let Some(main_hud) = query.iter().next() {
        commands.entity(main_hud).despawn_recursive();
    }

    if let Some(main_hud_cam) = cam_query.iter().next() {
        commands.entity(main_hud_cam).despawn_recursive();
    }
}