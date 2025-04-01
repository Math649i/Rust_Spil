use bevy::prelude::*;
use bevy::ecs::schedule::NextState;

use crate::resources::{Score, GameState};
use crate::components::{Player, Obstacle};
use crate::systems::setup::setup;
use crate::systems::shop::ShopUI;

pub fn restart_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut score: ResMut<Score>,
    mut commands: Commands,
    text_entities: Query<Entity, With<Text>>,
    player_query: Query<Entity, With<Player>>,
    obstacle_query: Query<Entity, With<Obstacle>>,
    camera_query: Query<Entity, With<Camera>>,
    shop_query: Query<Entity, With<ShopUI>>, // ✅ added this
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        println!("🔄 Restarting Game...");

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
        for entity in shop_query.iter() {
            commands.entity(entity).despawn_recursive(); // ✅ despawn UI properly
        }

        score.0 = 0.0;

        next_state.set(GameState::Running);
    }
}
