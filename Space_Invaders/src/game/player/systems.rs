use std::time::Instant;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::game::enemy::components::{Enemy, Spit};
use crate::game::player::components::{Player, Shot};

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
            texture: asset_server.load("sprites/ship.png"),
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

#[derive(Resource)]
pub struct LastShotTime {
    last_shot: Instant,
}
pub fn setup_LastShotTime(mut commands: Commands) {
    commands.insert_resource(LastShotTime {
        last_shot: Instant::now() - std::time::Duration::from_secs(1), // Ensure shots can fire immediately
    });
}

pub fn spawn_shot(
    mut commands: Commands,
    mut last_shot_time: ResMut<LastShotTime>, // Track the time of the last shot
    asset_server: Res<AssetServer>,
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>, // Use Input for key presses
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::ArrowUp) || keyboard_input.just_pressed(KeyCode::KeyW) {
            let now = Instant::now();
            if now.duration_since(last_shot_time.last_shot).as_secs_f32() >= 0.5 {
                last_shot_time.last_shot = now;

                let translation = player_transform.translation;
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            ..default()
                        },
                        transform: Transform::from_xyz(translation.x, translation.y, translation.z),
                        texture: asset_server.load("sprites/player_shot.png"),
                        ..default()
                    },
                    Shot {},
                ));
            }
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
    }
}

pub fn despawn_shot(
    mut commands: Commands,
    shot_query: Query<(Entity, &Transform), With<Shot>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();
    let y_threshold = window.height();

    for (entity, transform) in shot_query.iter() {
        if transform.translation.y > y_threshold {
            commands.entity(entity).despawn();
            println!("Despawning shot at position: {:?}", transform.translation);
        }
    }
}

pub fn hit_detection(
    mut commands: Commands,
    mut param_set: ParamSet<(
        Query<(Entity, &Transform), With<Spit>>,
        Query<(Entity, &Transform), With<Player>>,
    )>,
) {
    let spit_data: Vec<_> = {
        let spits = param_set.p0();
        spits.iter().map(|(entity, transform)| (entity, transform.translation)).collect()
    };

    let mut player = param_set.p1();
    for (spit_entity, spit_position) in spit_data {
        for (player_entity, player_transform) in player.iter_mut() {
            let player_position = player_transform.translation;

            let shot_size = Vec3::new(2.0, 10.0, 0.0);
            let player_size = Vec3::new(32.0, 32.0, 0.0);

            if (spit_position.x < player_position.x + player_size.x &&
                spit_position.x + shot_size.x > player_position.x &&
                spit_position.y < player_position.y + player_size.y &&
                spit_position.y + shot_size.y > player_position.y) {

                commands.entity(player_entity).despawn();
                commands.entity(spit_entity).despawn();

                break;
            }
        }
    }
}