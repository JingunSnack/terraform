use bevy::prelude::*;

use crate::block::Block;
use crate::GameOver;

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
            .add_system(update_score)
            .add_system(update_high_scores)
            .add_system(print_score)
            .add_system(print_high_scores);
    }
}

fn update_score(block_query: Query<&Transform, With<Block>>, mut score: ResMut<Score>) {
    let mut count = 0;
    for transform in &block_query {
        if transform.scale.y > 1.0 {
            count += 1;
        }
    }
    if score.value != count {
        score.value = count;
    }
}

fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    score: Res<Score>,
    mut high_scores: ResMut<HighScores>,
) {
    for _ in game_over_event_reader.iter() {
        println!("game over! score: {}", score.value);
        high_scores.scores.push(("Player".to_string(), score.value));
    }
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
