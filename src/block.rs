use bevy::prelude::*;

#[derive(Component)]
pub struct Block;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_blocks).add_system(count_blocks);
    }
}

fn init_blocks(
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

fn count_blocks(block_query: Query<&Transform, With<Block>>) {
    let mut total = 0;
    let mut count = 0;
    for transform in &block_query {
        total += 1;
        if transform.scale.y > 1.0 {
            count += 1;
        }
    }
    // TODO: GameOver (or Win) condition
    if count == total {
        println!("{} all are collected", count);
    }
}
