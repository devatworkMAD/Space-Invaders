use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::game::defense::components::Brick;

pub fn build_defense(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    // Call build_wall for each wall with different x and y offsets
    build_wall(&mut commands, &window_query, &asset_server, 50.0, 1000.0);  // Wall 1
    build_wall(&mut commands, &window_query, &asset_server, 150.0, 1000.0); // Wall 2
    build_wall(&mut commands, &window_query, &asset_server, 250.0, 1000.0);   // Wall 3
    build_wall(&mut commands, &window_query, &asset_server, 350.0, 1000.0);  // Wall 4
}

fn build_wall(
    commands: &mut Commands,
    window_query: &Query<&Window, With<PrimaryWindow>>,
    asset_server: &Res<AssetServer>,
    x: f32,
    y: f32,
) {
    let window = window_query.get_single().unwrap();
    for column in 0..50 {
        for row in 0..25 {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(
                        column as f32 + x,
                        row as f32 + y * 0.05,
                        0.0,
                    ),
                    texture: asset_server.load("sprites/Pixel.png"),
                    ..default()
                },
                Brick {},
            ));
        }
    }
}


use bevy::ecs::system::ParamSet;
use crate::game::enemy::components::Spit;
use crate::game::player::components::Shot;

pub fn shot_hit_detection(
    mut commands: Commands, // Allows despawning entities
    mut param_set: ParamSet<(
        Query<(Entity, &Transform), With<Shot>>, // Immutable query for shots
        Query<(Entity, &Transform), With<Brick>>, // Mutable access to bricks for despawning
    )>,
) {
    // Collect all shot positions into a vector
    let shot_data: Vec<_> = {
        let shots = param_set.p0();
        shots.iter().map(|(entity, transform)| (entity, transform.translation)).collect()
    };
    // Now access bricks safely and check for collisions
    let mut bricks = param_set.p1(); // Scoped mutable borrow for bricks
    for (shot_entity, shot_position) in shot_data {
        for (brick_entity, brick_transform) in bricks.iter_mut() {
            let brick_position = brick_transform.translation;

            // Example collision detection: AABB
            let shot_size = Vec3::new(2.0, 10.0, 0.0); // Assuming 1x1 size for shot
            let brick_size = Vec3::new(1.0, 1.0, 0.0); // Example size for brick

            if (shot_position.x < brick_position.x + brick_size.x &&
                shot_position.x + shot_size.x > brick_position.x &&
                shot_position.y < brick_position.y + brick_size.y &&
                shot_position.y + shot_size.y > brick_position.y) {

                // Collision detected, despawn the brick
                commands.entity(brick_entity).despawn();
                commands.entity(shot_entity).despawn();
            }
        }
    }
}

pub fn spit_hit_detection(
    mut commands: Commands, // Allows despawning entities
    mut param_set: ParamSet<(
        Query<(Entity, &Transform), With<Spit>>, // Immutable query for shots
        Query<(Entity, &Transform), With<Brick>>, // Mutable access to bricks for despawning
    )>,
) {
    // Collect all spit positions into a vector
    let spit_data: Vec<_> = {
        let spits = param_set.p0();
        spits.iter().map(|(entity, transform)| (entity, transform.translation)).collect()
    };
    // Now access bricks safely and check for collisions
    let mut bricks = param_set.p1(); // Scoped mutable borrow for bricks
    for (spit_entity, spit_position) in spit_data {
        for (brick_entity, brick_transform) in bricks.iter_mut() {
            let brick_position = brick_transform.translation;

            // Example collision detection: AABB
            let spit_size = Vec3::new(2.0, 10.0, 0.0); // Assuming 1x1 size for shot
            let brick_size = Vec3::new(1.0, 1.0, 0.0); // Example size for brick

            if (spit_position.x < brick_position.x + brick_size.x &&
                spit_position.x + spit_size.x > brick_position.x &&
                spit_position.y < brick_position.y + brick_size.y &&
                spit_position.y + spit_size.y > brick_position.y) {

                // Collision detected, despawn the brick
                commands.entity(brick_entity).despawn();
                commands.entity(spit_entity).despawn();
            }
        }
    }
}