use bevy::prelude::*;

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

#[derive(Resource, PartialEq)]
pub enum GameState {
    Running,
    GameOver,
}

#[derive(Resource)]
pub struct Score(pub f32, pub f32);
