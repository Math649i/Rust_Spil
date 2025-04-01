use bevy::prelude::*;
use bevy::window::close_on_esc;
use bevy::ecs::schedule::common_conditions::in_state;

mod constants;
mod components;
mod resources;
mod systems {
    pub mod setup;
    pub mod obstacles;
    pub mod collision;
    pub mod score;
    pub mod restart;
    pub mod coin;
    pub mod shop;
    pub mod menu;
    pub mod movement;
}

use resources::*;
use systems::setup::setup;
use systems::movement::player_movement;
use systems::obstacles::{spawn_obstacles, move_obstacles};
use systems::collision::check_collisions;
use systems::score::update_score;
use systems::restart::restart_game;
use systems::coin::{spawn_coins, move_coins, collect_coins, CoinSpawnTimer};
use systems::shop::open_shop;
use systems::menu::{spawn_main_menu, handle_play_button};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_resource::<SpawnTimer>()
        .init_resource::<Score>()
        .init_resource::<CoinWallet>()
        .insert_resource(CurrentSkin { color: Color::WHITE })
        .insert_resource(CoinSpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_state::<GameState>()
        .add_systems(Startup, spawn_main_menu)
        .add_systems(Update, handle_play_button.run_if(in_state(GameState::Menu)))
        .add_systems(OnEnter(GameState::Running), setup)
        .add_systems(
            Update,
            (
                player_movement,
                spawn_obstacles,
                move_obstacles,
                check_collisions,
                update_score,
                spawn_coins,
                move_coins,
                collect_coins,
            )
                .run_if(in_state(GameState::Running)),
        )
        .add_systems(
            Update,
            restart_game.run_if(in_state(GameState::GameOver)),
        )
        .add_systems(
            Update,
            open_shop.run_if(
                in_state(GameState::Running).or_else(in_state(GameState::GameOver)),
            ),
        )
        .add_systems(Update, close_on_esc)
        .run();
}
