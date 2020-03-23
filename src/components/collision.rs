use specs::{Component, DenseVecStorage, World, WorldExt};
//use specs::shred::{Dispatcher};
use ggez::nalgebra::{Point2,Vector2,distance};
use wrapped2d::b2;
use wrapped2d::user_data::NoUserData;


use crate::physics;
use crate::physics::{PhysicsWorld, PhysicsBody, PhysicsBodyHandle};
use crate::components::player::{CharacterDisplayComponent};

#[derive(Debug)]
pub enum CollisionShape {
    Circle(f32),
    Square(f32)
}

#[derive(Debug)]
pub struct Collision {
    pub shape: CollisionShape,
    pub mass: f32,
    pub friction: f32,
    pub body_handle: Option<PhysicsBodyHandle>,
    // generic collision information
    pub dim_1: f32,
    pub dim_2: f32,
    pub pos: Point2::<f32>,
    pub vel: Vector2::<f32>,
}

impl Collision {
    #[allow(dead_code)]
    pub fn new() -> Collision {
        Collision {
            shape: CollisionShape::Circle(32.0),
            mass: 1.0,
            friction: 0.05,
            body_handle: None,
            dim_1: 1.0,
            dim_2: 1.0,
            pos: Point2::new(0.0,0.0),
            vel: Vector2::new(0.0,0.0),
        }
    }
    pub fn new_specs(m: f32, f: f32) -> Collision {
        Collision {
            shape: CollisionShape::Circle(32.0),
            mass: m,
            friction: f,
            body_handle: None,
            dim_1: 1.0,
            dim_2: 1.0,
            pos: Point2::new(0.0,0.0),
            vel: Vector2::new(0.0,0.0),
        }
    }
    pub fn new_circle(radius: f32) -> Collision {
        Collision {
            shape: CollisionShape::Circle(radius),
            mass: 1.0,
            friction: 0.05,
            body_handle: None,
            dim_1: 1.0,
            dim_2: 1.0,
            pos: Point2::new(0.0,0.0),
            vel: Vector2::new(0.0,0.0),
        }
    }
    pub fn new_square(radius: f32) -> Collision {
        Collision {
            shape: CollisionShape::Square(radius),
            mass: 1.0,
            friction: 0.05,
            body_handle: None,
            dim_1: 1.0,
            dim_2: 1.0,
            pos: Point2::new(0.0,0.0),
            vel: Vector2::new(0.0,0.0),
        }
    }


    // Create the physics body as a static body
    pub fn create_static_body(&mut self, world: &mut World, physics_world: &mut PhysicsWorld) {
        
        let body_handle = physics::add_static_body_box(physics_world, &Point2::<f32>::new(self.pos.x,self.pos.y), 
            self.dim_1, self.dim_2);

        self.body_handle = Some(body_handle);
    }

    // Create the physics body as a dynamic body
    pub fn create_dynamic_body(&mut self, world: &mut World, physics_world: &mut PhysicsWorld) {
        
        let body_handle = physics::add_dynamic_body_box(physics_world, &Point2::<f32>::new(self.pos.x,self.pos.y), 
            self.dim_1, self.dim_2);

        self.body_handle = Some(body_handle);
    }

    // example to get info from physics body
    pub fn get_physics_body_mass(&self, physics_world: &mut PhysicsWorld) -> f32 {
        let body_handle = self.body_handle.unwrap();
        let body = physics_world.body(body_handle);

        let mass = body.mass();
        
        mass
    }

    pub fn update_body(&mut self, physics_world: &mut PhysicsWorld, character: &mut CharacterDisplayComponent) {
        if let Some(body_handle) = self.body_handle {
            let mut body = physics_world.body_mut(body_handle);

            character.apply_collision(&mut body);

            // let move_amt = 1000.0;
            // if character.going_right {
            //     //let new_lin_vel = physics::create_pos(&Point2::new(self.vel.x, self.vel.y));
                
            //     body.apply_force_to_center(&physics::PhysicsVec {x:move_amt,y: -10.0}, true);
            //     println!("applied right force");
            // }
            // if character.going_left {
            //     //let new_lin_vel = physics::create_pos(&Point2::new(self.vel.x, self.vel.y));
            //     body.apply_force_to_center(&physics::PhysicsVec {x:-move_amt,y: 0.0}, true);
            //     println!("applied left force");
            // }
            // if character.going_up {
            //     //let new_lin_vel = physics::create_pos(&Point2::new(self.vel.x, self.vel.y));
            //     body.apply_force_to_center(&physics::PhysicsVec {x:0.0,y: -move_amt}, true);
            //     println!("applied up force");
            // }
            // if character.going_down {
            //     //let new_lin_vel = physics::create_pos(&Point2::new(self.vel.x, self.vel.y));
            //     body.apply_force_to_center(&physics::PhysicsVec {x:0.0,y: move_amt}, true);
            //     println!("applied down force");
            // }

            //let new_lin_vel = physics::create_pos(&Point2::new(self.vel.x, self.vel.y));
            
            //body.apply_force_to_center(&new_lin_vel, true);
            // if new_lin_vel.x != 0.0 || new_lin_vel.y != 0.0 {
            //      body.set_linear_velocity(&new_lin_vel);
            // }
            //let curr_pos = physics::get_pos(body.position());
            //self.pos.x = curr_pos.x;
            //self.pos.y = curr_pos.y;        
    
            //println!("New position: {}, {}, new velocity: {}, {}", &self.pos.x, &self.pos.y, &new_lin_vel.x, &new_lin_vel.y);
        }
    }

    pub fn update_component(&mut self, physics_world: &mut PhysicsWorld) {
        if let Some(body_handle) = self.body_handle {
            let body = physics_world.body(body_handle);

            let curr_pos = physics::get_pos(body.position());
            self.pos.x = curr_pos.x;
            self.pos.y = curr_pos.y;
            let curr_vel = body.linear_velocity();
            self.vel.x = curr_vel.x;
            self.vel.y = curr_vel.y;
    
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


