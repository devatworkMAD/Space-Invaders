use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, helloWorld)
        .run();
}


fn helloWorld(){
    println!("Hello, world!");
}