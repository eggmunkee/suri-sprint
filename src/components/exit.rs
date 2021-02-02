
use ggez::nalgebra as na;
use specs::{Component, DenseVecStorage, World, WorldExt};
use specs_derive::*;
use wrapped2d::b2;


use crate::components::sprite::{SpriteComponent};
use crate::components::collision::{Collision};
use crate::core::physics;
use crate::core::physics::{PhysicsWorld};

#[derive(Debug,Component)]
#[storage(DenseVecStorage)]
pub struct ExitComponent {
    pub normal: na::Vector2::<f32>,
    pub name: String,
    pub destination: String,
}

impl ExitComponent {
    pub fn new(portal_name: String, destination_name: String) -> ExitComponent {

        let mut portal = ExitComponent {
            normal: na::Vector2::new(0.0, -1.0),
            name: portal_name,
            destination: destination_name,
        };

        portal
    }

}


// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    world.register::<ExitComponent>();
}