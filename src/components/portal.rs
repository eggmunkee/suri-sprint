
use ggez::nalgebra as na;
use specs::{Component, DenseVecStorage, World, WorldExt};
use specs_derive::*;
use wrapped2d::b2;


use crate::components::sprite::{SpriteComponent};
use crate::components::anim_sprite::{AnimSpriteComponent};
use crate::components::collision::{Collision};
use crate::core::physics;
use crate::core::physics::{PhysicsWorld};

#[derive(Debug,Component)]
#[storage(DenseVecStorage)]
pub struct PortalComponent {
    pub normal: na::Vector2::<f32>,
    pub screen_facing: bool,
    pub name: String,
    pub destination: String,
    pub anim_timer: f32,
    pub is_enabled: bool,
}

impl PortalComponent {
    pub fn new(portal_name: String, destination_name: String) -> PortalComponent {

        let mut portal = PortalComponent {
            normal: na::Vector2::new(0.0, -1.0),
            screen_facing: true,
            name: portal_name,
            destination: destination_name,
            anim_timer: 0.0,
            is_enabled: true,
        };

        portal
    }

    pub fn update_sprite(&mut self, delta_time: f32, collision: &mut Collision, sprite: &mut SpriteComponent, physics_world: &mut PhysicsWorld) {

        if !self.is_enabled { 
            if sprite.alpha != 0.25 { sprite.alpha = 0.25; }
            return; 
        }

        if sprite.alpha != 1.0 { sprite.alpha = 1.0; }

        let spin_spd = 2.5; // 0.75

        // get sprite angle and rotate over time
        let mut sprite_angle = collision.get_body_angle(physics_world);
        //println!("Portal body angle: {}", &sprite_angle);
        if self.destination.is_empty() {
            sprite_angle -= spin_spd * delta_time;
            if sprite_angle < 0.0 {
                sprite_angle += 2.0 * b2::PI;
            }
        }
        else {
            sprite_angle += spin_spd * delta_time; //0.75 * delta_time;
            if sprite_angle >= 2.0 * b2::PI {
                sprite_angle -= 2.0 * b2::PI;
            }
        }
        //println!("New portal angle: {}", &sprite_angle);
        //collision.angle = sprite_angle;
        collision.update_body_angle(physics_world, sprite_angle);

        //let mut sprite_alpha = sprite.alpha;

    }

    pub fn update_anim_sprite(&mut self, delta_time: f32, collision: &mut Collision, sprite: &mut AnimSpriteComponent, physics_world: &mut PhysicsWorld) {

        if !self.is_enabled { 
            if sprite.curr_animation != "off" {
                sprite.curr_animation = "off".to_string();
                sprite.set_frame(0);
            }
            return; 
        }

        if sprite.curr_animation != "on" {
            sprite.curr_animation = "on".to_string();
            sprite.set_frame(0);
        }
        

        //let mut sprite_alpha = sprite.alpha;

    }

}


// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    world.register::<PortalComponent>();
}