use bevy::prelude::*;
use crate::resources::{Score, GameState};

pub fn update_score(
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
