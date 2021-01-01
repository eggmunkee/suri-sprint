
use ggez::{Context};
use ggez::graphics;
use ggez::graphics::{Rect,Image,Color,DrawParam,WrapMode,BlendMode};
use ggez::nalgebra as na;
use specs::{Component, DenseVecStorage, World, WorldExt};
//use specs::shred::{Dispatcher};
use specs_derive::*;
use rand::prelude::*;
use serde::{Deserialize,de::DeserializeOwned};

// ================================

//use crate::game_state::{GameState};
use crate::components::collision::{Collision};
use crate::resources::{ImageResources,ShaderResources,ShaderInputs,GameStateResource};
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

#[derive(Debug,Deserialize)]
pub struct ShaderConfig {
    pub vert_path: String,
    pub frag_path: String,
    pub blend_modes: Vec<String>,
}

impl Default for ShaderConfig {
    fn default() -> Self {
        ShaderConfig {
            vert_path: "".to_string(),
            frag_path: "".to_string(),
            blend_modes: vec![],
        }
    }
}


#[derive(Debug,Default,Deserialize)]
pub struct SpriteConfig {
    pub spritesheet: bool,
    pub path: String,
    pub scale: (f32, f32),
    pub z_order: f32,
    pub alpha: f32,
    pub src: (f32, f32, f32, f32),
    pub shader: Option<String>,
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

        println!("Loading Sprite from config: {:?}", &config.path);
        Self::init_images(world, ctx, config.path.clone());

        let mut sprite = SpriteComponent::new(ctx, &config.path, config.z_order);

        sprite.scale.x = config.scale.0;
        sprite.scale.y = config.scale.1;
        sprite.alpha = config.alpha;
        sprite.src = Rect::new(config.src.0, config.src.1, config.src.2, config.src.3);
        sprite.shader = config.shader;

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
    pub angle: f32,
    pub src: Rect,
    pub visible: bool,
    pub toggleable: bool,
    pub shader: Option<String>,
}

impl SpriteComponent {
    pub fn new(ctx: &mut Context, char_img: &String, z_order: f32) -> SpriteComponent {
        
        SpriteComponent {
            //image: image,
            path: char_img.clone(),
            scale: na::Vector2::new(1.0,1.0),
            z_order: z_order,
            alpha: 1.0,
            angle: 0.0,
            src: Rect::new(0.0, 0.0, 1.0, 1.0),
            visible: true,
            toggleable: false,
            shader: None,
        }
    }

    pub fn set_src(&mut self, src: &(f32, f32, f32, f32)) {
        self.src = Rect::new(src.0, src.1, src.2, src.3);
    }
}


impl super::RenderTrait for SpriteComponent {
    fn draw(&self, ctx: &mut Context, world: &World, ent: Option<u32>, pos: na::Point2::<f32>, item_index: u32) {
        if !self.visible { return; }

        let mut rng = rand::thread_rng();

        // get sprite base angle
        let mut angle = self.angle;
        // Override angle with collision angle
        if let Some(ent_id) = ent {
            let collision_reader = world.read_storage::<Collision>();
            let entity = world.entities().entity(ent_id);
            if let Some(coll) = collision_reader.get(entity) {
                angle = coll.angle;
            }

        }

        let gs_res = world.fetch::<GameStateResource>();

        let level_run_time = gs_res.level_world_seconds;
        let game_run_time = gs_res.game_run_seconds;

        let mut shader_res = world.fetch_mut::<ShaderResources>();
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
            if self.src.x + self.src.h > 1.0 || self.src.y + self.src.h > 1.0 {
                //println!("Rendering source outside image: {:?}", &self.src);
            }
            texture.set_wrap(WrapMode::Tile, WrapMode::Tile);

            let mut _lock : Option<ggez::graphics::ShaderLock> = None;
            if let Some(shader_name) = &self.shader {
                if let Ok(shader_ref) = shader_res.shader_ref(shader_name.clone()) {
                    let mut dim = shader_ref.send(ctx, ShaderInputs {game_time: game_run_time});
                    _lock = Some(ggez::graphics::use_shader(ctx, shader_ref));
                }
            }

            if let Err(_) = ggez::graphics::draw(ctx, texture, DrawParam::new()
                    .src(self.src)
                    .dest(draw_pos.clone())
                    .rotation(angle) //rotation
                    .offset(na::Point2::new(0.5f32,0.5f32))
                    .scale(self.scale)
                    .color(Color::new(1.0,1.0,1.0,self.alpha))) { 
                _draw_ok = false;
                println!("Failed to render sprite image");
            }
        }
        else {
            println!("Couldn't get sprite texture: {}", &self.path);
        }

    }
}


#[derive(Debug,Component)]
#[storage(DenseVecStorage)]
pub struct MultiSpriteComponent {
    //pub image: Image, // component owns image
    pub sprites: Vec<SpriteComponent>,
    //pub debug_font: graphics::Font,
}

impl MultiSpriteComponent {
    pub fn new(ctx: &mut Context) -> MultiSpriteComponent {
        
        MultiSpriteComponent {
            //image: image,
            sprites: vec![],
        }
    }
}


impl super::RenderTrait for MultiSpriteComponent {
    fn draw(&self, ctx: &mut Context, world: &World, ent: Option<u32>, pos: na::Point2::<f32>, item_index: u32) {
        //println!("BallRender...");
        let mut rng = rand::thread_rng();

        if item_index >= 0 && (item_index as usize) < self.sprites.len() {

            if let Some(sprite) = self.sprites.get(item_index as usize) {
                sprite.draw(ctx, world, ent, pos, 0);
                // // get sprite base angle
                // let mut angle = sprite.angle;
                // // Override angle with collision angle
                // if let Some(ent_id) = ent {
                //     let collision_reader = world.read_storage::<Collision>();
                //     let entity = world.entities().entity(ent_id);
                //     if let Some(coll) = collision_reader.get(entity) {
                //         angle = coll.angle;
                //     }

                // }

                // let mut images = world.fetch_mut::<ImageResources>();
                // let texture_ref = images.image_ref(sprite.path.clone());

                // let mut _draw_ok = true;
                // // get centered draw position based on image dimensions
                // //let draw_pos = na::Point2::<f32>::new(pos.x - (w as f32 / 2.0), pos.y - (h as f32 / 2.0));
                // let draw_pos = na::Point2::<f32>::new(pos.x, pos.y);
                // // color part:  ,Color::new(1.0,0.7,0.7,1.0)
                // if let Ok(mut texture) = texture_ref {
                //     let w = texture.width();
                //     let h = texture.height();
                //     texture.set_wrap(WrapMode::Tile, WrapMode::Tile);
                //     if let Err(_) = ggez::graphics::draw(ctx, texture, (
                //                 draw_pos.clone(),
                //                 angle, //rotation
                //                 na::Point2::new(0.5f32,0.5f32),
                //                 sprite.scale,
                //                 Color::new(1.0,1.0,1.0,sprite.alpha))) { 
                //         _draw_ok = false;
                //         println!("Failed to render sprite image");
                //     }
                // }
                // else {
                //     println!("Couldn't get sprite texture: {}", &sprite.path);
                // }
            }

            
        }

        

    }
}


// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    //world.register::<PlayerComponent>();
    world.register::<SpriteComponent>();
    world.register::<MultiSpriteComponent>();
}