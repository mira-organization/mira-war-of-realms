use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::window::PrimaryWindow;
use crate::colors::Colored;
use crate::{Radius, UiGenID};
use crate::UiElementState;

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
#[require(UiGenID, UiElementState, SliderStyle)]
pub struct Slider {
    pub value: i32,
    pub step: i32,
    pub min: i32,
    pub max: i32,
}

impl Default for Slider {
    fn default() -> Self {
        Self {
            value: 0,
            step: 1,
            min: 0,
            max: 100,
        }
    }
}

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct SliderStyle {
    pub width: Val,
    pub height: Val,
    pub background_color: Color,
    pub border_color: Color,
    pub border_radius: Radius,
    pub border: UiRect,
    pub track_background_color: Color,
    pub thumb_color: Color,
    pub thump_width: Val,
    pub thumb_height: Val,
    pub thumb_border: UiRect,
    pub thumb_border_radius: Radius,
}

impl Default for SliderStyle {
    fn default() -> Self {
        Self {
            width: Val::Px(300.),
            height: Val::Px(10.),
            background_color: Colored::white(),
            border_color: Colored::transparent(),
            border: UiRect::all(Val::Px(0.)),
            border_radius: Radius::all(Val::Px(5.)),
            track_background_color: Colored::main_accent(),
            thumb_color: Colored::white(),
            thump_width: Val::Px(20.),
            thumb_height: Val::Px(20.),
            thumb_border: UiRect::all(Val::Px(0.)),
            thumb_border_radius: Radius::all(Val::Percent(50.)),
        }
    }
}

#[derive(Component)]
pub struct SliderRoot;

#[derive(Component)]
pub struct SliderTrack;

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct SliderThumb {
    pub current_x: f32,
}

pub struct SliderUIPlugin;

impl Plugin for SliderUIPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Slider>();
        app.register_type::<SliderStyle>();
        app.register_type::<SliderThumb>();
        app.add_systems(Update, build_detect_slider);
    }
}

fn build_detect_slider(mut commands: Commands, query: Query<(Entity, &Slider, &SliderStyle), Without<SliderRoot>>) {
    for (entity, _, style) in query.iter() {
        // Main Slider Node
        commands.entity(entity).insert((
            Node {
                width: style.width,
                height: style.height,
                display: Display::Flex,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
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
            SliderRoot
        )).with_children(|builder| {

            // Slider Track
            builder.spawn((
                Node {
                    width: Val::Px(0.),
                    height: Val::Percent(100.),
                    ..default()
                },
                BorderRadius {
                    top_left: style.border_radius.top_left,
                    top_right: style.border_radius.top_right,
                    bottom_left: style.border_radius.bottom_left,
                    bottom_right: style.border_radius.bottom_right,
                },
                BackgroundColor(style.track_background_color),
                RenderLayers::layer(1),
                PickingBehavior::IGNORE,
                SliderTrack,
            ));

            // Slider Thumb
            builder.spawn((
                Node {
                    width: style.thump_width,
                    height: style.thumb_height,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                BackgroundColor(style.thumb_color),
                BorderRadius {
                    top_left: style.thumb_border_radius.top_left,
                    top_right: style.thumb_border_radius.top_right,
                    bottom_left: style.thumb_border_radius.bottom_left,
                    bottom_right: style.thumb_border_radius.bottom_right,
                },
                BoxShadow {
                    color: Color::BLACK,
                    spread_radius: Val::Px(3.),
                    blur_radius: Val::Px(3.),
                    y_offset: Val::Px(0.),
                    x_offset: Val::Px(0.),
                    ..default()
                },
                RenderLayers::layer(1),
                SliderThumb { current_x: 0. },
            )).observe(on_move_thumb);

        });
    }
}

fn on_move_thumb(
    event: Trigger<Pointer<Drag>>,
    mut query: Query<(&mut UiElementState, &mut Slider, &Node, &Children, &Parent), (With<Slider>, Without<SliderTrack>, Without<SliderThumb>)>,
    mut track_query: Query<&mut Node, (With<SliderTrack>, Without<SliderThumb>, Without<Slider>)>,
    mut thumb_query: Query<(&mut Node, &mut SliderThumb), (With<SliderThumb>, Without<SliderTrack>, Without<Slider>)>,
    parent_check: Query<&Node, (Without<Slider>, Without<SliderTrack>, Without<SliderThumb>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();
    let view_port = Vec2::new(window.resolution.width(), window.resolution.height());

    for (_, mut slider, node, children, main_parent) in query.iter_mut() {
        let slider_width;
        if let Ok(parent_node) = parent_check.get(main_parent.get()) {
            slider_width = node.width.resolve(parent_node.width.resolve(0.0, view_port).unwrap_or(0.0), view_port).unwrap_or(10.0);
        } else {
            slider_width = node.width.resolve(0.0, view_port).unwrap_or(10.0);
        }

        for child in children.iter() {
            if event.target.eq(child) {
                let next_child = children.iter().next();
                if let Ok((mut thumb_node, mut thumb)) = thumb_query.get_mut(*child) {
                    thumb.current_x += event.event.delta.x;
                    thumb.current_x = thumb.current_x.clamp(0.0, slider_width);

                    thumb_node.left = Val::Px(thumb.current_x - 10.0);

                    if let Some(track_child) = next_child {
                        if let Ok(mut track_node) = track_query.get_mut(*track_child) {
                            track_node.width = Val::Px(thumb.current_x);
                        }
                    }

                    let percent = thumb.current_x / slider_width;
                    let range = (slider.max - slider.min) as f32;
                    let raw_value = slider.min as f32 + percent * range;
                    let stepped_value = ((raw_value / slider.step as f32).round() * slider.step as f32) as i32;
                    slider.value = stepped_value;
                }
            }
        }
    }
}

