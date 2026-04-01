use std::default;

use bevy::prelude::*;

#[derive(Copy,Clone,Debug,Hash,PartialEq,Eq,Default,States)]
pub enum GameState {
    #[default]
    Start,
    
}