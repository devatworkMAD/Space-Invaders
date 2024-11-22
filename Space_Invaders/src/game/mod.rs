mod player;
mod systems;
mod defense;

use player::PlayerPlugin;

use bevy::prelude::*;
use crate::game::defense::DefensePlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((PlayerPlugin, DefensePlugin));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}