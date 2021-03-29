use std::cmp;
use ggez::{Context};
use ggez::graphics;
use ggez::graphics::{Rect,Image,Color,DrawParam,ShaderLock};
use ggez::nalgebra as na;
use ggez::nalgebra::{Point2};
use specs::{ Component, DenseVecStorage, World, WorldExt, Entity };
//use specs::shred::{Dispatcher};
use rand::prelude::*;

use crate::core::game_state::{GameState};
use crate::resources::{ImageResources,ShaderResources,GameStateResource};
use crate::components::collision::{Collision};
use crate::components::npc::{NpcComponent};
use crate::components::particle_sys::{ParticleSysComponent};
use crate::components::{Velocity,PhysicsUpdateTrait};
use crate::entities::player::{PlayerCharacter};
use crate::entities::level_builder::{LevelType};
use crate::core::physics;
use crate::core::physics::{PhysicsBody,PhysicsWorld};
use crate::render::dialog::{DialogRenderer};

// #[derive(Debug,Copy,Clone)]
// pub enum AnimState {
//     Idle(u32, f32),
//     Fall(u32, f32),
//     Jump(u32, f32),
//     Walk(u32, f32),
//     Sit(u32, f32)
// }

// impl AnimState {
//     fn frame_time(time_mult: f32, time_delta: f32) -> f32 {
//         time_mult * time_delta
//     }

//     fn update_value(current_state: &mut AnimState, t: f32) {
//         match current_state {
//             AnimState::Idle(frame, ref mut time) => {
//                 *time += t;
//             },
//             _ => {},
//         }
//     }

//     pub fn advance(mut current_state: AnimState, time_delta: f32) -> AnimState {

//         let mut time_mult = 1.0;
//         let mut time_limit = 1.0;
//         let mut time = 0.0;

//         // get multiplier for time based on state / frame
//         match &current_state {
//             AnimState::Idle(_, t) => {
//                 time_mult *= 1.5;
//             },
//             _ => {},
//         };

//         // get generic frame time delta
//         time = Self::frame_time(time_mult, time_delta);

//         // update frame time
//         Self::update_value(&mut current_state, time);

//         // If the current frame is past its time limit, find next state
//         if time >= time_limit {
//             match &current_state {
//                 AnimState::Idle(frame, time) => {
//                     AnimState::Fall(0,0.0)
//                 },
//                 _ => current_state,
//             }
//         }
//         // Return current state with updated timm
//         else{
//             current_state
//         }

        

//     }
// }

const WALK_SET : u32 = 0;
const WALK_FRAMES : u32 = 6;
const JUMP_SET : u32 = 1;
const JUMP_FRAMES : u32 = 7;
const FALL_ONLY_SET : u32 = 2;
const FALL_ONLY_FRAMES : u32 = 4;
const IDLE_SET : u32 = 3;
const IDLE_FRAMES : u32 = 4;
const SIT_SET : u32 = 4;
const SIT_FRAMES : u32 = 5;

#[derive(Debug,Clone)]
pub struct AnimSnapshotInfo {
    pub frame_num: u32,
    pub anim_set: u32,
    pub facing_right: bool,
}


#[derive(Debug)]
pub struct CharacterDisplayComponent {
    pub player_number: i32,
    pub player_char: PlayerCharacter,
    pub is_controlled: bool,
    pub is_controllable: bool,
    // image path
    pub spritesheet_path: String,
    pub spritesheet_cols: f32,
    pub spritesheet_rows: f32,
    // movement status
    pub going_left: bool,
    pub going_right: bool,
    pub going_up: bool,
    pub going_down: bool,
    pub meowing: bool,
    // facing status
    pub facing_right: bool,
    // animation status
    //pub anim_state: AnimState,
    pub anim_frame: u32,
    pub anim_set: u32,
    pub anim_frame_time: f32,
    // breath cycle
    //pub breath_cycle: f32,
    // roation
    pub rot: f32,
    // jump variables
    pub in_jump: bool,
    pub jump_duration: f32,
    pub jump_lift_time: f32,
    pub jump_lift_time_min: f32,
    pub jump_lift_time_max: f32,
    // stand/fall status
    pub since_stand: f32,
    pub since_move: f32,
    pub in_fall: bool,
    pub fall_anim_dir: i32,
    pub in_walk: bool,
    pub in_idle: bool,
    // meow status
    pub since_meow: f32,
    // area status
    pub in_exit: bool,
    pub in_portal: bool,
    pub exit_id: i32,
    pub portal_id: i32,
    pub since_warp: f32,
    // character status
    pub speed_level: i32,
    // input
    pub input_enabled: bool,
    // animation history
    pub frame_history: Vec::<AnimSnapshotInfo>,    
}
impl Component for CharacterDisplayComponent {
    type Storage = DenseVecStorage<Self>;
}

impl CharacterDisplayComponent {
    pub fn new(ctx: &mut Context, char_img: &String, player_char: PlayerCharacter) -> CharacterDisplayComponent {
        //let image = Image::new(ctx, char_img.clone()).unwrap();

        CharacterDisplayComponent {
            player_number: 0,
            is_controlled: true,
            is_controllable: true,
            //image: image,
            spritesheet_path: char_img.clone(),
            spritesheet_cols: match &player_char {
                PlayerCharacter::Suri => 10.0,
                PlayerCharacter::Milo => 8.0,
            },
            spritesheet_rows: match &player_char {
                PlayerCharacter::Suri => 10.0,
                PlayerCharacter::Milo => 10.0,
            },
            player_char: player_char,
            going_left: false,
            going_right: false,
            going_up: false,
            going_down: false,
            meowing: false,
            facing_right: true,
            //anim_state: AnimState::Fall(0,0.0),
            anim_frame: 3,
            anim_set: JUMP_SET,
            anim_frame_time: 0.0,
            //breath_cycle: 0.0,
            rot: 0.0,
            in_jump: false,
            jump_duration: 0.0,
            jump_lift_time: 0.125,
            jump_lift_time_min: 0.125,
            jump_lift_time_max: 0.3,
            since_stand: 0.5,
            since_move: 0.0,
            in_fall: true,
            fall_anim_dir: 1,
            in_walk: false,
            in_idle: false,
            since_meow: 1.0,
            in_exit: false,
            in_portal: false,
            exit_id: -1,
            portal_id: -1,
            since_warp: 0.0,
            speed_level: 0,
            input_enabled: true,
            frame_history: vec![],
        }
    }

    /* ANIMATION STATES =============================================== */

    pub fn clear_anim_state(&mut self) {
        self.in_jump = false;
        self.in_fall = false;
        self.in_walk = false;
        self.in_idle = false;
        //self.anim_state = AnimState::Fall(0,0.0);
    }

    // JUMP ------------------------------

    pub fn start_jump(&mut self) {
        self.anim_set = JUMP_SET;
        self.anim_frame = 0;
        self.anim_frame_time = 0.0;
        self.clear_anim_state();
        self.in_jump = true;
        self.jump_duration = 0.0;
        self.since_stand = 50.0;
        if self.jump_lift_time < self.jump_lift_time_min {
            self.jump_lift_time = self.jump_lift_time_min;             
        }
    }

    pub fn process_jump(&mut self, time_delta: f32) {
        if self.jump_duration < self.jump_lift_time {
            self.jump_duration += time_delta;                
            //println!("In jump! {}", &self.jump_duration);
        }
        else {
            //self.start_fall();
            self.going_up = false;
            
            // self.in_jump = false;
            // self.jump_duration = 0.0;
            // self.since_stand = 0.0;
            //println!("Start fall! {}", &self.jump_duration);
        }
    }

    fn process_jump_animation(&mut self, _body_movement: na::Vector2::<f32>, time_delta: f32) {
        self.anim_frame_time += time_delta * 10.0;

        if self.anim_frame_time > 2.0 {
            self.anim_frame += 1;
            if self.anim_frame > JUMP_FRAMES - 1 {
                self.start_fall();
                self.anim_frame = JUMP_FRAMES - 1;
                //self.anim_set = 2;
                //self.anim_frame = 3;
                //self.fall_anim_dir = -1;
            }
            self.anim_frame_time = 0.0;
        }
    }


    // FALL ------------------------------
    pub fn start_fall(&mut self) {

        if self.in_jump {
            //self.anim_frame = 3 - (self.anim_frame % 4);
            self.fall_anim_dir = 1;
        }
        else {
            self.anim_frame = 6;
            self.fall_anim_dir = -1;
        }
        self.anim_set = JUMP_SET;
        self.anim_frame_time = 0.0;
        self.clear_anim_state();
        self.in_fall = true;
        self.jump_duration = 0.0;
        //self.since_stand = 0.0;
    }

    pub fn process_fall(&mut self, time_delta: f32) {
        
        self.jump_duration += time_delta;
        self.since_stand += time_delta;

        if self.going_up && self.recent_stand() {
            self.start_jump();
        }
        else {
            //self.going_up = false;
        }
    }

    fn process_fall_animation(&mut self, _body_movement: na::Vector2::<f32>, time_delta: f32) {
        self.anim_frame_time += time_delta * 10.0;

        if self.fall_anim_dir < 0 {
            if self.anim_frame_time > 1.3 {
                self.anim_frame -= 1;
                if self.anim_frame <= 3 {
                    self.anim_frame = 3;
                    self.fall_anim_dir = 1;
                }
                self.anim_frame_time = 0.0;
            }
        }
        else {
            if self.anim_frame_time > 2.0 {
                self.anim_frame += 1;
                if self.anim_frame >= JUMP_FRAMES - 1 {
                    self.anim_frame = JUMP_FRAMES - 1;
                    //self.fall_anim_dir = -1;
                }
                self.anim_frame_time = 0.0;
            }
        }
    }



    // STAND ------------------------------
    pub fn start_walk(&mut self) {
        self.anim_set = WALK_SET;
        self.anim_frame = 0;
        self.anim_frame_time = 0.0;
        self.clear_anim_state();
        self.in_walk = true;
        self.jump_duration = 0.0;
        self.since_stand = 0.0;
        //self.jump_lift_time = self.jump_lift_time_min;
    }

    pub fn process_walk(&mut self, time_delta: f32) {
        if self.going_up {
            if self.jump_duration >= 0.0 && self.recent_stand() {
                //println!("Start jump! {}", &self.jump_duration);
                self.jump_lift_time *= 1.33;
                if self.jump_lift_time > self.jump_lift_time_max {
                    self.jump_lift_time = self.jump_lift_time_max;
                }
                self.start_jump();
                return;
            }
            // else {
            //     //println!("Can't start jump! {}", &self.jump_duration);
            //     self.going_up = false;
            //     // if self.recent_stand() == false {
            //     //     self.jump_lift_time = self.jump_lift_time_min;
            //     // }
            // }
        }
        
        {
            self.going_up = false;
            if self.jump_duration >= 0.0 {
                self.jump_lift_time = 0.0;
            }
        }
    }

    pub fn process_walk_overhead(&mut self, time_delta: f32) {
        if !self.in_walk && !self.in_idle {
            self.start_walk();
        }
    }

    fn process_walk_animation(&mut self, body_movement: na::Vector2::<f32>, time_delta: f32, facing_right: bool, level_type: &LevelType) {
        self.anim_set = WALK_SET;
            self.anim_frame = self.anim_frame % WALK_FRAMES;
            let move_anim_amt = match level_type {
                LevelType::Platformer => {
                    0.5 * body_movement.x.abs().max(2.0).min(30.0)
                },
                LevelType::Overhead => {
                    0.5 * (body_movement.x.abs() + body_movement.y.abs()).max(2.0).min(30.0)
                }
            };
            self.anim_frame_time += time_delta * 10.0 * move_anim_amt;
            if self.anim_frame_time > 1.5 {
                self.anim_frame_time = 0.0;
                // flip animation direction if going against facing direction
                if (body_movement.x < 0.0 && facing_right) || 
                    (body_movement.x > 0.0 && !facing_right) {
                    // Advance frame backward
                    if self.anim_frame == 0 {
                        self.anim_frame = WALK_FRAMES - 1;
                    }
                    else {
                        self.anim_frame -= 1;
                    }
                }
                else {
                    // Advance frame forward
                    self.anim_frame += 1;
                    if self.anim_frame > WALK_FRAMES - 1 {
                        self.anim_frame = 0;
                    }
                }
            }
    }    

    // IDLE -----------------------------------------------

    pub fn start_idle(&mut self) {
        self.anim_set = IDLE_SET;
        self.anim_frame = 0;
        self.anim_frame_time = 0.0;
        self.clear_anim_state();
        self.in_idle = true;
        self.jump_duration = 0.0;
        self.since_stand = 0.0;
        self.fall_anim_dir = 1;
        self.jump_lift_time = self.jump_lift_time_min;
    }

    pub fn process_idle(&mut self, time_delta: f32) {
        if self.going_up {
            if self.jump_duration >= 0.0 && self.recent_stand() {
                //println!("Start jump! {}", &self.jump_duration);
                // self.jump_lift_time += 0.25;
                // if self.jump_lift_time > self.jump_lift_time_max {
                //     self.jump_lift_time = self.jump_lift_time_max;
                // }
                self.jump_lift_time = 0.0;
                self.start_jump();

            }
            else {
                //println!("Can't start jump! {}", &self.jump_duration);
                self.going_up = false;
                // if self.recent_stand() == false {
                //     self.jump_lift_time = self.jump_lift_time_min;
                // }
            }
        }
    }


    fn process_idle_animation(&mut self, _body_movement: na::Vector2::<f32>, time_delta: f32, _facing_right: bool, _level_type: &LevelType) {
        let mut rng = thread_rng();

        self.since_move += time_delta;
        // After a wait time, go into idle animation
        if self.since_move >= 1.0 {
            // If just starting idle, pick idle or sit animation set
            if self.anim_set != IDLE_SET && self.anim_set != SIT_SET {
                self.start_idle();
                if rng.gen::<f32>() < 0.66 {
                    self.anim_set = IDLE_SET;
                }
                else {
                    self.anim_set = SIT_SET;
                }
            }

            //self.anim_frame = self.anim_frame % 5;
            self.anim_frame_time += time_delta * 10.0;

            // time to advance frame
            if self.anim_frame_time > 2.0 {
                if self.anim_set == SIT_SET && self.fall_anim_dir == -1 {
                    if self.anim_frame > 0 {
                        self.anim_frame -= 1;
                    }
                    else {
                        self.anim_set = IDLE_SET;
                        self.fall_anim_dir = 1;
                    }
                    // random frame time within range 2.0 to 3.0
                    self.anim_frame_time = rng.gen::<f32>() * -1.0;
                }
                else {
                    self.anim_frame += 1;
                    // random frame time within range 2.0 to 5.5
                    self.anim_frame_time = rng.gen::<f32>() * -3.5;
                }
                
                // Maybe skip blink frame
                if self.anim_set == IDLE_SET && self.anim_frame == 2 {
                    if rng.gen::<f32>() > 0.4 {
                        self.anim_frame += 1;
                    }
                }
                // wrap around idle animation
                if self.anim_set == IDLE_SET && self.anim_frame > IDLE_FRAMES - 1 {
                    self.anim_frame = 0;
                    // Maybe go into sit
                    if rng.gen::<f32>() > 0.95 {
                        self.anim_set = SIT_SET;
                        //self.since_move = 1.0; // "reset" since move after idle switch
                    }
                }
                else if self.anim_set == SIT_SET && self.anim_frame > SIT_FRAMES - 1 {
                    self.anim_frame = SIT_FRAMES - 1;
                    self.anim_frame_time = rng.gen::<f32>() * -7.5;

                    if rng.gen::<f32>() > 0.95 {
                        self.fall_anim_dir = -1;
                        //self.since_move = 1.0; // "reset" since move after idle switch
                    }
                }
            }

        }
    }    

    pub fn recent_stand(&self) -> bool {
        // if actually standing, or since stand counter is low
        (!self.in_jump && !self.in_fall) || self.since_stand < 0.15
    }

    pub fn update(&mut self, body_movement: na::Vector2::<f32>, time_delta: f32) {

        self.since_meow += time_delta;
        if self.meowing {
            if self.since_meow < 0.15 { //0.35
                self.meowing = false;
            }
        }
        self.since_warp += time_delta;

        // Handle going up based on state
        if self.in_jump {
            self.process_jump(time_delta);
        }
        else if self.in_fall {
            self.process_fall(time_delta);
        }
        else if self.in_walk {
            self.process_walk(time_delta);
        }
        else if self.in_idle {
            self.process_idle(time_delta);
        }


        //self.interp_breath(0.08);

        self.update_animation(body_movement, time_delta, &LevelType::Platformer);

        while self.frame_history.len() >= 50 {
            self.frame_history.remove(0);
        }
        self.frame_history.push(AnimSnapshotInfo {
            frame_num: self.anim_frame,
            anim_set: self.anim_set,
            facing_right: self.facing_right
        });
                

        //self.apply_inputs(coll);
    }

    pub fn update_overhead(&mut self, body_movement: na::Vector2::<f32>, time_delta: f32) {

        self.since_meow += time_delta;
        if self.meowing {
            if self.since_meow < 0.15 { //0.35
                self.meowing = false;
            }
        }
        self.since_warp += time_delta;

        // Handle going up based on state
        self.process_walk_overhead(time_delta);

        //self.interp_breath(0.08);

        self.update_animation(body_movement, time_delta, &LevelType::Overhead);


        //self.apply_inputs(coll);

        while self.frame_history.len() >= 50 {
            self.frame_history.remove(0);
        }
        self.frame_history.push(AnimSnapshotInfo {
            frame_num: self.anim_frame,
            anim_set: self.anim_set,
            facing_right: self.facing_right
        });

    }

    fn process_facing_moving(&mut self, body_movement: na::Vector2::<f32>, time_delta: f32) -> (bool, bool) {
        let mut is_moving = false;
        if self.going_left {
            self.facing_right = false;
            is_moving = true;
        }
        else if self.going_right {
            self.facing_right = true;
            is_moving = true;
        }
        if is_moving {
            self.since_move = 0.0;
        }

        (self.facing_right, is_moving)
    }

    fn process_in_move_anim(&mut self, body_movement: na::Vector2::<f32>, time_delta: f32, is_moving: bool, level_type: &LevelType) -> bool {
        match level_type {
            LevelType::Platformer => {
                is_moving || body_movement.x.abs() > 0.5
            },
            LevelType::Overhead => {
                is_moving || body_movement.x.abs() > 0.5 || body_movement.y.abs() > 0.5
            }
        }
    }




    fn update_animation(&mut self, body_movement: na::Vector2::<f32>, time_delta: f32, level_type: &LevelType) {

        let (facing_right, is_moving) = self.process_facing_moving(body_movement, time_delta);
        //let anim_moving = self.process_in_move_anim(body_movement, time_delta, is_moving);

        match self {
            Self { in_jump: true , .. } => {
                self.process_jump_animation(body_movement, time_delta);
            },
            Self { in_fall: true , .. } => {
                self.process_fall_animation(body_movement, time_delta);
            },
            Self { .. } => {
                match self.process_in_move_anim(body_movement, time_delta, is_moving, level_type) {
                    true => {
                        self.process_walk_animation(body_movement, time_delta, facing_right, level_type);
                    },
                    false => {
                        self.process_idle_animation(body_movement, time_delta, facing_right, level_type);
                    }
                }

            }
        }

        // // JUMP/FALL ANIMATION
        // if self.in_jump || self.in_fall {            

        //     match self.in_jump {
        //         true => {
        //             self.process_jump_animation(body_movement, time_delta);
        //         },
        //         false => {
        //             self.process_fall_animation(body_movement, time_delta);
        //         }
        //     }

        // }
        // // WALKING ANIMATION
        // else if self.process_in_move_anim(body_movement, time_delta, is_moving) { // is_moving || body_movement.x.abs() > 0.5
        //     self.process_walk_animation(body_movement, time_delta, facing_right);
        // }
        // // IDLE ANIMATION
        // else {
        //     self.process_idle_animation(body_movement, time_delta, facing_right);

        // }

    }

    // pub fn interp_breath(&mut self, cycle_speed: f32) {
    //     let two_pi = 2.0*3.14159;

    //     if !self.going_left && !self.going_right && !self.going_up && !self.going_down {
    //         self.breath_cycle += cycle_speed;
    //         if self.breath_cycle >= two_pi {
    //             self.breath_cycle -= two_pi;
    //         }
    //     }
    //     else {
    //         self.breath_cycle = 0.0;
    //     }

    //     // self.rot += 0.01;
    //     // if self.rot >= two_pi {
    //     //     self.rot -= two_pi;
    //     // }
    // }

    pub fn scale_by_speed_level(&self, base: f32) -> f32 {
        match self.speed_level {
            3 => base * 1.2,
            2 => base * 1.1,
            1 => base * 1.0,
            _ => base * 0.9,
        }
    }

    pub fn get_base_x_move_amount(&self) -> f32 {
        let base = match self.player_char {
            PlayerCharacter::Suri => 10.0, //15.0,
            PlayerCharacter::Milo => 16.0, //20.0,
        };
        self.scale_by_speed_level(base)
    }

    pub fn get_base_y_move_amount(&self) -> f32 {
        let base = match self.player_char {
            PlayerCharacter::Suri => 15.0, //15.0,
            PlayerCharacter::Milo => 24.0, //20.0,
        };
        self.scale_by_speed_level(base)
    }


    pub fn apply_movement(&mut self, body: &mut physics::PhysicsBody, time_delta: f32, level_type: &LevelType) {
        let x_move_amt = self.get_base_x_move_amount(); //1300.0;
        let y_move_amt = self.get_base_y_move_amount(); //1300.0;
        let up_mult = 3.0;
        let mut lin_vel = body.linear_velocity().clone();
        let decr_amt = (1.0 - (1.0 * time_delta)).max(0.0);
        let loc_cent = body.local_center().clone();
        if self.going_right {
            //let new_lin_vel = physics::create_pos(&Point2::new(self.vel.x, self.vel.y));

            match level_type {
                LevelType::Platformer => {
                    if lin_vel.x < self.scale_by_speed_level(12.0) {
                        //body.apply_force_to_center(&physics::PhysicsVec {x:move_amt,y: 0.0}, true);
                        
                        body.apply_linear_impulse(&physics::PhysicsVec {x:x_move_amt * time_delta,y: 0.0}, &loc_cent, true);
                        //body.apply_force_to_center(&physics::PhysicsVec {x:move_amt,y: 0.0}, true);
                    }
                    
                    //println!("applied right force");
                },
                LevelType::Overhead => {
                    if lin_vel.x < self.scale_by_speed_level(5.0) {
                        //body.apply_force_to_center(&physics::PhysicsVec {x:move_amt * 0.5 * time_delta,y: 0.0}, true);
                        body.apply_linear_impulse(&physics::PhysicsVec {x:x_move_amt * time_delta,y: 0.0}, &loc_cent, true);
                    }
                },
            }
        }
        else if self.going_left {
            //let new_lin_vel = physics::create_pos(&Point2::new(self.vel.x, self.vel.y));
            match level_type {
                LevelType::Platformer => {
                    if lin_vel.x > self.scale_by_speed_level(-12.0) {

                        //let loc_cent = body.local_center().clone();
                        body.apply_linear_impulse(&physics::PhysicsVec {x:-x_move_amt * time_delta,y: 0.0}, &loc_cent, true);
                        //body.apply_force_to_center(&physics::PhysicsVec {x:-move_amt,y: 0.0}, true);
                    }
                        //println!("applied left force");
                },
                LevelType::Overhead => {
                    if lin_vel.x > self.scale_by_speed_level(-5.0) {
                        //body.apply_force_to_center(&physics::PhysicsVec {x:-move_amt * 0.5 * time_delta,y: 0.0}, true);
                        body.apply_linear_impulse(&physics::PhysicsVec {x:-x_move_amt * time_delta,y: 0.0}, &loc_cent, true);
                    }
                }
            }
        }
        else {
            match level_type {
                LevelType::Platformer => {},
                LevelType::Overhead => {
                    lin_vel.x = lin_vel.x * decr_amt;
                    body.set_linear_velocity(&physics::PhysicsVec { x: lin_vel.x, y: lin_vel.y });
                }
            }
        }
        if self.going_up {
            //let new_lin_vel = physics::create_pos(&Point2::new(self.vel.x, self.vel.y));
            match level_type {
                LevelType::Platformer => {
                    if body.linear_velocity().y > self.scale_by_speed_level(-12.0) && self.in_jump {
                        //body.apply_force_to_center(&physics::PhysicsVec {x:0.0,y: -up_mult * move_amt * time_delta}, true);
                        body.apply_linear_impulse(&physics::PhysicsVec {x:0.0,y: -up_mult * y_move_amt * time_delta}, &loc_cent, true);
                    }
                    else if body.linear_velocity().y > self.scale_by_speed_level(12.0) && self.in_fall {
                        //body.apply_force_to_center(&physics::PhysicsVec {x:0.0,y: -move_amt * 0.25 * time_delta}, true);
                        body.apply_linear_impulse(&physics::PhysicsVec {x:0.0,y: -y_move_amt * 0.25 * time_delta}, &loc_cent, true);
                    }
                },
                LevelType::Overhead => {
                    if lin_vel.y > self.scale_by_speed_level(-5.0) {
                        //body.apply_force_to_center(&physics::PhysicsVec {x:0.0,y: -move_amt * time_delta}, true);
                        body.apply_linear_impulse(&physics::PhysicsVec {x:0.0,y: -x_move_amt * time_delta}, &loc_cent, true);
                    }
                }
            }
            //println!("applied up force");
        }
        else if self.going_down {
            //let new_lin_vel = physics::create_pos(&Point2::new(self.vel.x, self.vel.y));
            match level_type {
                LevelType::Platformer => {
                    //body.apply_force_to_center(&physics::PhysicsVec {x:0.0,y: move_amt * time_delta}, true);
                    body.apply_linear_impulse(&physics::PhysicsVec {x:0.0,y: y_move_amt * time_delta}, &loc_cent, true);
                },
                LevelType::Overhead => {
                    if lin_vel.y < self.scale_by_speed_level(5.0) {
                        //body.apply_force_to_center(&physics::PhysicsVec {x:0.0,y: move_amt * time_delta}, true);
                        body.apply_linear_impulse(&physics::PhysicsVec {x:0.0,y: x_move_amt * time_delta}, &loc_cent, true);
                    }
                }
            }

            //println!("applied down force");
        }
        else {
            match level_type {
                LevelType::Platformer => {},
                LevelType::Overhead => {
                    lin_vel = body.linear_velocity().clone();
                    lin_vel.y = lin_vel.y * decr_amt;
                    body.set_linear_velocity(&physics::PhysicsVec { x: lin_vel.x, y: lin_vel.y });
                }
            }
        }
    }

    pub fn apply_movement_new(&mut self, body: &mut physics::PhysicsBody, time_delta: f32) {
        let mut lateral_move_amt = 15.0; // 15.0;
        let mut vertical_move_amt = 75.0; //75 // 45.0;
        //let mut move_amt = 15.0; //1300.0;
        //let up_mult = 3.0;
        let x_vel = body.linear_velocity().x;
        let y_vel = body.linear_velocity().y;

        if self.recent_stand() == false {
            lateral_move_amt *= 0.5;
        }

        // if self.jump_duration > 0.33 * self.jump_lift_time {
        //     vertical_move_amt *= 0.66;
        // }
        // if self.jump_duration > 0.66 * self.jump_lift_time {
        //     vertical_move_amt *= 0.33;
        // }

        if self.going_right {
            //let new_lin_vel = physics::create_pos(&Point2::new(self.vel.x, self.vel.y));
            if x_vel < 12.0 {
                body.apply_force_to_center(&physics::PhysicsVec {x:lateral_move_amt,y: 0.0}, true);
            }
            
            //println!("applied right force");
        }
        if self.going_left {
            //let new_lin_vel = physics::create_pos(&Point2::new(self.vel.x, self.vel.y));
            if x_vel > -12.0 {
                body.apply_force_to_center(&physics::PhysicsVec {x:-lateral_move_amt,y: 0.0}, true);
            }
                //println!("applied left force");
        }
        if self.going_up {
            //let new_lin_vel = physics::create_pos(&Point2::new(self.vel.x, self.vel.y));
            //if body.linear_velocity().y > -25.0 {
                //body.apply_force_to_center(&physics::PhysicsVec {x:0.0,y: -vertical_move_amt - y_vel * 0.1}, true);
                body.set_linear_velocity(&physics::PhysicsVec {x:x_vel,y: -10.0
                    + 5.0 * (1.0 - self.jump_duration * 10.0).max(0.0)
                    + ((-5.0 + self.jump_duration * 10.0) * 2.0).max(0.0).min(9.9)});
            //}
            //println!("applied up force");
        }
        if self.going_down {
            if y_vel < 0.0 {
                body.set_linear_velocity(&physics::PhysicsVec {x: x_vel, y: y_vel + 5.0 * time_delta});
            }
            //let new_lin_vel = physics::create_pos(&Point2::new(self.vel.x, self.vel.y));
            body.apply_force_to_center(&physics::PhysicsVec {x:0.0,y: 0.5 * vertical_move_amt}, true);
            //println!("applied down force");
        }
    }


    // pub fn apply_inputs(&mut self, coll: &mut Collision) {
    //     let mut vel = coll.vel;
    //     // Single axis vector length
    //     let mut vec_amt = 300.0;
    //     // IS the Input direction Multi-axis, i.e. UP + RIGHT is multi-axis
    //     let multi_axis = (self.going_left && (self.going_up || self.going_down))
    //         || (self.going_right && (self.going_up || self.going_down));
    //     // reduce vector length with two axis
    //     if multi_axis {
    //         vec_amt = 0.71 * vec_amt;
    //     }
    //     vec_amt = vec_amt;// * 0.05;
    //     // Apply vector length to velocity X
    //     if self.going_left {
    //         vel.x -= vec_amt;
    //     }
    //     else if self.going_right {
    //         vel.x += vec_amt;
    //     }
    //     else {
    //         //vel.x = vel.x * 0.995;
    //     }
    //     // Apply vector length to velocity Y
    //     if self.going_up {
    //         vel.y -= vec_amt * 1.5;
    //     }
    //     else if self.going_down {
    //         vel.y += vec_amt;
    //     }
    //     else {
    //         // if vel.y < 0.0 {
    //         //     vel.y = vel.y * 0.98;
    //         // }
    //         //vel.x = vel.x * 0.995;
    //     }

    //     //vel.x = vel.x.max(-70.0).min(70.0);
    //     //vel.y = vel.y.max(-80.0);
    // }

    pub fn get_spritesheet_frame(&self, frame_num: u32, set_num: u32)
        -> (f32, f32, f32, f32) {

        let ss_cells = self.spritesheet_cols;
        let ss_rows = self.spritesheet_rows;
        let src_x = 0.0 + (frame_num as f32) / ss_cells;
        let src_y = 0.0 + (set_num as f32) / ss_rows;
        let (src_w, src_h) = (1.0 / ss_cells, 1.0 / ss_rows);

        (src_x, src_y, src_w, src_h)
    }

    

}

impl super::RenderItemTarget for CharacterDisplayComponent {

    fn render_item(game_state: &GameState, ctx: &mut Context, entity: &Entity,
        pos: &na::Point2<f32>, item_index: usize) {
            let world = &game_state.world;
            let character_reader = world.read_storage::<CharacterDisplayComponent>();

            // Get Sprite Component to call draw method            
            if let Some(character) = character_reader.get(entity.clone()) {
                use crate::components::{RenderTrait};
                character.draw(ctx, world, Some(entity.id()), pos.clone(), item_index);
            }
        }
}

impl super::CharLevelInteractor for CharacterDisplayComponent {
    fn set_standing(&mut self, is_standing: bool) {
        match is_standing {
            true => {
                if self.in_jump || self.in_fall {
                    self.start_walk();
                }
            },
            false => {
                if !self.in_jump && !self.in_fall {
                    self.start_fall();
                }
            }
        }
    }
}

impl PhysicsUpdateTrait for CharacterDisplayComponent {
    fn pre_physics_update(&mut self, world: &World, physics_world: &mut PhysicsWorld, time_delta: f32, 
        opt_collision: &mut Option<&mut Collision>,
        opt_character: &mut Option<&mut CharacterDisplayComponent>,
        opt_npc: &mut Option<&mut NpcComponent>,
        //level_bounds: &LevelBounds,
        //game_state: &GameStateResource,
        entity: &Entity) {

        //println!("Pre Physics update {:?}", &entity);

    }

    fn post_physics_update(&mut self, world: &World, physics_world: &mut PhysicsWorld, time_delta: f32, 
        opt_collision: &mut Option<&mut Collision>,
        opt_character: &mut Option<&mut CharacterDisplayComponent>,
        opt_npc: &mut Option<&mut NpcComponent>,
        //game_state: &GameStateResource,
        entity: &Entity) {
        
        //println!("Post Physics update {:?}", &entity);
        // println!(" Borrow Particle Sys {:?}", &entity);
        // let mut particle_sys_res = world.write_storage::<ParticleSysComponent>();
        //println!(" Borrow GameStateRes {:?}", &entity);
        let mut particle_sys_res = world.write_storage::<ParticleSysComponent>();
        //let mut game_state_res = world.fetch_mut::<GameStateResource>();
        //if let Some() = world.get_mut::<ParticleSysComponent>();
        //println!(" Get Particle Sys ref {:?}", &entity);
        if let Some(mut particle_sys) = particle_sys_res.get_mut(*entity) {

            if let Some(ref mut collision) = opt_collision {
                let coll_velx = collision.vel.x; //get_avg_x(5);
                let coll_vely = collision.vel.y; //get_avg_y(5);
                particle_sys.world_vel.0 = coll_velx * 5.0;
                particle_sys.world_vel.1 = coll_vely * 5.0;
            }


            particle_sys.set_logic_value( match (self.in_walk || self.in_idle) && self.since_move < 0.05 {
                false => false,
                true => true,
            });
            
            //println!("Post Physics update {:?} Particle Sys Viz: {}", &entity, &particle_sys.visible);
            //game_state_res.points = game_state_res.points + 1;
        }

    }
}

impl super::RenderTrait for CharacterDisplayComponent {
    fn draw(&self, ctx: &mut Context, world: &World, ent: Option<u32>, pos: na::Point2::<f32>, item_index: usize) {
        //println!("PlayerRenderTrait drawing...");
        let frame_num : i32 = {
            let game_state_res = world.fetch::<GameStateResource>();

            // let gs = &*game_state_res;
            // 0.0
            game_state_res.level_frame_num
        };
        let mut rng = rand::thread_rng();
        let mut _draw_ok = true;
        // color part:  ,Color::new(1.0,0.7,0.7,1.0)
        let texture_scale = 1.5;//self.breath_cycle.cos() * 0.02;
        let mut angle = 0.0;
        let mut pos_history = Vec::<Point2::<f32>>::new();
        let mut trail_frame_history = Vec::<AnimSnapshotInfo>::new();
        //let mut trail_set_history = Vec::<u32>::new();
        if let Some(ent_id) = ent {
            let collision_reader = world.read_storage::<Collision>();
            let entity = world.entities().entity(ent_id);
            if let Some(coll) = collision_reader.get(entity) {
                angle = coll.angle;

                if coll.pos_history.len() > 0 {
                    let mut hist_index = 0;
                    let trans_frame_num = (frame_num as f32 * 0.66).round() as i32;
                    let every_x_frames = 7;
                    for pos in coll.pos_history.iter() {
                        if (-trans_frame_num - hist_index) % every_x_frames == 0 {
                            pos_history.push(pos.clone());
                        }                    
                        hist_index += 1;
                    }
                    hist_index = 0;
                    for anim_frame_info in self.frame_history.iter() {
                        if (-trans_frame_num - hist_index) % every_x_frames == 0 {
                            trail_frame_history.push(anim_frame_info.clone());
                        }                    
                        hist_index += 1;
                    }
                    /*hist_index = 0;
                    for set_num in self.anim_set_history.iter() {
                        if (-trans_frame_num - hist_index) % every_x_frames == 0 {
                            trail_set_history.push(*set_num);
                        }                    
                        hist_index += 1;
                    }*/
                }
            }

        }

        let draw_pos = na::Point2::<f32>::new(pos.x, pos.y);
    
        let mut shader_res = world.fetch_mut::<ShaderResources>();
        let mut image_resources = world.fetch_mut::<ImageResources>();

        let exhaust_radius = 27.0;
        let self_rot = self.rot;
        let texture_green = if self.jump_duration <= 0.0 {
            0.0
        } else if self.jump_duration > 1.0 {
            1.0
        } else {
            self.jump_duration
        };

        // if let Ok(rect) = graphics::Mesh::new_circle(
        //     ctx,
        //     graphics::DrawMode::fill(),
        //     na::Point2::new(0.0,0.0),
        //     3.0, 0.7,
        //     graphics::Color::new(1.0,texture_green,1.0,1.0),
        // ) {
            

        {


            
            // Draw spritesheet texture
            let image_ref = image_resources.image_ref(self.spritesheet_path.clone());
            if let Ok(image) = image_ref {
                // Get starting x/y in spritesheet space (0.0-1.0,0.0-1.0)
                
                // let ss_cells = self.spritesheet_cols;
                // let ss_rows = self.spritesheet_rows;
                // let src_x = 0.0 + (self.anim_frame as f32) / ss_cells;
                // let src_y = 0.0 + (self.anim_set as f32) / ss_rows;
                // let (src_w, src_h) = (1.0 / ss_cells, 1.0 / ss_rows);

                if self.speed_level >= 2 {
                    let num_frames = pos_history.len().min(trail_frame_history.len());
                    let mut frame_num = 0;
                    for frame_num in 0..num_frames {
                        let pos_frame = pos_history.get(frame_num).unwrap();
                        let trail_anim_info = trail_frame_history.get(frame_num).unwrap();
                        let trail_frame_num = trail_anim_info.frame_num;
                        let trail_anim_set = trail_anim_info.anim_set;
                        let frame_alpha = (frame_num + 1) as f32 / (num_frames + 1) as f32;
                        // Mirror X-scale if facing left
                        let mut x_scale = texture_scale;
                        if trail_anim_info.facing_right == false {
                            x_scale = -x_scale;
                        }

                        let (src_x, src_y, src_w, src_h) = self.get_spritesheet_frame(trail_frame_num, trail_anim_set);
                        
                        if let Err(_) = ggez::graphics::draw(ctx, image, 
                            // Setup draw parameters
                            DrawParam::default()
                            .src(Rect::new(src_x,src_y,src_w,src_h)) // set texture source rectangle
                            .dest(na::Point2::new(pos_frame.x, pos_frame.y - 10.0)) // world space location for texture
                            .scale(na::Vector2::new(x_scale,texture_scale)) // set draw scale,including x-mirroring
                            .offset(na::Point2::new(0.5,0.5)) // set anchor point at middle of image rect
                            .rotation(angle) // would rotate if altered
                            .color(Color::new(1.0, 1.0, 1.0, frame_alpha))
                        ) {
                            _draw_ok = false;
                        }
                        //frame_num += 1;
                    }
                }

                let (src_x, src_y, src_w, src_h) = self.get_spritesheet_frame(self.anim_frame, self.anim_set);
                // Mirror X-scale if facing left
                let mut x_scale = texture_scale;
                if self.facing_right == false {
                    x_scale = -x_scale;
                }

                // Use shader if needed
                let mut _lock : Option<ggez::graphics::ShaderLock> = None;
                /* if self.in_idle {
                    if let Ok(shader_ref) = shader_res.shader_ref("suri_shader".to_string()) {
                        _lock = Some(ggez::graphics::use_shader(ctx, shader_ref));
                    }
                }
                else */
                {
                    match &self.player_char {
                        PlayerCharacter::Suri => {
                            if let Ok(shader_ref) = shader_res.shader_ref("suri_shadow".to_string()) {
                                _lock = Some(ggez::graphics::use_shader(ctx, shader_ref));
                            }
                        },
                        PlayerCharacter::Milo => {
                            if let Ok(shader_ref) = shader_res.shader_ref("milo_shadow".to_string()) {
                                _lock = Some(ggez::graphics::use_shader(ctx, shader_ref));
                            }
                        },
                    }
                    
                }

                let texture_position = na::Point2::new(draw_pos.x , draw_pos.y - 10.0);
                if let Err(_) = ggez::graphics::draw(ctx, image, 
                    // Setup draw parameters
                    DrawParam::default()
                    .src(Rect::new(src_x,src_y,src_w,src_h)) // set texture source rectangle
                    .dest(texture_position) // world space location for texture
                    .scale(na::Vector2::new(x_scale,texture_scale)) // set draw scale,including x-mirroring
                    .offset(na::Point2::new(0.5,0.5)) // set anchor point at middle of image rect
                    .rotation(angle) // would rotate if altered
                ) {
                    _draw_ok = false;
                }
            }                    

        }

        if let Some(ent_id) = ent {
            //println!("Before ParticleSys borrow");
            let psys_reader = world.read_storage::<ParticleSysComponent>();
            //println!("After ParticleSys borrow");
            let entity = world.entities().entity(ent_id);
            //println!("Before Psys Get from Entity");
            if let Some(psys) = psys_reader.get(entity) {
                //println!("Before Draw Psys");
                //psys.draw(ctx, world, Some(ent_id), pos.clone(), 0);
                //println!("After Draw Psys");
            }

        }

        //}

    }
}




// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    //world.register::<PlayerComponent>();
    world.register::<CharacterDisplayComponent>();
}



            // if let Ok(rect) = graphics::Mesh::new_rectangle(
            //     ctx,
            //     graphics::DrawMode::stroke(1.0),
            //     graphics::Rect::from([-18.0,-18.0,18.0,18.0]),
            //     Color::new(1.0,0.0,0.0,1.0),
            // ) {
            //     if let Err(_) = graphics::draw(ctx, &rect, 
            //         DrawParam::default()
            //         .dest(na::Point2::new(pos.x, pos.y))
            //         //.scale(na::Vector2::new(x_scale, breath_scale))
            //         //.offset(na::Point2::new(0.5,0.5))                
            //     ) {
                    
            //     };  
            // }

            // if self.since_meow < 0.25 {
            //     let font = image_resources.font;
            //     let typeText = String::from("*MEOW*");
            //     // match &self.in_fall {
            //     //     true => {
            //     //         typeText.push_str(&format!("FALL {}", &self.anim_frame).to_string());
            //     //     },
            //     //     false => match &self.in_jump {
            //     //         true => {
            //     //             typeText.push_str(&format!("JUMP {}", &self.anim_frame).to_string());
            //     //         },
            //     //         false => {
            //     //             typeText.push_str(&format!("STAND {}", &self.anim_frame).to_string());
            //     //         }                    
            //     //     }
            //     // };
                
            //     let curr_transform = ggez::graphics::transform(ctx);
            //     ggez::graphics::pop_transform(ctx);
            //     ggez::graphics::apply_transformations(ctx);

            //     let mut text = ggez::graphics::Text::new(typeText);
            //     text.set_font(font, ggez::graphics::Scale::uniform(18.0));
            //     let (width, height) = ggez::graphics::size(ctx);
            //     let text_size_x = text.width(ctx) as f32 / 2.0;
            //     let text_size_y = text.height(ctx) as f32 / 2.0;
            //     if let Err(_) = graphics::draw(ctx, &text,
            //         DrawParam::new()
            //         //.dest(na::Point2::new(20.0, 20.0))   //width*0.5-text_size_x+draw_pos.x, height*0.5-text_size_y+draw_pos.y))
            //         .dest(na::Point2::new(width * 0.4995 - text_size_x, height * 0.5495 - text_size_y )) // - text_size_x, draw_pos.y - text_size_y + 28.0))
            //         //.scale(na::Vector2::new(x_scale.abs(), x_scale.abs()))
            //         .color(ggez::graphics::Color::new(0.0, 0.0, 0.0, 1.0))
            //         //.scale(na::Vector2::new(5.0, 5.0))
            //         //.offset(na::Point2::new(text_size_x, text_size_y))
            //         //.scale(na::Vector2::new(x_scale.abs(),x_scale.abs())) 
            //         //.offset(na::Point2::new(text_size_x, text_size_y))
            //     ) {
            //         _draw_ok = false;
            //     }

            //     if let Err(_) = graphics::draw(ctx, &text,
            //         DrawParam::new()
            //         .dest(na::Point2::new(width * 0.5005 - text_size_x, height * 0.5495 - text_size_y )) // - text_size_x, draw_pos.y - text_size_y + 28.0))
            //         .color(ggez::graphics::Color::new(0.0, 0.0, 0.0, 1.0))
            //     ) {
            //         _draw_ok = false;
            //     }

            //     if let Err(_) = graphics::draw(ctx, &text,
            //         DrawParam::new()
            //         .dest(na::Point2::new(width * 0.50 - text_size_x, height * 0.5505 - text_size_y )) // - text_size_x, draw_pos.y - text_size_y + 28.0))
            //         .color(ggez::graphics::Color::new(0.0, 0.0, 0.0, 1.0))
            //     ) {
            //         _draw_ok = false;
            //     }

            //     if let Err(_) = graphics::draw(ctx, &text,
            //         DrawParam::new()
            //         .dest(na::Point2::new(width * 0.5 - text_size_x, height * 0.55 - text_size_y )) // - text_size_x, draw_pos.y - text_size_y + 28.0))
            //         //.color(ggez::graphics::Color::new(0.0, 0.0, 0.0, 1.0))
            //     ) {
            //         _draw_ok = false;
            //     }

            //     ggez::graphics::push_transform(ctx, Some(curr_transform));
            //     ggez::graphics::apply_transformations(ctx);
            // }
            