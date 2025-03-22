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
}

use bevy::prelude::*;
use resources::{GameState, Score, SpawnTimer};
use systems::{
    collision::check_collisions,
    obstacles::{spawn_obstacles, move_obstacles},
    player::player_movement,
    restart::restart_game,
    score::update_score,
    setup::setup,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(GameState::Running)
        .insert_resource(SpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .insert_resource(Score(0.0, 1.0))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            update_score,
            restart_game,
            player_movement.run_if(|state: Res<GameState>| *state == GameState::Running),
            spawn_obstacles.run_if(|state: Res<GameState>| *state == GameState::Running),
            move_obstacles.run_if(|state: Res<GameState>| *state == GameState::Running),
            check_collisions,
        ))
        .run();
}
