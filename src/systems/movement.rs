use bevy::prelude::*;
use crate::components::Player;
use crate::constants::{GRAVITY, JUMP_VELOCITY, GROUND_Y, CEILING_Y};
use crate::resources::Score;

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Player)>,
    score: Res<Score>,
) {
    if let Ok((mut transform, mut player)) = query.get_single_mut() {
        let delta_time = time.delta_seconds();

        let should_jump = keyboard_input.just_pressed(KeyCode::Space);

        if score.0 < 100.0 {
            // Standard gravity jump
            if should_jump && player.on_ground {
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
            // Ceiling flipping jump
            player.velocity = 0.0;
            if should_jump {
                player.flipped = !player.flipped;
            }

            let target_y = if player.flipped { CEILING_Y } else { GROUND_Y };
            let move_speed = 500.0;

            if (transform.translation.y - target_y).abs() < move_speed * delta_time {
                transform.translation.y = target_y;
            } else {
                let direction = if transform.translation.y < target_y { 1.0 } else { -1.0 };
                transform.translation.y += move_speed * delta_time * direction;
            }

            transform.rotation = if player.flipped {
                Quat::from_rotation_z(std::f32::consts::PI)
            } else {
                Quat::IDENTITY
            };
        }
    }
}
