
use ggez::{Context};
use ggez::graphics;
use ggez::graphics::{Image,Color,DrawParam};
use ggez::nalgebra as na;
use specs::{Component, DenseVecStorage, World, WorldExt};
//use specs::shred::{Dispatcher};
use rand::prelude::*;

//use crate::game_state::{GameState};
use crate::resources::{ImageResources};

#[derive(Debug)]
pub struct BallDisplayComponent {
    //pub image: Image, // component owns image
    pub path: String,
    pub flash: bool,
    //pub debug_font: graphics::Font,
}
impl Component for BallDisplayComponent {
    type Storage = DenseVecStorage<Self>;
}

impl BallDisplayComponent {
    pub fn new(ctx: &mut Context, char_img: &String, flash: bool) -> BallDisplayComponent {
        
        //let image = Image::new(ctx, char_img.clone()).unwrap();

        //let font = graphics::Font::new(ctx, "/FreeMonoBold.ttf").unwrap();        
        // let text = graphics::Text::new((format!("#{}", &entity.id()), font, 14.0));
        // let (tw, th) = (text.width(ctx), text.height(ctx));
        
        // if let Err(_) = ggez::graphics::draw(ctx, &text, (na::Point2::new(
        //         pos.x - (tw as f32 / 2.0), pos.y + 35.0 - (th as f32 / 2.0)), 
        //         Color::new(0.0,0.0,0.0,1.0)) ) {
        //     draw_ok = false;
        // }
        
        BallDisplayComponent {
            //image: image,
            path: char_img.clone(),
            flash: flash
        }
    
        
    }
    // pub fn draw(&self, ctx: &mut Context, pos: na::Point2::<f32>) {
    //     let mut rng = rand::thread_rng();
    //     let mut draw_ok = true;
    //     let w = self.image.width();
    //     let h = self.image.height();
    //     let draw_pos = na::Point2::<f32>::new(pos.x - (w as f32 / 2.0), pos.y - (h as f32 / 2.0));
    //     // color part:  ,Color::new(1.0,0.7,0.7,1.0)
    //     if let Err(_) = ggez::graphics::draw(ctx, &self.image, (draw_pos.clone(),)) { // add back x/y pos  //
    //         draw_ok = false;
    //     }

    //     if let Ok(rect) = graphics::Mesh::new_rectangle(
    //         ctx,
    //         graphics::DrawMode::fill(),
    //         graphics::Rect::from([0.0,0.0,4.0,4.0]),
    //         graphics::WHITE,
    //     ) {
    //         let mut col_vals: (u8,) = rng.gen();
    //         //println!("Entity {}, Circle pos: {:?}", ent.id(), pos);
    //         if let Err(_) = graphics::draw(ctx, &rect, (na::Point2::new(pos.x-2.0, pos.y-2.0),
    //                 Color::from_rgba(col_vals.0,col_vals.0,col_vals.0,255) )) {
    //             draw_ok = false;
    //         };  
    //     }
    // }
}

// pub trait PlayerRenderTrait {
//     fn draw(&self, ctx: &mut Context, pos: na::Point2::<f32>);
// }

impl super::RenderTrait for BallDisplayComponent {
    fn draw(&self, ctx: &mut Context, world: &World, _ent: Option<u32>, pos: na::Point2::<f32>) {
        //println!("BallRender...");
        let mut rng = rand::thread_rng();

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
                        0.0f32, //rotation
                        na::Point2::new(0.5f32,0.5f32),
                        na::Vector2::new(1.0f32,1.0f32),
                        Color::new(1.0,1.0,1.0,1.0))) { // add back x/y pos  //
                _draw_ok = false;
                println!("Failed to render ball image");
            }
        }
        else {
            println!("Couldn't get texture: {}", &self.path);
        }

        // if let Some(entity_id) = ent {

        //     //let img_res = self.type_id()

        //     //let font = graphics::Font::new(ctx, "/FreeMonoBold.ttf").unwrap();        
        //     // let font = self.debug_font.clone();
        //     // let id_text : String = format!("#{}", &entity.id());
        //     // let text = graphics::Text::new((id_text, font, 14.0));
        //     // let (tw, th) = (text.width(ctx), text.height(ctx));
            
        //     // if let Err(_) = ggez::graphics::draw(ctx, &text, (na::Point2::new(
        //     //         pos.x - (tw as f32 / 2.0), pos.y + 35.0 - (th as f32 / 2.0)), 
        //     //         Color::new(0.0,0.0,0.0,1.0)) ) {
        //     //     draw_ok = false;
        //     // }
        // }
        
        if self.flash {
            // draw translucent flashing circle over interior of circle texture
            match graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                na::Point2::new(0.0, 0.0),
                23.0, // bleed over 2 pixel edge of texture 
                0.5,
                graphics::WHITE,
            ) 
            {
                Ok(circle) => {
                    let mut col_vals: (u8,u8,u8) = rng.gen();
                    if col_vals.0 < 27 { col_vals.0 += 228; }
                    if col_vals.1 < 27 { col_vals.1 += 228; }
                    if col_vals.2 < 27 { col_vals.2 += 228; }
                    let mut _draw_ok = true;
                    //println!("Entity {}, Circle pos: {:?}", ent.id(), pos);
                    if let Err(_) = graphics::draw(ctx, &circle, DrawParam::default()
                                .dest(na::Point2::new(pos.x, pos.y))
                                .scale(na::Vector2::new(1.0f32,1.0f32))
                                .rotation(0.75)
                                .color(Color::from_rgba(col_vals.0,col_vals.1,col_vals.2,128)) ) {
                        _draw_ok = false;
                    }; 
                }
                _ => {}
            };
        }
        

           


        // let mut draw_ok = true;
        // let w = self.image.width();
        // let h = self.image.height();
        // let draw_pos = na::Point2::<f32>::new(pos.x - (w as f32 / 2.0), pos.y - (h as f32 / 2.0));
        // // color part:  ,Color::new(1.0,0.7,0.7,1.0)
        // if let Err(_) = ggez::graphics::draw(ctx, &self.image, (draw_pos.clone(),)) { // add back x/y pos  //
        //     draw_ok = false;
        // }

        // if let Ok(rect) = graphics::Mesh::new_rectangle(
        //     ctx,
        //     graphics::DrawMode::stroke(1.0),
        //     graphics::Rect::from([0.0,0.0,50.0,50.0]),
        //     graphics::BLACK,
        // ) {
        //     let mut col_vals: (u8,) = rng.gen();
        //     //println!("Entity {}, Circle pos: {:?}", ent.id(), pos);
        //     if let Err(_) = graphics::draw(ctx, &rect, (na::Point2::new(pos.x-25.0, pos.y-25.0), )) {
                
        //     };  
        // }
    }
}




// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    //world.register::<PlayerComponent>();
    world.register::<BallDisplayComponent>();
}