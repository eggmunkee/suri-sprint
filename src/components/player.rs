use ggez::{Context};
use ggez::graphics;
use ggez::graphics::{Rect,Image,Color,DrawParam,ShaderLock};
use ggez::nalgebra as na;
use specs::{ Component, DenseVecStorage, World, WorldExt };
//use specs::shred::{Dispatcher};
use rand::prelude::*;

//use crate::game_state::{GameState};
use crate::resources::{ImageResources,ShaderResources};
use crate::components::collision::{Collision};
use crate::components::{Velocity};
use crate::physics;
use crate::physics::{PhysicsBody};

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

#[derive(Debug)]
pub struct CharacterDisplayComponent {
    // image path
    pub spritesheet_path: String,
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
    // input
    pub input_enabled: bool,
}
impl Component for CharacterDisplayComponent {
    type Storage = DenseVecStorage<Self>;
}

impl CharacterDisplayComponent {
    pub fn new(ctx: &mut Context, char_img: &String) -> CharacterDisplayComponent {
        //let image = Image::new(ctx, char_img.clone()).unwrap();

        CharacterDisplayComponent {
            //image: image,
            spritesheet_path: char_img.clone(),
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
            input_enabled: true,
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
    }

    pub fn process_jump(&mut self, time_delta: f32) {
        if self.jump_duration < 0.3 {
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
            self.going_up = false;
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
    }

    pub fn process_walk(&mut self, time_delta: f32) {
        if self.going_up {
            if self.jump_duration >= 0.0 && self.recent_stand() {
                //println!("Start jump! {}", &self.jump_duration);
                self.start_jump();
            }
            else {
                //println!("Can't start jump! {}", &self.jump_duration);
                self.going_up = false;
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
    }

    pub fn process_idle(&mut self, time_delta: f32) {
        if self.going_up {
            if self.jump_duration >= 0.0 && self.recent_stand() {
                //println!("Start jump! {}", &self.jump_duration);
                self.start_jump();
            }
            else {
                //println!("Can't start jump! {}", &self.jump_duration);
                self.going_up = false;
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
            if self.since_meow < 0.35 {
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

        self.update_animation(body_movement, time_delta);


        //self.apply_inputs(coll);
    }

    fn update_animation(&mut self, body_movement: na::Vector2::<f32>, time_delta: f32) {
        let mut rng = thread_rng();

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

        // JUMP/FALL ANIMATION
        if self.in_jump || self.in_fall {            

            self.anim_frame_time += time_delta * 10.0;

            match self.in_jump {
                true => {
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
                },
                false => {
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
            }

        }
        // WALKING ANIMATION
        else if is_moving || body_movement.x.abs() > 0.5 {
            self.anim_set = WALK_SET;
            self.anim_frame = self.anim_frame % WALK_FRAMES;
            let move_anim_amt = 0.5 * body_movement.x.abs().max(2.0).min(30.0);
            self.anim_frame_time += time_delta * 10.0 * move_anim_amt;
            if self.anim_frame_time > 1.5 {
                self.anim_frame_time = 0.0;
                // flip animation direction if going against facing direction
                if (body_movement.x < 0.0 && self.facing_right) || 
                    (body_movement.x > 0.0 && !self.facing_right) {
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
        // IDLE ANIMATION
        else {
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


            //self.anim_set = 0;
            //self.anim_frame = 0;
            //self.anim_frame_time = 0.0;
        }

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

    pub fn apply_movement(&mut self, body: &mut physics::PhysicsBody) {
        let move_amt = 15.0; //1300.0;
        let up_mult = 3.0;
        if self.going_right {
            //let new_lin_vel = physics::create_pos(&Point2::new(self.vel.x, self.vel.y));
            if body.linear_velocity().x < 12.0 {
                body.apply_force_to_center(&physics::PhysicsVec {x:move_amt,y: 0.0}, true);
            }
            
            //println!("applied right force");
        }
        if self.going_left {
            //let new_lin_vel = physics::create_pos(&Point2::new(self.vel.x, self.vel.y));
            if body.linear_velocity().x > -12.0 {
                body.apply_force_to_center(&physics::PhysicsVec {x:-move_amt,y: 0.0}, true);
            }
                //println!("applied left force");
        }
        if self.going_up {
            //let new_lin_vel = physics::create_pos(&Point2::new(self.vel.x, self.vel.y));
            if body.linear_velocity().y > -12.0 {
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

    // Update animation status from collision standing status
    pub fn update_body_status(&mut self, is_standing: bool) {

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


impl super::RenderTrait for CharacterDisplayComponent {
    fn draw(&self, ctx: &mut Context, world: &World, ent: Option<u32>, pos: na::Point2::<f32>, item_index: u32) {
        //println!("PlayerRenderTrait drawing...");
        let mut rng = rand::thread_rng();
        let mut _draw_ok = true;
        // color part:  ,Color::new(1.0,0.7,0.7,1.0)
        let texture_scale = 1.5;//self.breath_cycle.cos() * 0.02;
        let mut angle = 0.0;
        if let Some(ent_id) = ent {
            let collision_reader = world.read_storage::<Collision>();
            let entity = world.entities().entity(ent_id);
            if let Some(coll) = collision_reader.get(entity) {
                angle = coll.angle;
            }

        }

        let draw_pos = na::Point2::<f32>::new(pos.x, pos.y);
    
        let mut image_resources = world.fetch_mut::<ImageResources>();

        let exhaust_radius = 27.0;
        let self_rot = self.rot;
        if let Ok(rect) = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(0.0,0.0),
            3.0, 0.7,
            graphics::WHITE,
        ) {
            let col_vals: (u8,) = rng.gen();

            let mut x_scale = texture_scale;
            if self.facing_right == false {
                x_scale = -x_scale;
            }

            {
                let mut shader_res = world.fetch_mut::<ShaderResources>();
                let mut _lock : Option<ggez::graphics::ShaderLock> = None;
                if let Ok(shader_ref) = shader_res.shader_ref("suri_shader".to_string()) {
                    if self.in_idle {
                        _lock = Some(ggez::graphics::use_shader(ctx, shader_ref));
                    }
                
                    let image_ref = image_resources.image_ref(self.spritesheet_path.clone());
                    if let Ok(image) = image_ref {
                        // let w = image.width();
                        // let h = image.height();
        
                        let src_x = 0.0 + 0.1 * (self.anim_frame as f32);
                        let src_y = 0.0 + 0.1 * (self.anim_set as f32);
        
                        // if !self.going_left && !self.going_right {
                        //     src_x = 0.0;
                        // }
                
                        let texture_position = na::Point2::new(draw_pos.x , draw_pos.y - 10.0);
                        if let Err(_) = ggez::graphics::draw(ctx, image, 
                            DrawParam::default()
                            .src(Rect::new(src_x,src_y,0.1,0.1))
                            .dest(texture_position)
                            .scale(na::Vector2::new(x_scale,texture_scale))
                            .offset(na::Point2::new(0.5,0.5))
                            .rotation(angle)
                        ) {
                            //(draw_pos.clone(),)) { // add back x/y pos  //
                            _draw_ok = false;
                        }
                    }                    
                }


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

            if self.since_meow < 0.25 {
                let font = image_resources.font;
                let typeText = String::from("*meow*");
                // match &self.in_fall {
                //     true => {
                //         typeText.push_str(&format!("FALL {}", &self.anim_frame).to_string());
                //     },
                //     false => match &self.in_jump {
                //         true => {
                //             typeText.push_str(&format!("JUMP {}", &self.anim_frame).to_string());
                //         },
                //         false => {
                //             typeText.push_str(&format!("STAND {}", &self.anim_frame).to_string());
                //         }                    
                //     }
                // };
                //let curr_transform = ggez::graphics::transform();


                let text = ggez::graphics::Text::new(typeText);
                //let text_size_x = text.width(ctx) as f32 / 2.0;
                //let text_size_y = text.height(ctx) as f32 / 2.0;
                if let Err(_) = graphics::draw(ctx, &text,
                    DrawParam::default()
                    .dest(na::Point2::new(draw_pos.x, draw_pos.y)) // - text_size_x, draw_pos.y - text_size_y + 28.0))
                    //.offset(na::Point2::new(text_size_x, text_size_y))
                ) {
                    _draw_ok = false;
                }
            }
            

        }

    }
}




// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    //world.register::<PlayerComponent>();
    world.register::<CharacterDisplayComponent>();
}