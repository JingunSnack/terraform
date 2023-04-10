use bevy::prelude::*;

use crate::AppState;

#[derive(Component)]
pub struct MainMenu;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)));
    }
}

fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            MainMenu,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Press Enter to Play",
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

fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    for main_menu_entity in &main_menu_query {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}
