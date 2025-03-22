use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub velocity: f32,
    pub on_ground: bool,
    pub flipped: bool,
}

#[derive(Component)]
pub struct Obstacle;

#[derive(Component)]
pub struct Coin;

#[derive(Component)]
pub struct MainCamera; // ✅ Used to track and despawn the camera on restart
