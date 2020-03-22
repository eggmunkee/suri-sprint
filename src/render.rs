
use ggez::{Context, GameResult, GameError};

use ggez::graphics;
use ggez::nalgebra as na;
use ggez::graphics::{Color,DrawParam,set_window_title};

use specs::{Entity,World,WorldExt,System,WriteStorage};
use specs::join::Join;
use rand::prelude::*;

//use crate::resources::{ImageResources};
use crate::components::{Position,Velocity,DisplayComp,DisplayCompType,RenderTrait};
use crate::components::ball::{BallDisplayComponent};
use crate::components::player::{PlayerComponent,CharacterDisplayComponent};
use crate::game_state::{GameState,State};

// pub mod circle;
// pub mod square;
pub struct Renderer {

}

// impl RenderTrait for Renderer {
//     fn draw(&self, ctx: &mut Context, world: &World, entity: Option<u32>, pos: na::Point2<f32>) {

//     }
// }

impl Renderer {

    fn call_renderer(ctx: &mut Context, world: &World, entity: Entity, pt: &na::Point2<f32>) {
        
        //let (entity, render) = Self::get_renderer(world, entity);
        {
            //let world = &.world;
            let ch_disp_comp = world.read_storage::<CharacterDisplayComponent>();
            let ch_disp_comp_res = ch_disp_comp.get(entity);
            if let Some(res) = ch_disp_comp_res {
                //render = res;
                res.draw(ctx, &world, Some(entity.id()), pt.clone());
            }
        }

        {
            let ball_disp_comp = world.read_storage::<BallDisplayComponent>();
            let ball_disp_comp_res = ball_disp_comp.get(entity);
            if let Some(res) = ball_disp_comp_res {
                //res.draw(ctx, &mut game_state.world, Some(ent.clone()), pt.clone());
                res.draw(ctx, &world, Some(entity.id()), pt.clone());
            }
        }
    }

    pub fn render_frame(game_state: &GameState, world: &World, ctx: &mut Context) -> GameResult {
        
        graphics::clear(ctx, [0.15, 0.21, 0.3, 1.0].into());

        let mut render_objects : Vec<(u32,na::Point2<f32>)> = vec![];
        
        {
            let pos = game_state.world.read_storage::<Position>();
            //let char_disp = game_state.world.read_storage::<CharacterDisplayComponent>();
            let entities = game_state.world.entities();

            let char_disp = game_state.world.read_storage::<CharacterDisplayComponent>();
            for (char_disp,pos,ent) in (&char_disp,&pos,&entities).join() {
                //char_disp.draw(ctx, na::Point2::new(pos.x, pos.y));
                //let rend_trt : &RenderTrait = char_disp as &RenderTrait;

                render_objects.push(
                    (ent.id(),na::Point2::new(pos.x, pos.y))
                )

                //rend_trt.draw(game_state, ctx, Some(&ent), na::Point2::new(pos.x, pos.y));
            }

            let ball_disp = game_state.world.read_storage::<BallDisplayComponent>();
            for (ball_disp,pos,ent) in (&ball_disp,&pos,&entities).join() {
                //let rend_trt : &RenderTrait = ball_disp as &RenderTrait;

                render_objects.push(
                    (ent.id(),na::Point2::new(pos.x, pos.y))
                )
                //rend_trt.draw(game_state, ctx, Some(&ent), na::Point2::new(pos.x, pos.y));
            }
      
        }

        // TODO: implement Z-ordering here
        // remove first render object - the player
        let r0 = render_objects.remove(0);
        // reverse order, so first are drawn last
        render_objects.reverse();
        // add player render object to end - drawn very last
        render_objects.push(r0);

        let render_count = render_objects.len();

        for (ent, pt) in render_objects.iter() {
            let entity = game_state.world.entities().entity(ent.clone()); //  ;;fetch::<BallDisplayComponent>();
            if entity.gen().is_alive() {
                //let render : &mut RenderTrait;

                Self::call_renderer(ctx, world, entity, pt);

                //let (entity, render) = Self::get_renderer(world, entity);
                // {
                //     //let world = &.world;
                //     let ch_disp_comp = world.read_storage::<CharacterDisplayComponent>();
                //     let ch_disp_comp_res = ch_disp_comp.get(entity);
                //     if let Some(res) = ch_disp_comp_res {
                //         //render = res;
                //         res.draw(ctx, &world, Some(ent.clone()), pt.clone());
                //     }
                // }

                // {
                //     let ball_disp_comp = game_state.world.read_storage::<BallDisplayComponent>();
                //     let ball_disp_comp_res = ball_disp_comp.get(entity);
                //     if let Some(res) = ball_disp_comp_res {
                //         //res.draw(ctx, &mut game_state.world, Some(ent.clone()), pt.clone());
                //         res.draw(ctx, &world, Some(ent.clone()), pt.clone());
                //     }
                // }

                
            }
            //rend_trt.draw(game_state, ctx, Some(ent.clone()), pt.clone());
        }


        match &game_state.current_state {
            State::Paused => {
                let mut draw_ok = true;

                let (w, h) = (game_state.window_w, game_state.window_h);
                let cent_x = w as f32 / 2.0;
                let cent_y = h as f32 / 2.0;
                let text_w = game_state.paused_text.width(ctx);
                let text_h = game_state.paused_text.height(ctx);

                // Render paused graphis
                if let Err(_) = graphics::draw(ctx, &game_state.paused_text, 
                        DrawParam::new()
                        .dest(na::Point2::new(cent_x-2.0-(text_w as f32 / 2.0),cent_y+2.0-(text_h as f32 / 2.0)))
                        .color(Color::new(0.0,0.0,0.0,1.0))
                        ) {
                    draw_ok = false;
                };
                if let Err(_) = graphics::draw(ctx, &game_state.paused_text, //(na::Point2::new(cent_x,cent_y),
                        //Color::new(0.8,0.85,1.0,1.0)) ) 
                        DrawParam::new()
                        .dest(na::Point2::new(cent_x-(text_w as f32 / 2.0),cent_y-(text_h as f32 / 2.0)))
                        .color(Color::new(0.8,0.85,1.0,1.0))
                        ) {
                    draw_ok = false;
                };

                if !draw_ok {
                    println!("Draw error occurred");
                }
            },
            _ => {}
        }

        // Update framerate on title every 5 frames
        if ggez::timer::ticks(ctx) % 5 == 0 {
            let fps = ggez::timer::fps(ctx);
            set_window_title(ctx, format!("GGEZ ~~~ DEMO ({:.1} fps for {} render objs)", &fps, &render_count).as_str());
        }

        graphics::present(ctx)?;

        Ok(())
    }

}


// impl<'a> System<'a> for Renderer {
//     type SystemData = WriteStorage<'a, Velocity>;

//     fn run(&mut self, mut vel: Self::SystemData) {

//     }

// }



/*
    // Old test render code


    // pub fn circs(game_state: &mut GameState, ctx: &mut Context) -> GameResult<()> {
    //     let mut rng = rand::thread_rng();
    //     match (graphics::Mesh::new_circle(
    //         ctx,
    //         graphics::DrawMode::fill(),
    //         na::Point2::new(0.0, 0.0),
    //         20.0,
    //         4.0,
    //         graphics::WHITE,
    //     ),
    //     graphics::Mesh::new_rectangle(
    //         ctx,
    //         graphics::DrawMode::fill(),
    //         graphics::Rect::from([-10.0,-10.0,10.0,10.0]),
    //         graphics::WHITE,
    //     )) {
    //         (Ok(circle),Ok(rect)) => {
                
                
    //             // Get Position read storage - access to positions of all entities
    //             //let mut image_res = game_state.world.fetch_mut::<ImageResources>();
    //             let pos = game_state.world.read_storage::<Position>();
    //             let disp = game_state.world.read_storage::<DisplayComp>();
    //             // Get entities list
    //             let ent = game_state.world.entities();
    //             let mut draw_ok = true;
    //             // iterator positions and entities together read-only
    //             for (pos, ent, disp) in (&pos, &ent, &disp).join() {
    //                 //println!("Display type: {:?}", disp);
    //                 if let DisplayCompType::DrawCircle = disp.display_type {
    //                     if disp.circle {
    //                         let mut col_vals: (u8,u8,u8) = rng.gen();
    //                         //println!("Entity {}, Circle pos: {:?}", ent.id(), pos);
    //                         if let Err(_) = graphics::draw(ctx, &circle, DrawParam::default()
    //                                     .dest(na::Point2::new(pos.x, pos.y))
    //                                     .scale(na::Vector2::new(1.0f32,1.0f32))
    //                                     .color(Color::from_rgba(col_vals.0,col_vals.1,col_vals.2,200)) ) {
    //                             draw_ok = false;
    //                         };    
    //                     }
    //                     else {
    //                         let mut col_vals: (u8,) = rng.gen();
    //                         //println!("Entity {}, Circle pos: {:?}", ent.id(), pos);
    //                         if let Err(_) = graphics::draw(ctx, &rect, (na::Point2::new(pos.x, pos.y),
    //                                 Color::from_rgba(col_vals.0,col_vals.0,col_vals.0,255) )) {
    //                             draw_ok = false;
    //                         };  
    //                         // if let Ok(image_ref) = image_res.image_ref(String::from("/icon.png")) {
    //                         //     if let Err(_) = graphics::draw(ctx, image_ref, (na::Point2::new(pos.x+15.0, pos.y+15.0),)) {
    //                         //         draw_ok = false;
    //                         //     }
    //                         // }
    //                         // else {
    //                         //     draw_ok = false;
    //                         // }

    //                     }
    //                 }
    //                 else if let DisplayCompType::DrawSelf = disp.display_type {
    //                     //draw_func(game_state, &ent, ctx);
    //                     //rend_list.push(disp);
    //                 }
    //             }

    //             let char_disp = game_state.world.read_storage::<CharacterDisplayComponent>();
    //             for (char_disp,pos) in (&char_disp,&pos).join() {
    //                 //char_disp.draw(ctx, na::Point2::new(pos.x, pos.y));
    //                 let rend_trt : &RenderTrait = char_disp as &RenderTrait;

    //                 rend_trt.draw(ctx, na::Point2::new(pos.x, pos.y));
    //             }

    //             match draw_ok {
    //                 true => Ok(()),
    //                 _ => Err(GameError::RenderError(String::from("circs draw error")))
    //             }
                
    //         },
    //         (_,_) => Err(GameError::RenderError(String::from("circs build error")))
    //     }
    // }

*/