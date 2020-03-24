
use ggez::{Context, GameResult, GameError};

use ggez::graphics;
use ggez::nalgebra as na;
use ggez::graphics::{Color,DrawParam,set_window_title};

use specs::{Entity,World,WorldExt,System,WriteStorage};
use specs::join::Join;
use rand::prelude::*;

//use crate::resources::{ImageResources};
use crate::components::{Position,Velocity,DisplayComp,DisplayCompType,RenderTrait};
use crate::components::sprite::{SpriteComponent};
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

    pub fn render_frame(game_state: &GameState, world: &World, ctx: &mut Context) -> GameResult {
        
        graphics::clear(ctx, [0.15, 0.21, 0.3, 1.0].into());

        let mut render_objects : Vec<(u32,na::Point2<f32>)> = vec![];
        
        // BUILD RENDER OBJECTS LIST -----------------------------------------------------------------
        {
            let pos = game_state.world.read_storage::<Position>();
            //let char_disp = game_state.world.read_storage::<CharacterDisplayComponent>();
            let entities = game_state.world.entities();

            // Get read storage for all display components
            let sprite_disp = game_state.world.read_storage::<SpriteComponent>();
            let char_disp = game_state.world.read_storage::<CharacterDisplayComponent>();
            let ball_disp = game_state.world.read_storage::<BallDisplayComponent>();
            for (opt_sprite_disp,opt_char_disp,opt_ball_disp,pos,ent) in 
                ((&sprite_disp).maybe(), (&char_disp).maybe(),(&ball_disp).maybe(),&pos,&entities).join() {
                // Check for any of the display components
                let has_display_comp = match opt_ball_disp {
                    Some(_) => true,
                    _ => match opt_char_disp {
                        Some(_) => true,
                        _ => match opt_sprite_disp {
                            Some(_) => true,
                            _ => false
                        }
                    }
                };

                // If any display component found, add to render objs list
                if has_display_comp {
                    render_objects.push(
                        (ent.id(),na::Point2::new(pos.x, pos.y))
                    )
    
                }

            }

        }

        // ORDER RENDER OBJECTS -----------------------------------------------------------------
        // TODO: implement Z-ordering here
        // remove first render object - the player
        let r0 = render_objects.remove(0);
        // reverse order, so first are drawn last
        render_objects.reverse();
        // add player render object to end - drawn very last
        render_objects.push(r0);

        let render_count = render_objects.len();

        // RENDER OBJECT LIST -----------------------------------------------------------------
        for (ent, pt) in render_objects.iter() {
            // Get entity by id
            let entity = game_state.world.entities().entity(ent.clone());
            // If entity is still alive, render it
            if entity.gen().is_alive() {
                // Call generic renderer, which calls on render component to draw
                Self::call_renderer(ctx, world, entity, pt);
            }
        }

        // RENDER UI --------------------------------------------------------------------------
        match &game_state.current_state {

            // DRAW PAUSED MESSAGE IN PAUSED STATE -----------------------------------------------------------------
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
        if ggez::timer::ticks(ctx) % 10 == 0 {
            let fps = ggez::timer::fps(ctx);
            set_window_title(ctx, format!("GGEZ ~~~ DEMO ({:.1} fps for {} render objs)", &fps, &render_count).as_str());
        }

        graphics::present(ctx)?;

        Ok(())
    }

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

        {
            let sprite_disp_comp = world.read_storage::<SpriteComponent>();
            let sprite_disp_comp_res = sprite_disp_comp.get(entity);
            if let Some(res) = sprite_disp_comp_res {
                //res.draw(ctx, &mut game_state.world, Some(ent.clone()), pt.clone());
                res.draw(ctx, &world, Some(entity.id()), pt.clone());
            }
        }
    }

}
