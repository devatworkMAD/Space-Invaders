mod game;
mod systems;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};
use crate::game::GamePlugin;
use crate::systems::spawn_camera;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Space Invaders".into(),
                    name: Some("bevy.app".into()),
                    resolution: (480.0, 480.0).into(),
                    present_mode: PresentMode::AutoVsync,
                    // Tells Wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: true,
                    // Tells Wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    visible: true,
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_plugins(GamePlugin)
        .add_systems(Startup, spawn_camera)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}