use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(move_player)
            .add_system(confine_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.5,
                sectors: 20,
                stacks: 20,
            })),
            material: materials.add(Color::rgb(0.05, 1.0, 0.05).into()),
            transform: Transform::from_xyz(0.0, 1.5, 0.0),
            ..default()
        },
        Player,
    ));
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(-1.0, 0.0, -1.0);
        }
        if keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 1.0);
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(1.0, 0.0, 1.0);
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, -1.0);
        }

        if direction.length() > 0.0 {
            transform.translation += direction.normalize() * 10.0 * time.delta_seconds();
        }
    }
}

fn confine_player(mut query: Query<&mut Transform, With<Player>>) {
    if let Ok(mut transform) = query.get_single_mut() {
        if transform.is_changed() {
            if transform.translation.x < -10.0 {
                transform.translation.x = -10.0;
            }
            if transform.translation.x > 10.0 {
                transform.translation.x = 10.0;
            }
            if transform.translation.z < -10.0 {
                transform.translation.z = -10.0;
            }
            if transform.translation.z > 10.0 {
                transform.translation.z = 10.0;
            }
        }
    }
}
