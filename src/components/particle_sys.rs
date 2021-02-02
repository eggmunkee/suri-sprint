
use ggez::{Context,GameResult};
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
use crate::components::sprite::{SpriteConfig,SpriteComponent};


#[derive(Debug,Default,Deserialize)]
pub struct ParticleSysConfig {
    pub width: f32,
    pub height: f32,
    pub scale: (f32, f32),
    pub ang: f32,
    pub ang_vel: f32,
    pub vel: (f32, f32),
    pub vel_acc: (f32, f32),
    pub ang_range: f32,
    pub ang_vel_range: f32,
    pub vel_range: (f32, f32),
    pub vel_acc_range: (f32, f32),
    pub sprite: Option<String>,
    pub start_alpha: f32,
    pub end_alpha: f32,
    pub color: Option<(f32, f32, f32, f32)>,
    pub emit_rate: f32,
    pub max_particles: usize,
    pub ttl: f32,
}

impl ParticleSysConfig {

    pub fn init_images(world: &mut World, ctx: &mut Context, path: String) {
        if let Some(mut images) = world.get_mut::<ImageResources>() {

            let has_image = images.has_image(path.clone());
            if !has_image {
                if let Ok(()) = images.load_image(path.clone(), ctx) {

                }
                else {
                    println!("Failed to load image: {}", &path);
                }
            }
        }
    }

    pub fn create_from_config(world: &mut World, ctx: &mut Context, config_path: String) -> ParticleSysComponent {

        let maybe_config = get_ron_config::<ParticleSysConfig>(config_path.to_string());

        let config = maybe_config.expect(&format!("Invalid ParticleSysConfig at {}", &config_path));

        if let Some(sprite_path) = &config.sprite {
            //println!("Loading Sprite from config: {:?}", &sprite_path);
            SpriteConfig::init_images(world, ctx, sprite_path.clone());
        }


        //println!("Loaded ParticleSys from config");
        //println!("{:?}", &config);

        let mut sys = ParticleSysComponent::new(ctx);
        sys.width = config.width;
        sys.height = config.height;
        sys.scale = config.scale;
        sys.ang = config.ang;
        sys.ang_vel = config.ang_vel;
        sys.vel = config.vel;
        sys.vel_acc = config.vel_acc;
        sys.ang_range = config.ang_range;
        sys.ang_vel_range = config.ang_vel_range;
        sys.vel_range = config.vel_range;
        sys.vel_acc_range = config.vel_acc_range;
        sys.sprite = config.sprite;
        sys.start_alpha = config.start_alpha;
        sys.end_alpha = config.end_alpha;
        sys.color = config.color;
        sys.emit_rate = config.emit_rate;
        sys.max_particles = config.max_particles;
        sys.particle_ttl = config.ttl;

        sys.init();

        sys
    }
}

#[derive(Debug,Default,Deserialize)]
pub struct ParticleData {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub visible: bool,
    pub alpha: f32,
    pub lifetime: f32,
    pub alive: bool,
}

impl ParticleData {
    pub fn create() -> Self {
        ParticleData {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            visible: true,
            alpha: 0.8,
            lifetime: 0.0,
            alive: true,
        }
    }
}


#[derive(Debug,Component)]
#[storage(DenseVecStorage)]
pub struct ParticleSysComponent {
    //pub image: Image, // component owns image
    pub z_order: f32,
    pub visible: bool,
    pub toggleable: bool,
    pub width: f32,
    pub height: f32,
    pub scale: (f32, f32),
    pub ang: f32,
    pub ang_vel: f32,
    pub vel: (f32, f32),
    pub vel_acc: (f32, f32),
    pub ang_range: f32,
    pub ang_vel_range: f32,
    pub vel_range: (f32, f32),
    pub vel_acc_range: (f32, f32),
    pub sprite: Option<String>,
    pub start_alpha: f32,
    pub end_alpha: f32,
    pub color: Option<(f32, f32, f32, f32)>,
    pub emit_timer: f32,
    pub emit_rate: f32,
    pub max_particles: usize,
    pub particle_ttl: f32,
    pub particles: Vec<ParticleData>,
}

impl ParticleSysComponent {
    pub fn new(ctx: &mut Context) -> ParticleSysComponent {
        
        let psys = ParticleSysComponent {
            z_order: 900.0,
            visible: true,
            toggleable: false,
            width: 10.0,
            height: 10.0,
            scale: (1.0, 1.0),
            ang: 0.0,
            ang_vel: 0.0,
            vel: (0.0, -10.0),
            vel_acc: (0.0, 10.0),
            ang_range: 0.0,
            ang_vel_range: 0.0,
            vel_range: (0.0, 0.0),
            vel_acc_range: (0.0, 0.0),
            sprite: None,
            start_alpha: 1.0,
            end_alpha: 0.2,
            color: None,
            emit_timer: 0.0,
            emit_rate: 0.2,
            max_particles: 100,
            particle_ttl: 2.0,
            particles: vec![],
        };

        psys
    }

    pub fn init(&mut self) {
        for i in 0..1000 {
            self.update(0.05);
        }
    }

    pub fn emit(&mut self) {

        let curr_count = self.particles.len();
        if curr_count >= self.max_particles {
            self.particles.remove(curr_count - 1);
        }

        let mut rng = rand::thread_rng();

        let mut particle = ParticleData::create();
        let xrand = (rng.gen::<f32>() - 0.5) * self.width;
        let yrand = (rng.gen::<f32>() - 0.5) * self.height;
        let vxrand = (rng.gen::<f32>() - 0.5) * self.vel_range.0;
        let vyrand = (rng.gen::<f32>() - 0.5) * self.vel_range.1;
        particle.x = xrand;
        particle.y = yrand;
        particle.vx = self.vel.0 + vxrand;
        particle.vy = self.vel.1 + vyrand;

        self.particles.insert(0, particle);
    }

    pub fn update(&mut self, time_delta: f32) {
        self.emit_timer += time_delta;

        while self.emit_timer > self.emit_rate {
            self.emit_timer -= self.emit_rate;
            self.emit();
        }

        let mut dead_indices : Vec<usize> = vec![];
        let mut index : usize = 0;
        for particle in &mut self.particles {
            if particle.alive == false {
                dead_indices.insert(0, index);
            }
            else {
                particle.y += 0.5 * time_delta;
                particle.lifetime += time_delta;
                if particle.lifetime > self.particle_ttl {
                    particle.alive = false;
                    dead_indices.insert(0, index);
                }
                else {
                    particle.x += particle.vx * time_delta;
                    particle.y += particle.vy * time_delta;
    
                    particle.vx += self.vel_acc.0 * time_delta;
                    particle.vy += self.vel_acc.1 * time_delta;
                }
            }
            
            index += 1;
        }
        
        for dead_part_index in dead_indices.iter() {
            self.particles.remove(*dead_part_index);
        }

    }
}


impl super::RenderTrait for ParticleSysComponent {
    fn draw(&self, ctx: &mut Context, world: &World, ent: Option<u32>, pos: na::Point2::<f32>, _item_index: usize) {
        if !self.visible { return; }

        let mut rng = rand::thread_rng();

        // get sprite base angle
        let mut angle = self.ang;
        
        let gs_res = world.fetch::<GameStateResource>();

        let level_run_time = gs_res.level_world_seconds;
        let game_run_time = gs_res.game_run_seconds;

        let mut shader_res = world.fetch_mut::<ShaderResources>();
        let mut images = world.fetch_mut::<ImageResources>();

        let draw_pos = na::Point2::<f32>::new(pos.x, pos.y);

        let mut texture_ref : Option<GameResult<&mut Image>> = None;
        let mut no_texture = true;
        if let Some(sprite) = &self.sprite {
            texture_ref = Some(images.image_ref(sprite.clone()));

            no_texture = false;
        }

        // if let Ok(rect) = ggez::graphics::Mesh::new_circle(ctx, 
        //     ggez::graphics::DrawMode::Stroke(ggez::graphics::StrokeOptions::default()),
        //     na::Point2::<f32>::new(0.0, 0.0),
        //     self.width, 0.5,
        //     ggez::graphics::Color::new(1.0, 1.0, 0.0, 0.5)
        // ) {
        //     ggez::graphics::draw(ctx, &rect, DrawParam::default()
        //         .dest(draw_pos.clone())
        //         .offset(na::Point2::new(self.width, self.width))
        //         .color(Color::new(1.0,0.0,1.0,0.1))
        //     );
        // }

        for particle in &self.particles {
            if particle.alive == false {
                continue;
            }

            let start_to_end = particle.lifetime / self.particle_ttl;
            let calc_alpha = ((self.start_alpha * (1.0 - start_to_end) ) + (self.end_alpha * start_to_end)).max(0.0).min(1.0);

            if no_texture {

                if let Ok(rect) = ggez::graphics::Mesh::new_circle(ctx, 
                    ggez::graphics::DrawMode::Stroke(ggez::graphics::StrokeOptions::default()),
                    na::Point2::<f32>::new(particle.x, particle.y),
                    5.0, 0.5,
                    ggez::graphics::Color::new(1.0, 1.0, 0.0, 0.5)
                ) {
                    ggez::graphics::draw(ctx, &rect, DrawParam::default()
                        .dest(draw_pos.clone())
                        .offset(na::Point2::new(self.width, self.width))
                        .color(Color::new(1.0,1.0,1.0,calc_alpha))
                    );
                }

            }
            
            else {
                if let Some(ref mut texture_rep_res) = texture_ref {
                    if let Ok(ref mut texture) = texture_rep_res {
                        let w = texture.width();
                        let h = texture.height();

                        let scaled_w = 0.0; //(w as f32) * self.scale.0;
                        let scaled_h = 0.0; //(h as f32) * self.scale.1;
                        //texture.set_wrap(WrapMode::Border, WrapMode::Border);
            
                        // let mut _lock : Option<ggez::graphics::ShaderLock> = None;
                        // if let Some(shader_name) = &self.shader {
                        //     if let Ok(shader_ref) = shader_res.shader_ref(shader_name.clone()) {
                        //         let mut dim = shader_ref.send(ctx, ShaderInputs {game_time: game_run_time});
                        //         _lock = Some(ggez::graphics::use_shader(ctx, shader_ref));
                        //     }
                        // }
                        let p_draw_pos = na::Point2::new(draw_pos.x + particle.x - (scaled_w * 0.5), draw_pos.y + particle.y - (scaled_h * 0.5));
            
                        if let Err(_) = ggez::graphics::draw(ctx, *texture, DrawParam::new()
                                .src( Rect::new(0.0, 0.0, 1.0, 1.0) )
                                .dest(p_draw_pos.clone())
                                .rotation(0.0) //rotation
                                .offset(na::Point2::new(0.5f32,0.5f32))
                                .scale(na::Vector2::new(self.scale.0,self.scale.1))
                                .color(Color::new(1.0,1.0,1.0,calc_alpha))) { 
                            //_draw_ok = false;
                            println!("Failed to render particle sys image");
                        }
                    }
                    
                }
                else {
                    //println!("Texture not ready for particle sys image: {:?}", &self.sprite);
                }
            }
            
        }

        

        

    }
}

// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    //world.register::<PlayerComponent>();
    world.register::<ParticleSysComponent>();
}