use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::game::enemy::components::Enemy;

pub fn spawn_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    println!("spawning enemy");
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() * 0.5, window.height() * 0.5, 0.0),
            texture: asset_server.load("invader.png"),
            ..default()
        },
        Enemy {},
    ));
}