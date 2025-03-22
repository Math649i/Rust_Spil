use bevy::prelude::*;

use crate::resources::{GameState, Score, CurrentSkin};
use crate::components::{Player, Obstacle, MainCamera};
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
    camera_query: Query<Entity, With<MainCamera>>, // ✅ Added
    skin: Res<CurrentSkin>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        println!("🔄 Restarting Game...");

        *game_state = GameState::Running;
        score.0 = 0.0;

        // ✅ Despawn all relevant entities
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

        // ✅ Respawn everything
        setup(commands, asset_server, windows, skin);
    }
}
