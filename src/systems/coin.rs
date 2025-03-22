use bevy::prelude::*;
use rand::Rng;

use crate::components::{Coin, Player};
use crate::resources::{Score, CoinWallet};
use crate::constants::OBSTACLE_SPEED;

#[derive(Resource)]
pub struct CoinSpawnTimer(pub Timer);

/// Spawns coins in the middle of the screen after score 100
pub fn spawn_coins(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
    time: Res<Time>,
    mut timer: ResMut<CoinSpawnTimer>,
) {
    if score.0 < 100.0 {
        return;
    }

    timer.0.tick(time.delta());
    if !timer.0.finished() {
        return;
    }

    let mut rng = rand::thread_rng();
    let x = rng.gen_range(300.0..500.0);
    let y = 0.0;

    let texture = asset_server.load("coin.png");

    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform::from_xyz(x, y, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(60.0)), // ✅ Larger size
                ..default()
            },
            ..default()
        },
        Coin,
    ));
}

/// Moves coins left and despawns them off-screen
pub fn move_coins(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform), With<Coin>>,
) {
    let delta_time = time.delta_seconds();

    for (entity, mut transform) in query.iter_mut() {
        transform.translation.x += OBSTACLE_SPEED * delta_time;

        if transform.translation.x < -400.0 {
            commands.entity(entity).despawn();
        }
    }
}

/// Collects coins when player touches them
pub fn collect_coins(
    mut commands: Commands,
    mut wallet: ResMut<CoinWallet>,
    player_query: Query<&Transform, With<Player>>,
    coin_query: Query<(Entity, &Transform), With<Coin>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (coin_entity, coin_transform) in coin_query.iter() {
            let distance = player_transform.translation.distance(coin_transform.translation);
            if distance < 30.0 {
                wallet.coins += 1;
                commands.entity(coin_entity).despawn();
                println!("💰 Coin collected! Total: {}", wallet.coins);
            }
        }
    }
}
