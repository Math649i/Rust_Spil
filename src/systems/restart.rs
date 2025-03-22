use bevy::prelude::*;

use crate::resources::{GameState, Score};
use crate::components::{Player, Obstacle};
use crate::systems::setup::setup;

pub fn restart_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut score: ResMut<Score>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window>,
    text_entities: Query<Entity, With<Text>>,
    player_query: Query<Entity, With<Player>>,
    obstacle_query: Query<Entity, With<Obstacle>>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        println!("🔄 Restarting Game...");

        *game_state = GameState::Running;
        score.0 = 0.0;

        for entity in text_entities.iter() {
            commands.entity(entity).despawn();
        }
        for entity in player_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in obstacle_query.iter() {
            commands.entity(entity).despawn();
        }

        setup(commands, asset_server, windows);
    }
}
