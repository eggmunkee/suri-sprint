
use ggez::nalgebra as na;
use specs::{Component, DenseVecStorage, World, WorldExt};
use specs_derive::*;
use wrapped2d::b2;
use std::ops::Deref;


use crate::components::{WorldUpdateTrait};
use crate::components::collision::{Collision};
//use crate::core::physics;
use crate::core::{PhysicsWorld};

const PRESS_RATIO : f32 = 0.70;
const DOWN_TO_TRIGGER: f32 = 0.2;

#[derive(Debug,Component)]
#[storage(DenseVecStorage)]
pub struct ButtonComponent {
    pub button_trigger: i32,
    pub is_pressed: bool,
    pub name: String,
    pub repeat_trigger: bool,
    pub trigger_cooldown: f32,
    pub timer: f32,
    pub triggered: bool,
}

impl ButtonComponent {
    pub fn new(name: String) -> ButtonComponent {

        let mut button = ButtonComponent {
            button_trigger: -1,
            is_pressed: false,
            name: name,
            repeat_trigger: true,
            trigger_cooldown: 1.0,
            timer: 1.0,
            triggered: false,
        };

        button
    }

    pub fn is_active(&self) -> bool {
        if self.triggered {
            true
        }
        else {
            false
        }
    }

    pub fn set_trigger(&mut self, trigger_id: i32) {
        self.button_trigger = trigger_id;
    }

}

impl WorldUpdateTrait for ButtonComponent {
    fn update(&mut self, delta_time: f32, collision: &mut Collision, physics_world: &mut PhysicsWorld) {

        //self.triggered = false;
        self.is_pressed = false;

        if let Some(handle) = collision.body_handle {

            let body = physics_world.body(handle);

            let mut joints = body.joints();
            let mut up_ratio = 1.0;

            if let Some((other_body_handle, joint_handle)) = &joints.nth(0) {
                let joint = physics_world.joint(*joint_handle);
                let j_act = &*joint;
                let j = j_act.deref();
                match j {
                    b2::UnknownJoint::Prismatic(prism_joint) => {
                        let lower_limit = prism_joint.lower_limit();
                        let upper_limit = prism_joint.upper_limit();
                        let linear_val = prism_joint.joint_translation();
                        up_ratio = (linear_val - lower_limit) / (upper_limit - lower_limit);                        
                    },
                    _ => {}
                }
                
            }

            //println!("Button linear translation: {}", &up_ratio);
            if up_ratio <= PRESS_RATIO {
                self.is_pressed = true;
            }

        }

        // pressed - timer down to DOWN_TO_TRIGGER, then triger
        if self.is_pressed {
            self.timer -= delta_time;
    
            if self.timer < DOWN_TO_TRIGGER { 
                self.timer = DOWN_TO_TRIGGER; 
            }

            if self.timer == DOWN_TO_TRIGGER {
                self.triggered = true;
                if self.repeat_trigger == false {
                    self.timer = self.trigger_cooldown;
                }
            }
            else {
                self.triggered = false;
            }
            
        }
        // non-pressed cool down timer to zero
        else {
            if self.triggered {
                self.triggered = false;
                self.timer = self.trigger_cooldown;
            }

            if self.timer > 0.0 {
                self.timer -= delta_time;
    
                if self.timer < 0.0 { self.timer = 0.0; }
            }
        } 
    
    }
}

#[derive(Debug,Component)]
#[storage(DenseVecStorage)]
pub struct ButtonTriggerComponent {
    pub button: i32,
}

impl ButtonTriggerComponent {
    pub fn new() -> ButtonTriggerComponent {

        let mut button = ButtonTriggerComponent {
            button: -1,
        };

        button
    }

    pub fn set_button(&mut self, button_id: i32) {
        self.button = button_id;
    }
}

// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    world.register::<ButtonComponent>();
    world.register::<ButtonTriggerComponent>();
}