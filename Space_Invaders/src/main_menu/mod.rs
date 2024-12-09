mod components;
mod styles;
mod systems;

use systems::interactions::*;
use systems::layout::*;

use bevy::prelude::*;

use crate::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_systems(Update, spawn_main_menu).init_schedule(OnEnter(AppState::MainMenu))
            // Systems
            .add_systems(Update, (
                interact_with_play_button,
                interact_with_quit_button
            ).run_if(in_state(AppState::MainMenu)))
            // OnExit State Systems
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}
