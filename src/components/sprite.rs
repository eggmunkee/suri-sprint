
use ggez::{Context};
use ggez::graphics;
use ggez::graphics::{Image,Color,DrawParam,WrapMode};
use ggez::nalgebra as na;
use specs::{Component, DenseVecStorage, World, WorldExt};
//use specs::shred::{Dispatcher};
use specs_derive::*;
use rand::prelude::*;
use serde::{Deserialize,de::DeserializeOwned};

// ================================

//use crate::game_state::{GameState};
use crate::components::collision::{Collision};
use crate::resources::{ImageResources};
use crate::conf::*;

#[allow(dead_code)]
#[derive(Copy,Clone,Debug)]
pub enum SpriteLayer {
    BG = 0,
    BGNear = 50,
    World = 100,
    Entities = 300,
    PlayerBehind = 490,
    Player = 500,
    PlayerFront = 510,
    UI = 1000
}

impl SpriteLayer {
    pub fn to_z(&self) -> f32 {
        let l : i32 = *self as i32;
        l as f32
    }
}


#[derive(Debug,Default,Deserialize)]
pub struct SpriteConfig {
    pub spritesheet: bool,
    pub path: String,
    pub scale: (f32, f32),
    pub z_order: f32,
    pub alpha: f32,
}

impl SpriteConfig {

    pub fn init_images(world: &mut World, ctx: &mut Context, path: String) {
        if let Some(mut images) = world.get_mut::<ImageResources>() {

            let has_image = images.has_image(path.clone());
            if !has_image {
                images.load_image(path.clone(), ctx);
            }
        }
    }

    pub fn create_from_config(world: &mut World, ctx: &mut Context, config_path: String) -> SpriteComponent {

        let maybe_config = get_ron_config::<SpriteConfig>(config_path.to_string());

        let config = maybe_config.expect(&format!("Invalid SpriteConfig at {}", &config_path));

        Self::init_images(world, ctx, config.path.clone());

        let mut sprite = SpriteComponent::new(ctx, &config.path, config.z_order);

        sprite.scale.x = config.scale.0;
        sprite.scale.y = config.scale.1;
        sprite.alpha = config.alpha;

        sprite
    }
}

#[derive(Debug,Component)]
#[storage(DenseVecStorage)]
pub struct SpriteComponent {
    //pub image: Image, // component owns image
    pub path: String,
    pub scale: na::Vector2::<f32>,
    pub z_order: f32,
    pub alpha: f32,
    //pub debug_font: graphics::Font,
}

impl SpriteComponent {
    pub fn new(ctx: &mut Context, char_img: &String, z_order: f32) -> SpriteComponent {
        
        SpriteComponent {
            //image: image,
            path: char_img.clone(),
            scale: na::Vector2::new(1.0,1.0),
            z_order: z_order,
            alpha: 1.0,
        }
    
        
    }
}


impl super::RenderTrait for SpriteComponent {
    fn draw(&self, ctx: &mut Context, world: &World, ent: Option<u32>, pos: na::Point2::<f32>) {
        //println!("BallRender...");
        let mut rng = rand::thread_rng();

        let mut angle = 0.0;
        if let Some(ent_id) = ent {
            let collision_reader = world.read_storage::<Collision>();
            let entity = world.entities().entity(ent_id);
            if let Some(coll) = collision_reader.get(entity) {
                angle = coll.angle;
            }

        }

        let mut images = world.fetch_mut::<ImageResources>();
        let texture_ref = images.image_ref(self.path.clone());

        let mut _draw_ok = true;
        // get centered draw position based on image dimensions
        //let draw_pos = na::Point2::<f32>::new(pos.x - (w as f32 / 2.0), pos.y - (h as f32 / 2.0));
        let draw_pos = na::Point2::<f32>::new(pos.x, pos.y);
        // color part:  ,Color::new(1.0,0.7,0.7,1.0)
        if let Ok(mut texture) = texture_ref {
            let w = texture.width();
            let h = texture.height();
            texture.set_wrap(WrapMode::Tile, WrapMode::Tile);
            if let Err(_) = ggez::graphics::draw(ctx, texture, (
                        draw_pos.clone(),
                        angle, //rotation
                        na::Point2::new(0.5f32,0.5f32),
                        self.scale,
                        Color::new(1.0,1.0,1.0,self.alpha))) { 
                _draw_ok = false;
                println!("Failed to render sprite image");
            }
        }
        else {
            println!("Couldn't get sprite texture: {}", &self.path);
        }

    }
}




// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    //world.register::<PlayerComponent>();
    world.register::<SpriteComponent>();
}