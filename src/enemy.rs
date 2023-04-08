use bevy::prelude::*;

use crate::block::Block;
use crate::player::Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer {
            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
        }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_system(move_enemy)
            .add_system(destory_blocks)
            .add_system(tick_enemy_spawn_timer)
            .add_system(spawn_enemies_over_time)
            .add_system(kill_player);
    }
}

fn move_enemy(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut enemy_transform in &mut enemy_query {
            let direction = player_transform.translation - enemy_transform.translation;
            enemy_transform.translation += direction.normalize() * 1.0 * time.delta_seconds();
        }
    }
}

fn destory_blocks(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    block_query: Query<(Entity, &Transform), (With<Block>, Without<Enemy>)>,
) {
    for (enemy_entity, enemy_transform) in &enemy_query {
        for (block_entity, block_transform) in &block_query {
            if block_transform.scale.y > 1.0
                && enemy_transform
                    .translation
                    .distance(block_transform.translation)
                    <= 1.2
            {
                match commands.get_entity(enemy_entity) {
                    Some(c) => c.despawn_recursive(),
                    None => (),
                }
                match commands.get_entity(block_entity) {
                    Some(c) => c.despawn_recursive(),
                    None => (),
                }
            }
        }
    }
}

fn kill_player(
    mut commands: Commands,
    enemy_query: Query<&Transform, With<Enemy>>,
    player_query: Query<(Entity, &Transform), (With<Player>, Without<Enemy>)>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single() {
        for enemy_transform in &enemy_query {
            if enemy_transform
                .translation
                .distance(player_transform.translation)
                < 1.0
            {
                match commands.get_entity(player_entity) {
                    Some(c) => c.despawn_recursive(),
                    None => (),
                }
            }
        }
    }
}

fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

fn spawn_enemies_over_time(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.timer.finished() {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius: 0.5,
                    sectors: 20,
                    stacks: 20,
                })),
                material: materials.add(Color::rgb(1.0, 0.05, 0.05).into()),
                transform: Transform::from_xyz(7.0, 1.5, 7.0),
                ..default()
            },
            Enemy,
        ));

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius: 0.5,
                    sectors: 20,
                    stacks: 20,
                })),
                material: materials.add(Color::rgb(1.0, 0.05, 0.05).into()),
                transform: Transform::from_xyz(-7.0, 1.5, -7.0),
                ..default()
            },
            Enemy,
        ));
    }
}
