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
                    texture: asset_server.load("Pixel.png"),
                    ..default()
                },
                Brick {},
            ));
        }
    }
}
