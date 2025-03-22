use bevy::prelude::*;
use crate::constants::GROUND_Y;
use crate::components::{Player, MainCamera};
use crate::resources::CurrentSkin;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window>,
    skin: Res<CurrentSkin>,
) {
    // ✅ Spawn the camera and tag it
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera,
    ));

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
    let skin_color = skin.color;

    commands.spawn((
        SpriteBundle {
            texture: player_texture,
            sprite: Sprite {
                color: skin_color,
                ..default()
            },
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
