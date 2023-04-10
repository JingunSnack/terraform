use bevy::prelude::*;

use crate::block::Block;
use crate::AppState;

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Score {
        Score { value: 0 }
    }
}

#[derive(Resource, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}

impl Default for HighScores {
    fn default() -> HighScores {
        HighScores { scores: Vec::new() }
    }
}

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HighScores>()
            .init_resource::<Score>()
            .add_systems(
                (update_score, print_score, print_high_scores).in_set(OnUpdate(AppState::InGame)),
            )
            .add_system(update_high_scores.in_schedule(OnEnter(AppState::GameOver)));
    }
}

fn update_score(
    block_query: Query<&Transform, With<Block>>,
    mut score: ResMut<Score>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let mut total = 0;
    let mut count = 0;
    for transform in &block_query {
        if transform.scale.y > 1.0 {
            count += 1;
        }
        total += 1;
    }
    if score.value != count {
        score.value = count;
    }
    if total == count {
        next_state.set(AppState::GameOver);
    }
}

fn update_high_scores(score: Res<Score>, mut high_scores: ResMut<HighScores>) {
    println!("game over! score: {}", score.value);
    high_scores.scores.push(("Player".to_string(), score.value));
}

fn print_score(score: Res<Score>) {
    if score.is_changed() {
        println!("{}", score.value);
    }
}

fn print_high_scores(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("high scores: {:?}", high_scores)
    }
}
