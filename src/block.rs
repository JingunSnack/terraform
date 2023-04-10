use bevy::prelude::*;

use crate::AppState;

#[derive(Component)]
pub struct Block;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_blocks.in_schedule(OnEnter(AppState::InGame)))
            .add_system(despawn_blocks.in_schedule(OnExit(AppState::InGame)));
    }
}

fn spawn_blocks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for x in -10..11 {
        for z in -10..11 {
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.99 })),
                    material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                    transform: Transform::from_xyz(x as f32, 0.5, z as f32),
                    ..default()
                },
                Block,
            ));
        }
    }
}

fn despawn_blocks(mut commands: Commands, block_query: Query<Entity, With<Block>>) {
    for block_entity in &block_query {
        commands.entity(block_entity).despawn_recursive();
    }
}
