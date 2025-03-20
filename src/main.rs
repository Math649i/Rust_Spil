use bevy::prelude::*;
use bevy::asset::LoadState;
use rand::Rng;
use std::time::Duration;

const PLAYER_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const OBSTACLE_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const GROUND_Y: f32 = -240.0;
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

#[derive(Resource)]
struct Score(f32);

#[derive(Resource, PartialEq)]
enum GameState {
    Running,
    GameOver,
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(GameState::Running)
        .insert_resource(SpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .insert_resource(Score(0.0))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            update_score,
            restart_game,
            game_logic.run_if(|state: Res<GameState>| *state == GameState::Running),
            spawn_obstacles.run_if(|state: Res<GameState>| *state == GameState::Running),
            move_obstacles.run_if(|state: Res<GameState>| *state == GameState::Running),
            check_collisions,
        ))
        .run();
}



fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window>,
    camera_query: Query<Entity, With<Camera2d>>,
) {
    // Only spawn a camera if one doesn't already exist
    if camera_query.is_empty() {
        commands.spawn(Camera2dBundle::default());
    }

    let window = windows.single();
    let window_width = window.width();
    let window_height = window.height();

    let background_texture = asset_server.load("background.png");
    commands.spawn(SpriteBundle {
        texture: background_texture,
        transform: Transform::from_xyz(0.0, 0.0, -1.0),
        sprite: Sprite {
            custom_size: Some(Vec2::new(window_width, window_height)),
            ..default()
        },
        ..default()
    });

    let player_texture = asset_server.load("player.png");
    commands.spawn((
        SpriteBundle {
            texture: player_texture.clone(),
            transform: Transform::from_xyz(-200.0, GROUND_Y, 0.0),
            ..default()
        },
        Player {
            velocity: 0.0,
            on_ground: true,
        },
    ));

    // Score Text
    commands.spawn((
        TextBundle::from_section(
            "Score: 0",
            TextStyle {
                font: asset_server.load("FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::WHITE,
            },
        )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Percent(50.0),
                ..default()
            }),
    ));
}


fn update_score(time: Res<Time>, mut score: ResMut<Score>, mut text_query: Query<&mut Text>, game_state: Res<GameState>) {
    if *game_state == GameState::Running { // ✅ Corrected check
        score.0 += time.delta_seconds() * 10.0;
        if let Ok(mut text) = text_query.get_single_mut() {
            text.sections[0].value = format!("Score: {:.0}", score.0);
        }
    }
}


fn log_background_loaded(asset_server: Res<AssetServer>, background: Query<&Handle<Image>>) {
    if let Ok(handle) = background.get_single() {
        if asset_server.get_load_state(handle) == Some(LoadState::Loaded) {
            println!("✅ Background loaded successfully!");
        } else {
            println!("⏳ Background is still loading...");
        }
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    if let Ok((mut transform, mut player)) = query.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) && player.on_ground {
            player.velocity = JUMP_VELOCITY;
            player.on_ground = false;
        }

        let delta_time = time.delta_seconds();
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
    asset_server: Res<AssetServer>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        let mut rng = rand::rng();
        let spawn_x = 400.0;
        let spawn_time = rng.random_range(MIN_SPAWN_TIME..MAX_SPAWN_TIME);
        timer.0.set_duration(Duration::from_secs_f32(spawn_time));

        let obstacle_texture = asset_server.load("spike.png");
        commands.spawn((
            SpriteBundle {
                texture: obstacle_texture,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(30.0, 30.0)),
                    ..default()
                },
                transform: Transform::from_xyz(spawn_x, GROUND_Y - 10.0, 0.0),
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
    let delta_time = time.delta_seconds();

    for (entity, mut transform) in query.iter_mut() {
        transform.translation.x += OBSTACLE_SPEED * delta_time;
        if transform.translation.x < -400.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn check_collisions(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut score: ResMut<Score>,
    player_query: Query<&Transform, With<Player>>,
    obstacle_query: Query<(Entity, &Transform), With<Obstacle>>,
    asset_server: Res<AssetServer>,
) {
    if *game_state == GameState::GameOver {
        return; // Prevent collision detection when the game is over
    }

    if let Ok(player_transform) = player_query.get_single() {
        for (entity, obstacle_transform) in obstacle_query.iter() {
            let collision = player_transform.translation.x < obstacle_transform.translation.x + OBSTACLE_SIZE.x
                && player_transform.translation.x + PLAYER_SIZE.x > obstacle_transform.translation.x
                && player_transform.translation.y < obstacle_transform.translation.y + OBSTACLE_SIZE.y
                && player_transform.translation.y + PLAYER_SIZE.y > obstacle_transform.translation.y;

            if collision {
                println!("💥 Game Over! Final Score: {}", score.0);

                *game_state = GameState::GameOver; // Stop all movement

                // Remove all obstacles and player
                for (obstacle_entity, _) in obstacle_query.iter() {
                    commands.entity(obstacle_entity).despawn();
                }

                // Display "Game Over" text
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




fn restart_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut score: ResMut<Score>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window>,
    text_entities: Query<Entity, With<Text>>,
    player_query: Query<Entity, With<Player>>,
    obstacle_query: Query<Entity, With<Obstacle>>,
    camera_query: Query<Entity, With<Camera2d>>,  // ✅ Add this
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        println!("🔄 Restarting Game...");

        *game_state = GameState::Running;  // ✅ Set game state back to "Running"
        score.0 = 0.0;

        // Remove all existing entities (player, obstacles, text)
        for entity in text_entities.iter() {
            commands.entity(entity).despawn();
        }
        for entity in player_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in obstacle_query.iter() {
            commands.entity(entity).despawn();
        }

        // Re-run setup to respawn everything
        setup(commands, asset_server, windows, camera_query); // ✅ Now passes all required parameters
    }
}




fn game_logic(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Player)>,
    game_state: Res<GameState>,
) {
    if *game_state == GameState::GameOver {
        return; // Prevent updates when game is over
    }

    if let Ok((mut transform, mut player)) = query.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) && player.on_ground {
            player.velocity = JUMP_VELOCITY;
            player.on_ground = false;
        }

        let delta_time = time.delta_seconds();
        player.velocity += GRAVITY * delta_time;
        transform.translation.y += player.velocity * delta_time;

        if transform.translation.y <= GROUND_Y {
            transform.translation.y = GROUND_Y;
            player.velocity = 0.0;
            player.on_ground = true;
        }
    }
}