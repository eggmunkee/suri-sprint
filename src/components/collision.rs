use specs::{Component, DenseVecStorage, World, WorldExt};
//use specs::shred::{Dispatcher};
use ggez::nalgebra::{Point2,Vector2,distance};
use wrapped2d::b2;
use wrapped2d::user_data::NoUserData;
use rand::prelude::*;


use crate::entities::level_builder::{LevelBounds};
use crate::physics;
use crate::physics::{PhysicsWorld, PhysicsBody, PhysicsBodyHandle, CollisionCategory, CollideType};
use crate::components::player::{CharacterDisplayComponent};
use crate::components::npc::{NpcComponent};

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
    pub is_sensor: bool,
    pub dim_1: f32,
    pub dim_2: f32,
    pub pos: Point2::<f32>,
    pub vel: Vector2::<f32>,
    pub angle: f32,
    pub collision_category: CollisionCategory,
    pub collision_mask: Vec::<CollisionCategory>,
    // area status
    pub enable_warp: bool,
    pub in_exit: bool,
    pub in_portal: bool,
    pub exit_id: i32,
    pub portal_id: i32,
    pub last_portal_id: i32,
    pub since_warp: f32,
    // collision status
    // body contacts list  (entity_id, collide_type)
    pub body_contacts: Vec::<(i32, CollideType)>,
    
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
            is_sensor: false,
            dim_1: 1.0,
            dim_2: 1.0,
            pos: Point2::new(0.0,0.0),
            vel: Vector2::new(0.0,0.0),
            angle: 0.0,
            collision_category: CollisionCategory::Level,
            collision_mask: vec![CollisionCategory::Level,CollisionCategory::Etherial],
            in_exit: false,
            in_portal: false,
            exit_id: -1,
            portal_id: -1,
            last_portal_id: -1,
            since_warp: 0.2,
            enable_warp: false,            
            body_contacts: vec![],
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
            is_sensor: false,
            dim_1: dim_1,
            dim_2: dim_2,
            pos: Point2::new(0.0,0.0),
            vel: Vector2::new(0.0,0.0),
            angle: 0.0,
            collision_category: CollisionCategory::Level,
            collision_mask: vec![CollisionCategory::Level,CollisionCategory::Etherial],
            in_exit: false,
            in_portal: false,
            exit_id: -1,
            portal_id: -1,
            last_portal_id: -1,
            since_warp: 0.2,
            enable_warp: false,
            body_contacts: vec![],
        }
    }

    pub fn set_active(&mut self, physics_world: &mut PhysicsWorld) {
        self.body_handle = match self.body_handle {
            Some(handle) => {
                println!("Destroying physics body: {:?}", &handle);
                physics_world.destroy_body(handle);
                None
            }, 
            _ => None
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

    pub fn get_movement(&self) -> Vector2::<f32> {
        Vector2::new(self.vel.x, self.vel.y)
    }

    pub fn clear_pre_physics_state(&mut self) {
        self.last_portal_id = self.portal_id;
        self.in_portal = false;
        self.in_exit = false;
        self.exit_id = -1;
        self.portal_id = -1;
    }



    // Create the physics body as a static body
    pub fn create_static_body_box(&mut self, physics_world: &mut PhysicsWorld) {
        
        let body_handle = physics::add_static_body_box(physics_world, &Point2::<f32>::new(self.pos.x,self.pos.y), self.angle,
            self.dim_1, self.dim_2, self.density, self.restitution, self.collision_category, &self.collision_mask, self.is_sensor);

        self.body_handle = Some(body_handle);
    }

    // Create the physics body as a static body
    pub fn create_static_body_circle(&mut self, physics_world: &mut PhysicsWorld) {
        
        let body_handle = physics::add_static_body_circle(physics_world, &self.pos, 
            self.dim_1, self.density, self.restitution, self.collision_category, &self.collision_mask, self.is_sensor);

        self.body_handle = Some(body_handle);
    }


    // Create the physics body as a dynamic body
    pub fn create_kinematic_body_circle(&mut self, physics_world: &mut PhysicsWorld) {
        
        let body_handle = physics::add_kinematic_body_circle(physics_world, &self.pos, &self.vel,
            self.dim_1, self.density, self.restitution, self.collision_category, &self.collision_mask, true, self.is_sensor);

        self.body_handle = Some(body_handle);
    }
    // Create the physics body as a dynamic body
    pub fn create_kinematic_body_box_upright(&mut self, physics_world: &mut PhysicsWorld) {
        
        let body_handle = physics::add_kinematic_body_box(physics_world, &self.pos, &self.vel,
            self.dim_1, self.dim_2, self.density, self.restitution, self.collision_category, &self.collision_mask, true, self.is_sensor);

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

    pub fn can_use_portal(&self) -> bool {
        if self.since_warp < 0.5 {
            false
        }
        else {
            true
        }
    }

    pub fn pre_physics_hook(&mut self, physics_world: &mut PhysicsWorld, time_delta: f32, 
        opt_character: Option<&mut CharacterDisplayComponent>,
        opt_npc: Option<&mut NpcComponent>,
        level_bounds: &LevelBounds) {

        let mut rng = rand::thread_rng();

        self.body_contacts.clear();

        self.since_warp += time_delta;

        if let Some(body_handle) = self.body_handle {
            let mut body = physics_world.body_mut(body_handle);

            self.clear_pre_physics_state();

            if let Some(character) = opt_character {
                // have character handle applying inputs to collision body
                character.apply_movement(&mut body);

                character.in_exit = false;
                character.in_portal = false;
                character.exit_id = -1;
                character.portal_id = -1;
            }

            if let Some(npc) = opt_npc {
                // have character handle applying inputs to collision body
                npc.apply_movement(&mut body);

            }

            let mut curr_pos = physics::get_pos(body.position());

            let mut updated_pos = false;
            
            if curr_pos.y > level_bounds.max_y {
                curr_pos.y = level_bounds.min_y;

                //let new_x = (4800.0 * rng.gen::<f32>()) + 100.0;

                // move falling objects inward from edges as they wrap to the top
                // if curr_pos.x > 4900.0 {
                //     curr_pos.x = new_x;
                // }
                // if curr_pos.x < 100.0 {
                //     curr_pos.x = new_x;
                // }

                updated_pos = true;
            }

            if curr_pos.x < level_bounds.min_x {
                curr_pos.x = level_bounds.max_x - 1.0;
                updated_pos = true;
            }
            else if curr_pos.x > level_bounds.max_x {
                curr_pos.x = level_bounds.min_x + 1.0;
                updated_pos = true;
            }

            if updated_pos {
                //println!("collider new position: {}, {}", &curr_pos.x, &curr_pos.y);
                //self.update_body_transfrom(physics_world, &curr_pos, &mut body);

                let phys_pos = physics::create_pos(&curr_pos);
                let curr_ang = body.angle();
                body.set_transform(&phys_pos, curr_ang);
            }
    
        }

    }

    pub fn update_body_transform(&mut self, physics_world: &mut PhysicsWorld, position: &Point2::<f32>) {

        if let Some(body_handle) = self.body_handle {
            let mut body = physics_world.body_mut(body_handle);


            let phys_pos = physics::create_pos(position);

            let curr_ang = body.angle();
            body.set_transform(&phys_pos, curr_ang);
        }
    }

    pub fn update_body_velocity(&mut self, physics_world: &mut PhysicsWorld, velocity: &Vector2::<f32>) {

        if let Some(body_handle) = self.body_handle {
            let mut body = physics_world.body_mut(body_handle);

            self.vel.x = velocity.x;
            self.vel.y = velocity.y;
            body.set_linear_velocity(&b2::Vec2{x: self.vel.x, y: self.vel.y});
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

            //if self.in_portal && self.last_portal_id == self.portal_id 
    
            for (entity_id, collide_type) in &self.body_contacts {
                //println!("Body contact: {} {:?}", &entity_id, &collide_type);

                // match &collide_type {
                //     CollideType::Player_Portal
                // }
            }

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


