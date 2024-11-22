mod components;
mod systems;

use systems::*;

use bevy::prelude::*;



#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct DefenseSystemSet;
pub struct DefensePlugin;
impl Plugin for DefensePlugin {
    fn build(&self, app: &mut App) {
        println!("build DefensePlugin");
        app
            .add_systems(Startup, build_defense)
            .add_systems(Update, hit_detection);
    }
}