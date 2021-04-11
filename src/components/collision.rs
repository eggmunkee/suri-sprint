use specs::{Component, DenseVecStorage, World, WorldExt, Entity};
//use specs::shred::{Dispatcher};
use ggez::nalgebra::{Point2,Vector2,distance};
use wrapped2d::b2;
use wrapped2d::user_data::NoUserData;
use wrapped2d::b2::{MetaBody};
//use wrapped2d::dynamics
use rand::prelude::*;


use crate::core::physics as physics;
use crate::core::physics::{PhysicsWorld, PhysicsBody, PhysicsVec, PhysicsBodyType, PhysicsBodyHandle, EntityType,
    CollisionCategory, CollisionBit, CollideType};
use crate::entities::level_builder::{LevelType};
use crate::components::{PhysicsUpdateTrait};
use crate::components::player::{CharacterDisplayComponent};
use crate::components::npc::{NpcComponent};
use crate::resources::{GameStateResource};

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
    pub entity_type: EntityType,
    pub collision_category: CollisionCategory,
    pub collision_mask: Vec::<CollisionCategory>,
    // area status
    pub enable_warp: bool,
    pub in_exit: bool,
    pub in_portal: bool,
    pub trigger_portal_warp: bool,
    pub exit_id: i32,
    pub portal_id: i32,
    pub last_portal_id: i32,
    pub since_warp: f32,
    // collision status
    // body contacts list  (entity_id, collide_type)
    pub body_contacts: Vec::<(i32, CollideType)>,
    pub toggleable: bool,
    pub is_obstructing: bool,
    pub flag_obstruction: bool,
    pub vel_last: Vector2::<f32>,
    pub vel_history: Vec::<Vector2::<f32>>,
    pub pos_last: Point2::<f32>,
    pub pos_history: Vec::<Point2::<f32>>,
    pub delete_flag: bool,
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
            entity_type: EntityType::None,
            collision_category: CollisionCategory::Level,
            collision_mask: vec![CollisionCategory::Level,CollisionCategory::Etherial],
            in_exit: false,
            in_portal: false,
            trigger_portal_warp: false,
            exit_id: -1,
            portal_id: -1,
            last_portal_id: -1,
            since_warp: 0.2,
            enable_warp: false,            
            body_contacts: vec![],
            toggleable: false,
            is_obstructing: true,
            flag_obstruction: false,
            vel_last: Vector2::new(0.0,0.0),
            vel_history: vec![],
            pos_last: Point2::new(0.0,0.0),
            pos_history: vec![],
            delete_flag: false,
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
            entity_type: EntityType::None,
            collision_category: CollisionCategory::Level,
            collision_mask: vec![CollisionCategory::Level,CollisionCategory::Etherial],
            in_exit: false,
            in_portal: false,
            trigger_portal_warp: false,
            exit_id: -1,
            portal_id: -1,
            last_portal_id: -1,
            since_warp: 0.2,
            enable_warp: false,
            body_contacts: vec![],
            toggleable: false,
            is_obstructing: true,
            flag_obstruction: false,
            vel_last: Vector2::new(0.0,0.0),
            vel_history: vec![],
            pos_last: Point2::new(0.0,0.0),
            pos_history: vec![],
            delete_flag: false,
        }
    }

    pub fn set_active(&mut self, physics_world: &mut PhysicsWorld, is_active: bool) {
        match self.body_handle {
            Some(handle) => {
                //println!("Destroying physics body: {:?}", &handle);
                let mut body = physics_world.body_mut(handle);
                body.set_active(is_active);
            }, 
            _ => {}
        }
    }

    pub fn set_linear_damping(&mut self, physics_world: &mut PhysicsWorld, damping: f32) {
        match self.body_handle {
            Some(handle) => {
                //println!("Destroying physics body: {:?}", &handle);
                let mut body = physics_world.body_mut(handle);
                body.set_linear_damping(damping);
            }, 
            _ => {}
        }
    }

    pub fn set_gravity_scale(&mut self, physics_world: &mut PhysicsWorld, new_scale: f32) {
        match self.body_handle {
            Some(handle) => {
                //println!("Destroying physics body: {:?}", &handle);
                let mut body = physics_world.body_mut(handle);
                body.set_gravity_scale(new_scale);
            }, 
            _ => {}
        }
    }

    pub fn get_gravity_scale(&mut self, physics_world: &mut PhysicsWorld) -> f32 {        
        let grav_scale = match self.body_handle {
            Some(handle) => {
                //println!("Destroying physics body: {:?}", &handle);
                let mut body = physics_world.body_mut(handle);
                body.gravity_scale()
            }, 
            _ => 0.0
        };
        grav_scale
    }

    pub fn destroy_body(&mut self, physics_world: &mut PhysicsWorld) {
        self.body_handle = match self.body_handle {
            Some(handle) => {
                //println!("Destroying physics body: {:?}", &handle);
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
        if self.last_portal_id != -1 || self.portal_id != -1 {            
            //println!("Clearing portal info: Last {:?} New Last {:?}", &self.last_portal_id, &self.portal_id);
            //println!("  Trigger portal warp: {} In Portal: {}", &self.trigger_portal_warp, &self.in_portal);
            //println!("  Curr pos: {:?} vel: {:?}", &self.pos, &self.vel);
            if let Some(body_handle) = self.body_handle {
                //println!("  Body handle: {:?}", &body_handle);
            }
        }
        self.last_portal_id = self.portal_id;
        self.trigger_portal_warp = false;
        self.in_portal = false;
        self.in_exit = false;
        self.exit_id = -1;
        self.portal_id = -1;
    }



    // Create the physics body as a static body
    pub fn create_static_body_box(&mut self, physics_world: &mut PhysicsWorld) {
        
        let body_handle = physics::add_static_body_box(physics_world, &Point2::<f32>::new(self.pos.x,self.pos.y), self.angle,
            self.dim_1, self.dim_2, self.density, self.restitution, self.entity_type, self.collision_category, &self.collision_mask, self.is_sensor, true);

        self.body_handle = Some(body_handle);
    }

    // Create the physics body as a static body
    pub fn create_static_body_circle(&mut self, physics_world: &mut PhysicsWorld, fixed_rot: bool) {
        
        let body_handle = physics::add_static_body_circle(physics_world, &self.pos, 
            self.dim_1, self.density, self.restitution, self.entity_type, self.collision_category, &self.collision_mask, self.is_sensor, fixed_rot);

        self.body_handle = Some(body_handle);
    }


    // Create the physics body as a kinematic body - CIRCLE
    pub fn create_kinematic_body_circle(&mut self, physics_world: &mut PhysicsWorld, fixed_rot: bool) {
        
        let body_handle = physics::add_kinematic_body_circle(physics_world, &self.pos, &self.vel,
            self.dim_1, self.density, self.restitution, self.entity_type, self.collision_category, &self.collision_mask, fixed_rot, self.is_sensor);

        self.body_handle = Some(body_handle);
    }
    // Create the physics body as a kinematic body - FIXED ROTATION BOX
    pub fn create_kinematic_body_box_upright(&mut self, physics_world: &mut PhysicsWorld) {
        
        let body_handle = physics::add_kinematic_body_box(physics_world, &self.pos, &self.vel, self.angle,
            self.dim_1, self.dim_2, self.density, self.restitution, self.entity_type, self.collision_category, &self.collision_mask, true, self.is_sensor);

        self.body_handle = Some(body_handle);
    }
    // Create the physics body as a kinematic body - ROTABLE ROX
    // pub fn create_kinematic_body_box_rotable(&mut self, physics_world: &mut PhysicsWorld) {
        
    //     let body_handle = physics::add_kinematic_body_box(physics_world, &self.pos,  &self.vel, self.angle,
    //         self.dim_1, self.dim_2, self.density, self.restitution, self.entity_type, self.collision_category, &self.collision_mask, false, self.is_sensor);

    //     self.body_handle = Some(body_handle);
    // }

    // Create the physics body as a dynamic body
    pub fn create_dynamic_body_box_upright(&mut self, physics_world: &mut PhysicsWorld) {
        
        let body_handle = physics::add_dynamic_body_box(physics_world, &self.pos, self.angle,
            self.dim_1, self.dim_2, self.density, self.restitution, self.entity_type, self.collision_category, &self.collision_mask, true);

        self.body_handle = Some(body_handle);
    }

    // Create the physics body as a dynamic body
    pub fn create_dynamic_body_box_rotable(&mut self, physics_world: &mut PhysicsWorld) {
        
        let body_handle = physics::add_dynamic_body_box(physics_world, &Point2::<f32>::new(self.pos.x,self.pos.y), self.angle,
            self.dim_1, self.dim_2, self.density, self.restitution, self.entity_type, self.collision_category, &self.collision_mask, false);

        self.body_handle = Some(body_handle);
    }

    // Create the physics body as a dynamic body
    pub fn create_dynamic_body_circle(&mut self, physics_world: &mut PhysicsWorld) {
        
        let body_handle = physics::add_dynamic_body_circle(physics_world, &Point2::<f32>::new(self.pos.x,self.pos.y), 
            self.dim_1, self.density, self.restitution, self.entity_type, self.collision_category, &self.collision_mask, false);

        self.body_handle = Some(body_handle);
    }

    // Create the physics body as a dynamic body
    pub fn create_dynamic_body_circle_fixed(&mut self, physics_world: &mut PhysicsWorld) {
        
        let body_handle = physics::add_dynamic_body_circle(physics_world, &Point2::<f32>::new(self.pos.x,self.pos.y), 
            self.dim_1, self.density, self.restitution, self.entity_type, self.collision_category, &self.collision_mask, true);

        self.body_handle = Some(body_handle);
    }

    pub fn set_obstructing(&mut self, obstructing: bool) {
        if self.is_obstructing != obstructing {
            self.is_obstructing = obstructing;
            self.flag_obstruction = true;
            println!("[COLLISION] FLAGGED - OBSTRUCTING: {}", &self.is_obstructing);
        }
    }

    pub fn can_use_portal(&self) -> bool {
        if self.since_warp < 0.5 {
            false
        }
        else {
            true
        }
    }

    pub fn get_avg_x(&self, frame_count: usize) -> f32 {

        let mut x_sum = 0.0;
        let mut x_count = 0;
        let hist_len = self.vel_history.len();
        for i in (0..hist_len).rev() {
            if let Some(vel) = self.vel_history.get(i) {
                if x_count < frame_count {
                    x_sum += vel.x;
                    x_count += 1;
                }
            }
        }
        
        if x_count > 0 {
            //println!("avg x count: {}", &x_count);
            x_sum / x_count as f32
        }
        else {
            0.0
        }
    }

    pub fn get_avg_y(&self, frame_count: usize) -> f32 {

        let mut y_sum = 0.0;
        let mut y_count = 0;
        let hist_len = self.vel_history.len();
        for i in (0..hist_len).rev() {
            if let Some(vel) = self.vel_history.get(i) {
                if y_count < frame_count {
                    y_sum += vel.y;
                    y_count += 1;
                }
            }
        }
        
        if y_count > 0 {
            //println!("avg y count: {}", &y_count);
            y_sum / y_count as f32
        }
        else {
            0.0
        }
    }

    pub fn get_body_angle(&mut self, physics_world: &mut PhysicsWorld) -> f32 {
        if let Some(body_handle) = self.body_handle {
            let mut body = physics_world.body_mut(body_handle);

            let curr_ang = body.angle();
            curr_ang
        }
        else {
            self.angle
        }
    }

    pub fn update_body_angle(&mut self, physics_world: &mut PhysicsWorld, angle: f32) {
        if let Some(body_handle) = self.body_handle {
            let mut body = physics_world.body_mut(body_handle);

            let curr_pos = body.position().clone();

            //let curr_ang = body.angle();
            body.set_transform(&curr_pos, angle);
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

    pub fn update_body_obstruction(&mut self, physics_world: &mut PhysicsWorld, is_obstruction: bool) {

        if let Some(body_handle) = self.body_handle {
            println!("Update body obj on {:?}", &body_handle);
            let mut body = physics_world.body_mut(body_handle);

            let phys_body_type = body.body_type();
            println!(" Body Type: {:?}", &phys_body_type);

            let mut first_fixture : Option<b2::FixtureHandle> = None;

            for (fixture, meta) in body.fixtures() {
                println!(" Update body obstruction has 1st fixture. Obstruct: {}", &is_obstruction);
                first_fixture = Some(fixture);
                break;
            }

            let category_bits = match is_obstruction {
                true => self.collision_category.to_bits(),
                false => 0
            };
            let mask_bits = match is_obstruction {
                true => (&self.collision_mask).to_bits(),
                false => 0
            };

            if let Some(fixture) = first_fixture {
                //fixture.set_filter_data();
                println!(" Updating filter data: {}, {}", &category_bits, &mask_bits);
                let mut obj_fixture = body.fixture_mut(fixture);
                obj_fixture.set_filter_data(&b2::Filter {
                    category_bits: category_bits,
                    mask_bits: mask_bits,
                    group_index: 0,
                });
            }
        }
    }
    

    
}

impl PhysicsUpdateTrait for Collision {
    fn pre_physics_update(&mut self, world: &World, physics_world: &mut PhysicsWorld, time_delta: f32, 
        opt_collision: &mut Option<&mut Collision>,
        opt_character: &mut Option<&mut CharacterDisplayComponent>,
        opt_npc: &mut Option<&mut NpcComponent>,
        //level_bounds: &LevelBounds,
        //game_state: &GameStateResource,
        entity: &Entity) {

        let mut rng = rand::thread_rng();

        let game_state = world.fetch::<GameStateResource>();

        if self.toggleable && self.flag_obstruction {            
            let entity_type = &self.entity_type;
            println!("TOGGLEABLE OBSTRUCTION Entity {:?} Body {:?}", &entity_type, &self.body_handle);
            self.update_body_obstruction(physics_world, self.is_obstructing);
            self.flag_obstruction = false;
        }

        self.body_contacts.clear();

        self.since_warp += time_delta;

        if let Some(body_handle) = self.body_handle {

            self.clear_pre_physics_state();

            let mut body = physics_world.body_mut(body_handle);

            let mut curr_pos = physics::get_pos(body.position());
            let mut curr_vel = physics::PhysicsVec { x: self.vel.x, y: self.vel.y };

            let mut updated_pos = false;
            let mut movement_applied = false;
            if let Some(ref mut character) = opt_character {
                // have character handle applying inputs to collision body
                character.apply_movement(&mut body, time_delta, match character.go_anywhere_mode {
                    false => &game_state.level_type, true => &LevelType::Overhead });

                character.in_exit = false;
                character.in_portal = false;
                character.exit_id = -1;
                character.portal_id = -1;

                movement_applied = true;

                if character.go_anywhere_mode {
                    // Get updated velocity after movement was applied
                    curr_vel = body.linear_velocity().clone();
                    self.vel.x = curr_vel.x;
                    self.vel.y = curr_vel.y;

                    //println!("Character pos {:?} vel {:?}", &curr_pos, &curr_vel);
                    // Convert velocity values to game scale
                    let sc_vel_x = physics::get_size(curr_vel.x);
                    let sc_vel_y = physics::get_size(curr_vel.y);
                    // Update position from velocity
                    curr_pos.x += sc_vel_x * time_delta;
                    curr_pos.y += sc_vel_y * time_delta;
                    // Create new character position in phys scale
                    let phys_pos = physics::create_pos(&curr_pos);
                    let curr_ang = body.angle();
                    // Update body transform
                    body.set_transform(&phys_pos, curr_ang);
                    // Set collision component position
                    self.pos.x = curr_pos.x;
                    self.pos.y = curr_pos.y;
                }

                
            }

            if let (Some(ref mut npc), false) = (opt_npc, movement_applied ) {
                // have character handle applying inputs to collision body
                npc.apply_movement(&mut body, time_delta);

            }

            
            let body_type = body.body_type();

            if body_type != PhysicsBodyType::Static { 

                let mut updated_pos = false;

                if curr_pos.y > game_state.level_bounds.max_y {
                    curr_pos.y = game_state.level_bounds.min_y;
    
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
    
                if curr_pos.x < game_state.level_bounds.min_x {
                    curr_pos.x = game_state.level_bounds.max_x - 1.0;
                    updated_pos = true;
                }
                else if curr_pos.x > game_state.level_bounds.max_x {
                    curr_pos.x = game_state.level_bounds.min_x + 1.0;
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

        if (self.vel_last.y < -0.03 && self.vel.y > 0.03) ||
            (self.vel_last.y > 0.03 && self.vel.y < -0.03) {
            //println!("Y Flipped from {} to {}", &self.vel_last.y, &self.vel.y);
        }
        self.vel_last.x = self.vel.x;
        self.vel_last.y = self.vel.y;

        while self.vel_history.len() >= 50 {
            self.vel_history.remove(0);
        }
        self.vel_history.push(self.vel_last.clone());

        self.pos_last.x = self.pos.x;
        self.pos_last.y = self.pos.y;

        while self.pos_history.len() >= 50 {
            self.pos_history.remove(0);
        }
        self.pos_history.push(self.pos_last.clone());

    }

    fn post_physics_update(&mut self, world: &World, physics_world: &mut PhysicsWorld, time_delta: f32, 
        opt_collision: &mut Option<&mut Collision>,
        opt_character: &mut Option<&mut CharacterDisplayComponent>,
        opt_npc: &mut Option<&mut NpcComponent>,
        //game_state: &GameStateResource,
        entity: &Entity) {

        if let Some(body_handle) = self.body_handle {
            let mut body = physics_world.body_mut(body_handle);

            let curr_pos = physics::get_pos(body.position());

            let apply_phys_update = match opt_character {
                Some(character) => !character.go_anywhere_mode,
                None => true,
            };
            
            if apply_phys_update
            {
                self.pos.x = curr_pos.x;
                self.pos.y = curr_pos.y;
                // self.pos.x = curr_pos.x;
                // self.pos.y = curr_pos.y;
                let curr_vel = body.linear_velocity();    
                self.vel.x = curr_vel.x;
                self.vel.y = curr_vel.y;

                self.angle = body.angle();
            }
            // EXPERIMENTAL OVERRIDE PHYSICS RESULTS FROM Collision component
            else {
                // Set collider position back to physics body
                let phys_pos = physics::create_pos(&self.pos);
                let curr_ang = body.angle();
                body.set_transform(&phys_pos, curr_ang);
                // Set collider velocity back to physics body
                body.set_linear_velocity(&PhysicsVec { x: self.vel.x, y: self.vel.y });
            }

            if self.in_portal && self.last_portal_id != self.portal_id {
                //println!("PORTAL CHANGE - triggering warp. From {:?} to {:?}", &self.last_portal_id, &self.portal_id);
                //self.trigger_portal_warp = true;
            }
    
            for (entity_id, collide_type) in &self.body_contacts {
                //println!("Body contact: {} {:?} Self Type: {:?}", &entity_id, &collide_type, &self.entity_type);

                // match &collide_type {
                //     CollideType::Player_Portal
                // }
            }

            // if curr_vel.x == 0.0 && self.vel_last.x.abs() > 0.0001 {
            //     //println!("X Stopped from {}", &self.vel_last.x);
            // }
            // if curr_vel.y == 0.0 && self.vel_last.y.abs() > 0.0001 {
            //     //println!("Y Stopped from {}", &self.vel_last.y);
            // }

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


