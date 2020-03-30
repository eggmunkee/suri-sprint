use specs::{Component, DenseVecStorage, World, WorldExt};
//use specs::shred::{Dispatcher};
use ggez::nalgebra::{Point2,Vector2,distance};
use wrapped2d::b2;
use wrapped2d::user_data::NoUserData;
use rand::prelude::*;


use crate::physics;
use crate::physics::{PhysicsWorld, PhysicsBody, PhysicsBodyHandle, CollisionCategory};
use crate::components::player::{CharacterDisplayComponent};

#[derive(Debug)]
pub struct Collision {
    // depricated fields
    // pub shape: CollisionShape,
    // pub mass: f32,
    // pub friction: f32,
    // real physics props
    pub density: f32,
    pub restitution: f32,
    pub body_handle: Option<PhysicsBodyHandle>,
    // generic collision information
    pub dim_1: f32,
    pub dim_2: f32,
    pub pos: Point2::<f32>,
    pub vel: Vector2::<f32>,
    pub angle: f32,
    pub collision_category: CollisionCategory,
    pub collision_mask: Vec::<CollisionCategory>,
}

impl Collision {
    #[allow(dead_code)]
    pub fn new() -> Collision {
        Collision {
            // shape: CollisionShape::Circle(32.0),
            // mass: 1.0,
            // friction: 0.05,
            density: 1.0,
            restitution: 0.25,
            body_handle: None,
            dim_1: 1.0,
            dim_2: 1.0,
            pos: Point2::new(0.0,0.0),
            vel: Vector2::new(0.0,0.0),
            angle: 0.0,
            collision_category: CollisionCategory::Level,
            collision_mask: vec![CollisionCategory::Level,CollisionCategory::Ghost],
        }
    }
    pub fn new_specs(density: f32, restitution: f32, dim_1: f32, dim_2: f32) -> Collision {
        Collision {
            // shape: CollisionShape::Circle(32.0),
            // mass: 1.0,
            // friction: 0.0,
            density: density,
            restitution: restitution,
            body_handle: None,
            dim_1: dim_1,
            dim_2: dim_2,
            pos: Point2::new(0.0,0.0),
            vel: Vector2::new(0.0,0.0),
            angle: 0.0,
            collision_category: CollisionCategory::Level,
            collision_mask: vec![CollisionCategory::Level,CollisionCategory::Ghost],
        }
    }

    pub fn destroy_body(&mut self, physics_world: &mut PhysicsWorld) {
        self.body_handle = match self.body_handle {
            Some(handle) => {
                println!("Destroying physics body: {:?}", &handle);
                physics_world.destroy_body(handle);
                None
            }, 
            _ => None
        }
    }

    // Create the physics body as a static body
    pub fn create_static_body_box(&mut self, physics_world: &mut PhysicsWorld) {
        
        let body_handle = physics::add_static_body_box(physics_world, &Point2::<f32>::new(self.pos.x,self.pos.y), self.angle,
            self.dim_1, self.dim_2, self.density, self.restitution, self.collision_category, &self.collision_mask);

        self.body_handle = Some(body_handle);
    }

    // Create the physics body as a static body
    pub fn create_static_body_circle(&mut self, physics_world: &mut PhysicsWorld) {
        
        let body_handle = physics::add_static_body_circle(physics_world, &self.pos, 
            self.dim_1, self.density, self.restitution, self.collision_category, &self.collision_mask);

        self.body_handle = Some(body_handle);
    }


    // Create the physics body as a dynamic body
    pub fn create_kinematic_body_circle(&mut self, physics_world: &mut PhysicsWorld, is_sensor: bool) {
        
        let body_handle = physics::add_kinematic_body_circle(physics_world, &self.pos, &self.vel,
            self.dim_1, self.density, self.restitution, self.collision_category, &self.collision_mask, true, is_sensor);

        self.body_handle = Some(body_handle);
    }
    // Create the physics body as a dynamic body
    pub fn create_kinematic_body_box_upright(&mut self, physics_world: &mut PhysicsWorld, is_sensor: bool) {
        
        let body_handle = physics::add_kinematic_body_box(physics_world, &self.pos, &self.vel,
            self.dim_1, self.dim_2, self.density, self.restitution, self.collision_category, &self.collision_mask, true, is_sensor);

        self.body_handle = Some(body_handle);
    }

    // Create the physics body as a dynamic body
    pub fn create_dynamic_body_box_upright(&mut self, physics_world: &mut PhysicsWorld) {
        
        let body_handle = physics::add_dynamic_body_box(physics_world, &self.pos, 
            self.dim_1, self.dim_2, self.density, self.restitution, self.collision_category, &self.collision_mask, true);

        self.body_handle = Some(body_handle);
    }

    // Create the physics body as a dynamic body
    pub fn create_dynamic_body_box_rotable(&mut self, physics_world: &mut PhysicsWorld) {
        
        let body_handle = physics::add_dynamic_body_box(physics_world, &Point2::<f32>::new(self.pos.x,self.pos.y), 
            self.dim_1, self.dim_2, self.density, self.restitution, self.collision_category, &self.collision_mask, false);

        self.body_handle = Some(body_handle);
    }

    // Create the physics body as a dynamic body
    pub fn create_dynamic_body_circle(&mut self, physics_world: &mut PhysicsWorld) {
        
        let body_handle = physics::add_dynamic_body_circle(physics_world, &Point2::<f32>::new(self.pos.x,self.pos.y), 
            self.dim_1, self.density, self.restitution, self.collision_category, &self.collision_mask, false);

        self.body_handle = Some(body_handle);
    }

    // Create the physics body as a dynamic body
    pub fn create_dynamic_body_circle_fixed(&mut self, physics_world: &mut PhysicsWorld) {
        
        let body_handle = physics::add_dynamic_body_circle(physics_world, &Point2::<f32>::new(self.pos.x,self.pos.y), 
            self.dim_1, self.density, self.restitution, self.collision_category, &self.collision_mask, true);

        self.body_handle = Some(body_handle);
    }

    pub fn pre_physics_hook(&mut self, physics_world: &mut PhysicsWorld, opt_character: Option<&mut CharacterDisplayComponent>) {

        let mut rng = rand::thread_rng();

        if let Some(body_handle) = self.body_handle {
            let mut body = physics_world.body_mut(body_handle);

            if let Some(character) = opt_character {
                // have character handle applying inputs to collision body
                character.apply_collision(&mut body);
            }

            let mut curr_pos = physics::get_pos(body.position());

            let mut updated_pos = false;
            
            if curr_pos.y > 13000.0 {
                curr_pos.y = -1500.0;

                let new_x = (4800.0 * rng.gen::<f32>()) + 100.0;

                // move falling objects inward from edges as they wrap to the top
                if curr_pos.x > 4900.0 {
                    curr_pos.x = new_x;
                }
                if curr_pos.x < 100.0 {
                    curr_pos.x = new_x;
                }

                updated_pos = true;
            }

            if curr_pos.x < -1250.0 {
                curr_pos.x = 6750.0;
                updated_pos = true;
            }
            else if curr_pos.x > 7000.0 {
                curr_pos.x = -1000.0;
                updated_pos = true;
            }

            if updated_pos {
                //println!("collider new position: {}, {}", &curr_pos.x, &curr_pos.y);
                let phys_pos = physics::create_pos(&curr_pos);

                let curr_ang = body.angle();
                body.set_transform(&phys_pos, curr_ang);
            }
    
        }

    }

    pub fn post_physics_hook(&mut self, physics_world: &mut PhysicsWorld) {
        if let Some(body_handle) = self.body_handle {
            let body = physics_world.body(body_handle);

            let curr_pos = physics::get_pos(body.position());
            self.pos.x = curr_pos.x;
            self.pos.y = curr_pos.y;
            let curr_vel = body.linear_velocity();
            self.vel.x = curr_vel.x;
            self.vel.y = curr_vel.y;

            self.angle = body.angle();
    
            //println!("New position: {}, {} Velocity: {}, {}", &self.pos.x, &self.pos.y, &self.vel.x, &self.vel.y);
        }
    }
}

impl Component for Collision {
    type Storage = DenseVecStorage<Self>;
}

// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    world.register::<Collision>();
}


