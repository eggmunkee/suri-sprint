
use std::collections::hash_map::*;
use ggez;
use ggez::graphics;
use ggez::event::{self, KeyCode, KeyMods, MouseButton};
use winit::dpi::{LogicalPosition};
//use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult, GameError};
use ggez::conf::{NumSamples,WindowSetup,WindowMode};
use ggez::graphics::{Rect,Color,Image,set_window_title};

use specs::{Builder, Component, DispatcherBuilder, Dispatcher, ReadStorage, WriteStorage, 
    System, VecStorage, World, WorldExt, RunNow};
use specs::Join;
use rand::prelude::*;

use wrapped2d::b2;
use wrapped2d::user_data::*;
use wrapped2d::user_data::NoUserData;
// =====================================

use crate::resources::{InputResource,WorldAction,GameStateResource};
use crate::components::{Position};
use crate::components::collision::{Collision};
use crate::components::sprite::{SpriteLayer,SpriteComponent};
use crate::components::meow::{MeowComponent};
use crate::components::player::{CharacterDisplayComponent};
use crate::systems::{InterActorSys,InputSystem};
use crate::world::{create_world,create_dispatcher};
use crate::entities::ghost::{GhostBuilder};
use crate::entities::meow::{MeowBuilder};
use crate::entities::platform::{PlatformBuilder};
use crate::physics;
use crate::physics::{PhysicsWorld, PhysicsBody, PhysicsBodyHandle};
use crate::render;
use crate::input::{InputMap,MouseInput};

#[derive(Copy,Clone,Debug)]
pub enum State {
    Running,
    Paused,    
}

//impl Copy for State

// Main game state struct
pub struct GameState {
    pub window_title: String,
    pub current_state: State,
    pub window_w: f32,
    pub window_h: f32,
    //pub dispatcher: Dispatcher<'a,'a>,
    pub world: World,
    pub font: graphics::Font,
    pub phys_world: PhysicsWorld,
    pub display_scale: f32,
    //pub image_lookup: HashMap<String,usize>,
    //pub images: Vec<Image>
    pub paused_text: graphics::Text,

    pub current_offset: na::Point2::<f32>,
    pub click_info: Vec::<(b2::BodyHandle,b2::FixtureHandle)>,
}

impl GameState {
    pub fn new(ctx: &mut Context, title: String, window_mode: WindowMode) -> GameResult<GameState> {

        // Create physics world to place in game state resource
        let mut physics_world = physics::create_physics_world();

        // Create game state related to window size/mode
        let (win_w, win_h) = ggez::graphics::drawable_size(ctx);
        let game_state_resource = GameStateResource {
            window_w: win_w, window_h: win_h, window_mode: window_mode,
            delta_seconds: 0.15
        };

        // get window
        let window = ggez::graphics::window(ctx);
        window.set_position((1100.0,50.0).into());

        let font = graphics::Font::new(ctx, "/FreeMonoBold.ttf")?;
        let text = graphics::Text::new(("PAUSED", font, 52.0));

        println!("World init");
        let ecs_world = create_world(ctx, game_state_resource, &mut physics_world);

        // Create main state instance with dispatcher and world
        let mut s = GameState { 
            window_title: title,
            current_state: State::Running,
            window_w: win_w,
            window_h: win_h,
            display_scale: 2.0,
            //dispatcher: create_dispatcher(), 
            world: ecs_world,
            font: font,
            phys_world: physics_world,
            // image_lookup: HashMap::<String,usize>::new(),
            // images: Vec::<Image>::new(),
            paused_text: text,
            current_offset: na::Point2::new(0.0,0.0),
            click_info: vec![],
        };

        Ok(s)
    }

}

impl GameState {
    #[allow(dead_code)]
    pub fn pause(&mut self) {
        let curr_st = self.current_state;
        match curr_st {
            State::Running => { self.current_state = State::Paused; }
            _ => {}
        }        
    }
    #[allow(dead_code)]
    pub fn play(&mut self) {
        let curr_st = self.current_state;
        match curr_st {
            State::Paused => { self.current_state = State::Running; }
            _ => {}
        }        
    }

    pub fn set_frame_time(&mut self, time_delta: f32) {
        // get game resource to set delta
        let world = &mut self.world;
        let mut game_res = world.fetch_mut::<GameStateResource>();
        game_res.delta_seconds = time_delta;

    }
  

    pub fn run_update_systems(&mut self, ctx: &mut Context, time_delta: f32) {
        let world = &mut self.world;

        // Run Input System
        {
            // outputs: meow locations and clicked entity info
            let mut input_sys = InputSystem::new();
            input_sys.run_now(&world);

            // Process meow creation
            for m in &input_sys.meows {
                // Create a meow bubble
                MeowBuilder::build(world, ctx, &mut self.phys_world, m.0.x, m.0.y, m.1.x, m.1.y, 20.0, 20.0);
            }

            // Get display size for click position calculations
            let dim = ggez::graphics::drawable_size(ctx);
            let mut display_offset = self.current_offset;
            display_offset.x += dim.0 as f32 / 2.0;
            display_offset.y += dim.1 as f32 / 2.0;

            // list for entities found at click
            let mut entity_clicked : Vec<u32> = vec![];

            for click in &input_sys.click_info {
                // get center x/y based on click and display offset
                // to get from screen coordinates to game object coords
                let center_x = click.x - display_offset.x;
                let center_y = click.y - display_offset.y;
                // println!("Click position: {}, {}", &click.x, &click.y);
                // println!("Display offset: {}, {}", &display_offset.x, &display_offset.y);
                // println!("Click game pos: {}, {}", &(click.x-1.0-display_offset.x), &(click.y-1.0-display_offset.y));
                // create bounding box for click position to check colliders
                let mut aabb = b2::AABB::new();
                aabb.lower = physics::create_pos(&na::Point2::new(center_x-0.1, center_y-0.1));
                aabb.upper = physics::create_pos(&na::Point2::new(click.x+0.1, click.y+0.1));
        
                {
                    let physics = &self.phys_world;
                    // create object which received click collide info
                    let mut click_info = GameStateClickInfo {
                        click_info: vec![]
                    };
                    // query physics world with aabb, updating click_info
                    physics.query_aabb(&mut click_info, &aabb);
        
                    // go through click info from query
                    for (b, f) in &click_info.click_info {
                        println!("Clicked {:?},{:?}", &b, &f);
                        // get physics body
                        let body = self.phys_world.body(*b);
                        // get body user data with entity id
                        let body_data = &*body.user_data();
                        let ent_id = body_data.entity_id;
                        // add entity id to vector
                        entity_clicked.push(ent_id);
                    }
        
                    // clear click info from results object
                    click_info.click_info.clear();
                    drop(click_info);
        
                }
            }

            for ent in &entity_clicked {
                println!("Entity {:?} clicked", &ent);
            }

            input_sys.meows.clear();
            input_sys.click_info.clear();
            drop(input_sys);
        }

        // Meow "system" - updates meow state and components
        {
            // Operator on meows, collisions and sprite components
            let mut meow_writer = world.write_storage::<MeowComponent>();
            let mut collision_writer = world.write_storage::<Collision>();
            let mut sprite_writer = world.write_storage::<SpriteComponent>();
            let entities = world.entities();                

            for (meow, coll, sprite, ent) in (&mut meow_writer, &mut collision_writer, &mut sprite_writer, &entities).join() {
                // update meow components
                meow.update(time_delta, coll, sprite, &mut self.phys_world);

                if meow.meow_time < 0.0 {

                    // Destroy physics body of collision component
                    coll.destroy_body(&mut self.phys_world);
                    // Delete entity from ecs world
                    entities.delete(ent);
                }
            }

        }
    }

    pub fn run_update_step(&mut self, ctx: &mut Context, time_delta: f32) {
        
        // Save frame time
        self.set_frame_time(time_delta);

        // Update components
        self.run_update_systems(ctx, time_delta);

        // Cleanup the world state after changes
        self.world.maintain();

        // Run physics update frame
        physics::advance_physics(&mut self.world, &mut self.phys_world, time_delta);
    }
}

// Struct which holds body/fixture physics query results
pub struct GameStateClickInfo {
    pub click_info: Vec::<(b2::BodyHandle,b2::FixtureHandle)>,
}
impl b2::QueryCallback for GameStateClickInfo {

    fn report_fixture(
        &mut self, 
        body: b2::BodyHandle, 
        fixture: b2::FixtureHandle
    ) -> bool {
        //println!()

        self.click_info.push((body.clone(), fixture.clone()));

        true
    }
}


impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {

        let time_scale = 1.0;
        let delta_s = time_scale * (ggez::timer::duration_to_f64(ggez::timer::delta(ctx)) as f32);
  
        // Only update world state when game is running (not paused)
        match &self.current_state {
            State::Running => {

                self.run_update_step(ctx, delta_s);

            },
            _ => {
                
            }
        }

        // always ok result
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Call rendering module        

        let mut renderer = render::Renderer::new();

        let gr = renderer.render_frame(self, &self.world, ctx);

        self.current_offset = renderer.display_offset;

        // Yield process to os
        ggez::timer::yield_now();

        gr

    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        let button_index = match button {
            MouseButton::Left => {
                Some(0usize)
            },
            MouseButton::Middle => {
                Some(1)
            },
            MouseButton::Right => {
                Some(2)
            }
            _ => None
        };
        if let Some(index) = button_index {
            InputMap::mouse_button_down(&mut self.world, ctx, index.clone());
        }
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        let button_index = match button {
            MouseButton::Left => {
                Some(0usize)
            },
            MouseButton::Middle => {
                Some(1)
            },
            MouseButton::Right => {
                Some(2)
            }
            _ => None
        };
        if let Some(index) = button_index {
            InputMap::mouse_button_up(&mut self.world, ctx, index.clone());
        }
        
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, xrel: f32, yrel: f32) {
        InputMap::mouse_set_pos(&mut self.world, _ctx, x, y);
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        //println!("Mousewheel event, x: {}, y: {}", x, y);
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
        repeat: bool,
    ) {

        if repeat {
            if keycode == KeyCode::Subtract {
                if self.display_scale > 0.25 {
                    self.display_scale -= 0.05;
                }            
            }
            else if keycode == KeyCode::Equals {
                if self.display_scale < 2.75 {
                    self.display_scale += 0.05;
                }            
            }
    
        }


        InputMap::key_down(&mut self.world, ctx, keycode, keymod);
    }

    fn key_up_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
    ) {

        if keycode == KeyCode::P {
            match self.current_state {
                State::Paused => {
                    self.play();
                },
                State::Running => {
                    self.pause();
                }
            }
        }
        else if keycode == KeyCode::J {
            // Get world action if any
            //println!("Processing AddCircle action");
            let mut rng = rand::thread_rng();

            let test : u16 = rng.gen::<u16>();
            if test % 5 == 0 {
                crate::entities::platform::PlatformBuilder::build_dynamic(&mut self.world, ctx, &mut self.phys_world, 100.0, 400.0,
                    50.0, 15.0, 0.0, SpriteLayer::Entities.to_z());
            }
            else {
                crate::entities::ghost::GhostBuilder::build_collider(&mut self.world, ctx, &mut self.phys_world, 100.0, 400.0, 0.0, 0.0,
                    30.0, 0.15, 25.0, 25.0);
            }


        }
        else if keycode == KeyCode::Subtract {
            if self.display_scale > 0.25 {
                self.display_scale -= 0.05;
            }            
        }
        else if keycode == KeyCode::Equals {
            if self.display_scale < 2.75 {
                self.display_scale += 0.05;
            }            
        }

        InputMap::key_up(&mut self.world, ctx, keycode, keymod);
    }

    fn text_input_event(&mut self, _ctx: &mut Context, ch: char) {
        //println!("Text input: {}", ch);
    }

    fn focus_event(&mut self, _ctx: &mut Context, gained: bool) {
        if gained {
            println!("Focus gained");
        } else {
            println!("Focus lost");
        }
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        //println!("Resized: {}, {}", &width, &height);

        // set game state w/h
        let mut game_state_writer = self.world.fetch_mut::<GameStateResource>();

        self.window_w = width;
        self.window_h = height;

        game_state_writer.window_w = width;
        game_state_writer.window_h = height;

        let mut mode = game_state_writer.window_mode;

        mode.width = width;
        mode.height = height;
        //println!("New window mode {:?}", &mode);

        ggez::graphics::set_screen_coordinates(ctx, Rect::new(0.0, 0.0, width, height));

        drop(game_state_writer);
        
    }
}