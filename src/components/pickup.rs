
use ggez::nalgebra as na;
use specs::{Component, DenseVecStorage, World, WorldExt};
use specs_derive::*;
use wrapped2d::b2;


use crate::components::sprite::{SpriteComponent};
use crate::components::anim_sprite::{AnimSpriteComponent};
use crate::components::collision::{Collision};
use crate::core::physics;
use crate::core::physics::{PhysicsWorld,PickupItemType};

#[derive(Debug,Component)]
#[storage(DenseVecStorage)]
pub struct PickupComponent {
    pub pickup_type: PickupItemType,
    pub picked_up: bool,
    pub in_pickup_anim: bool,
}

impl PickupComponent {
    pub fn new() -> Self {
        Self {
            pickup_type: PickupItemType::Point,
            picked_up: false,
            in_pickup_anim: false,
        }
    }

    pub fn update(&mut self, time_delta: f32, collision: &mut Collision, sprite: &mut AnimSpriteComponent) {
        if self.picked_up && !self.in_pickup_anim {
            if sprite.curr_animation != "explode" {
                sprite.set_animation("explode");
                //sprite.curr_animation = "explode".to_string();
                sprite.set_frame(0);
            }
            self.in_pickup_anim = true;
        }
        else if self.in_pickup_anim {
            if sprite.curr_anim_finished {
                collision.delete_flag = true;
            }
        }
    }

    pub fn pickup(&mut self) {
        self.picked_up = true;
    }
}


// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    world.register::<PickupComponent>();
}