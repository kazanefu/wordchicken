#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::prelude::*;
mod state_manager;
mod util;
mod start;
mod set_target;
mod guess;
mod result;

static WORDS_CSV: &str = include_str!("words.csv");
static EMBEDDINGS_BYTES: &[u8] = include_bytes!("../../../assets/embedding.bin");

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    bevy::asset::embedded_asset!(app, "fonts/NotoSansJP-Bold.ttf");
    app.add_plugins(state_manager::GameStatePlugin)
        .add_plugins(util::words::WordPlugin)
        .add_plugins(start::StartPlugin)
        .add_plugins(set_target::SetTargetTextPlugin)
        .add_plugins(guess::GuessPlugin)
        .add_plugins(result::ResultPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
