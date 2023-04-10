use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use block::BlockPlugin;
use enemy::EnemyPlugin;
use nova::NovaPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;

mod block;
mod enemy;
mod nova;
mod player;
mod score;

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_state::<AppState>()
        .add_startup_system(setup)
        .add_plugin(NovaPlugin)
        .add_plugin(BlockPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(ScorePlugin)
        .add_system(in_game)
        .add_system(main_menu)
        .add_system(exit_game)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            scale: 10.0,
            scaling_mode: bevy::render::camera::ScalingMode::FixedVertical(2.0),
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 10.0, 0.0),
        point_light: PointLight {
            intensity: 10000.0,
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
}

fn in_game(keyboard_input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if keyboard_input.just_pressed(KeyCode::G) {
        next_state.set(AppState::InGame);
    }
}

fn main_menu(keyboard_input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if keyboard_input.just_pressed(KeyCode::M) {
        next_state.set(AppState::MainMenu);
    }
}

fn exit_game(keyboard_input: Res<Input<KeyCode>>, mut app_exit_event_writer: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}
