use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::window::PrimaryWindow;
use crate::{Radius, UiGenID};
use crate::colors::Colored;
use crate::UiElementState;

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
#[require(UiGenID, UiElementState, ChoiceBoxStyle)]
pub struct ChoiceBox {
    pub value: ChoiceOption,
    pub options: Vec<ChoiceOption>
}

impl Default for ChoiceBox {
    fn default() -> Self {
        Self {
            value: ChoiceOption { selected: true, ..default() },
            options: vec![ChoiceOption { selected: true, ..default() }, ChoiceOption::new("Option 2", false)]
        }
    }
}

#[derive(Component, Reflect, Debug, Clone)]
pub struct ChoiceOption {
    pub label: String,
    pub internal_val: String,
    pub icon: Option<Handle<Image>>,
    pub selected: bool,
    pub disabled: bool
}

impl Default for ChoiceOption {
    fn default() -> Self {
        Self {
            label: String::from("Option 1"),
            internal_val: String::from("option1"),
            icon: None,
            selected: false,
            disabled: false
        }
    }
}

impl ChoiceOption {
    pub fn new(label: &str, disabled: bool) -> Self {
        Self {
            label: label.to_string(),
            internal_val: label.to_lowercase().trim().to_string(),
            icon: None,
            selected: false,
            disabled
        }
    }
}

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct ChoiceBoxStyle {
    pub width: Val,
    pub height: Val,
    pub background_color: Color,
    pub border_color: Color,
    pub focus_color: Color,
    pub border: UiRect,
    pub border_radius: Radius,
    pub drop_background: Color,
    pub drop_icon: Option<Handle<Image>>,
}

impl Default for ChoiceBoxStyle {
    fn default() -> Self {
        Self {
            width: Val::Px(300.),
            height: Val::Px(50.),
            background_color: Colored::white(),
            border_color: Colored::main_gray(),
            focus_color: Colored::main_accent(),
            border: UiRect::all(Val::Px(2.)),
            border_radius: Radius::all(Val::Px(8.)),
            drop_icon: None,
            drop_background: Colored::blue_white(),
        }
    }
}

#[derive(Component)]
pub struct ChoiceRoot;

#[derive(Component)]
pub struct ChoiceOptionRoot;

#[derive(Component)]
pub struct ChoiceLayoutBoxRoot;

pub struct ChoiceBoxUiPlugin;

impl Plugin for ChoiceBoxUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ChoiceBox>();
        app.register_type::<ChoiceOption>();
        app.register_type::<ChoiceBoxStyle>();
        app.add_systems(Update, build_detect_choice_box);
    }
}

fn build_detect_choice_box(
    mut commands: Commands,
    query: Query<(Entity, &ChoiceBox, &ChoiceBoxStyle), Without<ChoiceRoot>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {

    let window = window_query.single();
    let view_port = Vec2::new(window.resolution.width(), window.resolution.height());

    for (entity, choice_box, style) in query.iter() {
        commands.entity(entity).insert((
            Node {
                width: style.width,
                min_width: Val::Px(125.0),
                height: style.height,
                min_height: Val::Px(25.),
                margin: UiRect::top(Val::Px(30.)),
                ..default()
            },
            BorderRadius {
                top_left: style.border_radius.top_left,
                top_right: style.border_radius.top_right,
                bottom_left: style.border_radius.bottom_left,
                bottom_right: style.border_radius.bottom_right,
            },
            BackgroundColor(style.background_color),
            BoxShadow {
                color: Color::BLACK,
                spread_radius: Val::Px(3.),
                blur_radius: Val::Px(3.),
                y_offset: Val::Px(0.),
                x_offset: Val::Px(0.),
                ..default()
            },
            RenderLayers::layer(1),
            ChoiceRoot
        )).with_children(|builder| {

            // Choice Option Field
            builder.spawn((
                Node {
                    width: Val::Px(style.width.resolve(0.0, view_port).unwrap_or(50.0) - 50.),
                    height: Val::Percent(100.),
                    display: Display::Flex,
                    justify_content: JustifyContent::FlexStart,
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(10.),
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Percent(8.)),
                    ..default()
                },
                BorderRadius {
                    top_right: Val::Px(0.),
                    bottom_right: Val::Px(0.),
                    top_left: style.border_radius.top_left,
                    bottom_left: style.border_radius.bottom_left,
                },
                BackgroundColor(style.background_color),
                RenderLayers::layer(1),
            )).with_children(|builder| {

                if let Some(icon) = choice_box.value.icon.clone() {
                    builder.spawn((
                        ImageNode {
                            image: icon,
                            ..default()
                        },
                        RenderLayers::layer(1),
                    ));
                }

                builder.spawn((
                    Text::new(choice_box.value.label.clone()),
                    TextColor(Colored::font_black_100()),
                    TextFont {
                        font_size: 16.,
                        ..default()
                    },
                    RenderLayers::layer(1),
                ));

            });

            // Icon for drop down
            builder.spawn((
                Node {
                    width: Val::Px(50.),
                    height: Val::Percent(100.),
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Colored::blue_white()),
                BorderRadius {
                    top_right: style.border_radius.top_right,
                    bottom_right: style.border_radius.bottom_right,
                    top_left: Val::Px(0.),
                    bottom_left: Val::Px(0.),
                },
                RenderLayers::layer(1),
            )).with_children(|builder| {
                if let Some(icon) = style.drop_icon.clone() {
                    builder.spawn((
                        ImageNode {
                            image: icon,
                            ..default()
                        },
                        RenderLayers::layer(1),
                        PickingBehavior::IGNORE
                    ));
                }
            });

            // Option Layout Content
            builder.spawn((
                Node {
                    width: Val::Percent(100.),
                    min_height: Val::Px(100.),
                    max_height: Val::Px(150.),
                    position_type: PositionType::Absolute,
                    display: Display::Flex,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Start,
                    flex_direction: FlexDirection::Column,
                    overflow: Overflow {
                        y: OverflowAxis::Scroll,
                        x: OverflowAxis::Hidden,
                    },
                    top: Val::Px(51.),
                    ..default()
                },
                BackgroundColor(Colored::white()),
                BoxShadow {
                    color: Color::BLACK,
                    spread_radius: Val::Px(2.),
                    blur_radius: Val::Px(2.),
                    y_offset: Val::Px(2.),
                    x_offset: Val::Px(0.),
                    ..default()
                },
                RenderLayers::layer(1),
                ChoiceLayoutBoxRoot
            )).with_children(|builder| {

                for option in choice_box.options.iter() {
                    builder.spawn((
                        Node {
                            width: Val::Percent(100.),
                            height: Val::Px(50.),
                            padding: UiRect::all(Val::Percent(8.)),
                            display: Display::Flex,
                            justify_content: JustifyContent::FlexStart,
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(10.),
                            ..default()
                        },
                        BackgroundColor(if option.selected { Colored::main_accent() } else { Colored::white() }),
                        RenderLayers::layer(1),
                        ChoiceOptionRoot
                    )).with_children(|builder| {
                        if let Some(icon) = option.icon.clone() {
                            builder.spawn((
                                ImageNode {
                                    image: icon,
                                    ..default()
                                },
                                RenderLayers::layer(1),
                            ));
                        }

                        builder.spawn((
                            Text::new(option.label.clone()),
                            TextColor(if option.selected { Colored::white() } else { Colored::font_black_100() }),
                            TextFont {
                                font_size: 16.,
                                ..default()
                            },
                            RenderLayers::layer(1),
                        ));
                    });
                }

            });

        });
    }
}