use bevy::prelude::*;
mod util;

static WORDS_CSV: &str = include_str!("words.csv");

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}