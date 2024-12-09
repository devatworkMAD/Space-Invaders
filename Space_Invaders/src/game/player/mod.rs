pub(crate) mod components;
mod systems;

use systems::*;

use bevy::prelude::*;
use crate::AppState;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        println!("build PlayerPlugin");
        app
            .add_systems(Startup, (
                spawn_player,
                setup_LastShotTime))
            .add_systems(Update,
                         (
                             player_movement,
                             confine_player_movement
                         ).in_set(MovementSystemSet).run_if(in_state(AppState::Game)))
            .add_systems(Update,
                         (
                             spawn_shot,
                             progress_shot,
                             despawn_shot,
                             hit_detection
                             ).run_if(in_state(AppState::Game)));
    }
}
