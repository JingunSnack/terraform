use bevy::prelude::*;

#[derive(Component)]
pub struct Block;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_blocks);
    }
}

fn init_blocks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for x in -10..11 {
        for z in -10..11 {
            if (x as i32).abs() + (z as i32).abs() <= 5 {
                continue;
            }
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
