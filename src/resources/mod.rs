// use std::fmt;
// use std::fmt::{Display};
use std::collections::{HashMap};
use std::collections::hash_map::{Entry};
use ggez::graphics;
use ggez::graphics::{Image,Font};
use ggez::{Context,GameResult,GameError};
use ggez::conf::{WindowMode};
use specs::{World};
//use ggez::nalgebra as na;
// -------------------------

//use crate::core::physics::{PhysicsWorld,create_physics_world};
use crate::core::input::{InputKey};
use crate::entities::level_builder::{LevelBounds,LevelType};

mod game_state;
mod camera;
mod input;
mod image;
mod connection;
mod shaders;
mod log;

pub use crate::resources::game_state::*;
pub use crate::resources::camera::{Camera};
pub use crate::resources::input::*;
pub use crate::resources::image::*;
pub use crate::resources::connection::*;
pub use crate::resources::shaders::*;
pub use crate::resources::log::*;

/*
#[derive(Default,Debug)]
pub struct GameStateResource {
    pub window_w: f32,
    pub window_h: f32,
    pub window_mode: WindowMode,
    pub display_offset: (f32, f32),
    pub delta_seconds: f32,
    pub level_world_seconds: f32,
    pub game_run_seconds: f32,
    pub level_bounds: LevelBounds,
    pub level_type: LevelType,

    // global player stats
    pub player_count: i32,
    pub player_1_char_num: i32,

    // ai info
    pub player_target_loc: (f32, f32),

    // game status info
    pub points: i32,
}*/




pub fn add_resources(world: &mut World, ctx: &mut Context) {

    //let (win_w, win_h) = ggez::graphics::drawable_size(ctx);
    // let curr_win_mode = ggez::graphics::get_mode(ctx);
    // world.insert(GameStateResource {
    //     window_w: win_w, window_h: win_h,
    // });

    world.insert(GameLog {
        entries: vec![],
        max_keep: 10,
    });

    // Insert the Input Resource which holds game inputs state
    world.insert(InputResource { 
        dirs_pressed: [false,false,false,false],
        jump_pressed: false,
        mouse_x: 0.0,
        mouse_y: 0.0,
        mouse_down: [false,false,false],
        fire_pressed: false,
        use_pressed: false,
        actions: vec![],
        keys_pressed: vec![],
        exit_flag: false,
        //cmd_text: String::new(),
    });


    // Insert Image Resources to hold and lend refs to images and the system font
    world.insert(ImageResources {
        image_lookup: HashMap::new(),
        images: Vec::<Image>::new(),
        font: graphics::Font::new(ctx, "/FreeMonoBold.ttf").unwrap(),
    });


    // Insert Shader Resources - preload all required shader files
    let mut shaders = ShaderResources::new();
    shaders.load_shader("overlay".to_string(), "shaders/overlay_shader".to_string(), ctx).expect("MISSING REQUIREMENT");
    shaders.load_shader("suri_shader".to_string(), "shaders/suri_shader".to_string(), ctx).expect("MISSING REQUIREMENT");
    shaders.load_shader("meow_shader".to_string(), "shaders/meow_shader".to_string(), ctx).expect("MISSING REQUIREMENT");
    shaders.load_shader("suri_shadow".to_string(), "shaders/suri_shadow".to_string(), ctx).expect("MISSING REQUIREMENT");
    shaders.load_shader("milo_shadow".to_string(), "shaders/milo_shadow".to_string(), ctx).expect("MISSING REQUIREMENT");

    world.insert(shaders);

    world.insert(ConnectionResource::new());

    world.insert(Camera { display_offset: (0.0, 0.0), snap_view: true, target_offset: (0.0, 0.0), following: None });

}