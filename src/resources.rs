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

#[derive(Resource)]
pub struct CoinWallet {
    pub coins: u32,
}

#[derive(Resource)]
pub struct CurrentSkin {
    pub color: Color,
}
