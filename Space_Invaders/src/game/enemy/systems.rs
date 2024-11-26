use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::PrimaryWindow;
use crate::game::enemy::components::Enemy;

pub fn spawn_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let sprite_size = 32.0;
    let rows = 5;
    let columns = 11;

    let window = window_query.get_single().unwrap();

    for row in 0..rows{
        for column in 0..columns{
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(sprite_size, sprite_size)),
                        anchor: Anchor::BottomLeft,
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        (window.width() * 0.5 - (columns as f32 * sprite_size * 0.5)) + sprite_size * column as f32,
                        100.0 + sprite_size * row as f32,
                        0.0
                    ),
                    texture: asset_server.load("invader.png"),
                    ..default()
                },
                Enemy {},
            ));
        }
    }
}

pub fn hit_detection(
    mut commands: Commands, // Allows despawning entities
    mut param_set: ParamSet<(
        Query<&Transform, With<crate::game::player::components::Shot>>, // Immutable query for shots
        Query<(Entity, &Transform), With<Enemy>>, // Mutable access to bricks for despawning
    )>,
) {
    // Collect all shot positions into a vector
    let shot_positions: Vec<_> = {
        let shots = param_set.p0(); // Scoped mutable borrow for shots
        shots.iter().map(|transform| transform.translation).collect()
    };

    // Now access enemies safely and check for collisions
    let mut enemies = param_set.p1();
    for shot_position in shot_positions {
        for (enemy_entity, enemy_transform) in enemies.iter_mut() {
            let enemy_position = enemy_transform.translation;

            // Example collision detection: AABB
            let shot_size = Vec3::new(2.0, 10.0, 0.0); // Assuming 1x1 size for shot
            let enemy_size = Vec3::new(32.0, 32.0, 0.0); // Example size for brick

            if (shot_position.x < enemy_position.x + enemy_size.x &&
                shot_position.x + shot_size.x > enemy_position.x &&
                shot_position.y < enemy_position.y + enemy_size.y &&
                shot_position.y + shot_size.y > enemy_position.y) {

                // Collision detected, despawn the brick
                commands.entity(enemy_entity).despawn();
            }
        }
    }
}

