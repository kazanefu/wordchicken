use bevy::prelude::*;

#[derive(Copy,Clone,Debug,Hash,PartialEq,Eq,Default,States)]
pub enum GameState {
    #[default]
    Start,
    TargetTextSetting,
    Guess,
    Result,
}

pub struct GameStatePlugin;
impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
    }
}