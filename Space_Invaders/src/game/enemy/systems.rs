use std::time::Instant;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::PrimaryWindow;
use rand::Rng;
use crate::game::enemy::components::{Enemy, Spit};
use crate::game::player::components::Shot;

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
                        (window.height() - rows as f32 * sprite_size - 2.0 * sprite_size) + sprite_size * row as f32,
                        0.0
                    ),
                    texture: asset_server.load("sprites/invader.png"),
                    ..default()
                },
                Enemy {},
            ));
        }
    }
}


#[derive(Resource)]
pub struct Direction {
    left: bool,
    switch: bool,
}
pub fn setup_Direction(mut commands: Commands) {
    commands.insert_resource(Direction {
        left: true,
        switch: false
    });
}

pub fn move_enemy_x(
    mut enemy_query: Query<(Entity, &mut Transform), With<Enemy>>,
    time: Res<Time>,
    mut direction: ResMut<Direction>
){
    let speed_x = 25.0;

        for (enemy, mut transform) in enemy_query.iter_mut() {
            if direction.left {
                transform.translation.x -= time.delta_seconds() * speed_x;
            }else{
                transform.translation.x += time.delta_seconds() * speed_x;
            }
        }
}

pub fn move_enemy_y(
    mut enemy_query: Query<(&mut Transform, &Sprite), With<Enemy>>,
    mut direction: ResMut<Direction>
){
    if direction.switch{
        direction.switch = false;
        for (mut transform, sprite) in enemy_query.iter_mut() {
            transform.translation.y -= sprite.custom_size.map_or(0.0, |size| size.y) / 2.0;
        }
    }
}

pub fn switch_direction(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut enemy_query: Query<(&mut Transform, &Sprite), With<Enemy>>,
    mut direction: ResMut<Direction>
){
    if let Ok(window) = window_query.get_single() {
        for (mut transform, sprite) in enemy_query.iter_mut() {
            if transform.translation.x <= 0.0{
                direction.left = false;
                direction.switch = true;
            }

            if transform.translation.x >= window.width() - sprite.custom_size.map_or(0.0, |size| size.x){
                direction.left = true;
                direction.switch = true;
            }
        }
    }
}

pub fn hit_detection(
    mut commands: Commands,
    mut param_set: ParamSet<(
        Query<(Entity, &Transform), With<crate::game::player::components::Shot>>,
        Query<(Entity, &Transform), With<Enemy>>,
    )>,
) {
    let shot_data: Vec<_> = {
        let shots = param_set.p0();
        shots.iter().map(|(entity, transform)| (entity, transform.translation)).collect()
    };

    let mut enemies = param_set.p1();
    for (shot_entity, shot_position) in shot_data {
        for (enemy_entity, enemy_transform) in enemies.iter_mut() {
            let enemy_position = enemy_transform.translation;

            let shot_size = Vec3::new(2.0, 10.0, 0.0);
            let enemy_size = Vec3::new(32.0, 32.0, 0.0);

            if (shot_position.x < enemy_position.x + enemy_size.x &&
                shot_position.x + shot_size.x > enemy_position.x &&
                shot_position.y < enemy_position.y + enemy_size.y &&
                shot_position.y + shot_size.y > enemy_position.y) {

                commands.entity(enemy_entity).despawn();
                commands.entity(shot_entity).despawn();

                break;
            }
        }
    }
}

#[derive(Resource)]
pub struct LastSpitTime {
    last_shot: Instant,
}
pub fn setup_LastSpitTime(mut commands: Commands) {
    commands.insert_resource(LastSpitTime {
        last_shot: Instant::now() - std::time::Duration::from_secs(1), // Ensure shots can fire immediately
    });
}
pub fn spawn_spit(
    mut commands: Commands,
    time: Res<Time>,
    mut last_spit_time: ResMut<LastSpitTime>, // Track the time of the last shot
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    keyboard_input: Res<ButtonInput<KeyCode>>, // Use Input for key presses
) {
    let mut rng = rand::thread_rng();
    for (enemy_transform) in enemy_query.iter_mut() {
            let now = Instant::now();
            if now.duration_since(last_spit_time.last_shot).as_secs_f32() >= rng.gen_range(0.5..1.0) {
                last_spit_time.last_shot = now;

                let translation = enemy_transform.translation;
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            ..default()
                        },
                        transform: Transform::from_xyz(translation.x, translation.y, translation.z),
                        texture: asset_server.load("sprites/spit.png"),
                        ..default()
                    },
                    Spit {},
                ));
            }

    }
}

pub fn progress_spit(
    mut spit_query: Query<&mut Transform, With<Spit>>,
    time: Res<Time>,
){
    let speed = 400.0;

    for mut shot_transform in spit_query.iter_mut() {
        shot_transform.translation.y -= time.delta_seconds() * speed;
    }
}

pub fn despawn_shot(
    mut commands: Commands,
    spit_query: Query<(Entity, &Transform), With<Spit>>,
) {
    let y_threshold = 0.0;

    for (entity, transform) in spit_query.iter() {
        if transform.translation.y < y_threshold {
            commands.entity(entity).despawn();
            println!("Despawning spit at position: {:?}", transform.translation);
        }
    }
}

