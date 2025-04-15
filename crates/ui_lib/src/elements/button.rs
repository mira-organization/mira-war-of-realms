use crate::UiElementState;
use crate::UiGenID;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::winit::cursor::CursorIcon;
use system::data::CursorIcons;
use crate::colors::Colored;
use crate::Radius;

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
#[require(ButtonStyle, UiElementState, UiGenID)]
pub struct UiButton(pub String);

impl Default for UiButton {
    fn default() -> Self {
        Self {
            0: String::from("Button"),
        }
    }
}

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct ButtonStyle {
    pub width: Val,
    pub height: Val,
    pub color: Color,
    pub font_size: f32,
    pub disabled_color: Color,
    pub background_color: Color,
    pub border_color: Color,
    pub focus_color: Color,
    pub hover_color: Color,
    pub border: UiRect,
    pub border_radius: Radius,
    pub padding: UiRect,
    pub node_spacing: Val,
    pub image: Option<Handle<Image>>,
    pub image_orientation: ButtonImageOrientation,
    pub image_color: Color,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            width: Val::Px(200.),
            height: Val::Px(40.),
            color: Colored::font_black_100(),
            font_size: 16.,
            disabled_color: Colored::disable_ui_color(),
            background_color: Colored::blue_white(),
            border_color: Colored::main_gray(),
            focus_color: Colored::main_accent(),
            hover_color: Colored::main_accent_lighter(),
            border: UiRect::all(Val::Px(2.)),
            border_radius: Radius::all(Val::Px(2.5)),
            padding: UiRect::horizontal(Val::Px(5.0)),
            node_spacing: Val::Px(20.0),
            image: None,
            image_orientation: ButtonImageOrientation::default(),
            image_color: Colored::main_accent(),
        }
    }
}

#[derive(Reflect, Default, Clone, PartialEq, Debug)]
pub enum ButtonImageOrientation {
    Left,
    #[default]
    Right
}

#[derive(Component)]
pub struct ButtonRoot;

#[derive(Component)]
pub struct ButtonText;

#[derive(Component)]
pub struct ButtonImage;

pub struct ButtonUiPlugin;

impl Plugin for ButtonUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Button>();
        app.register_type::<ButtonStyle>();
        app.add_systems(Update, (build_detect_button, check_selected_status));
    }
}

fn build_detect_button(
    mut commands: Commands,
    query: Query<(Entity, &ButtonStyle, &UiButton), Without<ButtonRoot>>
) {
    for (entity, button_style, button) in &query {
        commands.entity(entity)
            .insert((
                Node {
                    width: button_style.width,
                    height: button_style.height,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Row,
                    column_gap: if button_style.image.is_some() { button_style.node_spacing } else { Val::Px(0.) },
                    padding: button_style.padding,
                    border: button_style.border,
                    ..default()
                },
                BorderRadius {
                    top_left: button_style.border_radius.top_left,
                    top_right: button_style.border_radius.top_right,
                    bottom_left: button_style.border_radius.bottom_left,
                    bottom_right: button_style.border_radius.bottom_right
                },
                BackgroundColor(button_style.background_color),
                BorderColor(button_style.border_color),
                BoxShadow {
                    color: button_style.border_color,
                    x_offset: Val::Px(0.),
                    y_offset: Val::Px(0.),
                    spread_radius: Val::Px(1.),
                    blur_radius: Val::Px(1.),
                },
                RenderLayers::layer(1),
                ButtonRoot
            ))
            .observe(on_hover_enter)
            .observe(on_hover_leave)
            .with_children(|builder| {

                if button_style.image_orientation.eq(&ButtonImageOrientation::Left) {
                    if let Some(image) = button_style.image.clone() {
                        builder.spawn((
                            ImageNode {
                                image,
                                color: button_style.image_color,
                                ..default()
                            },
                            RenderLayers::layer(1),
                            PickingBehavior::IGNORE,
                            ButtonImage,
                        ));
                    }
                }

                builder.spawn((
                    Text::new(button.0.clone()),
                    TextFont {
                        font_size: button_style.font_size,
                        ..default()
                    },
                    PickingBehavior::IGNORE,
                    TextColor(button_style.color),
                    RenderLayers::layer(1),
                    ButtonText
                ));

                if button_style.image_orientation.eq(&ButtonImageOrientation::Right) {
                    if let Some(image) = button_style.image.clone() {
                        builder.spawn((
                            ImageNode {
                                image,
                                color: button_style.image_color,
                                ..default()
                            },
                            RenderLayers::layer(1),
                            PickingBehavior::IGNORE,
                            ButtonImage,
                        ));
                    }
                }
            });
    }
}

fn on_hover_enter(
    event: Trigger<Pointer<Over>>,
    mut commands: Commands,
    mut query: Query<(&mut UiElementState, &mut ButtonStyle, &mut BorderColor, &mut BoxShadow, Entity), With<UiButton>>,
    window: Single<Entity, With<Window>>,
    cursor_icons: Res<CursorIcons>
) {
    let target = event.target;

    if let Ok((mut state, button_style,  mut border_color, mut shadow, _)) = query.get_mut(target) {
        if !state.selected {
            border_color.0 = button_style.hover_color;
        }
        shadow.color = button_style.hover_color;
        shadow.spread_radius = Val::Px(10.);
        shadow.blur_radius = Val::Px(10.);
        state.hovered = true;
        commands.entity(*window).insert(cursor_icons.0[4].clone());
    }
}

fn on_hover_leave(
    event: Trigger<Pointer<Out>>,
    mut query: Query<(&mut UiElementState, &mut ButtonStyle, &mut BorderColor, &mut BoxShadow, Entity), With<UiButton>>,
    mut cursor_query: Query<&mut CursorIcon>,
    cursor_icons: Res<CursorIcons>
) {
    let target = event.target;

    if let Ok((mut state, button_style,  mut border_color, mut shadow, _)) = query.get_mut(target) {
        if !state.selected {
            border_color.0 = button_style.border_color;
        }
        shadow.color = button_style.border_color;
        shadow.spread_radius = Val::Px(1.);
        shadow.blur_radius = Val::Px(1.);
        state.hovered = false;

        let Ok(mut cursor_icon) = cursor_query.get_single_mut() else {
            return;
        };

        *cursor_icon = cursor_icons.0[0].clone();
    }
}

fn check_selected_status(mut query: Query<(&UiElementState, &ButtonStyle, &mut BorderColor), With<UiButton>>) {
    for (state, button_style, mut border_color) in query.iter_mut() {
        if state.selected {
            border_color.0 = button_style.focus_color;
        } else {
            border_color.0 = button_style.border_color;
        }
    }
}