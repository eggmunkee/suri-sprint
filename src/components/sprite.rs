
use ggez::{Context};
use ggez::graphics;
use ggez::graphics::{Image,Color,DrawParam};
use ggez::nalgebra as na;
use specs::{Component, DenseVecStorage, World, WorldExt};
//use specs::shred::{Dispatcher};
use specs_derive::*;
use rand::prelude::*;

//use crate::game_state::{GameState};
use crate::components::collision::{Collision};
use crate::resources::{ImageResources};

#[derive(Debug,Component)]
#[storage(DenseVecStorage)]
pub struct SpriteComponent {
    //pub image: Image, // component owns image
    pub path: String,
    pub scale: na::Vector2::<f32>,
    //pub debug_font: graphics::Font,
}
// impl Component for SpriteComponent {
//     type Storage = DenseVecStorage<Self>;
// }

impl SpriteComponent {
    pub fn new(ctx: &mut Context, char_img: &String) -> SpriteComponent {
        
        SpriteComponent {
            //image: image,
            path: char_img.clone(),
            scale: na::Vector2::new(1.0,1.0),
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
        let mut texture_ref = images.image_ref(self.path.clone());

        let mut _draw_ok = true;
        // get centered draw position based on image dimensions
        //let draw_pos = na::Point2::<f32>::new(pos.x - (w as f32 / 2.0), pos.y - (h as f32 / 2.0));
        let draw_pos = na::Point2::<f32>::new(pos.x, pos.y);
        // color part:  ,Color::new(1.0,0.7,0.7,1.0)
        if let Ok(texture) = texture_ref {
            let w = texture.width();
            let h = texture.height();
            if let Err(_) = ggez::graphics::draw(ctx, texture, (
                        draw_pos.clone(),
                        angle, //rotation
                        na::Point2::new(0.5f32,0.5f32),
                        self.scale,
                        Color::new(1.0,1.0,1.0,1.0))) { // add back x/y pos  //
                _draw_ok = false;
                println!("Failed to render ball image");
            }
        }
        else {
            println!("Couldn't get texture: {}", &self.path);
        }

    }
}




// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    //world.register::<PlayerComponent>();
    world.register::<SpriteComponent>();
}