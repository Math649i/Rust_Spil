mod constants;
mod components;
mod resources;

mod systems {
    pub mod collision;
    pub mod obstacles;
    pub mod player;
    pub mod restart;
    pub mod score;
    pub mod setup;
    pub mod coin;
    pub mod shop;
}

use bevy::prelude::*;
use resources::{GameState, Score, SpawnTimer, CoinWallet, CurrentSkin};
use systems::{
    collision::check_collisions,
    obstacles::{spawn_obstacles, move_obstacles},
    player::player_movement,
    restart::restart_game,
    score::update_score,
    setup::setup,
    coin::{spawn_coins, collect_coins, move_coins, CoinSpawnTimer},
    shop::open_shop,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // === Resources ===
        .insert_resource(GameState::Running)
        .insert_resource(Score(0.0, 1.0))
        .insert_resource(SpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .insert_resource(CoinSpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .insert_resource(CoinWallet { coins: 0 })
        .insert_resource(CurrentSkin { color: Color::WHITE })
        // === Startup ===
        .add_systems(Startup, setup)
        // === Main Game Update ===
        .add_systems(Update, (
            update_score,
            restart_game,
            open_shop,
            player_movement.run_if(|state: Res<GameState>| *state == GameState::Running),
            spawn_obstacles.run_if(|state: Res<GameState>| *state == GameState::Running),
            move_obstacles.run_if(|state: Res<GameState>| *state == GameState::Running),
            check_collisions,
            spawn_coins.run_if(|state: Res<GameState>| *state == GameState::Running),
            move_coins.run_if(|state: Res<GameState>| *state == GameState::Running),
            collect_coins.run_if(|state: Res<GameState>| *state == GameState::Running),
        ))
        .run();
}
