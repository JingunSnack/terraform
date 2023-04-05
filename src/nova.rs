use bevy::prelude::*;

use crate::block::Block;

#[derive(Component)]
pub struct Nova;

pub struct NovaPlugin;

impl Plugin for NovaPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_nova)
            .add_system(move_nova)
            .add_system(limit_nova_movement)
            .add_system(update_nova)
            .add_system(release_nova);
    }
}

fn init_nova(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cylinder {
                radius: 0.4,
                height: 0.01,
                resolution: 100,
                ..default()
            })),
            material: materials.add(Color::rgba(0.0, 1.0, 0.0, 0.5).into()),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
        Nova,
    ));
}

fn update_nova(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Nova>>) {
    if let Ok(mut nova) = query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::Space) {
            nova.scale += Vec3::new(0.25, 0.0, 0.25);
        }
    }
}

fn move_nova(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Nova>>,
    time: Res<Time>,
) {
    if let Ok(mut nova) = query.get_single_mut() {
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
            nova.translation += direction.normalize() * 10.0 * time.delta_seconds();
        }
    }
}

fn limit_nova_movement(mut query: Query<&mut Transform, With<Nova>>) {
    if let Ok(mut nova) = query.get_single_mut() {
        if nova.is_changed() {
            if nova.translation.x < -10.0 {
                nova.translation.x = -10.0;
            }
            if nova.translation.x > 10.0 {
                nova.translation.x = 10.0;
            }
            if nova.translation.z < -10.0 {
                nova.translation.z = -10.0;
            }
            if nova.translation.z > 10.0 {
                nova.translation.z = 10.0;
            }
        }
    }
}

fn release_nova(
    keyboard_input: Res<Input<KeyCode>>,
    mut nova_query: Query<&mut Transform, With<Nova>>,
    mut block_query: Query<
        (&mut Transform, &mut Handle<StandardMaterial>),
        (With<Block>, Without<Nova>),
    >,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Ok(mut nova) = nova_query.get_single_mut() {
        if keyboard_input.just_released(KeyCode::Space) {
            for (mut transform, handle) in &mut block_query {
                if transform.translation.distance(nova.translation) < nova.scale.x * 0.4 {
                    if let Some(material) = materials.get_mut(&handle) {
                        let mut green_color = material.base_color.g() * 1.2;
                        if green_color > 1.5 {
                            green_color = 1.5;
                            transform.scale = Vec3::new(1.0, 2.0, 1.0);
                        }
                        material.base_color.set_g(green_color);
                    }
                }
            }
            nova.scale = Vec3::new(1.0, 1.0, 1.0);
        }
    }
}
