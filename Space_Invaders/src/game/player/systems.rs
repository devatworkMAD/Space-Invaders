use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::{Player, Shot};

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // This is the player sprite size.

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    println!("spawning player");
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() * 0.5, window.height() * 0.05, 0.0),
            texture: asset_server.load("ship.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0; // 32.0
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;

        let mut translation = player_transform.translation;

        // Bound the player x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        player_transform.translation = translation;
    }
}

pub fn spawn_shot(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
){
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            let window = window_query.get_single().unwrap();
            let mut translation = player_transform.translation;

            println!("spawning shot");
            let window = window_query.get_single().unwrap();

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(translation.x, translation.y, translation.z),
                    texture: asset_server.load("player_shot.png"),
                    ..default()
                },
                Shot {},
            ));
        }
    }
}

pub fn progress_shot(
    mut shot_query: Query<&mut Transform, With<Shot>>,
    time: Res<Time>,
){
    let speed = 400.0;

    for mut shot_transform in shot_query.iter_mut() {
        shot_transform.translation.y += time.delta_seconds() * speed;
        println!("Shot position: {:?}", shot_transform.translation);
    }
}

pub fn despawn_shot(
    mut commands: Commands,
    shot_query: Query<(Entity, &Transform), With<Shot>>,
) {
    let y_threshold = 480.0;

    for (entity, transform) in shot_query.iter() {
        if transform.translation.y > y_threshold {
            commands.entity(entity).despawn();
            println!("Despawning shot at position: {:?}", transform.translation);
        }
    }
}
