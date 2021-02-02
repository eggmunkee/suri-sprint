
// Graphics engine, math, graphics context, window mode
use ggez;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use ggez::conf::{WindowMode};
// SPECS game world - world and system running support
use specs::{World, WorldExt, RunNow};
// Physics class support
use wrapped2d::b2;

//use std::collections::hash_map::*;
//use ggez::event::{self, KeyCode, KeyMods, MouseButton};
//use ggez::event::{GamepadId, Button, Axis};
//use winit::dpi::{LogicalPosition};
//use ggez::graphics;
//use ggez::graphics::{Rect};
//use specs::Join;
//use rand::prelude::*;
//use std::collections::{HashMap};
//use wrapped2d::user_data::*;

// =====================================
// GAME STATE internal support
// Audio system
use crate::core::audio::{Audio};
// Create Specs world support
use crate::core::world::{create_world};
// Physics support
use crate::core::physics;
use crate::core::physics::{PhysicsWorld,ContactFilterConfig};
// Game InputKey support
use crate::core::input::{InputKey};
// Core Game System - runs other systems and physics simulation
use crate::core::system::{CoreSystem};
// Global SPECS Resource classes
use crate::resources::{InputResource,GameStateResource,ConnectionResource};
// Level definition support
use crate::entities::level_builder::{LevelConfig,LevelBounds,LevelType};
// Render class support
use crate::render;

/* use crate::components::{Position};
use crate::components::collision::{Collision};
use crate::components::sprite::{SpriteLayer,SpriteComponent};
use crate::components::anim_sprite::{AnimSpriteComponent};
use crate::components::meow::{MeowComponent};
use crate::components::pickup::{PickupComponent};
use crate::components::exit::{ExitComponent};
use crate::components::portal::{PortalComponent};
use crate::components::button::{ButtonComponent};
use crate::components::player::{CharacterDisplayComponent};
use crate::components::npc::{NpcComponent};
use crate::systems::logic::{LogicSystem};
use crate::systems::animation::{AnimationSystem};
use crate::systems::{InputSystem};
use crate::entities::meow::{MeowBuilder};
use crate::systems::particles::{ParticleSystem};*/
//use crate::entities::ghost::{GhostBuilder};
//use crate::entities::platform::{PlatformBuilder};
//use crate::entities::empty_box::{BoxBuilder};

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
    pub gravity_scale: f32,
    pub gravity_x: f32,
    //pub image_lookup: HashMap<String,usize>,
    //pub images: Vec<Image>
    pub paused_text: graphics::Text,
    pub cursor_offset: na::Point2::<f32>,
    pub ghost_player_contact: bool,

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
    pub fn new(ctx: &mut Context, title: String, window_mode: WindowMode,
        music_volume: f32, gravity: f32) -> GameResult<GameState> {

        println!("* Creating GameState...");

        let ghost_player_contact = true;
        // Create physics world to place in game state resource
        println!("  o Physics init...");
        let mut physics_world = physics::create_physics_world_2d_grav((gravity, 0.0));
        // FIX CONTACT FILTER ERROR - TOGGLEABLE PLATFORM BUG - still impassable after UPdate body objstruction to false
        // let contact_f : ContactFilterConfig = ContactFilterConfig::new(ghost_player_contact);
        // physics_world.set_contact_filter(Box::from(contact_f));

        // Create game state related to window size/mode
        let (win_w, win_h) = ggez::graphics::drawable_size(ctx);
        let game_state_resource = GameStateResource {
            window_w: win_w, window_h: win_h, window_mode: window_mode,
            delta_seconds: 0.15, level_bounds: LevelBounds::new(-500.0, -500.0, 3000.0, 3000.0),
            level_world_seconds: 0.0, game_run_seconds: 0.0, player_target_loc: (500.0, 500.0),
            player_count: 0, player_1_char_num: -1, level_type: LevelType::default(),
        };

        // get window
        println!("  o Window init...");
        let window = ggez::graphics::window(ctx);
        window.set_position((1100.0,50.0).into());

        let font = graphics::Font::new(ctx, "/FreeMonoBold.ttf")?;
        let text = graphics::Text::new(("PAUSED", font, 52.0));

        println!("  o World init...");
        let ecs_world = create_world(ctx, game_state_resource, &mut physics_world);

        println!("  o Audio init...");
        let mut audio_engine = Audio::new(ctx);
        audio_engine.set_volume(music_volume);

        // Create main state instance with dispatcher and world
        let mut s = GameState { 
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
            gravity_scale: gravity,
            gravity_x: 0.0,
            ghost_player_contact: ghost_player_contact,
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
            debug_logic_frames: 0,
        };

        s.update_contact_filter();

        Ok(s)
    }

}

impl GameState {

    pub fn set_gravity(&mut self, gravity: f32) {
        self.gravity_scale = gravity;
        physics::update_world_gravity(&mut self.phys_world, gravity);
    }

    pub fn set_gravity_ext(&mut self, gravity: (f32, f32)) {
        //self.gravity_scale = gravity;
        physics::update_world_gravity_2d(&mut self.phys_world, gravity);
    }

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

    // #[allow(dead_code)]
    // pub fn run_dialog_update(&mut self, _ctx: &mut Context, _time_delta: f32) {
        
    //     {
    //         let world = &mut self.world;
    //         let mut input_sys = InputSystem::new();
    //         input_sys.run_now(&world);
    //     }
    // }

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
            time_delta *= 1.0;// 0.3;
        }
        time_delta
    }

    // pub fn run_update_step(&mut self, ctx: &mut Context, time_delta: f32) {

    //     CoreSystem::run_update_step(self, ctx, time_delta);
    // }

    pub fn clear_world(&mut self) {
        println!("Clearing world...");
        // Clear world of entity and component data
        self.world.delete_all();

        // Clear Connection logic resources
        let mut conn_res = self.world.fetch_mut::<ConnectionResource>();
        conn_res.clear();        
    }

    pub fn clear_physics(&mut self) {
        println!("Clearing physics world...");
        // Drop and replace the physics world
        //self.phys_world = physics::create_physics_world(self.gravity_scale);
        self.phys_world = physics::create_physics_world_2d_grav((self.gravity_x, self.gravity_scale));
        
        self.update_contact_filter();
    }

    pub fn update_contact_filter(&mut self) {
        // FIX CONTACT FILTER ERROR - TOGGLEABLE PLATFORM BUG - still impassable after UPdate body objstruction to false
        let contact_f : ContactFilterConfig = ContactFilterConfig::new(self.ghost_player_contact);
        self.phys_world.set_contact_filter(Box::from(contact_f));
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

    pub fn set_level_type(&mut self) {
        let mut lvl_type = LevelType::default();
        if let Some(level_type) = &self.level.level_type {
            lvl_type = level_type.clone();
        }
        let world = &mut self.world;
        let mut game_res = world.fetch_mut::<GameStateResource>();
        game_res.level_type = lvl_type.clone();
        drop(game_res);
        
        match &lvl_type {
            LevelType::Platformer => {
                self.set_gravity_ext((self.gravity_x, self.gravity_scale));
            },
            LevelType::Overhead => {
                self.set_gravity_ext((0.0, 0.0));
            }
        };
    }

    pub fn start_warp(&mut self, level_name: String, entry_name: String) {
        //self.running_state = RunningState::Dialog(format!("Level {}, entry {}", &level_name, &entry_name));

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

        if self.level.soundtrack != "" {
            self.audio.play_music(ctx, format!("/audio/{}", &self.level.soundtrack));
        }

        // set game state level bounds
        self.set_level_bounds();
        // get game state level type
        self.set_level_type();

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
        self.snap_view = true;

        //self.stop_music(ctx);

        self.clear_world();
        self.clear_physics();
        {
            let mut game_state = self.world.fetch_mut::<GameStateResource>();
            game_state.player_count = 0;
            game_state.player_1_char_num = 1;
        }

        self.actual_load_level(ctx, level_name, entry_name);

        if self.current_entry_name.is_empty() {
            self.running_state = RunningState::Dialog(format!("Suri enters {}.\r\n\r\n\r\n\r\nPress Meow to Continue...", &self.level.name));
            self.audio.set_dimmed(true);
        }
        else {
            self.running_state = RunningState::Playing;
        }

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

    pub fn handle_update_event(&mut self, ctx: &mut Context) -> GameResult {
        let time_scale = 1.0;
        let delta_s = time_scale * (ggez::timer::duration_to_f64(ggez::timer::delta(ctx)) as f32);
  
        // Only update world state when game is running (not paused)
        match &self.current_state {
            State::Running => {

                //self.run_update_step(ctx, delta_s);
                CoreSystem::run_update_step(self, ctx, delta_s);

            },
            State::Paused => {

                self.paused_anim += delta_s;

                // Run one update step per second while paused
                if self.paused_anim > 1.0 {
                    //self.run_update_step(ctx, delta_s);
                    CoreSystem::run_update_step(self, ctx, delta_s);
                    self.paused_anim = 0.0;
                }
            }
        }

        // always ok result
        Ok(())
    }

    pub fn handle_render_event(&mut self, ctx: &mut Context) -> GameResult {
        
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
}

