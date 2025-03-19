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

pub const PLAYER_VOID_THRESHOLD: f32 = -100.0;