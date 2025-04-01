use bevy::prelude::*;
use bevy::ecs::schedule::State;

use crate::resources::{Score, GameState};

pub fn update_score(
    time: Res<Time>,
    mut score: ResMut<Score>,
    mut text_query: Query<&mut Text>,
    game_state: Res<State<GameState>>,
) {
    if game_state.get() == &GameState::Running {
        score.0 += time.delta_seconds() * 10.0;
        score.1 = 1.0 + (score.0 / 500.0);

        if let Ok(mut text) = text_query.get_single_mut() {
            text.sections[0].value = format!("Score: {:.0}", score.0);
        }
    }
}
