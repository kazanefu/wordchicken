use bevy::prelude::*;
mod util;
mod state_manager;

static WORDS_CSV: &str = include_str!("words.csv");
static EMBEDDINGS_BYTES: &[u8] = include_bytes!("../../../assets/embedding.bin");

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(util::words::WordPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
