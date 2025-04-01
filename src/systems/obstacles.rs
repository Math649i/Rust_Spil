use bevy::prelude::*;
use rand::{rng, Rng};
use std::time::Duration;

use crate::components::Obstacle;
use crate::resources::{SpawnTimer, Score};
use crate::constants::{
    MIN_SPAWN_TIME, MAX_SPAWN_TIME, OBSTACLE_SIZE, GROUND_Y, CEILING_Y, OBSTACLE_SPEED,
};

pub fn spawn_obstacles(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        let mut rng = rng();
        let base_spawn_time = rng.gen_range(MIN_SPAWN_TIME..=MAX_SPAWN_TIME);
        let adjusted_spawn_time = (base_spawn_time / score.1).max(0.5);
        timer.0.set_duration(Duration::from_secs_f32(adjusted_spawn_time));

        let obstacle_texture = asset_server.load("spike.png");
        let floor_spike_x = rng.gen_range(350.0..450.0);
        let ceiling_spike_x = rng.gen_range(350.0..450.0);

        // Spawn floor spike
        commands.spawn((
            SpriteBundle {
                texture: obstacle_texture.clone(),
                sprite: Sprite {
                    custom_size: Some(OBSTACLE_SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(floor_spike_x, GROUND_Y, 0.0),
                ..default()
            },
            Obstacle,
        ));

        // Spawn ceiling spike if score high enough
        if score.0 >= 100.0 {
            commands.spawn((
                SpriteBundle {
                    texture: obstacle_texture,
                    sprite: Sprite {
                        custom_size: Some(OBSTACLE_SIZE),
                        flip_y: true,
                        ..default()
                    },
                    transform: Transform::from_xyz(ceiling_spike_x, CEILING_Y, 0.0),
                    ..default()
                },
                Obstacle,
            ));
        }
    }
}

pub fn move_obstacles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform), With<Obstacle>>,
) {
    let delta_time = time.delta_seconds();

    for (entity, mut transform) in query.iter_mut() {
        transform.translation.x += OBSTACLE_SPEED * delta_time;

        if transform.translation.x < -400.0 {
            commands.entity(entity).despawn();
        }
    }
}
