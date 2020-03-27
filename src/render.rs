
use ggez::{Context, GameResult, GameError};

use ggez::graphics;
use ggez::nalgebra as na;
use ggez::graphics::{Color,DrawParam,set_window_title};

use specs::{Entity,World,WorldExt,System,WriteStorage};
use specs::join::Join;
use rand::prelude::*;

//use crate::resources::{ImageResources};
use crate::components::{Position,Velocity,DisplayComp,DisplayCompType,RenderTrait};
use crate::components::sprite::{SpriteComponent,SpriteLayer};
use crate::components::ball::{BallDisplayComponent};
use crate::components::player::{CharacterDisplayComponent};
use crate::game_state::{GameState,State};

pub struct Renderer {

}

impl Renderer {

    pub fn render_frame(game_state: &GameState, world: &World, ctx: &mut Context) -> GameResult {
        
        graphics::clear(ctx, [0.2, 0.2, 0.2, 1.0].into());

        let mut render_objects : Vec<(u32,na::Point2<f32>,f32)> = vec![];
        let mut player_offset = na::Point2::<f32>::new(0.0,0.0);
        
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
                let mut z_order = 1.0;
                let has_display_comp = match opt_ball_disp {
                    Some(_) => true,
                    _ => match opt_char_disp {
                        Some(_) => {
                            player_offset.x = -pos.x;
                            player_offset.y = -pos.y;
                            z_order = SpriteLayer::Player.to_z();
                            true
                        },
                        _ => match opt_sprite_disp {
                            Some(sprite) => {
                                z_order = sprite.z_order;
                                true
                            },
                            _ => false
                        }
                    }
                };

                // If any display component found, add to render objs list
                if has_display_comp {
                    render_objects.push(
                        (ent.id(),na::Point2::new(pos.x, pos.y), z_order)
                    )
    
                }

            }

        }

        // ORDER RENDER OBJECTS -----------------------------------------------------------------
        // TODO: implement Z-ordering here
        // remove first render object - the player
        //let r0 = render_objects.remove(0);
        // reverse order, so first are drawn last
        render_objects.reverse();
        // add player render object to end - drawn very last
        //render_objects.push(r0);

        render_objects.sort_by(|a,b| {
            let by = &b.2;
            let ay = &a.2;

            if ay < by {
                std::cmp::Ordering::Less
            }
            else if ay > by {
                std::cmp::Ordering::Greater
            }
            else {
                std::cmp::Ordering::Equal
            }
        });

        // });

        let render_count = render_objects.len();

        Self::pre_render_list(ctx, world, &player_offset);

        // RENDER OBJECT LIST -----------------------------------------------------------------
        for (ent, pt, _) in render_objects.iter() {
            // Get entity by id
            let entity = game_state.world.entities().entity(ent.clone());
            // If entity is still alive, render it
            if entity.gen().is_alive() {
                // Call generic renderer, which calls on render component to draw
                Self::call_renderer(ctx, world, entity, pt);
            }
        }

        Self::post_render_list(ctx, world);

        // RENDER UI --------------------------------------------------------------------------
        match &game_state.current_state {

            // DRAW PAUSED MESSAGE IN PAUSED STATE -----------------------------------------------------------------
            State::Paused => {
                let mut draw_ok = true;

                let (w, h) = (game_state.window_w, game_state.window_h);
                let cent_x = w as f32 / 2.0;
                let cent_y = h as f32 / 5.0;
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
            set_window_title(ctx, format!("{} ({:.1} fps for {} render objs)", &game_state.window_title, &fps, &render_count).as_str());
        }

        graphics::present(ctx)?;

        Ok(())
    }

    fn call_renderer(ctx: &mut Context, world: &World, entity: Entity, pt: &na::Point2<f32>) {
        
        {
            // Try reading CharacterDisplayComponent to render
            let ch_disp_comp = world.read_storage::<CharacterDisplayComponent>();
            let ch_disp_comp_res = ch_disp_comp.get(entity);
            if let Some(res) = ch_disp_comp_res {
                // Call component render method
                res.draw(ctx, &world, Some(entity.id()), pt.clone());
            }
            else {
                // Try reading BallDisplayComponent to render
                {
                    let ball_disp_comp = world.read_storage::<BallDisplayComponent>();
                    let ball_disp_comp_res = ball_disp_comp.get(entity);
                    if let Some(res) = ball_disp_comp_res {
                        // Call component render method
                        res.draw(ctx, &world, Some(entity.id()), pt.clone());
                    }
                    else {
                        // Try reading SpriteComponent to render
                        {
                            let sprite_disp_comp = world.read_storage::<SpriteComponent>();
                            let sprite_disp_comp_res = sprite_disp_comp.get(entity);
                            if let Some(res) = sprite_disp_comp_res {
                                // Call component render method
                                res.draw(ctx, &world, Some(entity.id()), pt.clone());
                            }
                        }

                    }
                }

            }
        }

        

        
    }

    fn pre_render_list(ctx: &mut Context, world: &World, offset: &na::Point2<f32>) {
        let (width, height) = ggez::graphics::size(ctx);
        //let 
        let dp = DrawParam::new().dest(na::Point2::new(offset.x + (width / 2.0), offset.y + (height / 2.0)));
        let transform = dp.to_matrix();
        graphics::push_transform(ctx, Some(transform));

        graphics::apply_transformations(ctx);

    }

    fn post_render_list(ctx: &mut Context, world: &World) {

        graphics::pop_transform(ctx);
        graphics::apply_transformations(ctx);
    }

}
