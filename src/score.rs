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

#[derive(Component)]
pub struct ScoreUI;

#[derive(Component)]
pub struct ScoreUIText;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_system(spawn_score_ui.in_schedule(OnEnter(AppState::InGame)))
            .add_systems((update_score, update_score_ui).in_set(OnUpdate(AppState::InGame)))
            .add_system(despawn_score_ui.in_schedule(OnExit(AppState::InGame)));
    }
}

fn spawn_score_ui(mut commands: Commands, asset_server: Res<AssetServer>, score: Res<Score>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::End,
                    align_items: AlignItems::Start,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    margin: UiRect {
                        top: Val::Px(20.0),
                        right: Val::Px(20.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
            ScoreUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    format!("Score: {}", score.value),
                    TextStyle {
                        font: asset_server
                            .load("fonts/Caskaydia Cove Nerd Font Complete Mono Bold.otf"),
                        font_size: 32.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::Center)
                .with_style(Style { ..default() }),
                ScoreUIText,
            ));
        });
}

fn despawn_score_ui(mut commands: Commands, score_ui_query: Query<Entity, With<ScoreUI>>) {
    for score_ui_entity in &score_ui_query {
        commands.entity(score_ui_entity).despawn_recursive();
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

fn update_score_ui(
    score: Res<Score>,
    mut score_ui_text_query: Query<&mut Text, With<ScoreUIText>>,
) {
    if score.is_changed() {
        if let Ok(mut score_ui_text) = score_ui_text_query.get_single_mut() {
            score_ui_text.sections[0].value = format!("Score: {}", score.value);
        }
    }
}
