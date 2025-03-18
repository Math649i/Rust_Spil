use bevy::prelude::*;
use bevy::input::keyboard::KeyCode; // ✅ Fixed Import for Bevy 0.15.3
use bevy::input::ButtonInput; // ✅ Fixed Input Type
use rand::Rng;

const PLAYER_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const OBSTACLE_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const GROUND_Y: f32 = -100.0;
const GRAVITY: f32 = -600.0;
const JUMP_VELOCITY: f32 = 300.0;
const OBSTACLE_SPEED: f32 = -200.0;
const MIN_SPAWN_TIME: f32 = 1.0;
const MAX_SPAWN_TIME: f32 = 3.0;

#[derive(Component)]
struct Player {
    velocity: f32,
    on_ground: bool,
}

#[derive(Component)]
struct Obstacle;

#[derive(Resource)]
struct SpawnTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(SpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement, spawn_obstacles, move_obstacles, check_collisions))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Ground
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb_u8(34, 139, 34), // Green ground
                custom_size: Some(Vec2::new(1000.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, GROUND_Y - 25.0, 0.0),
            ..default()
        },
    ));

    // Player
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb_u8(210, 105, 30), // Brownish player
                custom_size: Some(PLAYER_SIZE),
                ..default()
            },
            transform: Transform::from_xyz(-200.0, GROUND_Y, 0.0),
            ..default()
        },
        Player {
            velocity: 0.0,
            on_ground: true,
        },
    ));
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>, // ✅ Fixed for Bevy 0.15.3
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    if let Ok((mut transform, mut player)) = query.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) && player.on_ground {
            player.velocity = JUMP_VELOCITY;
            player.on_ground = false;
        }

        let delta_time = time.delta().as_secs_f32();

        player.velocity += GRAVITY * delta_time;
        transform.translation.y += player.velocity * delta_time;

        if transform.translation.y <= GROUND_Y {
            transform.translation.y = GROUND_Y;
            player.velocity = 0.0;
            player.on_ground = true;
        }
    }
}

fn spawn_obstacles(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        let mut rng = rand::thread_rng();
        let spawn_x = 400.0;
        let spawn_time = rng.gen_range(MIN_SPAWN_TIME..MAX_SPAWN_TIME);
        timer.0.set_duration(std::time::Duration::from_secs_f32(spawn_time));

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb_u8(255, 0, 0), // Red obstacle
                    custom_size: Some(OBSTACLE_SIZE),
                    ..default()
                },
                transform: Transform::from_xyz(spawn_x, GROUND_Y, 0.0),
                ..default()
            },
            Obstacle,
        ));
    }
}

fn move_obstacles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform), With<Obstacle>>,
) {
    let delta_time = time.delta().as_secs_f32();

    for (entity, mut transform) in query.iter_mut() {
        transform.translation.x += OBSTACLE_SPEED * delta_time;

        if transform.translation.x < -400.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn check_collisions(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    obstacle_query: Query<(Entity, &Transform), With<Obstacle>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (entity, obstacle_transform) in obstacle_query.iter() {
            let collision = player_transform.translation.x < obstacle_transform.translation.x + OBSTACLE_SIZE.x &&
                player_transform.translation.x + PLAYER_SIZE.x > obstacle_transform.translation.x &&
                player_transform.translation.y < obstacle_transform.translation.y + OBSTACLE_SIZE.y &&
                player_transform.translation.y + PLAYER_SIZE.y > obstacle_transform.translation.y;

            if collision {
                println!("💥 Game Over! Restarting...");

                for (obstacle_entity, _) in obstacle_query.iter() {
                    commands.entity(obstacle_entity).despawn();
                }
                break;
            }
        }
    }
}