use bevy::prelude::*;
use bevy::ecs::schedule::States;

#[derive(Resource, States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    Running,
    GameOver,
}

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

impl Default for SpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(2.0, TimerMode::Repeating))
    }
}

#[derive(Resource)]
pub struct Score(pub f32, pub f32);

impl Default for Score {
    fn default() -> Self {
        Self(0.0, 1.0)
    }
}

#[derive(Resource)]
pub struct CoinWallet {
    pub coins: u32,
}

impl Default for CoinWallet {
    fn default() -> Self {
        Self { coins: 0 }
    }
}

#[derive(Resource)]
pub struct CurrentSkin {
    pub color: Color,
}
