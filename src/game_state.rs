
//use std::collections::hash_map::*;
use ggez;
use ggez::graphics;
use ggez::event::{self, KeyCode, KeyMods, MouseButton};
use ggez::event::{GamepadId, Button, Axis};
//use winit::dpi::{LogicalPosition};
//use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use ggez::conf::{WindowMode};
use ggez::graphics::{Rect};
use specs::{World, WorldExt, RunNow};
use specs::Join;
use rand::prelude::*;
use std::collections::{HashMap};

use wrapped2d::b2;
use wrapped2d::user_data::*;
// =====================================

use crate::audio::{Audio};
use crate::resources::{InputResource,GameStateResource};
use crate::components::{Position};
use crate::components::collision::{Collision};
use crate::components::sprite::{SpriteLayer,SpriteComponent};
use crate::components::meow::{MeowComponent};
use crate::components::exit::{ExitComponent};
use crate::components::portal::{PortalComponent};
use crate::components::button::{ButtonComponent};
use crate::components::player::{CharacterDisplayComponent};
use crate::components::npc::{NpcComponent};
use crate::systems::{InputSystem};
use crate::systems::logic::{LogicSystem};
use crate::world::{create_world};
use crate::entities::level_builder::{LevelConfig,LevelBounds};
//use crate::entities::ghost::{GhostBuilder};
use crate::entities::meow::{MeowBuilder};
//use crate::entities::platform::{PlatformBuilder};
//use crate::entities::empty_box::{BoxBuilder};
use crate::physics;
use crate::physics::{PhysicsWorld};
use crate::render;
use crate::input::{InputMap,InputKey};

#[derive(Clone,Debug,PartialEq)]
pub enum RunningState {
    Playing, // normal playing state - world & physics running
    Dialog(String),  // dialog being shown state - world paused
}

#[derive(Copy,Clone,Debug)]
pub enum State {
    Running,
    Paused,    
}
#[derive(Copy,Clone,Debug,PartialEq)]
pub enum GameMode {
    Play,
    Edit,
}

const WARP_TIME_LIMIT : f32 = 1.5f32;
const WARP_TIME_SCALE : f32  = 0.035;  //0.025f32;

// Main game state struct
pub struct GameState {
    pub window_title: String,
    pub current_state: State,
    pub running_state: RunningState,
    pub mode: GameMode,
    pub current_level_name: String,
    pub current_entry_name: String,
    pub level: LevelConfig,
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
    pub cursor_offset: na::Point2::<f32>,

    // Current view offset
    pub current_offset: na::Point2::<f32>,
    pub snap_view: bool,
    // what collision item was clicked
    pub click_info: Vec::<(b2::BodyHandle,b2::FixtureHandle)>,
    // level transition
    pub level_warping: bool,
    pub level_warp_timer: f32,
    pub warp_level_name: String,
    pub warp_level_entry_name: String,
    // paused anim
    pub paused_anim: f32,
    // audio
    pub audio: Audio,

    // debug flags
    pub debug_logic_frames: i32,
}

impl GameState {
    pub fn new(ctx: &mut Context, title: String, window_mode: WindowMode) -> GameResult<GameState> {

        // Create physics world to place in game state resource
        let mut physics_world = physics::create_physics_world();

        // Create game state related to window size/mode
        let (win_w, win_h) = ggez::graphics::drawable_size(ctx);
        let game_state_resource = GameStateResource {
            window_w: win_w, window_h: win_h, window_mode: window_mode,
            delta_seconds: 0.15, level_bounds: LevelBounds::new(-500.0, -500.0, 3000.0, 3000.0),
            level_world_seconds: 0.0, game_run_seconds: 0.0, 
        };

        // get window
        let window = ggez::graphics::window(ctx);
        window.set_position((1100.0,50.0).into());

        let font = graphics::Font::new(ctx, "/FreeMonoBold.ttf")?;
        let text = graphics::Text::new(("PAUSED", font, 52.0));

        println!("World init");
        let ecs_world = create_world(ctx, game_state_resource, &mut physics_world);

        let audio_engine = Audio::new(ctx);

        // Create main state instance with dispatcher and world
        let s = GameState { 
            window_title: title,
            current_state: State::Running,
            //running_state: RunningState::Playing, 
            running_state: RunningState::Dialog("Loading game".to_string()),
            mode: GameMode::Play,
            current_level_name: "start".to_string(),
            current_entry_name: "".to_string(),
            level: LevelConfig::new(),
            window_w: win_w,
            window_h: win_h,
            display_scale: 1.0,
            //dispatcher: create_dispatcher(), 
            world: ecs_world,
            font: font,
            phys_world: physics_world,
            // image_lookup: HashMap::<String,usize>::new(),
            // images: Vec::<Image>::new(),
            paused_text: text,
            cursor_offset: na::Point2::new(10.0,10.0),
            current_offset: na::Point2::new(0.0,0.0),
            snap_view: true,
            click_info: vec![],
            level_warping: false,
            level_warp_timer: 0.0,
            warp_level_name: "".to_string(),
            warp_level_entry_name: "".to_string(),
            paused_anim: 0.0,
            audio: audio_engine,
            debug_logic_frames: 2,
        };

        Ok(s)
    }

}

impl GameState {

    pub fn play_music(&mut self, ctx: &mut Context) {
        self.audio.set_dimmed(false);
        self.audio.play_music(ctx, "".to_string());
    }

    pub fn dim_music(&mut self, _ctx: &mut Context) {
        //self.audio.stop_music(ctx);
        self.audio.set_dimmed(true);
    }

    #[allow(dead_code)]
    pub fn pause(&mut self) {
        let curr_st = self.current_state;
        match curr_st {
            State::Running => { 
                self.current_state = State::Paused;
                self.paused_anim = 0.0;
                self.audio.pause_music();
            }
            _ => {}
        }        
    }
    #[allow(dead_code)]
    pub fn play(&mut self) {
        let curr_st = self.current_state;
        match curr_st {
            State::Paused => { 
                self.current_state = State::Running; 
                self.audio.resume_music();
            }
            _ => {}
        }        
    }

    pub fn reset_level_time(&mut self) {
        // get game resource to set delta
        let world = &mut self.world;
        let mut game_res = world.fetch_mut::<GameStateResource>();
        game_res.level_world_seconds = 0.0;
    }

    pub fn set_frame_time(&mut self, time_delta: f32) {
        // get game resource to set delta
        let world = &mut self.world;
        let mut game_res = world.fetch_mut::<GameStateResource>();
        game_res.delta_seconds = time_delta;
        game_res.level_world_seconds += time_delta;
    }

    pub fn update_run_time(&mut self, time_delta: f32) {
        // get game resource to set delta
        let world = &mut self.world;
        let mut game_res = world.fetch_mut::<GameStateResource>();
        game_res.game_run_seconds += time_delta;
    }
  
    pub fn set_running_state(&mut self, ctx: &mut Context, new_state: RunningState) {
        match new_state {
            RunningState::Playing => {
                self.play_music(ctx);
            },
            RunningState::Dialog(_) => {
                self.dim_music(ctx);
            }
        }

        self.running_state = new_state;
    }

    pub fn run_update_systems(&mut self, ctx: &mut Context, time_delta: f32) {

        let world = &mut self.world;

        {
            //let mut world = &mut self.world;
            let mut logic_sys = LogicSystem {
                show_debug_output: self.debug_logic_frames > 0
            };

            if logic_sys.show_debug_output {
                println!(" - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -");
                println!("Starting logic pass {} =============================================================", &self.debug_logic_frames);
            }
            logic_sys.run_now(&world);

            if self.debug_logic_frames > 0 {
                self.debug_logic_frames -= 1;
            }
        }

        // Run Input System - Mainly handle meow creation right now
        {
            // Run InputSystem on world
            // outputs: meow locations and clicked entity info
            let mut input_sys = InputSystem::new();
            input_sys.run_now(&world);

            // Process meow creation
            let mut meow_count : i32 = 0;
            for m in &input_sys.meows {
                // Create a meow bubble
                MeowBuilder::build(world, ctx, &mut self.phys_world, m.0.x, m.0.y, m.1.x, m.1.y, 20.0, 20.0);
                meow_count += 1;
            }
            if meow_count > 0 {
                self.audio.play_jump();
            }

            // CLICK - COLLIDER HANDLING CODE - in testing =========================
            // Get display size for click position calculations
            let dim = ggez::graphics::drawable_size(ctx);
            let mut display_offset = self.current_offset;
            display_offset.x += dim.0 as f32 / 2.0;
            display_offset.y += dim.1 as f32 / 2.0;

            // list for entities found at click
            let mut entity_clicked : Vec<u32> = vec![];

            let mut click_items : i32 = 0;

            // Iterate through any click_info item from input system
            for click in &input_sys.click_info {
                // get center x/y based on click and display offset
                // to get from screen coordinates to game object coords
                let center_x = click.x - display_offset.x;
                let center_y = click.y - display_offset.y;
                click_items += 1;
                // println!("Click position: {}, {}", &click.x, &click.y);
                // println!("Display offset: {}, {}", &display_offset.x, &display_offset.y);
                // println!("Click game pos: {}, {}", &(click.x-1.0-display_offset.x), &(click.y-1.0-display_offset.y));

                // create bounding box for click position to check colliders
                // very small rectangle around the cursor position translated into world coords
                let mut aabb = b2::AABB::new();
                // create physics-scale positions for bounding box
                aabb.lower = physics::create_pos(&na::Point2::new(center_x-0.5, center_y-0.5));
                aabb.upper = physics::create_pos(&na::Point2::new(center_x+0.5, center_y+0.5));
        
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

            let mut ent_click_ct : i32 = 0;
            for ent in &entity_clicked {
                println!("Entity {:?} clicked", &ent);
                ent_click_ct += 1;
            }
            if click_items > 0 {
                if ent_click_ct == 0 {
                    println!("No entity clicked");
                }
                else {
                    self.audio.play_jump();
                }
            }

            input_sys.meows.clear();
            input_sys.click_info.clear();
            drop(input_sys);
        }

        // Meow "system" - updates meow state and components
        //  This could be moved into a system as long as the physics data was accessible & writable from the system class

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
                    entities.delete(ent).expect("Failed to delete meow");
                }
            }
        }

        // Portal "system"
        {
            // Operator on meows, collisions and sprite components
            let mut portal_writer = world.write_storage::<PortalComponent>();
            let mut collision_writer = world.write_storage::<Collision>();
            let mut sprite_writer = world.write_storage::<SpriteComponent>();
            let entities = world.entities();                

            for (portal, coll, sprite, _ent) in (&mut portal_writer, &mut collision_writer, &mut sprite_writer, &entities).join() {
                // update portal components
                portal.update(time_delta, coll, sprite, &mut self.phys_world);
            }
        }

        // Button "system"
        let mut spawn_ghost = false;
        let mut spawn_box = false;
        let mut spawn_platform = false;
        let mut spawn_mouse = false;
        {
            // Operator on meows, collisions and sprite components
            let mut button_reader = world.write_storage::<ButtonComponent>();
            let mut collision_writer = world.write_storage::<Collision>();
            let entities = world.entities();                

            for (button, coll, _ent) in (&mut button_reader, &mut collision_writer, &entities).join() {
                // update button components
                button.update(time_delta, coll, &mut self.phys_world);
                if button.triggered {
                    //println!("Button {} triggered", &button.name);
                    if button.name == "ghost" {
                        spawn_ghost = true;
                    //     let test : u16 = rng.gen::<u16>();
                    //     crate::entities::ghost::GhostBuilder::build_collider(&mut self.world, ctx, &mut self.phys_world, 100.0, 400.0, 0.0, 0.0,
                    //         30.0, 0.15, 25.0, 25.0);
                    }
                    else if button.name == "box" {
                        spawn_box = true;
                    //     let test : u16 = rng.gen::<u16>();
                    //     let w = 10.0 + 0.001 * test as f32;
                    //     crate::entities::empty_box::BoxBuilder::build_dynamic(&mut self.world, ctx, &mut self.phys_world, 200.0, 100.0,
                    //         w, w, rng.gen::<f32>() * 2.0 * 3.14159, SpriteLayer::Entities.to_z());
                    }
                    else if button.name == "platform" {
                        spawn_platform = true;
                    //     let test : u16 = rng.gen::<u16>();
                    //     let w = 50.0 + 0.001 * test as f32;
                    //     let h = 10.0 + 0.00025 * test as f32;
                    //     crate::entities::platform::PlatformBuilder::build_dynamic(&mut self.world, ctx, &mut self.phys_world, 200.0, 100.0,
                    //         w, h, 0.0, SpriteLayer::Entities.to_z());                            
                    }
                    else if button.name == "mouse" {
                        spawn_mouse = true;
                    }
                }
            }
        }

        if spawn_mouse {
            crate::entities::mouse::MouseBuilder::build(&mut self.world, ctx, &mut self.phys_world, 150.0, -50.0, 12.0, 6.0, 0.0, SpriteLayer::Entities.to_z() );
        }
        if spawn_ghost {
            //let mut rng = rand::thread_rng();
            //let test : u16 = rng.gen::<u16>();
            crate::entities::ghost::GhostBuilder::build_collider(&mut self.world, ctx, &mut self.phys_world, 100.0, 400.0, 0.0, 0.0,
                30.0, 0.15, 25.0, 25.0);
        }
        if spawn_box {
            let mut rng = rand::thread_rng();
            let test : u16 = rng.gen::<u16>();
            let w = 10.0 + 0.001 * test as f32;
            crate::entities::empty_box::BoxBuilder::build_dynamic(&mut self.world, ctx, &mut self.phys_world, 200.0, 100.0,
                w, w, rng.gen::<f32>() * 2.0 * 3.14159, SpriteLayer::Entities.to_z());
        }
        if spawn_platform {
            let mut rng = rand::thread_rng();
            let test : u16 = rng.gen::<u16>();
            let w = 50.0 + 0.001 * test as f32;
            let h = 10.0 + 0.00025 * test as f32;
            crate::entities::platform::PlatformBuilder::build_dynamic(&mut self.world, ctx, &mut self.phys_world, 200.0, 100.0,
                w, h, 0.0, SpriteLayer::Entities.to_z());   
        }
    }

    #[allow(dead_code)]
    pub fn run_dialog_update(&mut self, _ctx: &mut Context, _time_delta: f32) {
        
        {
            let world = &mut self.world;
            let mut input_sys = InputSystem::new();
            input_sys.run_now(&world);
        }
    }


    pub fn run_post_physics_update(&mut self, _ctx: &mut Context, _time_delta: f32) {
        //let world = &mut self.world;

        let mut exit_name = "".to_string();
        let mut exit_entry_name = "".to_string();
        //let mut portal_id = -1;

        {
            // get character entities, handle portal & exit statuses
            let entities = self.world.entities();
            // operate on optional characters or npcs
            let mut char_res = self.world.write_storage::<CharacterDisplayComponent>();
            let mut npc_res = self.world.write_storage::<NpcComponent>();
            // position/velocity components
            let mut pos_res = self.world.write_storage::<Position>();
            //let mut vel_res = self.world.write_storage::<Velocity>();
            // collision component
            let mut coll_res = self.world.write_storage::<Collision>();
            let mut sprite_res = self.world.write_storage::<SpriteComponent>();

            // hash to store portal names and their positions - avoid needing to search for them later
            let mut portal_hash = HashMap::<String,(i32,f32,f32)>::new();
            let portal_res = self.world.read_storage::<PortalComponent>();
            // Insert portal information into hash
            for (portal, pos, _ent) in (&portal_res, &pos_res, &entities).join() {
                portal_hash.insert(portal.name.clone(), (_ent.id() as i32, pos.x, pos.y));
            }

            // Join entities and their components to process physics update
            for (_ent, mut character_opt, mut npc_opt,  mut pos, mut coll, mut sprite) in 
                (&entities, (&mut char_res).maybe(), (&mut npc_res).maybe(), &mut pos_res, &mut coll_res, (&mut sprite_res).maybe()).join() {

                let mut facing_right = true;

                // Handle character-exit specially
                if let Some(ref mut character) = character_opt {
                    if character.since_warp < 0.5 { continue; }
                    // Handle character entered an exit and not already level warping
                    if character.in_exit && !self.level_warping {
                        // Get exit 
                        let exit_id = character.exit_id as i32;
                        facing_right = character.facing_right;
        
                        //println!("Character exiting..., {}", &exit_id);
                        let exit_ent = self.world.entities().entity(exit_id as u32);
                        let exit_res = self.world.read_storage::<ExitComponent>();
                        if let Some(exit) = exit_res.get(exit_ent) {
                            println!("Exit info {:?}", &exit);
                            let mut exit_dest = exit.destination.clone();

                            if exit_dest.is_empty() == false {
                                if let Some(index) = exit_dest.find(":") {
                                    let (just_exit_name, entry_name) = exit_dest.split_at_mut(index);
                                    let (_, entry_name) = entry_name.split_at(1);
                                    exit_name = just_exit_name.to_string();
                                    exit_entry_name = entry_name.to_string();
                                }
                                else {
                                    exit_name = exit_dest;
                                    exit_entry_name = "".to_string();
                                }


                            }                            
                        }
        
                    }

                }

                // Handle Collider entered portal - generic portal behavior
                if coll.in_portal && coll.since_warp > 0.75 {
                    // get
                    let portal_id = coll.portal_id as i32;
                    //println!("Collider since warp: {}", &coll.since_warp);

                    //exit_id = character.exit_id as i32;

                    //let exit_res = world

                    //println!("Collider exiting..., {}", &portal_id);
                    
                    let portal_ent = self.world.entities().entity(portal_id as u32);
                    let portal_res = self.world.read_storage::<PortalComponent>();
                    if let Some(portal) = portal_res.get(portal_ent) {
                        //println!("Portal info {:?}", &portal);
                        let portal_dest = portal.destination.clone();
                        if portal_dest.is_empty() == false {
                            if let Some((_portal_id, x, y)) = portal_hash.get(&portal_dest) {

                                //println!("Portal at {}, {}", &x, &y);
                                pos.x = *x;
                                pos.y = *y;
                                let nvx = -coll.vel.x * 1.0;
                                let nvy = -coll.vel.y * 1.0;
                                //println!("Vel update from {},{} to {},{}", &vel.x, &vel.y, &nvx, &nvy);
                                if nvx > 0.0 {
                                    facing_right = true;
                                }
                                else if nvx < 0.0 {
                                    facing_right = false;
                                }
                                //vel.x = nvx;
                                //vel.y = nvy;
    
                                coll.pos.x = *x;
                                coll.pos.y = *y;
                                coll.vel.x = nvx;
                                coll.vel.y = nvy;
                                coll.since_warp = 0.0;
    
                                // Update Position and Velocity of collider body
                                coll.update_body_transform(&mut self.phys_world, &na::Point2::<f32>::new(*x, *y));
                                coll.update_body_velocity(&mut self.phys_world, &na::Vector2::<f32>::new(nvx, nvy));

                                if let Some(ref mut sprite_comp) = sprite {
                                    //sprite
                                    //sprite_comp
                                    if sprite_comp.scale[0] > 0.0 {
                                        sprite_comp.scale[0] = -sprite_comp.scale[0];
                                    }
                                    else if sprite_comp.scale[0] < 0.0 {
                                        sprite_comp.scale[0] = -sprite_comp.scale[0];
                                    }
                                }
                            }
                        }
                        
                    }

                    // Update Facing flags based on portal change for Char/NPC
                    if let Some(ref mut character) = character_opt {
                        character.facing_right = facing_right;
                    }
                    if let Some(ref mut npc) = npc_opt {
                        npc.facing_right = facing_right;
                    }
                }
            }

        }

        if self.level_warping == false && exit_name.is_empty() == false {
            self.start_warp(exit_name, exit_entry_name);
        }

    }

    pub fn process_time_delta(&mut self, ctx: &mut Context, time_delta: f32) -> f32 {
        let mut time_delta = time_delta;
        if self.level_warping {
            self.level_warp_timer += time_delta;
            time_delta *= WARP_TIME_SCALE;

            if self.level_warp_timer > WARP_TIME_LIMIT {
                let level_name = self.warp_level_name.clone();
                let entry_name = self.warp_level_entry_name.clone();
                self.load_level(ctx, level_name, entry_name);
                self.level_warp_timer = 0.0;
                self.level_warping = false;
            }
    
        }
        else {
            time_delta *= 1.0;
        }
        time_delta
    }

    pub fn run_update_step(&mut self, ctx: &mut Context, time_delta: f32) {

        self.update_run_time(time_delta);

        // Pre-process frame time
        let time_delta = self.process_time_delta(ctx, time_delta);

        // Save frame time
        self.set_frame_time(time_delta);

        let mut new_state = self.running_state.clone();
        let mut state_change = false;
        match &self.running_state { 
            RunningState::Playing => {
                // Update components
                self.run_update_systems(ctx, time_delta);

                // Cleanup the world state after changes
                self.world.maintain();


                // Run physics update frame
                physics::advance_physics(&mut self.world, &mut self.phys_world, time_delta);

                // Update components after physics
                self.run_post_physics_update(ctx, time_delta);
            },
            RunningState::Dialog(_message) => {
                let input_res = self.world.fetch::<InputResource>();
                new_state = InputSystem::handle_dialog_input(&input_res, &self, time_delta);
                if new_state == RunningState::Playing {
                    state_change = true;
                }
            }
        }
        if state_change {
            self.set_running_state(ctx, new_state); //running_state = new_state;
        }
    }

    pub fn clear_world(&mut self) {
        // Clear world of entity and component data
        self.world.delete_all();
    }

    pub fn clear_physics(&mut self) {
        // Drop and replace the physics world
        self.phys_world = physics::create_physics_world();
    }

    pub fn set_level_bounds(&mut self) {
        let bounds = &self.level.bounds;
        // get game resource to set delta
        let world = &mut self.world;
        let mut game_res = world.fetch_mut::<GameStateResource>();
        game_res.level_bounds = bounds.clone();

        println!("Level bounds: {},{} through {},{}", &game_res.level_bounds.min_x,
            &game_res.level_bounds.min_y, &game_res.level_bounds.max_x, &game_res.level_bounds.max_y);
    }

    pub fn start_warp(&mut self, level_name: String, entry_name: String) {
        self.warp_level_name = level_name;
        self.warp_level_entry_name = entry_name;
        self.level_warping = true;
        self.level_warp_timer = 0.0;
    }

    #[allow(dead_code)]
    pub fn save_level(&self, save_name: String) {
        // TEST CODE TO SAVE LEVEL CONFIG
        let mut save_path = String::from("levels/");
        save_path.push_str(&save_name);
        crate::conf::save_ron_config(save_path, &self.level);
    }

    pub fn actual_load_level(&mut self, ctx: &mut Context, level_name: String, entry_name: String) {
        // load level from file
        self.level = LevelConfig::load_level(&level_name);

        self.audio.set_dimmed(true);
        if self.level.soundtrack != "" {
            self.audio.play_music(ctx, format!("/audio/{}", &self.level.soundtrack));
        }


        self.set_level_bounds();

        // load level into world        
        let mut world = &mut self.world;
        // Get mut ref to new physics world
        let mut physics_world = &mut self.phys_world;
        &self.level.build_level(&mut world, ctx, &mut physics_world, entry_name);

        self.reset_level_time();
    }

    pub fn load_level(&mut self, ctx: &mut Context, level_name: String, entry_name: String) {
        self.current_level_name = level_name.clone();
        self.current_entry_name = entry_name.clone();
        if entry_name.is_empty() {
            self.running_state = RunningState::Dialog(format!("Level {}, entry {}", &level_name, &entry_name));
        }
        else {
            self.running_state = RunningState::Playing;
        }
        self.snap_view = true;

        //self.stop_music(ctx);

        self.clear_world();
        self.clear_physics();

        self.actual_load_level(ctx, level_name, entry_name);
    }

    pub fn game_key_down(&mut self, ctx: &mut Context, key: &InputKey) {
        match &key {
            InputKey::Exit => {
                ggez::event::quit(ctx);
            },
            InputKey::Pause => {
                match self.current_state {
                    State::Paused => {
                        self.play();
                    },
                    State::Running => {
                        match self.running_state {
                            RunningState::Playing => {
                                self.pause();
                            },
                            _ => {} // don't pause on dialogs
                        }
                    }
                }
            },
            _ => {}
        }
    }

    pub fn game_key_up(&mut self, _ctx: &mut Context, key: &InputKey) {
        match &key {
            InputKey::Exit => {

            },
            InputKey::Pause => {

            },
            _ => {}
        }
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
            State::Paused => {
                self.paused_anim += delta_s;
                if self.paused_anim > 1.0 {
                    self.paused_anim = 1.0;
                }
            }
        }

        // always ok result
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Call rendering module        

        let mut renderer = render::Renderer::new(self.paused_anim);

        // if self.snap_view {
        //     renderer.snap_view = true;
        //     self.snap_view = false;
        // }

        let gr = renderer.render_frame(self, ctx);

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
            InputMap::mouse_set_pos(&mut self.world, ctx, x, y);
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
            InputMap::mouse_set_pos(&mut self.world, ctx, x, y);
            InputMap::mouse_button_up(&mut self.world, ctx, index.clone());
        }
        
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, _xrel: f32, _yrel: f32) {
        InputMap::mouse_set_pos(&mut self.world, ctx, x, y);
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32) {
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
                if self.display_scale < 4.75 {
                    self.display_scale += 0.05;
                }            
            }
            //
            else if keycode == KeyCode::RBracket {
                let new_level = (self.audio.base_music_volume + 0.05).min(1.0);
                self.audio.set_volume(new_level);
            }
            else if keycode == KeyCode::LBracket {
                let new_level = (self.audio.base_music_volume - 0.05).max(0.0);
                self.audio.set_volume(new_level);
            }            
    
        }



        let key = InputMap::key_down(&mut self.world, ctx, keycode, keymod);
        if let Some(i_key) = key {
            self.game_key_down(ctx, &i_key);
        }
    }

    fn key_up_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
    ) {

        // if keycode == KeyCode::P {
        //     match self.current_state {
        //         State::Paused => {
        //             self.play();
        //         },
        //         State::Running => {
        //             match self.running_state {
        //                 RunningState::Playing => {
        //                     self.pause();
        //                 },
        //                 _ => {} // don't pause on dialogs
        //             }
        //         }
        //     }
        // }
        if keycode == KeyCode::J {
            // Get world action if any
            //println!("Processing AddCircle action");
            let mut rng = rand::thread_rng();

            let test : u16 = rng.gen::<u16>();
            if test % 5 == 0 {
                let w = 50.0 + 0.001 * test as f32;
                let h = 10.0 + 0.00025 * test as f32;
                crate::entities::platform::PlatformBuilder::build_dynamic(&mut self.world, ctx, &mut self.phys_world, 200.0, 100.0,
                    w, h, 0.0, SpriteLayer::Entities.to_z());
            }
            else if test % 4 == 0 {
                let w = 10.0 + 0.001 * test as f32;
                crate::entities::empty_box::BoxBuilder::build_dynamic(&mut self.world, ctx, &mut self.phys_world, 200.0, 100.0,
                    w, w, rng.gen::<f32>() * 2.0 * 3.14159, SpriteLayer::Entities.to_z());
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
            if self.display_scale < 4.75 {
                self.display_scale += 0.05;
            }            
        }
        // toggle edit mode - showing original level layout
        else if keycode == KeyCode::F1 {
            if self.mode == GameMode::Play {
                self.mode = GameMode::Edit;
            }
            else {
                self.mode = GameMode::Play;
            }
        }
        else if keycode == KeyCode::F10 {
            let mut game_state_writer = self.world.fetch_mut::<GameStateResource>();

            let mut new_fs_type : ggez::conf::FullscreenType = ggez::conf::FullscreenType::Windowed;
            match game_state_writer.window_mode.fullscreen_type {
                ggez::conf::FullscreenType::Windowed => {
                    new_fs_type = ggez::conf::FullscreenType::Desktop;
                },
                ggez::conf::FullscreenType::Desktop => {
                    new_fs_type = ggez::conf::FullscreenType::True;
                },
                ggez::conf::FullscreenType::True => {
                    new_fs_type = ggez::conf::FullscreenType::Windowed;
                }
            }
            game_state_writer.window_mode.fullscreen_type = new_fs_type;

            ggez::graphics::set_fullscreen(ctx, new_fs_type);
        }
        // reload current level
        else if keycode == KeyCode::R {
            self.load_level(ctx, self.current_level_name.clone(), self.current_entry_name.clone());
        }        
        //
        if keycode == KeyCode::RBracket {
            let new_level = (self.audio.base_music_volume + 0.05).min(1.0);
            self.audio.set_volume(new_level);
        }
        else if keycode == KeyCode::LBracket {
            let new_level = (self.audio.base_music_volume - 0.05).max(0.0);
            self.audio.set_volume(new_level);
        }
        else if keycode == KeyCode::K {
                
            self.set_running_state(ctx, RunningState::Dialog("K dialog".to_string()));
        }
        else if keycode == KeyCode::L {
            println!("DEBUG LOGIC 3x -------------------------------------------------");
            self.debug_logic_frames = 3;
        }
        

        let key = InputMap::key_up(&mut self.world, ctx, keycode, keymod);
        if let Some(i_key) = key {
            self.game_key_up(ctx, &i_key);
        }
        
    }

    fn gamepad_button_down_event(
        &mut self,
        ctx: &mut Context,
        btn: Button,
        id: GamepadId
    ) {
        //println!("gamepad_button_down: {:?}", &_btn);
        let key = InputMap::gamepad_button_down(&mut self.world, ctx, btn, id);
        if let Some(i_key) = key {
            self.game_key_down(ctx, &i_key);
        }
    }

    fn gamepad_button_up_event(
        &mut self,
        ctx: &mut Context,
        btn: Button,
        id: GamepadId
    ) {
        //println!("gamepad_button_up: {:?}", &_btn);
        let key = InputMap::gamepad_button_up(&mut self.world, ctx, btn, id);
        if let Some(i_key) = key {
            self.game_key_up(ctx, &i_key);
        }
    }

    fn gamepad_axis_event(
        &mut self,
        _ctx: &mut Context,
        _axis: Axis,
        _value: f32,
        _id: GamepadId
    ) {
        println!("gamepad_axis: {:?} {}", &_axis, &_value);

    }

    fn text_input_event(&mut self, _ctx: &mut Context, _ch: char) {
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

        ggez::graphics::set_screen_coordinates(ctx, Rect::new(0.0, 0.0, width, height)).expect("Failed to set coords on resize");

        drop(game_state_writer);
        
    }
}