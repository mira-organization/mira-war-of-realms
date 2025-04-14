use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use crate::UiGenID;
use crate::{Radius, UiElementState};
use crate::colors::Colored;

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
#[require(UiGenID, UiElementState, CheckBoxStyle)]
pub struct CheckBox {
    pub checked: bool,
    pub label: String,
}

impl Default for CheckBox {
    fn default() -> Self {
        Self {
            checked: false,
            label: String::from("Checkbox"),
        }
    }
}

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct CheckBoxStyle {
    pub width: Val,
    pub height: Val,
    pub color: Color,
    pub font_size: f32,
    pub label_space: Val,
    pub disabled_color: Color,
    pub background_color: Color,
    pub border_color: Color,
    pub focus_color: Color,
    pub hover_color: Color,
    pub border: UiRect,
    pub border_radius: Radius,
    pub padding: UiRect,
    pub check_size: f32,
    pub check_mark_size: f32,
    pub check_border: UiRect,
    pub check_border_radius: Radius,
    pub check_shape: Option<CheckShape>,
    pub check_color: Color,
    pub check_border_color: Color,
    pub check_image: Option<Handle<Image>>,
}

impl Default for CheckBoxStyle {
    fn default() -> Self {
        Self {
            width: Val::Px(200.0),
            height: Val::Px(50.),
            color: Colored::font_black_100(),
            font_size: 14.,
            label_space: Val::Px(20.),
            disabled_color: Colored::disable_ui_color(),
            background_color: Colored::white(),
            border_color: Colored::main_gray(),
            focus_color: Colored::main_accent(),
            hover_color: Colored::main_accent_lighter(),
            border: UiRect::all(Val::Px(0.)),
            border_radius: Radius::all(Val::Px(0.)),
            padding: UiRect::all(Val::Px(0.)),
            check_size: 22.5,
            check_mark_size: 10.,
            check_border: UiRect::all(Val::Px(2.)),
            check_border_radius: Radius::all(Val::Px(5.)),
            check_shape: Some(CheckShape::Square),
            check_color: Colored::main_accent(),
            check_border_color: Colored::main_gray(),
            check_image: None,
        }
    }
}

#[derive(Reflect, Debug, Clone, PartialEq, Eq)]
pub enum CheckShape {
    Square,
    Circle
}

#[derive(Component)]
pub struct CheckBoxRoot;

#[derive(Component)]
pub struct CheckBoxLabel;

#[derive(Component)]
pub struct CheckBoxMark;

#[derive(Component)]
pub struct DirectMark;

pub struct CheckBoxUiPPlugin;

impl Plugin for CheckBoxUiPPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CheckBox>();
        app.register_type::<CheckBoxStyle>();
        app.add_systems(Update, build_detect_checkbox);
    }
}

fn build_detect_checkbox(
    mut commands: Commands,
    //asset_server: Res<AssetServer>,
    query: Query<(Entity, &CheckBoxStyle, &CheckBox), Without<CheckBoxRoot>>,
) {
    for (entity, style, checkbox) in query.iter() {
        commands.entity(entity).insert((
            Node {
                width: style.width,
                height: style.height,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                column_gap: style.label_space,
                padding: style.padding,
                border: style.border,
                ..default()
            },
            BorderRadius {
                top_left: style.border_radius.top_left,
                top_right: style.border_radius.top_right,
                bottom_left: style.border_radius.bottom_left,
                bottom_right: style.border_radius.bottom_right,
            },
            BorderColor(style.border_color),
            BackgroundColor(style.background_color),
            RenderLayers::layer(1),
            CheckBoxRoot
        ))
            .observe(on_click)
            .with_children(|builder| {

            builder.spawn((
                Node {
                    width: Val::Px(style.check_size),
                    height: Val::Px(style.check_size),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: style.check_border,
                    padding: UiRect::all(Val::Px(2.)),
                    ..default()
                },
                RenderLayers::layer(1),
                BorderRadius {
                    top_left: style.check_border_radius.top_left,
                    top_right: style.check_border_radius.top_right,
                    bottom_left: style.check_border_radius.bottom_left,
                    bottom_right: style.check_border_radius.bottom_right,
                },
                BorderColor(style.check_border_color),
                CheckBoxMark,
                PickingBehavior::IGNORE
            ))
                .with_children(|child| {
                child.spawn((
                    Node {
                        width: Val::Px(style.check_mark_size),
                        height: Val::Px(style.check_mark_size),
                        ..default()
                    },
                    RenderLayers::layer(1),
                    BackgroundColor(if checkbox.checked { style.check_color } else { Colored::transparent() }),
                    PickingBehavior::IGNORE,
                    DirectMark
                ));
            });

            builder.spawn((
                Text::new(checkbox.label.clone()),
                TextFont {
                    font_size: style.font_size,
                    ..default()
                },
                PickingBehavior::IGNORE,
                TextColor(style.color),
                RenderLayers::layer(1),
                CheckBoxLabel
            ));

        });
    }
}

fn on_click(
    event: Trigger<Pointer<Click>>,
    mut query: Query<(&mut UiElementState, &mut CheckBox, &CheckBoxStyle, &mut BorderColor, Entity), With<CheckBox>>,
    mut inner_query: Query<&mut BackgroundColor, With<DirectMark>>,
) {
    let target = event.target;
    for (mut state, mut checkbox, style, mut border_color, entity) in query.iter_mut() {
        if target.eq(&entity) {
            border_color.0 = style.focus_color;
            state.selected = true;
            checkbox.checked = !checkbox.checked;
            info!("Checkbox clicked: {:?}", checkbox.checked);

            for mut background_color in inner_query.iter_mut() {
                if checkbox.checked {
                    background_color.0 = style.check_color;
                } else {
                    background_color.0 = Colored::transparent();
                }
            }
        }
    }
}