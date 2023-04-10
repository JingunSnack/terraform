use bevy::prelude::*;

use crate::block::Block;
use crate::player::Player;
use crate::AppState;

#[derive(Component)]
pub struct Nova;

pub struct NovaPlugin;

impl Plugin for NovaPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_nova.in_schedule(OnEnter(AppState::InGame)))
            .add_systems((move_nova, update_nova, release_nova).in_set(OnUpdate(AppState::InGame)))
            .add_system(despawn_nova.in_schedule(OnExit(AppState::InGame)));
    }
}

fn spawn_nova(
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
    if let Ok(mut transform) = query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::Space) {
            transform.scale += Vec3::new(0.25, 0.0, 0.25);
        }
    }
}

fn move_nova(
    player_query: Query<&Transform, With<Player>>,
    mut nova_query: Query<&mut Transform, (With<Nova>, Without<Player>)>,
) {
    if let Ok(mut nova_transform) = nova_query.get_single_mut() {
        if let Ok(player_transform) = player_query.get_single() {
            nova_transform.translation.x = player_transform.translation.x;
            nova_transform.translation.z = player_transform.translation.z;
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
    if let Ok(mut nova_transform) = nova_query.get_single_mut() {
        if keyboard_input.just_released(KeyCode::Space) {
            for (mut block_transform, handle) in &mut block_query {
                if block_transform
                    .translation
                    .distance(nova_transform.translation)
                    < nova_transform.scale.x * 0.4
                {
                    if let Some(block_material) = materials.get_mut(&handle) {
                        let mut green_color = block_material.base_color.g() * 1.2;
                        if green_color > 1.5 {
                            green_color = 1.5;
                            block_transform.scale = Vec3::new(1.0, 2.0, 1.0);
                        }
                        block_material.base_color.set_g(green_color);
                    }
                }
            }
            nova_transform.scale = Vec3::new(1.0, 1.0, 1.0);
        }
    }
}

fn despawn_nova(mut commands: Commands, nova_query: Query<Entity, With<Nova>>) {
    for nova_entity in &nova_query {
        commands.entity(nova_entity).despawn_recursive();
    }
}
