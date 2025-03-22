use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub velocity: f32,
    pub on_ground: bool,
    pub flipped: bool,
}

#[derive(Component)]
pub struct Obstacle;
