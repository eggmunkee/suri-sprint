
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

use crate::core::game_state::{GameState};
use crate::components::collision::{Collision};
use crate::components::sprite::{SpriteConfig};
use crate::resources::{ImageResources,ShaderResources,ShaderInputs,GameStateResource};
use crate::conf::*;

#[derive(Debug,Default,Deserialize)]
pub struct AnimGridLayout {
    pub cols: i32,
    pub rows: i32,
}

#[derive(Debug,Default,Deserialize)]
pub struct AnimationDef {
    pub name: String,
    pub frames: Vec<(i32, f32)>,
    pub looped: bool,
    pub reverse: bool,
    pub end_anim: Option< Vec<String> >,
}

#[derive(Debug,Default,Deserialize)]
pub struct AnimSpriteConfig {
    pub spritesheet: bool,
    pub path: String,
    pub scale: (f32, f32),
    pub z_order: f32,
    pub alpha: f32,
    pub src: (f32, f32, f32, f32),
    pub shader: Option<String>,
    pub grid_layout: Option<AnimGridLayout>,
    pub animations: Option< Vec<AnimationDef> >,
    pub start_animation: Option< Vec<String> >,
}

impl AnimSpriteConfig {

    pub fn init_images(world: &mut World, ctx: &mut Context, path: String) {
        if let Some(mut images) = world.get_mut::<ImageResources>() {

            let has_image = images.has_image(path.clone());
            if !has_image {
                images.load_image(path.clone(), ctx);
            }
        }
    }

    pub fn create_from_config(world: &mut World, ctx: &mut Context, config_path: String) -> AnimSpriteComponent {

        let maybe_config = get_ron_config::<AnimSpriteConfig>(config_path.to_string());

        let config = maybe_config.expect(&format!("Invalid AnimSpriteConfig at {}", &config_path));

        Self::init_images(world, ctx, config.path.clone());

        //println!("Loaded AnimSpriteComponent from config");
        //println!("{:?}", &config);

        let mut sprite = AnimSpriteComponent::new(ctx, &config.path, config.z_order);

        sprite.scale.x = config.scale.0;
        sprite.scale.y = config.scale.1;
        sprite.alpha = config.alpha;
        sprite.src = Rect::new(config.src.0, config.src.1, config.src.2, config.src.3);
        sprite.shader = config.shader;
        sprite.grid_layout = config.grid_layout;
        sprite.animations = config.animations;
        sprite.start_animation = config.start_animation;

        sprite
    }

    pub fn create_from_sprite_config(world: &mut World, ctx: &mut Context, config_path: String) -> AnimSpriteComponent {

        let maybe_config = get_ron_config::<SpriteConfig>(config_path.to_string());

        let config = maybe_config.expect(&format!("Invalid SpriteConfig at {}", &config_path));

        Self::init_images(world, ctx, config.path.clone());

        //println!("Loaded AnimSpriteComponent from config");
        //println!("{:?}", &config);

        let mut sprite = AnimSpriteComponent::new(ctx, &config.path, config.z_order);

        sprite.scale.x = config.scale.0;
        sprite.scale.y = config.scale.1;
        sprite.alpha = config.alpha;
        sprite.src = Rect::new(config.src.0, config.src.1, config.src.2, config.src.3);
        sprite.shader = config.shader;
        sprite.grid_layout = None;
        sprite.animations = None;
        sprite.start_animation = None;
        sprite.is_enabled = false;

        sprite
    }
}

#[derive(Debug,Component)]
#[storage(DenseVecStorage)]
pub struct AnimSpriteComponent {
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
    pub grid_layout: Option<AnimGridLayout>,
    pub animations: Option< Vec<AnimationDef> >,
    pub start_animation: Option< Vec<String> >,
    pub curr_animation: String,
    pub frame: i32,
    pub frame_index: i32,
    pub frame_timer: f32,
    pub curr_frame_length: f32,
    pub curr_anim_looped: bool,
    pub curr_anim_finished: bool,
    pub is_enabled: bool,
    pub pos_dir: bool,
}

impl AnimSpriteComponent {
    pub fn new(ctx: &mut Context, char_img: &String, z_order: f32) -> AnimSpriteComponent {
        
        AnimSpriteComponent {
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
            grid_layout: None,
            animations: None,
            start_animation: None,
            curr_animation: "".to_string(),
            frame: 0,
            frame_index: 0,
            frame_timer: 0.0,
            curr_frame_length: 0.0,
            curr_anim_looped: true,
            curr_anim_finished: false,
            is_enabled: true,
            pos_dir: true,
        }
    }

    pub fn set_src(&mut self, src: &(f32, f32, f32, f32)) {
        self.src = Rect::new(src.0, src.1, src.2, src.3);
    }

    pub fn get_num_frames(&self) -> i32 {
        let mut num_frames : i32 = 0;
        if let Some(anims) = &self.animations {
            for anim_def in anims.iter() {
                // correct animation name
                if anim_def.name == self.curr_animation {
                    num_frames = anim_def.frames.len() as i32;
                }
            }
        }
        num_frames
    }

    pub fn set_animation(&mut self, name: &str) {
        self.curr_animation = name.to_string();
    }

    // pub fn advance_frame(&mut self) {

    //     self.advance_frame_offset(1);
    // }
    pub fn advance_frame_offset(&mut self, offset: i32) {

        let num_frames = self.get_num_frames();
        let mut new_frame = self.frame + offset;
        let mut can_advance = true;
        if new_frame >= num_frames || new_frame < 0 {
            new_frame = new_frame % num_frames;
            if !self.curr_anim_looped {
                can_advance = false;
                self.curr_anim_finished = true;
            }
        }

        if can_advance {
            self.set_frame(new_frame);
        }        

    }

    pub fn set_frame(&mut self, frame: i32) {
        self.frame = frame;
        let mut new_frame_idx = -1;
        let mut new_frame_len = -1.0;
        let mut new_anim_looped = self.curr_anim_looped;
        if let Some(anims) = &self.animations {
            for anim_def in anims.iter() {
                // correct animation name
                if anim_def.name == self.curr_animation {
                    //println!("Set Frame {} - LOOP: {} CURR LOOP: {}", &frame, &anim_def.looped, &self.curr_anim_looped);
                    new_anim_looped = anim_def.looped;
                    //new_frame_len = anim_def
                    let mut frame_idx = 0;
                    // find correct frame definition with time
                    for (frame_num, frame_len) in anim_def.frames.iter() {
                        if frame_idx == frame {
                            new_frame_idx = *frame_num;
                            new_frame_len = *frame_len;
                        }
                        frame_idx += 1;
                    }
                }
            }
        }
        if new_frame_len > 0.0 && new_frame_idx >= 0 {
            self.frame_index = new_frame_idx;
            self.frame_timer = 0.0;
            self.curr_frame_length = new_frame_len;
            self.curr_anim_looped = new_anim_looped;
            self.curr_anim_finished = false;
            //println!("  * set vals: Fr.Idx: {} CURR LOOP: {}", &new_frame_idx, &self.curr_anim_looped);
        }
    }

    pub fn get_frame_time(&self) -> f32 {
        let mut res = 0.0;
        if let Some(grid_layout) = &self.grid_layout {
            let frame_num = grid_layout.cols * grid_layout.rows;
            if self.curr_animation != "" {
                res = 0.3;
            }
            else {
                res = 0.3;
            }
        }

        res
    }

    pub fn calc_frame_src(x: i32, y: i32, rows: i32, cols: i32) -> (f32, f32, f32, f32) {

        let mut res = (0.0, 0.0, 1.0, 1.0);
        
        // set origin as col / row
        res.0 = x as f32 / cols as f32;
        res.1 = y as f32 / rows as f32;
        // set width and height as cell size in relation to 1
        res.2 = 1.0 / cols as f32;
        res.3 = 1.0 / rows as f32;

        //println!("AnimSprite Frame [{}:{}] Src: {:?}", &self.frame, &self.frame_index, &res);

        res
    }

    pub fn get_frame_src(&self) -> (f32, f32, f32, f32) {
        let mut res = (0.0, 0.0, 1.0, 1.0);
        if let Some(grid_layout) = &self.grid_layout {
            let max_cols = grid_layout.cols.max(1);
            let max_rows = grid_layout.rows.max(1);
            let mut col = 0;
            let mut row = 0;
            let mut f = 0;
            // Determine col & row
            while (f < self.frame_index) {
                col += 1;
                if col >= max_cols {
                    col = 0;
                    row += 1;
                    if row >= max_rows {
                        col = 0;
                        row = 0;
                        break;
                    }
                }
                f += 1;
            }
            // set origin as col / row
            res.0 = col as f32 / max_cols as f32;
            res.1 = row as f32 / max_rows as f32;
            // set width and height as cell size in relation to 1
            res.2 = 1.0 / max_cols as f32;
            res.3 = 1.0 / max_rows as f32;
        }
        else {
            res = (self.src.x, self.src.y, self.src.w, self.src.h);
        }

        //println!("AnimSprite Frame [{}:{}] Src: {:?}", &self.frame, &self.frame_index, &res);

        res
    }

    pub fn update(&mut self, delta_time: f32) {

        if !self.is_enabled { 
            return; 
        }

        if self.curr_frame_length > 0.0 {
            if !self.curr_anim_finished {
                self.frame_timer += delta_time;

                if self.frame_timer >= self.curr_frame_length {
                    let frame_offset = match &self.pos_dir {
                        true => 1,
                        false => 2
                    };
                    self.advance_frame_offset(frame_offset);
                }
            }
            else {
                //println!(" *@@@ ANIMATION FINISHED @@@*");
            }
        }
        else {
            if self.curr_animation == "" {
                if let Some(defaults) = &self.start_animation {
                    if defaults.len() > 0 {
                        self.curr_animation = defaults[0].clone();
                    }
                }
                else {
                    self.is_enabled = false;
                    self.set_frame(0);
                    return;
                }

                let num_f = self.get_num_frames() as u32;
                let mut rng = rand::thread_rng();
                let mut init_frame = 0;
                if num_f > 0 {
                    init_frame = (rng.next_u32() % num_f) as i32;
                }
                //println!("Init frame {}", &init_frame);
                self.set_frame(init_frame);

            }
        }

    }
}

impl super::RenderItemTarget for AnimSpriteComponent {
    fn render_item(game_state: &GameState, ctx: &mut Context, entity: &Entity,
        pos: &na::Point2<f32>, item_index: usize) {
            let world = &game_state.world;
            let anim_sprite_reader = world.read_storage::<AnimSpriteComponent>();

            // Get Sprite Component to call draw method            
            if let Some(anim_sprite) = anim_sprite_reader.get(entity.clone()) {
                use crate::components::{RenderTrait};
                anim_sprite.draw(ctx, world, Some(entity.id()), pos.clone(), item_index);
            }
        }

}

impl super::RenderTrait for AnimSpriteComponent {
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

        let frame_src = self.get_frame_src();
        let frame_src_rect = Rect::new(frame_src.0, frame_src.1, frame_src.2, frame_src.3);

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
                    .src(frame_src_rect)
                    .dest(draw_pos.clone())
                    .rotation(angle) //rotation
                    .offset(na::Point2::new(0.5f32,0.5f32))
                    .scale(self.scale)
                    .color(Color::new(1.0,1.0,1.0,self.alpha))) { 
                _draw_ok = false;
                println!("Failed to render anim sprite image");
            }
        }
        else {
            println!("Couldn't get sprite texture: {}", &self.path);
        }

    }
}


#[derive(Debug,Component)]
#[storage(DenseVecStorage)]
pub struct MultiAnimSpriteComponent {
    //pub image: Image, // component owns image
    pub sprites: Vec<AnimSpriteComponent>,
    //pub debug_font: graphics::Font,
}

impl MultiAnimSpriteComponent {
    pub fn new(ctx: &mut Context) -> MultiAnimSpriteComponent {
        
        MultiAnimSpriteComponent {
            //image: image,
            sprites: vec![],
        }
    }
}


impl super::RenderTrait for MultiAnimSpriteComponent {
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
    world.register::<AnimSpriteComponent>();
}
