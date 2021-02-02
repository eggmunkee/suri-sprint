
use ggez::nalgebra as na;
use specs::{Component, DenseVecStorage, World, WorldExt};
use specs_derive::*;
use wrapped2d::b2;
use rand::prelude::*;


use crate::components::sprite::{SpriteComponent};
use crate::components::collision::{Collision};
use crate::core::physics;
use crate::core::physics::{PhysicsWorld};

#[derive(Debug,Component)]
#[storage(DenseVecStorage)]
pub struct NpcComponent {
    pub is_enabled: bool,
    pub going_left: bool,
    pub going_right: bool,
    pub going_up: bool,
    pub going_down: bool,
    // facing status
    pub facing_right: bool,
    // roation
    pub rot: f32,
    // jump variables
    pub in_jump: bool,
    pub jump_duration: f32,
    // stand/fall status
    pub since_stand: f32,
    pub since_move: f32,
    pub in_fall: bool,

    pub dec_timer: f32,

    pub tracking_target: bool,
    pub target_position: (f32, f32),
}

impl NpcComponent {
    pub fn new() -> NpcComponent {

        let mut npc = NpcComponent {
            is_enabled: true,
            going_left: false,
            going_right: false,
            going_up: false,
            going_down: false,
            facing_right: true,
        
            rot: 0.0,
            in_jump: false,
            jump_duration: 0.0,
            since_stand: 0.0,
            since_move: 0.0,
            in_fall: true,

            dec_timer: 0.0,

            tracking_target: true,
            target_position: (500.0, 500.0),
        };

        npc
    }

    pub fn update(&mut self, body_movement: na::Vector2::<f32>, time_delta: f32, x: f32, y: f32) {
        let mut rng = rand::thread_rng();

        self.since_stand += time_delta;

        if self.is_enabled { //body_movement.x < 1.0 && body_movement.x > -1.0 {
            self.dec_timer += time_delta * rng.gen::<f32>() * 2.0;
            if self.dec_timer > 0.5 {
                let dec = rng.gen::<f32>();
                if self.tracking_target {
                    if x < self.target_position.0 - 100.0 {
                        if dec < 0.85 {
                            self.going_right = true;
                            self.going_left = false;
                        }
                        else if dec < 0.89 {
                            self.going_left = true;
                            self.going_right = false;
                        }
                        else {
                            self.going_left = false;
                            self.going_right = false;
                        }

                    }
                    else if x > self.target_position.0 + 100.0 {
                        if dec < 0.85 {
                            self.going_left = true;
                            self.going_right = false;
                        }
                        else if dec < 0.89 {
                            self.going_right = true;
                            self.going_left = false;
                        }
                        else {
                            self.going_left = false;
                            self.going_right = false;
                        }
                    }
                    else {
                        if dec < 0.85 {
                            self.going_left = false;
                            self.going_right = false;
                        }
                        else if dec < 0.87 {
                            self.going_left = true;
                            self.going_right = false;
                        }
                        else if dec < 0.89 {
                            self.going_right = true;
                            self.going_left = false;
                        }
                    }

                    // if y > self.target_position.1 + 50.0 {
                    //     if dec < 0.85 {
                    //         self.going_up = true;
                    //         self.going_down = false;
                    //     }
                    //     else if dec > 0.89 {
                    //         self.going_down = true;
                    //         self.going_up = false;
                    //     }
                    //     else {
                    //         self.going_up = false;
                    //         self.going_down = false;
                    //     }
                    // }
                    // else if y < self.target_position.1 - 50.0{
                    //     if dec < 0.85 {
                    //         self.going_down = true;
                    //         self.going_up = false;
                    //     }
                    //     else if dec > 0.89 {
                    //         self.going_up = true;
                    //         self.going_down = false;
                    //     }
                    //     else {
                    //         self.going_up = false;
                    //         self.going_down = false;
                    //     }
                    // }
                    // else 
                    {
                        self.going_up = false;
                        self.going_down = false;
                    }
                    self.dec_timer = 0.0;
                }
                else {
                    if dec < 0.15 {
                        self.going_left = true;
                    }
                    else if dec < 0.3 {
                        self.going_right = true;
                    }
                    else if dec < 0.4 && (!self.in_jump && !self.in_fall) {
                        self.going_up = true;
                        let dec2 = rng.gen::<f32>();
                        if dec2 < 0.3 {
                            self.going_left = true;
                        }
                        else if dec2 < 0.6 {
                            self.going_right = true;
                        }
                    }
                    else if dec > 0.7 {
                        self.going_left = false;
                        self.going_right = false;
                        self.going_up = false;
                    }
                    self.dec_timer = 0.0;    
                }

            }
        }
        else {
            /*self.going_left = false;
            self.going_right = false;
            self.going_up = false;
            self.going_down = false;*/
        }

    }

    pub fn update_overhead(&mut self, body_movement: na::Vector2::<f32>, time_delta: f32, x: f32, y: f32) {
        let mut rng = rand::thread_rng();

        self.since_stand += time_delta;

        if self.is_enabled { //body_movement.x < 1.0 && body_movement.x > -1.0 {
            self.dec_timer += time_delta * rng.gen::<f32>() * 2.0;
            if self.dec_timer > 0.5 {
                let dec = rng.gen::<f32>();
                if self.tracking_target {
                    if x < self.target_position.0 - 100.0 {
                        if dec < 0.85 {
                            self.going_right = true;
                            self.going_left = false;
                        }
                        else if dec < 0.89 {
                            self.going_left = true;
                            self.going_right = false;
                        }
                        else {
                            self.going_left = false;
                            self.going_right = false;
                        }

                    }
                    else if x > self.target_position.0 + 100.0 {
                        if dec < 0.85 {
                            self.going_left = true;
                            self.going_right = false;
                        }
                        else if dec < 0.89 {
                            self.going_right = true;
                            self.going_left = false;
                        }
                        else {
                            self.going_left = false;
                            self.going_right = false;
                        }
                    }
                    else {
                        if dec < 0.85 {
                            self.going_left = false;
                            self.going_right = false;
                        }
                        else if dec < 0.87 {
                            self.going_left = true;
                            self.going_right = false;
                        }
                        else if dec < 0.89 {
                            self.going_left = false;
                            self.going_right = true;
                        }
                    }

                    if y < self.target_position.1 - 100.0 {
                        if dec < 0.85 {
                            self.going_up = false;
                            self.going_down = true;
                        }
                        else if dec < 0.87 {
                            self.going_down = false;
                            self.going_up = true;
                        }
                        else if dec < 0.89 {
                            self.going_down = false;
                            self.going_up = false;
                        }
                    }
                    else if y > self.target_position.1 + 100.0 {
                        if dec < 0.85 {
                            self.going_up = true;
                            self.going_down = false;
                        }
                        else if dec < 0.87 {
                            self.going_down = true;
                            self.going_up = false;
                        }
                        else if dec < 0.89 {
                            self.going_down = false;
                            self.going_up = false;
                        }
                    }
                    else {
                        if dec < 0.85 {
                            self.going_up = false;
                            self.going_down = false;
                        }
                        else if dec < 0.87 {
                            self.going_down = true;
                            self.going_up = false;
                        }
                        else if dec < 0.89 {
                            self.going_down = false;
                            self.going_up = true;
                        }
                    }

                    {
                        //self.going_up = false;
                        //self.going_down = false;
                    }
                    self.dec_timer = 0.0;
                }
                else {
                    if dec < 0.15 {
                        self.going_left = true;
                    }
                    else if dec < 0.3 {
                        self.going_right = true;
                    }
                    else if dec < 0.4 { //&& (!self.in_jump && !self.in_fall) {
                        self.going_up = true;
                        self.going_down = false;
                        let dec2 = rng.gen::<f32>();
                        if dec2 < 0.3 {
                            self.going_left = true;
                        }
                        else if dec2 < 0.6 {
                            self.going_right = true;
                        }
                    }
                    else if dec < 0.5 { //&& (!self.in_jump && !self.in_fall) {
                        self.going_down = true;
                        self.going_up = false;
                        let dec2 = rng.gen::<f32>();
                        if dec2 < 0.3 {
                            self.going_left = true;
                            self.going_right = false;
                        }
                        else if dec2 < 0.6 {
                            self.going_right = true;
                            self.going_left = false;
                        }
                    }
                    else if dec > 0.7 {
                        self.going_left = false;
                        self.going_right = false;
                        self.going_up = false;
                        self.going_down = false;
                    }
                    self.dec_timer = 0.0;    
                }

            }
        }
        else {
            /*self.going_left = false;
            self.going_right = false;
            self.going_up = false;
            self.going_down = false;*/
        }

    }    

    pub fn apply_movement(&mut self, body: &mut physics::PhysicsBody, time_delta: f32) {
        let move_amt = 2.0; //1300.0;
        let up_mult = 3.0;
        if self.going_right {
            //let new_lin_vel = physics::create_pos(&Point2::new(self.vel.x, self.vel.y));
            if body.linear_velocity().x < 5.0 {
                body.apply_force_to_center(&physics::PhysicsVec {x:move_amt,y: 0.0}, true);
            }
            
            //println!("applied right force");
        }
        if self.going_left {
            //let new_lin_vel = physics::create_pos(&Point2::new(self.vel.x, self.vel.y));
            if body.linear_velocity().x > -5.0 {
                body.apply_force_to_center(&physics::PhysicsVec {x:-move_amt,y: 0.0}, true);
            }
                //println!("applied left force");
        }
        if self.going_up {
            //let new_lin_vel = physics::create_pos(&Point2::new(self.vel.x, self.vel.y));
            if body.linear_velocity().y > -5.0 {
                body.apply_force_to_center(&physics::PhysicsVec {x:0.0,y: -up_mult * move_amt}, true);
            }
            //println!("applied up force");
        }
        if self.going_down {
            //let new_lin_vel = physics::create_pos(&Point2::new(self.vel.x, self.vel.y));
            body.apply_force_to_center(&physics::PhysicsVec {x:0.0,y: move_amt}, true);
            //println!("applied down force");
        }
    }


    pub fn start_jump(&mut self) {
        self.in_jump = true;
        self.jump_duration = 0.0;
        self.since_stand = 50.0;
    }


    // FALL ------------------------------
    pub fn start_fall(&mut self) {

        self.in_fall = true;
        self.jump_duration = 0.0;
    }

    // STAND ------------------------------
    pub fn start_walk(&mut self) {
        //self.in_walk = true;
        self.in_fall = false;
        self.in_jump = false;
        self.jump_duration = 0.0;
        self.since_stand = 0.0;
    }
}


impl super::CharLevelInteractor for NpcComponent {
    fn set_standing(&mut self, is_standing: bool) {
        match is_standing {
            true => {
                //println!("Npc set is standing");
                if self.in_jump || self.in_fall {
                    self.start_walk();
                }
            },
            false => {
                //println!("Npc set is not standing");
                if !self.in_jump && !self.in_fall {
                    self.start_fall();
                }
            }
        }
    }
}



// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    world.register::<NpcComponent>();
}