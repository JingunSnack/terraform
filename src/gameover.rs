use bevy::prelude::*;

use crate::{score::Score, AppState};

#[derive(Component)]
pub struct GameOverMenu;

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_game_over_menu.in_schedule(OnEnter(AppState::GameOver)))
            .add_system(despawn_game_over_menu.in_schedule(OnExit(AppState::GameOver)));
    }
}

fn spawn_game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>, score: Res<Score>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    margin: UiRect {
                        top: Val::Percent(10.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
            GameOverMenu,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    format!("High Score: {}\nPress Enter to Play Again", score.value),
                    TextStyle {
                        font: asset_server
                            .load("fonts/Caskaydia Cove Nerd Font Complete Mono Bold.otf"),
                        font_size: 64.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::Center)
                .with_style(Style { ..default() }),
            );
        });
}

fn despawn_game_over_menu(
    mut commands: Commands,
    game_over_menu_query: Query<Entity, With<GameOverMenu>>,
) {
    for game_over_menu_entity in &game_over_menu_query {
        commands.entity(game_over_menu_entity).despawn_recursive();
    }
}
