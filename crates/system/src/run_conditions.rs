use bevy::prelude::*;

pub fn detect_inputs(keyboard: Res<ButtonInput<KeyCode>>) -> bool {
    !keyboard.get_just_pressed().is_empty()
}