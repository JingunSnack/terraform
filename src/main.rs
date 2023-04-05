use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use block::BlockPlugin;
use nova::NovaPlugin;

mod block;
mod nova;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_plugin(NovaPlugin)
        .add_plugin(BlockPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.20 })),
        material: materials.add(Color::rgb(1.0, 0.05, 0.05).into()),
        transform: Transform::from_xyz(1.0, 4.0, 1.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.20 })),
        material: materials.add(Color::rgb(1.0, 0.05, 0.05).into()),
        transform: Transform::from_xyz(5.0, 4.0, 1.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.20 })),
        material: materials.add(Color::rgb(1.0, 0.05, 0.05).into()),
        transform: Transform::from_xyz(1.0, 4.0, -4.0),
        ..default()
    });
}
