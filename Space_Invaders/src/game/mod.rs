mod player;
mod systems;
mod defense;
mod enemy;

use player::PlayerPlugin;

use bevy::prelude::*;
use crate::game::defense::DefensePlugin;
use crate::game::enemy::EnemyPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((PlayerPlugin, DefensePlugin, EnemyPlugin));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}