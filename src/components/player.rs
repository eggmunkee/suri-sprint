use ggez::{Context};
use ggez::graphics;
use ggez::graphics::{Rect,Image,Color,DrawParam};
use ggez::nalgebra as na;
use specs::{ Component, DenseVecStorage, World, WorldExt };
//use specs::shred::{Dispatcher};
use rand::prelude::*;

//use crate::game_state::{GameState};
use crate::resources::{ImageResources};
use crate::components::collision::{Collision};
use crate::components::{Velocity};
use crate::physics;
use crate::physics::{PhysicsBody};

#[derive(Debug)]
pub struct PlayerComponent {
    pub player_name: String,
    pub life: i32,
    pub move_ramp_up: f32,
    //pub image: Image, // component owns image
    //pub path: String,
}
impl Component for PlayerComponent {
    type Storage = DenseVecStorage<Self>;
}

impl PlayerComponent {
    pub fn new() -> PlayerComponent {

        //let image = Image::new(ctx, char_img.clone()).unwrap();

        PlayerComponent {
            player_name: String::from("playername1"),
            life: 100,
            move_ramp_up: 0.0f32,
            // image: image,
            // path: char_img.clone()
        }
    }
    #[allow(dead_code)]
    pub fn set_name(&mut self, name: String) {
        self.player_name = name;
    }
}


#[derive(Debug)]
pub struct CharacterDisplayComponent {
    //pub image: Image, // component owns image
    pub path: String,
    pub going_left: bool,
    pub going_right: bool,
    pub going_up: bool,
    pub going_down: bool,
    pub facing_right: bool,
    pub anim_frame: u32,
    pub anim_set: u32,
    pub anim_frame_time: f32,
    pub breath_cycle: f32,
    pub rot: f32,
    pub in_jump: bool,
    pub jump_duration: f32,
    pub since_stand: f32,
}
impl Component for CharacterDisplayComponent {
    type Storage = DenseVecStorage<Self>;
}

impl CharacterDisplayComponent {
    pub fn new(ctx: &mut Context, char_img: &String) -> CharacterDisplayComponent {
        //let image = Image::new(ctx, char_img.clone()).unwrap();

        CharacterDisplayComponent {
            //image: image,
            path: char_img.clone(),
            going_left: false,
            going_right: false,
            going_up: false,
            going_down: false,
            facing_right: true,
            anim_frame: 0,
            anim_set: 0,
            anim_frame_time: 0.0,
            breath_cycle: 0.0,
            rot: 0.0,
            in_jump: false,
            jump_duration: 0.0,
            since_stand: 0.0,
        }
    }

    pub fn update(&mut self, coll: &mut Collision, time_delta: f32) {
        self.interp_breath(0.08);

        if coll.vel.x == 0.0 && coll.vel.y == 0.0 {
            self.since_stand = 0.0;
        }

        // decide if going_up is allowed with jump rules
        self.since_stand += time_delta;
        let recent_stand = self.since_stand < 0.6;
        if self.in_jump {
            if self.jump_duration < 2.5 {
                self.jump_duration += time_delta;                
                println!("In jump! {}", &self.jump_duration);
            }
            else if recent_stand {
                self.in_jump = false;
                self.jump_duration = 0.0;
                self.since_stand = 0.0;
                println!("Stop jump! {}", &self.jump_duration);
            }
            else {
                //println!("Freefall jump... {}", &self.jump_duration);
                self.jump_duration += time_delta;
                self.going_up = false;
            }
        }
        else if self.going_up {
            if self.jump_duration >= 0.0 && recent_stand {
                println!("Start jump! {}", &self.jump_duration);
                self.in_jump = true;
                self.jump_duration = 0.0;
            }
            else {
                println!("Can't start jump! {}", &self.jump_duration);
                self.going_up = false;
                if self.jump_duration < 0.0 {
                    self.jump_duration += time_delta;
                }
            }
        }
        else {
            //println!("Not in jump, not even trying!  {}", &self.jump_duration);
            self.in_jump = false;
            if self.jump_duration < 0.0 {
                self.jump_duration += time_delta;
            }
            else {}
        }

        self.update_animation(coll, time_delta);


        //self.apply_inputs(coll);
    }

    fn update_animation(&mut self, coll: &mut Collision, time_delta: f32) {

        let mut is_moving = false;
        if self.going_left {
            self.facing_right = false;
            is_moving = true;
        }
        else if self.going_right {
            self.facing_right = true;
            is_moving = true;
        }

        if self.in_jump {            
            if self.anim_set != 1 {
                self.anim_frame = 0;
                self.anim_set = 1;
                self.anim_frame_time = 0.0;
            }

            if coll.vel.y < 0.0 {

                self.anim_frame_time += time_delta;
                if self.anim_frame == 0 && self.anim_frame_time > 0.5 ||
                        self.anim_frame > 0 && self.anim_frame_time > 2.5 {
                    self.anim_frame += 1;
                    self.anim_frame_time = 0.0;

                    if self.anim_frame > 3 {
                        self.anim_frame = 3;
                    }
                }
            }
        }
        else if is_moving || coll.vel.x.abs() > 0.25 {
            self.anim_set = 0;
            self.anim_frame_time += time_delta * (0.5 * coll.vel.x.abs().max(2.0).min(30.0) );
            if self.anim_frame_time > 1.5 {
                self.anim_frame += 1;
                self.anim_frame_time = 0.0;

                if self.anim_frame > 5 {
                    self.anim_frame = 0;
                }
            }
        }
        else {
            self.anim_set = 0;
            //self.anim_frame = 0;
            //self.anim_frame_time = 0.0;
        }

    }

    pub fn interp_breath(&mut self, cycle_speed: f32) {
        let two_pi = 2.0*3.14159;

        if !self.going_left && !self.going_right && !self.going_up && !self.going_down {
            self.breath_cycle += cycle_speed;
            if self.breath_cycle >= two_pi {
                self.breath_cycle -= two_pi;
            }
        }
        else {
            self.breath_cycle = 0.0;
        }

        // self.rot += 0.01;
        // if self.rot >= two_pi {
        //     self.rot -= two_pi;
        // }
    }

    pub fn apply_collision(&mut self, body: &mut physics::PhysicsBody) {
        let move_amt = 25.0; //1300.0;
        let up_mult = 5.0;
        if self.going_right {
            //let new_lin_vel = physics::create_pos(&Point2::new(self.vel.x, self.vel.y));
            if body.linear_velocity().x < 10.0 {
                body.apply_force_to_center(&physics::PhysicsVec {x:move_amt,y: 0.0}, true);
            }
            
            //println!("applied right force");
        }
        if self.going_left {
            //let new_lin_vel = physics::create_pos(&Point2::new(self.vel.x, self.vel.y));
            if body.linear_velocity().x > -10.0 {
                body.apply_force_to_center(&physics::PhysicsVec {x:-move_amt,y: 0.0}, true);
            }
                //println!("applied left force");
        }
        if self.going_up {
            //let new_lin_vel = physics::create_pos(&Point2::new(self.vel.x, self.vel.y));
            if body.linear_velocity().y > -10.0 {
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


    pub fn apply_inputs(&mut self, coll: &mut Collision) {
        let mut vel = coll.vel;
        // Single axis vector length
        let mut vec_amt = 300.0;
        // IS the Input direction Multi-axis, i.e. UP + RIGHT is multi-axis
        let multi_axis = (self.going_left && (self.going_up || self.going_down))
            || (self.going_right && (self.going_up || self.going_down));
        // reduce vector length with two axis
        if multi_axis {
            vec_amt = 0.71 * vec_amt;
        }
        vec_amt = vec_amt;// * 0.05;
        // Apply vector length to velocity X
        if self.going_left {
            vel.x -= vec_amt;
        }
        else if self.going_right {
            vel.x += vec_amt;
        }
        else {
            //vel.x = vel.x * 0.995;
        }
        // Apply vector length to velocity Y
        if self.going_up {
            vel.y -= vec_amt * 1.5;
        }
        else if self.going_down {
            vel.y += vec_amt;
        }
        else {
            // if vel.y < 0.0 {
            //     vel.y = vel.y * 0.98;
            // }
            //vel.x = vel.x * 0.995;
        }

        //vel.x = vel.x.max(-70.0).min(70.0);
        //vel.y = vel.y.max(-80.0);
    }

    // pub fn interp_eye(&mut self, speed: f32) -> (f32, f32) {
    //     let (mut cx, mut cy) = (self.eye_curr_x, self.eye_curr_y);

    //     // Apply input direction to eye direction
    //     const offset_amt : f32 = 4.0;
    //     let mut eye_x_offset = 0.0;
    //     let mut eye_y_offset = 0.0;
    //     if self.going_left {
    //         eye_x_offset = -offset_amt;
    //     }
    //     else if self.going_right {
    //         eye_x_offset = offset_amt;
    //     }
    //     else {
    //         eye_x_offset = 0.0;
    //     }
    //     if self.going_up {
    //         eye_y_offset = -offset_amt;
    //     }
    //     else if self.going_down {
    //         eye_y_offset = offset_amt;
    //     }
    //     else {
    //         eye_y_offset = 0.0;
    //     }
    //     let (tx, ty) = (eye_x_offset, eye_y_offset);

    //     if cx != tx {
    //         if tx > cx { 
    //             cx = (cx + speed).min(tx);
    //         }
    //         else {
    //             cx = (cx - speed).max(tx);
    //         }
    //     }
    //     if cy != ty {
    //         if ty > cy { 
    //             cy = (cy + speed).min(ty);
    //         }
    //         else {
    //             cy = (cy - speed).max(ty);
    //         }
    //     }

    //     self.eye_curr_x = cx;
    //     self.eye_curr_y = cy;

    //     (cx, cy)
    // }
    // pub fn draw(&self, ctx: &mut Context, pos: na::Point2::<f32>) {
    //     let mut rng = rand::thread_rng();
    //     let mut draw_ok = true;
    //     let w = self.image.width();
    //     let h = self.image.height();
    //     let draw_pos = na::Point2::<f32>::new(pos.x - (w as f32 / 2.0), pos.y - (h as f32 / 2.0));
    //     // color part:  ,Color::new(1.0,0.7,0.7,1.0)
    //     if let Err(_) = ggez::graphics::draw(ctx, &self.image, (draw_pos.clone(),)) { // add back x/y pos  //
    //         draw_ok = false;
    //     }

    //     if let Ok(rect) = graphics::Mesh::new_rectangle(
    //         ctx,
    //         graphics::DrawMode::fill(),
    //         graphics::Rect::from([0.0,0.0,4.0,4.0]),
    //         graphics::WHITE,
    //     ) {
    //         let mut col_vals: (u8,) = rng.gen();
    //         //println!("Entity {}, Circle pos: {:?}", ent.id(), pos);
    //         if let Err(_) = graphics::draw(ctx, &rect, (na::Point2::new(pos.x-2.0, pos.y-2.0),
    //                 Color::from_rgba(col_vals.0,col_vals.0,col_vals.0,255) )) {
    //             draw_ok = false;
    //         };  
    //     }
    // }
}

// pub trait PlayerRenderTrait {
//     fn draw(&self, ctx: &mut Context, pos: na::Point2::<f32>);
// }

impl super::RenderTrait for CharacterDisplayComponent {
    fn draw(&self, ctx: &mut Context, world: &World, ent: Option<u32>, pos: na::Point2::<f32>) {
        //println!("PlayerRenderTrait drawing...");
        let mut rng = rand::thread_rng();
        let mut _draw_ok = true;
        // color part:  ,Color::new(1.0,0.7,0.7,1.0)
        let breath_scale = 1.5 + self.breath_cycle.cos() * 0.02;
        let breath_y_offset = self.breath_cycle.cos() * -0.3;
        let mut angle = 0.0;
        if let Some(ent_id) = ent {
            let collision_reader = world.read_storage::<Collision>();
            let entity = world.entities().entity(ent_id);
            if let Some(coll) = collision_reader.get(entity) {
                angle = coll.angle;
            }

        }

        let draw_pos = na::Point2::<f32>::new(pos.x, pos.y + breath_y_offset);

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

            // if self.going_up {
            //     for i in 0..5 {
            //         let col_vals: (u8,f32) = rng.gen();
            //         let scale = 1.0 + (i as f32) * 0.25;
            //         if let Err(_) = graphics::draw(ctx, &rect,
            //             DrawParam::default()
            //                     .dest(na::Point2::new(pos.x + (col_vals.1 - 0.5)* (i as f32*1.5) , pos.y+exhaust_radius + (i as f32 * 5.0) ))
            //                     .scale(na::Vector2::new(scale,scale))
            //                     .color(Color::from_rgba(col_vals.0,col_vals.0,col_vals.0,255-(i*50))) ) {
                        
                        
            //             // (na::Point2::new(
            //             //         pos.x-2.0 + (col_vals.1 - 0.5)* (i as f32*1.5) , pos.y+16.0 + (i as f32 * 7.0) ),
            //             //     Color::from_rgba(col_vals.0,col_vals.0,col_vals.0,255-(i*50)) )) {
            //             _draw_ok = false;
            //         };
            //     }
            // }
            // if self.going_right {
            //     for i in 0..5 {
            //         let col_vals: (u8,f32) = rng.gen();
            //         let scale = 1.0 + (i as f32) * 0.25;
            //         if let Err(_) = graphics::draw(ctx, &rect,
            //             DrawParam::default()
            //                     .dest(na::Point2::new(pos.x-exhaust_radius - (i as f32 * 5.0), pos.y + (col_vals.1 - 0.5)* (i as f32*1.5) ))
            //                     .scale(na::Vector2::new(scale,scale))
            //                     .color(Color::from_rgba(col_vals.0,col_vals.0,col_vals.0,255-(i*50))  )) {
                        
            //             // (na::Point2::new(
            //             //         pos.x-20.0 - (i as f32 * 7.0), pos.y-2.0 + (col_vals.1 - 0.5)* (i as f32*1.5)  ),
            //             //     Color::from_rgba(col_vals.0,col_vals.0,col_vals.0,255-(i*50))  )) {
            //             _draw_ok = false;
            //         };
            //     }
            // }
            // if self.going_left {
            //     for i in 0..5 {
            //         let col_vals: (u8,f32) = rng.gen();
            //         let scale = 1.0 + (i as f32) * 0.25;
            //         if let Err(_) = graphics::draw(ctx, &rect, 
            //             DrawParam::default()
            //                     .dest(na::Point2::new(pos.x+exhaust_radius + (i as f32 * 5.0), pos.y + (col_vals.1 - 0.5)* (i as f32*1.5)))
            //                     .scale(na::Vector2::new(scale,scale))
            //                     .color(Color::from_rgba(col_vals.0,col_vals.0,col_vals.0,255-(i*50))  )) {
            //             // (na::Point2::new(
            //             //         pos.x+16.0 + (i as f32 * 7.0), 
            //             //         pos.y-2.0 + (col_vals.1 - 0.5)* (i as f32*1.5) ),
            //             //     Color::from_rgba(col_vals.0,col_vals.0,col_vals.0,255-(i*50)) )) {
            //             _draw_ok = false;
            //         };
            //     }
            // }

            //let (curr_eye_x, curr_eye_y) = self.interp_eye((self.eye_curr_x, self.eye_curr_y),(eye_x_offset, eye_y_offset), 2.0);

            let mut x_scale = breath_scale;
            if self.facing_right == false {
                x_scale = -x_scale;
            }

            let mut image_resources = world.fetch_mut::<ImageResources>();
            let mut image_ref = image_resources.image_ref(self.path.clone());
            if let Ok(image) = image_ref {
                let w = image.width();
                let h = image.height();

                let src_x = 0.0 + 0.1 * (self.anim_frame as f32);
                let src_y = 0.0 + 0.1 * (self.anim_set as f32);

                // if !self.going_left && !self.going_right {
                //     src_x = 0.0;
                // }
        
                let text_pos = na::Point2::new(draw_pos.x , draw_pos.y - 10.0);
                if let Err(_) = ggez::graphics::draw(ctx, image, 
                    DrawParam::default()
                    .src(Rect::new(src_x,src_y,0.1,0.1))
                    .dest(text_pos)
                    .scale(na::Vector2::new(x_scale,breath_scale))
                    .offset(na::Point2::new(0.5,0.5))
                    .rotation(angle)
                ) {
                    //(draw_pos.clone(),)) { // add back x/y pos  //
                    _draw_ok = false;
                }
            }


            

            //println!("Entity {}, Circle pos: {:?}", ent.id(), pos);
            // if let Err(_) = graphics::draw(ctx, &rect, (na::Point2::new(pos.x+self.eye_curr_x, pos.y+self.eye_curr_y),
            //         Color::from_rgba(col_vals.0,col_vals.0,col_vals.0,255) )) {
            //     _draw_ok = false;
            // };


            // if let Ok(rect) = graphics::Mesh::new_rectangle(
            //     ctx,
            //     graphics::DrawMode::stroke(1.0),
            //     graphics::Rect::from([0.0,0.0,50.0,50.0]),
            //     Color::new(1.0,0.0,0.0,1.0),
            // ) {
            //     if let Err(_) = graphics::draw(ctx, &rect, (na::Point2::new(draw_pos.x-25.0, draw_pos.y-25.0), )) {
                    
            //     };  
            // }
            
        }

    }
}




// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    world.register::<PlayerComponent>();
    world.register::<CharacterDisplayComponent>();
}