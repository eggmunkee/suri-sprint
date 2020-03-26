

use specs::{Component, DenseVecStorage, World, WorldExt};
use specs_derive::*;

#[derive(Debug,Component)]
#[storage(DenseVecStorage)]
pub struct MeowComponent {
    pub meow_time: f32,
    pub meow_radius: f32,
}

impl MeowComponent {
    pub fn new() -> MeowComponent {

        MeowComponent {
            meow_time: 0.0,
            meow_radius: 20.0,
        }

    }
}


// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    world.register::<MeowComponent>();
}