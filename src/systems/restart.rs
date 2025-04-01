use bevy::prelude::*;
use bevy::ecs::schedule::NextState;

use crate::resources::{Score, CurrentSkin, GameState};
use crate::components::{Player, Obstacle, MainCamera};
use crate::systems::setup::setup;

pub fn restart_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut score: ResMut<Score>,
    mut commands: Commands,
    text_entities: Query<Entity, With<Text>>,
    player_query: Query<Entity, With<Player>>,
    obstacle_query: Query<Entity, With<Obstacle>>,
    camera_query: Query<Entity, With<Camera>>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        println!("🔄 Restarting Game...");

        // Despawn entities
        for entity in text_entities.iter() {
            commands.entity(entity).despawn();
        }
        for entity in player_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in obstacle_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in camera_query.iter() {
            commands.entity(entity).despawn();
        }

        // Reset score
        score.0 = 0.0;

        // Force state transition by first going to Menu (or any dummy state), then to Running
        next_state.set(GameState::Menu); // triggers next frame
        next_state.set(GameState::Running); // triggers OnEnter(Running)
    }
}

