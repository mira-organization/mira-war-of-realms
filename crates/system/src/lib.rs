#![feature(exact_size_is_empty)]

pub mod states;
pub mod utils;
pub mod config;
pub mod events;
pub mod service;
pub mod commons;
pub mod characters;
pub mod battle_commons;

pub mod run_conditions;
pub mod bundles;
pub mod data;

pub const PLAYER_VOID_THRESHOLD: f32 = -100.0;

pub const LOG_ENV_FILTER: &str = "info,\
wgpu_core=warn,wgpu_hal=error,\
offset_allocator=error,\
bevy_gltf=error, \
system=debug,\
naga=warn,\
bevy_render=info,\
symphonia_core=warn,\
symphonia_format_ogg=warn,\
symphonia_codec_vorbis=warn,\
mira_war_of_realms=debug,\
environment_lib=debug,\
audio_lib=debug,\
battle_lib=debug,\
ui_lib=debug,\
entities_lib=debug";