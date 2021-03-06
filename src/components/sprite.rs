
use ggez::{Context};
use ggez::graphics;
use ggez::graphics::{Rect,Image,Color,DrawParam,WrapMode,BlendMode};
use ggez::nalgebra as na;
use specs::{Component, DenseVecStorage, World, WorldExt, Entity};
//use specs::shred::{Dispatcher};
use specs_derive::*;
use rand::prelude::*;
use serde::{Deserialize,de::DeserializeOwned};

// ================================

use crate::entities::level::{LevelItem};
use crate::core::game_state::{GameState};
use crate::components::collision::{Collision};
use crate::resources::{ImageResources,ShaderResources,ShaderInputs,GameStateResource,Camera};
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

        //println!("Loading Sprite from config: {:?}", &config.path);
        Self::init_images(world, ctx, config.path.clone());

        let mut sprite = SpriteComponent::new(ctx, &config.path, config.z_order);

        sprite.scale.x = config.scale.0;
        sprite.scale.y = config.scale.1;
        sprite.alpha = config.alpha;
        sprite.src = Rect::new(config.src.0, config.src.1, config.src.2, config.src.3);
        sprite.shader = config.shader;

        sprite
    }

    pub fn create_from_path(world: &mut World, ctx: &mut Context, image_path: String) -> SpriteComponent {

        Self::init_images(world, ctx, image_path.clone());

        let mut sprite = SpriteComponent::new(ctx, &image_path, 9999.0);

        sprite.scale.x = 1.0;
        sprite.scale.y = 1.0;
        sprite.alpha = 1.0;
        sprite.src = Rect::new(0.0, 0.0, 1.0, 1.0);
        sprite.shader = None;

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

    /*pub fn render_item(game_state: &GameState, ctx: &mut Context, entity: &Entity,
        pos: &na::Point2<f32>, item_index: usize) {
            let world = &game_state.world;
            let sprite_reader = world.read_storage::<SpriteComponent>();
            let collision_reader = world.read_storage::<Collision>();

            // Get Sprite Component to call draw method            
            if let Some(sprite) = sprite_reader.get(entity.clone()) {
                use crate::components::{RenderTrait};
                sprite.draw(ctx, world, Some(entity.id()), pos.clone(), item_index);
            }
        }*/
}

impl super::RenderItemTarget for SpriteComponent {
    fn render_item(game_state: &GameState, ctx: &mut Context, entity: &Entity,
        pos: &na::Point2<f32>, item_index: usize) {
            let world = &game_state.world;
            let sprite_reader = world.read_storage::<SpriteComponent>();
            let collision_reader = world.read_storage::<Collision>();

            // Get Sprite Component to call draw method            
            if let Some(sprite) = sprite_reader.get(entity.clone()) {
                use crate::components::{RenderTrait};
                sprite.draw(ctx, world, Some(entity.id()), pos.clone(), item_index);
            }
        }
}


impl super::RenderTrait for SpriteComponent {
    fn draw(&self, ctx: &mut Context, world: &World, ent: Option<u32>, pos: na::Point2::<f32>, _item_index: usize) {
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

impl super::RenderItemTarget for MultiSpriteComponent {
    fn render_item(game_state: &GameState, ctx: &mut Context, entity: &Entity,
        pos: &na::Point2<f32>, item_index: usize) {
            let world = &game_state.world;
            let sprite_reader = world.read_storage::<MultiSpriteComponent>();

            // Get Sprite Component to call draw method            
            if let Some(sprite) = sprite_reader.get(entity.clone()) {
                use crate::components::{RenderTrait};
                sprite.draw(ctx, world, Some(entity.id()), pos.clone(), item_index);
            }
        }
}


impl super::RenderTrait for MultiSpriteComponent {
    fn draw(&self, ctx: &mut Context, world: &World, ent: Option<u32>, pos: na::Point2::<f32>, item_index: usize) {
        //println!("BallRender...");
        let mut rng = rand::thread_rng();

        if item_index >= 0 && item_index < self.sprites.len() {

            if let Some(sprite) = self.sprites.get(item_index) {
                sprite.draw(ctx, world, ent, pos, 0);

            }
        }
    }
}


#[derive(Debug,Component)]
#[storage(DenseVecStorage)]
pub struct ParallaxSpriteComponent {
    pub lvl_center: (f32, f32),
    //pub image: Image, // component owns image
    pub sprites: Vec<SpriteComponent>,
    //pub debug_font: graphics::Font,
    pub scroll_mults: Vec<f32>,
    pub offsets: Vec<(f32,f32)>,
}

impl ParallaxSpriteComponent {
    pub fn new(ctx: &mut Context) -> ParallaxSpriteComponent {
        
        ParallaxSpriteComponent {
            lvl_center: (0.0, 0.0),
            sprites: vec![],
            scroll_mults: vec![],
            offsets: vec![],
        }
    }

    pub fn add_sprite(&mut self, ctx: &mut Context, sprite: SpriteComponent, scroll_multiplier: f32, offset: (f32, f32)) -> i32 {
        // Push normal Sprite component to list
        self.sprites.push(sprite);
        // Push the scroll multiplier amount for this sprite
        self.scroll_mults.push(scroll_multiplier);
        self.offsets.push( offset.clone() );

        self.sprites.len() as i32 - 1
    }
}

impl super::RenderItemTarget for ParallaxSpriteComponent {
    fn render_item(game_state: &GameState, ctx: &mut Context, entity: &Entity,
        pos: &na::Point2<f32>, item_index: usize) {
            let world = &game_state.world;

            let (scrw, scrh) = (game_state.window_w as f32, game_state.window_h as f32);
            let camera = world.fetch::<Camera>();
            let mut display_offset = na::Point2::new(camera.display_offset.0, camera.display_offset.1);
            drop(camera);

            let plx_sprite_reader = world.read_storage::<ParallaxSpriteComponent>();

            if let Some(parallax_sprite) = plx_sprite_reader.get(entity.clone()) {

                let mut offset = (0.0, 0.0);
                let mut mult = 1.0;
                if item_index >= 0 && item_index < parallax_sprite.offsets.len() {
                    if let Some(offset_val) = parallax_sprite.offsets.get(item_index) {
                        offset.0 = offset_val.0;
                        offset.1 = offset_val.1;
                        //println!("  Render Parallax Item - scroll mult: {:?}", &offset);
                    }
                }
                if item_index >= 0 && item_index < parallax_sprite.scroll_mults.len() {
                    if let Some(multiplier) = parallax_sprite.scroll_mults.get(item_index) {
                        mult = *multiplier;
                        //println!("  Render Parallax Item - scroll mult: {}", &mult);
                    }
                }
                mult = mult.min(1.0).max(0.0);
                let anti_mult = 1.0 - mult;

                // Get Sprite Component to call draw method            
                // if let Some(sprite) = sprite_reader.get(entity.clone()) {
                //     use crate::components::{RenderTrait};
                //     sprite.draw(ctx, world, Some(entity.id()), pos.clone(), item_index);
                // }

                let mut curr_x_off = display_offset.x;// + (scrw * 0.5);
                let mut curr_y_off = display_offset.y;// - (scrh * 0.5);
                
                //println!("Render Parallax Item: {} Offset: ({}, {})", &item_index, &curr_x_off, &curr_y_off);

                curr_x_off = curr_x_off * mult - parallax_sprite.lvl_center.0 * anti_mult;
                curr_y_off = curr_y_off * mult - parallax_sprite.lvl_center.1 * anti_mult;
                //println!("Render Parallax Item - Updated Offset: ({}, {})", &curr_x_off, &curr_y_off);
                //println!("Render Parallax Item - Position: ({}, {})", &pos.x, &pos.y);

                let mut parallax_pos = pos.clone();
                parallax_pos.x -= curr_x_off;
                parallax_pos.y -= curr_y_off;
                parallax_pos.x += offset.0;
                parallax_pos.y += offset.1;

                //println!("Render Parallax Item - Position: ({}, {})", &parallax_pos.x, &parallax_pos.y);

                if item_index >= 0 && item_index < parallax_sprite.sprites.len() {
                    use crate::components::{RenderTrait};
                    if let Some(sprite) = parallax_sprite.sprites.get(item_index) {
                        sprite.draw(ctx, world, Some(entity.id()), parallax_pos, 0);
        
                    }
                }
            }
        }
}

impl super::RenderTrait for ParallaxSpriteComponent {
    fn draw(&self, ctx: &mut Context, world: &World, ent: Option<u32>, pos: na::Point2::<f32>, item_index: usize) {
        //println!("BallRender...");
        let mut rng = rand::thread_rng();

        if item_index >= 0 && item_index < self.sprites.len() {

            if let Some(sprite) = self.sprites.get(item_index) {
                sprite.draw(ctx, world, ent, pos, 0);

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
    world.register::<ParallaxSpriteComponent>();
}