use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;


const PLAYER_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const OBSTACLE_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const GROUND_Y: f32 = -240.0;
const CEILING_Y: f32 = 240.0;
const GRAVITY: f32 = -600.0;
const JUMP_VELOCITY: f32 = 300.0;
const OBSTACLE_SPEED: f32 = -200.0;
const MIN_SPAWN_TIME: f32 = 1.0;
const MAX_SPAWN_TIME: f32 = 3.0;

#[derive(Component)]
struct Player {
    velocity: f32,
    on_ground: bool,
    flipped: bool, // New flag for gravity switching
}

#[derive(Component)]
struct Obstacle;

#[derive(Resource)]
struct SpawnTimer(Timer);

#[derive(Resource, PartialEq)]
enum GameState {
    Running,
    GameOver,
}

#[derive(Resource)]
struct Score(f32, f32); // (current score, difficulty multiplier)


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(GameState::Running)
        .insert_resource(SpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .insert_resource(Score(0.0, 1.0)) // Initial score: 0, Initial difficulty: 1
        .add_systems(Startup, setup)
        .add_systems(Update, (
            update_score,
            restart_game,
            player_movement.run_if(|state: Res<GameState>| *state == GameState::Running),
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
) {
    commands.spawn(Camera2dBundle::default());

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
            texture: player_texture,
            transform: Transform::from_xyz(-200.0, GROUND_Y, 0.0),
            ..default()
        },
        Player {
            velocity: 0.0,
            on_ground: true,
            flipped: false,
        },
    ));

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



fn update_score(
    time: Res<Time>,
    mut score: ResMut<Score>,
    mut text_query: Query<&mut Text>,
    game_state: Res<GameState>,
) {
    if *game_state == GameState::Running {
        score.0 += time.delta_seconds() * 10.0;
        score.1 = 1.0 + (score.0 / 500.0);

        if let Ok(mut text) = text_query.get_single_mut() {
            text.sections[0].value = format!("Score: {:.0}", score.0);
        }
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Player)>,
    score: Res<Score>,
) {
    if let Ok((mut transform, mut player)) = query.get_single_mut() {
        let delta_time = time.delta_seconds();

        if score.0 < 100.0 {
            // ✅ Normal gravity before score 100
            if keyboard_input.just_pressed(KeyCode::Space) && player.on_ground {
                player.velocity = JUMP_VELOCITY;
                player.on_ground = false;
            }
            player.velocity += GRAVITY * delta_time;
            transform.translation.y += player.velocity * delta_time;

            if transform.translation.y <= GROUND_Y {
                transform.translation.y = GROUND_Y;
                player.velocity = 0.0;
                player.on_ground = true;
            }
        } else {
            // ✅ Fully remove gravity after score 100
            player.velocity = 0.0;

            // ✅ Flip Mechanic
            if keyboard_input.just_pressed(KeyCode::Space) {
                player.flipped = !player.flipped;
            }

            // ✅ Smooth transition to floor/ceiling
            let target_y = if player.flipped { CEILING_Y } else { GROUND_Y };
            let move_speed = 500.0;

            if (transform.translation.y - target_y).abs() < move_speed * delta_time {
                transform.translation.y = target_y;
            } else {
                let direction = if transform.translation.y < target_y { 1.0 } else { -1.0 };
                transform.translation.y += move_speed * delta_time * direction;
            }

            // ✅ Flip Player Rotation
            transform.rotation = if player.flipped {
                Quat::from_rotation_z(std::f32::consts::PI)
            } else {
                Quat::IDENTITY
            };
        }
    }
}


fn spawn_obstacles(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        let mut rng = rand::rngs::ThreadRng::default();
        let base_spawn_time = rng.gen_range(MIN_SPAWN_TIME..=MAX_SPAWN_TIME);
        let adjusted_spawn_time = (base_spawn_time / score.1).max(0.5);
        timer.0.set_duration(Duration::from_secs_f32(adjusted_spawn_time));

        let obstacle_texture = asset_server.load("spike.png");

        // Randomize X position for both spikes
        let floor_spike_x = rng.gen_range(350.0..450.0);
        let ceiling_spike_x = rng.gen_range(350.0..450.0);

        // Spawn Ground Spike
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

        // Spawn Ceiling Spike only after 100 score
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



fn move_obstacles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform), With<Obstacle>>,
) {
    let delta_time = time.delta_seconds();

    for (entity, mut transform) in query.iter_mut() {
        transform.translation.x += OBSTACLE_SPEED * delta_time;

        // ✅ Ensure obstacles despawn when off-screen
        if transform.translation.x < -400.0 {
            commands.entity(entity).despawn();
        }
    }
}


fn check_collisions(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut score: ResMut<Score>,
    player_query: Query<(&Transform, &Player), With<Player>>,
    obstacle_query: Query<(Entity, &Transform), With<Obstacle>>,
    asset_server: Res<AssetServer>,
) {
    if *game_state == GameState::GameOver {
        return;
    }

    if let Ok((player_transform, _)) = player_query.get_single() { // ✅ Fixed: "_" replaces unused "player"
        for (_, obstacle_transform) in obstacle_query.iter() { // ✅ Fixed: "_" replaces unused "entity"
            let collision = player_transform.translation.x < obstacle_transform.translation.x + OBSTACLE_SIZE.x
                && player_transform.translation.x + PLAYER_SIZE.x > obstacle_transform.translation.x
                && player_transform.translation.y < obstacle_transform.translation.y + OBSTACLE_SIZE.y
                && player_transform.translation.y + PLAYER_SIZE.y > obstacle_transform.translation.y;

            if collision {
                println!("💥 Game Over! Final Score: {:.0}", score.0);
                *game_state = GameState::GameOver;

                // Remove all obstacles
                for (obstacle_entity, _) in obstacle_query.iter() {
                    commands.entity(obstacle_entity).despawn();
                }

                // Display Game Over text
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
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        println!("🔄 Restarting Game...");

        *game_state = GameState::Running;
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

        // ✅ Fixed function call (removed extra parameter)
        setup(commands, asset_server, windows);
    }
}


fn game_logic(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Player)>,
    score: Res<Score>,
) {
    if let Ok((mut transform, mut player)) = query.get_single_mut() {
        let delta_time = time.delta_seconds();

        // **Before 100 Score: Normal Jumping & Gravity**
        if score.0 < 100.0 {
            if keyboard_input.just_pressed(KeyCode::Space) && player.on_ground {
                player.velocity = JUMP_VELOCITY;
                player.on_ground = false;
            }
            player.velocity += GRAVITY * delta_time;
            transform.translation.y += player.velocity * delta_time;

            // Ensure player lands on the ground
            if transform.translation.y <= GROUND_Y {
                transform.translation.y = GROUND_Y;
                player.velocity = 0.0;
                player.on_ground = true;
            }
        }
        // **After 100 Score: Completely Disable Gravity & Only Use Flip**
        else {
            if keyboard_input.just_pressed(KeyCode::Space) {
                player.flipped = !player.flipped;
            }

            // **Move the player directly to the ceiling or ground**
            let target_y = if player.flipped { 240.0 } else { GROUND_Y };
            let move_speed = 300.0; // Adjust movement speed

            // Move smoothly towards the target
            let direction = if transform.translation.y < target_y { 1.0 } else { -1.0 };
            transform.translation.y += move_speed * delta_time * direction;

            // Snap to the final position when close enough
            if (transform.translation.y - target_y).abs() < move_speed * delta_time {
                transform.translation.y = target_y;
                player.velocity = 0.0;
                player.on_ground = true;
            }

            // Rotate Player
            transform.rotation = if player.flipped {
                Quat::from_rotation_z(std::f32::consts::PI)
            } else {
                Quat::IDENTITY
            };
        }
    }
}
