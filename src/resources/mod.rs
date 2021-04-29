
use std::collections::{HashMap};
use ggez::graphics;
use ggez::graphics::{Image};
use ggez::{Context};
use specs::{World};

// -------------------------

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


pub fn add_resources(world: &mut World, ctx: &mut Context) {

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
        cmd_text: String::new(),
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