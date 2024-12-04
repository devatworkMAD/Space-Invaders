pub(crate) mod components;
mod systems;

use bevy::prelude::*;
use crate::game::enemy::systems::*;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        println!("build PlayerPlugin");
        app
            .add_systems(Startup, (spawn_enemy, setup_Direction))
            .add_systems(Update, (hit_detection, move_enemy_x, switch_direction, move_enemy_y));
    }
}
