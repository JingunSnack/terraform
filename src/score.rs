use bevy::prelude::*;

use crate::block::Block;

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Score {
        Score { value: 0 }
    }
}

#[derive(Resource)]
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
            .add_system(print_score);
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

fn print_score(score: Res<Score>) {
    if score.is_changed() {
        println!("{}", score.value);
    }
}
