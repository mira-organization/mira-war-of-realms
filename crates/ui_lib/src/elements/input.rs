use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::time::Duration;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::utils::HashMap;
use bevy::winit::cursor::CursorIcon;
use system::data::CursorIcons;
use crate::colors::Colored;
use crate::Radius;

static TEXT_FIELD_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
#[require(InputStyle, InputType)]
pub struct TextField {
    pub id: usize,
    pub text: String,
    pub placeholder: String,
    pub is_enabled: bool,
    pub is_focused: bool,
    pub is_hovered: bool,
    pub is_delete_after_enter: bool,
    pub cursor_position: usize,
    pub max_length: usize,
}

impl Default for TextField {
    fn default() -> Self {
        Self {
            id: TEXT_FIELD_COUNTER.fetch_add(1, Relaxed),
            text: String::from(""),
            is_enabled: true,
            is_focused: false,
            is_hovered: false,
            is_delete_after_enter: false,
            placeholder: String::from("Text"),
            cursor_position: 0,
            max_length: 20,
        }
    }
}

impl TextField {
    pub fn new(text: &str, placeholder: bool) -> Self {
        Self {
            text: if !placeholder { text.to_string() } else { String::from("") } ,
            placeholder: if placeholder { text.to_string() } else { String::from("") } ,
            ..default()
        }
    }
}

#[derive(Component, Reflect, Default, Debug, Eq, Clone, PartialEq)]
#[reflect(Component)]
pub enum InputType {
    #[default]
    Text,
    Password,
}

#[derive(Component, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct InputStyle {
    pub width: Val,
    pub height: Val,
    pub color: Color,
    pub font_size: f32,
    pub disabled_color: Color,
    pub background_color: Color,
    pub placeholder_color: Color,
    pub border_color: Color,
    pub focus_color: Color,
    pub hover_color: Color,
    pub border: UiRect,
    pub border_radius: Radius,
}

impl Default for InputStyle {
    fn default() -> Self {
        Self {
            width: Val::Px(150.),
            height: Val::Px(35.),
            color: Colored::font_black_100(),
            font_size: 14.,
            disabled_color: Colored::disable_ui_color(),
            background_color: Colored::blue_white(),
            placeholder_color: Colored::placeholder_ui_color(),
            border_color: Colored::black(),
            focus_color: Colored::main_accent(),
            hover_color: Colored::main_accent_lighter(),
            border: UiRect::all(Val::Px(2.)),
            border_radius: Radius::all(Val::Px(5.)),
        }
    }
}

#[derive(Component)]
pub struct TextFieldRoot;

#[derive(Component)]
pub struct TextFieldText;

#[derive(Component)]
pub struct TextCursor;

#[derive(Resource, Default)]
struct KeyRepeatTimers {
    timers: HashMap<KeyCode, Timer>,
}

#[derive(Resource)]
pub struct CursorBlinkTimer {
    pub timer: Timer,
}

impl Default for CursorBlinkTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.95, TimerMode::Repeating)
        }
    }
}

pub struct InputUiPlugin;

impl Plugin for InputUiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(KeyRepeatTimers::default());
        app.insert_resource(CursorBlinkTimer::default());
        app.register_type::<TextField>();
        app.register_type::<InputStyle>();
        app.register_type::<InputType>();
        app.add_systems(Update, (
            build_detect_input,
            handle_input_focus,
            handle_typing,
            update_cursor_position,
            update_cursor_visibility
        ));
    }
}

fn build_detect_input(
    mut commands: Commands,
    query: Query<(Entity, &InputStyle, &TextField), Without<TextFieldRoot>>
) {
    for (entity, input_style, text_field) in &query {
        commands.entity(entity)
            .insert((
                Node {
                    width: input_style.width,
                    height: input_style.height,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(5.)),
                    border: input_style.border,
                    ..default()
                },
                BorderRadius {
                    top_left: input_style.border_radius.top_left,
                    top_right: input_style.border_radius.top_right,
                    bottom_left: input_style.border_radius.bottom_left,
                    bottom_right: input_style.border_radius.bottom_right
                },
                BackgroundColor(input_style.background_color),
                BorderColor(input_style.border_color),
                RenderLayers::layer(1),
                TextFieldRoot
            ))
            .observe(on_hover_enter)
            .observe(on_hover_leave)
            .observe(on_click)
            .with_children(|builder| {

                builder.spawn((
                    Node {
                        width: Val::Px(1.5),
                        height: Val::Px(input_style.font_size + 2.0),
                        ..default()
                    },
                    BackgroundColor(input_style.color),
                    RenderLayers::layer(1),
                    Visibility::Hidden,
                    TextCursor
                ));

                let mut text = text_field.placeholder.clone();
                if text.is_empty() {
                    text = text_field.text.clone();
                }

                builder.spawn((
                    Text::new(text),
                    TextFont {
                        font_size: input_style.font_size,
                        ..default()
                    },
                    PickingBehavior::IGNORE,
                    TextColor(input_style.placeholder_color),
                    RenderLayers::layer(1),
                    TextFieldText
                ));
        });
    }
}

fn on_hover_enter(
    event: Trigger<Pointer<Over>>,
    mut commands: Commands,
    mut query: Query<(&mut TextField, &mut InputStyle, &mut BorderColor, Entity)>,
    window: Single<Entity, With<Window>>,
    cursor_icons: Res<CursorIcons>
) {
    let target = event.target;

    if let Ok((mut text_field, input_style, mut border_color, _)) = query.get_mut(target) {
        if !text_field.is_focused {
            border_color.0 = input_style.hover_color;
        }
        text_field.is_hovered = true;
        commands.entity(*window).insert(cursor_icons.0[3].clone());
    }
}

fn on_hover_leave(
    event: Trigger<Pointer<Out>>,
    mut query: Query<(&mut TextField, &mut InputStyle, &mut BorderColor, Entity)>,
    mut cursor_query: Query<&mut CursorIcon>,
    cursor_icons: Res<CursorIcons>
) {
    let target = event.target;

    if let Ok((mut text_field, input_style, mut border_color, _)) = query.get_mut(target) {
        if !text_field.is_focused {
            border_color.0 = input_style.border_color;
        }
        text_field.is_hovered = false;

        let Ok(mut cursor_icon) = cursor_query.get_single_mut() else {
            return;
        };

        *cursor_icon = cursor_icons.0[0].clone();
    }
}

fn on_click(event: Trigger<Pointer<Click>>,
            mut query: Query<(&mut TextField, &mut InputStyle, &mut BorderColor, Entity)>,
            mut cursor_query: Query<(&mut Visibility, &TextCursor)>
) {
    let target = event.target;

    for (mut text_field, style, mut border_color, entity) in query.iter_mut() {
        if target.eq(&entity) {
            border_color.0 = style.focus_color;
            text_field.is_focused = true;

            if let Ok((mut visibility, _)) = cursor_query.get_mut(entity) {
                *visibility = Visibility::Visible;
            }

            continue;
        }

        border_color.0 = style.border_color;
        text_field.is_focused = false;
    }
}

fn update_cursor_visibility(
    time: Res<Time>,
    mut cursor_blink_timer: ResMut<CursorBlinkTimer>,
    mut cursor_query: Query<(&mut Visibility, &mut BackgroundColor, &Parent), With<TextCursor>>,
    mut input_field_query: Query<(&TextField, &mut InputStyle, &InputType, &mut BorderColor, &Children), With<TextFieldRoot>>, // Assuming Focus component indicates if field is focused
    mut text_query: Query<(&mut Text, &mut TextColor), With<TextFieldText>>,
) {
    cursor_blink_timer.timer.tick(time.delta());

    for (mut visibility, mut background, parent) in cursor_query.iter_mut() {
        if let Ok((focus, style, in_type, mut border_color, children)) = input_field_query.get_mut(parent.get()) {
            // Show the cursor if the input field is focused
            if focus.is_focused {
                let alpha = (cursor_blink_timer.timer.elapsed_secs() * 2.0 * std::f32::consts::PI).sin() * 0.5 + 0.5;
                background.0.set_alpha(alpha);

                if !visibility.eq(&Visibility::Visible) {
                    border_color.0 = style.focus_color;

                    *visibility = Visibility::Visible;
                    for child in children.iter() {
                        if let Ok((mut text, _)) = text_query.get_mut(*child) {
                            if in_type.eq(&InputType::Password) {
                                let masked_text: String = "*".repeat(focus.text.chars().count());
                                text.0 = masked_text;
                            } else {
                                text.0 = focus.text.clone();
                            }
                        }
                    }
                }
            } else {
                if !visibility.eq(&Visibility::Hidden) {
                    border_color.0 = style.border_color;
                    *visibility = Visibility::Hidden;
                    for child in children.iter() {
                        if let Ok((mut text, mut color)) = text_query.get_mut(*child) {
                            if focus.text.is_empty() {
                                text.0 = focus.placeholder.clone();
                                color.0 = style.placeholder_color;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn update_cursor_position(
    mut cursor_query: Query<(&mut Node, &Parent), With<TextCursor>>,
    mut text_field_query: Query<(&mut TextField, &InputStyle, &Parent)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for (mut cursor_node, parent) in cursor_query.iter_mut() {
        // Get the associated TextField for the current entity
        if let Ok((mut text_field, style, _)) = text_field_query.get_mut(parent.get()) {
            // Handle arrow key movement
            if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
                text_field.cursor_position = text_field.cursor_position.saturating_sub(1); // Move cursor left
            }
            if keyboard_input.just_pressed(KeyCode::ArrowRight) {
                text_field.cursor_position += 1; // Move cursor right
            }

            // Ensure cursor position does not exceed text bounds
            text_field.cursor_position = text_field.cursor_position.min(text_field.text.len());

            // Update the cursor position based on the cursor position in the text
            let cursor_x_position = calculate_cursor_x_position(&text_field, text_field.cursor_position, style);
            cursor_node.left = Val::Px(cursor_x_position); // Set the cursor's X position
        }
    }
}

fn calculate_cursor_x_position(text_field: &TextField, cursor_pos: usize, style: &InputStyle) -> f32 {
    // Ensure the cursor position is within the bounds of the text
    if text_field.text.is_empty() || cursor_pos == 0 {
        return 0.0; // No text or cursor at the start
    }

    // Ensure the cursor position doesn't exceed the text length
    let cursor_pos = cursor_pos.min(text_field.text.len());

    // Calculate the width of the text up to the cursor position
    let text_substr = &text_field.text[..cursor_pos];
    let text_width = calculate_text_width(text_substr, style);

    text_width + 1.0 // Add some padding so the cursor isn't directly on the text
}

fn calculate_text_width(text: &str, style: &InputStyle) -> f32 {
    // Calculate text width based on font size
    let font_size = style.font_size; // Default font size if none is provided
    text.len() as f32 * font_size * 0.6 // Adjust factor based on font characteristics
}

fn handle_typing(
    time: Res<Time>,
    mut key_repeat: ResMut<KeyRepeatTimers>,
    mut query: Query<(&mut TextField, &InputStyle, &InputType, &Children)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut text_query: Query<(&mut Text, &mut TextColor), With<TextFieldText>>,
) {
    let shift = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
    let alt = keyboard.pressed(KeyCode::AltLeft) || keyboard.pressed(KeyCode::AltRight);

    let initial_delay = 0.3;
    let repeat_rate = 0.07;

    for (mut text_field, style, in_type, children) in query.iter_mut() {
        if text_field.is_focused {
            for child in children.iter() {
                if let Ok((mut text, mut text_color)) = text_query.get_mut(*child) {
                    // ENTER
                    if keyboard.just_pressed(KeyCode::Enter) {
                        text_field.is_focused = false;
                        if text_field.is_delete_after_enter {
                            text_field.text.clear();
                            text.0 = text_field.text.clone();
                        }
                        continue;
                    }

                    // BACKSPACE
                    if keyboard.just_pressed(KeyCode::Backspace) {
                        if text_field.cursor_position > 0 && !text_field.text.is_empty() {
                            let pos = text_field.cursor_position - 1;
                            text_field.cursor_position = pos;
                            text_field.text.remove(pos);
                            if in_type.eq(&InputType::Password) {
                                text.0.remove(pos);
                            } else {
                                text.0 = text_field.text.clone();
                            }
                        }
                        if text.0.is_empty() {
                            text_color.0 = style.placeholder_color;
                        }
                        key_repeat.timers.insert(
                            KeyCode::Backspace,
                            Timer::from_seconds(initial_delay, TimerMode::Once),
                        );
                        continue;
                    }

                    for key in keyboard.get_pressed() {
                        if let Some(char) = keycode_to_char(*key, shift, alt) {
                            if keyboard.just_pressed(*key) {
                                let pos = text_field.cursor_position;

                                if pos >= text_field.max_length {
                                    debug!("(Max Length {} / Current {}) was reached", text_field.max_length, pos);
                                    return;
                                }

                                if in_type.eq(&InputType::Password) {
                                    text_field.text.insert(pos, char);
                                    text_field.cursor_position += 1;
                                    text.0.insert(pos, '*');
                                } else {
                                    text_field.text.insert(pos, char);
                                    text_field.cursor_position += 1;
                                    text.0 = text_field.text.clone();
                                }
                                text_color.0 = style.color;
                                key_repeat.timers.insert(
                                    *key,
                                    Timer::from_seconds(initial_delay, TimerMode::Once),
                                );
                                continue;
                            }

                            if let Some(timer) = key_repeat.timers.get_mut(key) {
                                timer.tick(time.delta());
                                if timer.finished() {
                                    text_field.text.push(char);
                                    text_field.cursor_position += 1;
                                    if in_type.eq(&InputType::Password) {
                                        text.0.push('*');
                                    } else {
                                        text.0 = text_field.text.clone();
                                    }
                                    timer.set_duration(Duration::from_secs_f32(repeat_rate));
                                    timer.reset();
                                }
                            }
                        }
                    }

                    if keyboard.pressed(KeyCode::Backspace) {
                        if let Some(timer) = key_repeat.timers.get_mut(&KeyCode::Backspace) {
                            timer.tick(time.delta());
                            if timer.finished() {
                                if text_field.cursor_position > 0 && !text_field.text.is_empty() {
                                    text_field.text.pop();
                                    text_field.cursor_position -= 1;
                                    if in_type.eq(&InputType::Password) {
                                        text.0.pop();
                                    } else {
                                        text.0 = text_field.text.clone();
                                    }
                                    timer.set_duration(Duration::from_secs_f32(repeat_rate));
                                    timer.reset();
                                }
                            }
                        }
                    }

                    text_field.cursor_position = text_field.cursor_position.min(text_field.text.len());
                }
            }
        }
    }

    key_repeat.timers.retain(|key, _| keyboard.pressed(*key));
}


fn handle_input_focus(
    mut query: Query<(&mut TextField, Entity)>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let mut sorted_fields: Vec<_> = query.iter_mut().collect();
    sorted_fields.sort_by_key(|(field, _)| field.id);

    let mut any_focused = false;

    // TAB Focus
    if keyboard.just_pressed(KeyCode::Tab) {
        let len = sorted_fields.len();
        if len == 0 {
            return;
        }

        // Find the currently focused field and move the focus to the next one
        for i in 0..len {
            if sorted_fields[i].0.is_focused {
                sorted_fields[i].0.is_focused = false;

                let next = (i + 1) % len;  // Get the next field in the list

                if let Some(&mut (_, _)) = sorted_fields.get_mut(next) { // Set the focused color
                    sorted_fields[next].0.is_focused = true; // Set focus to the next field
                    any_focused = true;
                }

                break;  // Exit the loop after setting the next focus
            }
        }

        // If no focus was found, set the first field as focused
        if !any_focused && len > 0 {
            sorted_fields[0].0.is_focused = true;
        }
    }
}

fn keycode_to_char(key: KeyCode, shift: bool, alt: bool) -> Option<char> {
    match key {
        KeyCode::KeyA => Some(if shift { 'A' } else { 'a' }),
        KeyCode::KeyB => Some(if shift { 'B' } else { 'b' }),
        KeyCode::KeyC => Some(if shift { 'C' } else { 'c' }),
        KeyCode::KeyD => Some(if shift { 'D' } else { 'd' }),
        KeyCode::KeyE => Some(if shift { 'E' } else if alt { 'E' } else { 'e' }),
        KeyCode::KeyF => Some(if shift { 'F' } else { 'f' }),
        KeyCode::KeyG => Some(if shift { 'G' } else { 'g' }),
        KeyCode::KeyH => Some(if shift { 'H' } else { 'h' }),
        KeyCode::KeyI => Some(if shift { 'I' } else { 'i' }),
        KeyCode::KeyJ => Some(if shift { 'J' } else { 'j' }),
        KeyCode::KeyK => Some(if shift { 'K' } else { 'k' }),
        KeyCode::KeyL => Some(if shift { 'L' } else { 'l' }),
        KeyCode::KeyM => Some(if shift { 'M' } else { 'm' }),
        KeyCode::KeyN => Some(if shift { 'N' } else { 'n' }),
        KeyCode::KeyO => Some(if shift { 'O' } else { 'o' }),
        KeyCode::KeyP => Some(if shift { 'P' } else { 'p' }),
        KeyCode::KeyQ => Some(if shift { 'Q' } else if alt { '@' } else { 'q' }),
        KeyCode::KeyR => Some(if shift { 'R' } else { 'r' }),
        KeyCode::KeyS => Some(if shift { 'S' } else { 's' }),
        KeyCode::KeyT => Some(if shift { 'T' } else { 't' }),
        KeyCode::KeyU => Some(if shift { 'U' } else { 'u' }),
        KeyCode::KeyV => Some(if shift { 'V' } else { 'v' }),
        KeyCode::KeyW => Some(if shift { 'W' } else { 'w' }),
        KeyCode::KeyX => Some(if shift { 'X' } else { 'x' }),
        KeyCode::KeyY => Some(if shift { 'Z' } else { 'z' }),
        KeyCode::KeyZ => Some(if shift { 'Y' } else { 'y' }),
        KeyCode::Digit0 => Some(if shift { '=' } else if alt { '}' } else { '0' }),
        KeyCode::Digit1 => Some(if shift { '!' } else if alt { '1' } else { '1' }),
        KeyCode::Digit2 => Some(if shift { '"' } else if alt { '2' } else { '2' }),
        KeyCode::Digit3 => Some(if shift { '3' } else if alt { '3' } else { '3' }),
        KeyCode::Digit4 => Some(if shift { '$' } else if alt { '4' } else { '4' }),
        KeyCode::Digit5 => Some(if shift { '%' } else if alt { '5' } else { '5' }),
        KeyCode::Digit6 => Some(if shift { '&' } else if alt { '6' } else { '6' }),
        KeyCode::Digit7 => Some(if shift { '/' } else if alt { '{' } else { '7' }),
        KeyCode::Digit8 => Some(if shift { '(' } else if alt { '[' } else { '8' }),
        KeyCode::Digit9 => Some(if shift { ')' } else if alt { ']' } else { '9' }),
        KeyCode::NumpadMultiply => Some('*'),
        KeyCode::NumpadAdd => Some('+'),
        KeyCode::NumpadSubtract => Some('-'),
        KeyCode::NumpadDivide => Some('/'),
        KeyCode::NumpadDecimal => Some(','),
        KeyCode::Numpad0 => Some('0'),
        KeyCode::Numpad1 => Some('1'),
        KeyCode::Numpad2 => Some('2'),
        KeyCode::Numpad3 => Some('3'),
        KeyCode::Numpad4 => Some('4'),
        KeyCode::Numpad5 => Some('5'),
        KeyCode::Numpad6 => Some('6'),
        KeyCode::Numpad7 => Some('7'),
        KeyCode::Numpad8 => Some('8'),
        KeyCode::Numpad9 => Some('9'),
        KeyCode::Comma => Some(if shift {';'} else {','}),
        KeyCode::Period => Some(if shift {':'} else {'.'}),
        KeyCode::Slash => Some(if shift {'_'} else {'-'}),
        KeyCode::IntlBackslash => Some(if shift {'>'} else if alt {'|'} else {'<'}),
        KeyCode::Backquote => Some(if shift {'?'} else {'^'}),
        KeyCode::Minus => Some(if shift {'?'} else if alt {'\\'} else {'?'}),
        KeyCode::BracketRight => Some(if shift {'*'} else if alt {'~'} else {'+'}),
        KeyCode::Backslash => Some(if shift {'\''} else {'#'}),
        KeyCode::Space => Some(' '),
        _ => None,
    }
}





