pub(crate) mod components;
mod systems;

use bevy::prelude::*;
use crate::game::enemy::systems::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        println!("build PlayerPlugin");
        app
            .add_systems(Startup, (
                spawn_enemy))
            .add_systems(Update, hit_detection);
    }
}
