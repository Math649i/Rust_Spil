use bevy::math::Vec2;

pub const PLAYER_SIZE: Vec2 = Vec2::new(30.0, 30.0);
pub const OBSTACLE_SIZE: Vec2 = Vec2::new(30.0, 30.0);
pub const GROUND_Y: f32 = -240.0;
pub const CEILING_Y: f32 = 240.0;
pub const GRAVITY: f32 = -600.0;
pub const JUMP_VELOCITY: f32 = 300.0;
pub const OBSTACLE_SPEED: f32 = -200.0;
pub const MIN_SPAWN_TIME: f32 = 1.0;
pub const MAX_SPAWN_TIME: f32 = 3.0;