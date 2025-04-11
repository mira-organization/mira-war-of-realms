use bevy::prelude::*;
use crate::music::title_music::TitleMusic;

pub mod title_music;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TitleMusic);
    }
}