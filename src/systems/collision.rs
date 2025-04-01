use bevy::prelude::*;
use bevy::ecs::schedule::NextState;

use crate::components::{Player, Obstacle};
use crate::resources::{Score, GameState};
use crate::constants::{PLAYER_SIZE, OBSTACLE_SIZE};

pub fn check_collisions(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    score: ResMut<Score>,
    player_query: Query<(&Transform, &Player), With<Player>>,
    obstacle_query: Query<(Entity, &Transform), With<Obstacle>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((player_transform, _)) = player_query.get_single() {
        for (_, obstacle_transform) in obstacle_query.iter() {
            let collision = player_transform.translation.x < obstacle_transform.translation.x + OBSTACLE_SIZE.x
                && player_transform.translation.x + PLAYER_SIZE.x > obstacle_transform.translation.x
                && player_transform.translation.y < obstacle_transform.translation.y + OBSTACLE_SIZE.y
                && player_transform.translation.y + PLAYER_SIZE.y > obstacle_transform.translation.y;

            if collision {
                println!("💥 Game Over! Final Score: {:.0}", score.0);
                next_state.set(GameState::GameOver);

                for (obstacle_entity, _) in obstacle_query.iter() {
                    commands.entity(obstacle_entity).despawn();
                }

                commands.spawn(
                    TextBundle::from_section(
                        format!("Game Over!\nScore: {:.0}\nPress R to Restart", score.0),
                        TextStyle {
                            font: asset_server.load("FiraSans-Bold.ttf"),
                            font_size: 50.0,
                            color: Color::WHITE,
                        },
                    )
                        .with_style(Style {
                            position_type: PositionType::Absolute,
                            top: Val::Px(100.0),
                            left: Val::Px(250.0),
                            ..default()
                        }),
                );

                break;
            }
        }
    }
}
