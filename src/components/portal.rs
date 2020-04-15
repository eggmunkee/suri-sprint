
use ggez::nalgebra as na;
use specs::{Component, DenseVecStorage, World, WorldExt};
use specs_derive::*;
use wrapped2d::b2;


use crate::components::sprite::{SpriteComponent};
use crate::components::collision::{Collision};
use crate::physics;
use crate::physics::{PhysicsWorld};

#[derive(Debug,Component)]
#[storage(DenseVecStorage)]
pub struct PortalComponent {
    pub normal: na::Vector2::<f32>,
    pub name: String,
    pub destination: String,
    pub anim_timer: f32,
}

impl PortalComponent {
    pub fn new(portal_name: String, destination_name: String) -> PortalComponent {

        let mut portal = PortalComponent {
            normal: na::Vector2::new(0.0, -1.0),
            name: portal_name,
            destination: destination_name,
            anim_timer: 0.0,
        };

        portal
    }

    pub fn update(&mut self, delta_time: f32, collision: &mut Collision, sprite: &mut SpriteComponent, physics_world: &mut PhysicsWorld) {

        // get sprite angle and rotate over time
        let mut sprite_angle = collision.get_body_angle(physics_world);
        println!("Portal body angle: {}", &sprite_angle);
        sprite_angle += 0.75 * delta_time;
        if sprite_angle >= 2.0 * b2::PI {
            sprite_angle -= 2.0 * b2::PI;
        }
        println!("New portal angle: {}", &sprite_angle);
        //collision.angle = sprite_angle;
        collision.update_body_angle(physics_world, sprite_angle);

        //let mut sprite_alpha = sprite.alpha;

    }

}


// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    world.register::<PortalComponent>();
}