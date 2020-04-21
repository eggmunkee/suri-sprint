
use ggez::nalgebra as na;
use specs::{Component, DenseVecStorage, World, WorldExt};
use specs_derive::*;
use wrapped2d::b2;
use rand::prelude::*;


use crate::components::sprite::{SpriteComponent};
use crate::components::collision::{Collision};
use crate::physics;
use crate::physics::{PhysicsWorld};

#[derive(Debug,Component)]
#[storage(DenseVecStorage)]
pub struct LogicComponent {
    pub initial_value: bool, // base value
    pub input_value: bool,
    pub value: bool, // the node's result value
    pub is_active: bool, // If this node is generating an active signal
}

impl LogicComponent {
    pub fn new(is_enabled: bool) -> LogicComponent {

        let mut logic = LogicComponent {
            initial_value: is_enabled,
            input_value: false,
            value: is_enabled,
            is_active: false,
        };

        logic
    }

    fn calc_value(&mut self) {
        let mut calc_val = self.initial_value;
        if self.input_value {
            calc_val = !calc_val;
        }
        if self.is_active {
            calc_val = !calc_val;
        }
        self.value = calc_val;
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
        self.calc_value();
    }

    pub fn set_input_value(&mut self, input_value: bool) {
        self.input_value = input_value;
        self.calc_value();
    }

    pub fn update(&mut self, time_delta: f32) {

    }

}



// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    world.register::<LogicComponent>();
}