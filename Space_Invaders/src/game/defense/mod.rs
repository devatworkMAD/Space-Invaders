mod components;
mod systems;

use systems::*;

use bevy::prelude::*;
use crate::AppState;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct DefenseSystemSet;
pub struct DefensePlugin;
impl Plugin for DefensePlugin {
    fn build(&self, app: &mut App) {
        println!("build DefensePlugin");
        app
            .add_systems(Startup, build_defense)
            .add_systems(Update, (
                shot_hit_detection,
                spit_hit_detection
            ).run_if(in_state(AppState::Game)));
    }
}