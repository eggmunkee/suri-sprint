
use ggez::{Context, GameResult, GameError};

use ggez::graphics;
use ggez::nalgebra as na;
use ggez::graphics::{Color,DrawParam,Scale,set_window_title};

use specs::{Entity,World,WorldExt,System,WriteStorage};
use specs::join::Join;
use rand::prelude::*;


use crate::resources::{GameStateResource};
use crate::components::{Position,Velocity,RenderTrait};
use crate::components::sprite::{SpriteComponent,SpriteLayer,MultiSpriteComponent};
use crate::components::player::{CharacterDisplayComponent};
use crate::game_state::{GameState,State,GameMode,RunningState};
use crate::entities::level_builder::{LevelItem};


// Render object type for an entity
pub enum RenderObjectType {
    Character, // complex suri animation display
    Sprite, // generic single texture display
    MultiSprite, // generic multi texture display for an entity
    Button,
}

pub struct Renderer {
    pub display_offset: na::Point2::<f32>,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            display_offset: na::Point2::new(0.0,0.0),
        }
    }

    pub fn render_frame(&mut self, game_state: &GameState, world: &World, ctx: &mut Context) -> GameResult {
        
        let mut render_objects : Vec<(u32,na::Point2<f32>,f32,u32)> = vec![];
        //let mut player_offset = na::Point2::<f32>::new(0.0,0.0);

        let mut char_in_warp = false;
        let mut char_in_portal = false;

        // BUILD RENDER OBJECTS LIST -----------------------------------------------------------------
        {
           
            let pos = game_state.world.read_storage::<Position>();
            //let char_disp = game_state.world.read_storage::<CharacterDisplayComponent>();
            let entities = game_state.world.entities();

            // Get read storage for all display components
            let sprite_disp = game_state.world.read_storage::<SpriteComponent>();
            let multi_sprite_disp = game_state.world.read_storage::<MultiSpriteComponent>();
            let char_disp = game_state.world.read_storage::<CharacterDisplayComponent>();
            for (opt_sprite_disp,opt_char_disp,opt_multi_sprite,pos,ent) in 
                ((&sprite_disp).maybe(), (&char_disp).maybe(), (&multi_sprite_disp).maybe(),  &pos,&entities).join() {
                // Check for any of the display components
                match opt_char_disp {
                    Some(character) => {
                        self.display_offset.x = -pos.x;
                        self.display_offset.y = -pos.y;
                        let z_order = SpriteLayer::Player.to_z();
                        char_in_portal = character.in_exit || character.in_portal;

                        render_objects.push(
                            (ent.id(),na::Point2::new(pos.x, pos.y), z_order, 0)
                        );
                    },
                    _ => match opt_sprite_disp {
                        Some(sprite) => {
                            let z_order = sprite.z_order;

                            render_objects.push(
                                (ent.id(),na::Point2::new(pos.x, pos.y), z_order, 0)
                            );
                        },
                        _ => match opt_multi_sprite {
                            Some(multi_sprite) => {
                                let mut index : u32 = 0;
                                for sprite in &multi_sprite.sprites {
                                    let z_order = sprite.z_order;
    
                                    render_objects.push(
                                        (ent.id(),na::Point2::new(pos.x, pos.y), z_order, index)
                                    );
                                    index += 1;
                                }
                            },
                            _ => {}
                        }
                    }
                };

            }

        }

        if char_in_portal {
            graphics::clear(ctx, [0.5, 0.5, 0.6, 1.0].into());
        }
        else {
            graphics::clear(ctx, [0.2, 0.2, 0.25, 1.0].into());
        }
        


        // ORDER RENDER OBJECTS -----------------------------------------------------------------
        // TODO: implement Z-ordering here
        // remove first render object - the player
        //let r0 = render_objects.remove(0);
        // reverse order, so first are drawn last
        render_objects.reverse();
        // add player render object to end - drawn very last
        //render_objects.push(r0);

        // sort by Z order
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

        self.pre_render_list(game_state, ctx, world);

        {
            // draw level bounds
            let game_res = game_state.world.fetch::<GameStateResource>();
            let level_bounds = game_res.level_bounds.clone();
            let width = level_bounds.max_x - level_bounds.min_x;
            let height = level_bounds.max_y - level_bounds.min_y;

            let mut stroke_opt = ggez::graphics::StrokeOptions::DEFAULT.clone();
            stroke_opt.line_width = 5.0;

            if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
                ggez::graphics::DrawMode::Stroke(stroke_opt),
                ggez::graphics::Rect::new(0.0, 0.0, width, height),
                ggez::graphics::Color::new(1.0, 0.0, 0.0, 1.0)
            ) {
                ggez::graphics::draw(ctx, &rect, DrawParam::default()
                    .dest(na::Point2::new(level_bounds.min_x, level_bounds.min_y)) );
            }

        }

        // RENDER OBJECT LIST -----------------------------------------------------------------
        for (ent, pt, _, item_index) in render_objects.iter() {
            // Get entity by id
            let entity = game_state.world.entities().entity(ent.clone());
            // If entity is still alive, render it
            if entity.gen().is_alive() {
                // Call generic renderer, which calls on render component to draw
                Self::call_renderer(ctx, world, entity, pt, *item_index);
            }
        }

        if game_state.mode == GameMode::Edit {
            // draw level bounds
            //let level_items = &game_state.level.items;
            
            

            for item in &game_state.level.items {

                match &item {
                    LevelItem::Player{x, y} => {
                        let mut stroke_opt = ggez::graphics::StrokeOptions::DEFAULT.clone();
                        stroke_opt.line_width = 4.0;
                        if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
                            ggez::graphics::DrawMode::Stroke(stroke_opt),
                            ggez::graphics::Rect::new(0.0, 0.0, 10.0, 10.0),
                            ggez::graphics::Color::new(1.0, 0.0, 0.0, 1.0)
                        ) {
                            ggez::graphics::draw(ctx, &rect, DrawParam::default()
                                .dest(na::Point2::new(x - 5.0, y - 5.0)) );
                        }
                    },
                    LevelItem::Ghost{x, y, ..} => {
                        let mut stroke_opt = ggez::graphics::StrokeOptions::DEFAULT.clone();
                        stroke_opt.line_width = 4.0;
                        if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
                            ggez::graphics::DrawMode::Stroke(stroke_opt),
                            ggez::graphics::Rect::new(0.0, 0.0, 10.0, 10.0),
                            ggez::graphics::Color::new(1.0, 0.0, 1.0, 0.5)
                        ) {
                            ggez::graphics::draw(ctx, &rect, DrawParam::default()
                                .dest(na::Point2::new(x - 5.0, y - 5.0)) );
                        }
                    },
                    LevelItem::Portal{x, y, w, ..} => {
                        let mut stroke_opt = ggez::graphics::StrokeOptions::DEFAULT.clone();
                        stroke_opt.line_width = 4.0;
                        if let Ok(rect) = ggez::graphics::Mesh::new_circle(ctx, 
                            ggez::graphics::DrawMode::Stroke(stroke_opt),
                            na::Point2::<f32>::new(*x, *y),
                            *w, 0.5,
                            ggez::graphics::Color::new(1.0, 1.0, 0.0, 0.5)
                        ) {
                            ggez::graphics::draw(ctx, &rect, DrawParam::default()
                                .dest(na::Point2::new(*x-*w, *y-*w))
                                .offset(na::Point2::new(*w, *w))
                                
                            );
                        }
                    },
                    LevelItem::Exit{x, y, w, ..} => {
                        let mut stroke_opt = ggez::graphics::StrokeOptions::DEFAULT.clone();
                        stroke_opt.line_width = 4.0;
                        if let Ok(rect) = ggez::graphics::Mesh::new_circle(ctx, 
                            ggez::graphics::DrawMode::Stroke(stroke_opt),
                            na::Point2::<f32>::new(*x, *y),
                            *w, 0.5,
                            ggez::graphics::Color::new(0.0, 0.0, 1.0, 0.5)
                        ) {
                            ggez::graphics::draw(ctx, &rect, DrawParam::default()
                                .dest(na::Point2::<f32>::new(*x, *y)) );
                        }
                    },
                    LevelItem::Sprite{x, y, angle, ..} => {
                        let mut stroke_opt = ggez::graphics::StrokeOptions::DEFAULT.clone();
                        stroke_opt.line_width = 4.0;
                        if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
                            ggez::graphics::DrawMode::Stroke(stroke_opt),
                            ggez::graphics::Rect::new(0.0, 0.0, 10.0, 10.0),
                            ggez::graphics::Color::new(1.0, 1.0, 0.0, 0.5)
                        ) {
                            ggez::graphics::draw(ctx, &rect, DrawParam::default()
                                //.dest(na::Point2::new(x - 5.0, y - 5.0)) );
                                .dest(na::Point2::new(*x-5.0, *y-5.0))
                                .rotation(*angle));
                        }
                    },
                    LevelItem::Platform{x, y, w, h, ang, ..} => {
                        let mut stroke_opt = ggez::graphics::StrokeOptions::DEFAULT.clone();
                        stroke_opt.line_width = 4.0;
                        if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
                            ggez::graphics::DrawMode::Stroke(stroke_opt),
                            ggez::graphics::Rect::new(0.0, 0.0, w*2.0, h*2.0),
                            ggez::graphics::Color::new(1.0, 0.0, 0.0, 0.5)
                        ) {
                            ggez::graphics::draw(ctx, &rect, DrawParam::default()                                
                                .dest(na::Point2::new(*x-*w, *y-*h))
                                .offset(na::Point2::new(*w, *h))
                                .rotation(*ang)
                                 );
                        }
                    },
                    LevelItem::DynPlatform{x, y, w, h, ang, ..} => {
                        let mut stroke_opt = ggez::graphics::StrokeOptions::DEFAULT.clone();
                        stroke_opt.line_width = 4.0;
                        if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
                            ggez::graphics::DrawMode::Stroke(stroke_opt),
                            ggez::graphics::Rect::new(0.0, 0.0, w*2.0, h*2.0),
                            ggez::graphics::Color::new(1.0, 1.0, 0.0, 0.5)
                        ) {
                            ggez::graphics::draw(ctx, &rect, DrawParam::default()
                                .dest(na::Point2::new(*x-*w, *y-*h))
                                .offset(na::Point2::new(*w, *h))
                                .rotation(*ang)
                                 );
                        }
                    },
                    _ => {

                    }
                }

                
    
            }


        }

        self.post_render_list(ctx, world);

        match &game_state.running_state {
            // DISPLAY DIALOG TEXT
            RunningState::Dialog(msg) => {
                let mut draw_ok = true;
                let (w, h) = (game_state.window_w, game_state.window_h);
                let cent_x = w as f32 / 2.0;
                let cent_y = h as f32 / 4.0;

                let border_color = ggez::graphics::Color::new(0.7, 0.2, 0.7, 1.0);
                let bg_color = ggez::graphics::Color::new(0.3, 0.1, 0.3, 0.5);
                let mut stroke_options = ggez::graphics::StrokeOptions::DEFAULT;
                stroke_options.line_width = 5.0;

                if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
                    ggez::graphics::DrawMode::Fill(ggez::graphics::FillOptions::DEFAULT),
                    ggez::graphics::Rect::new(0.0, 0.0, w * 0.75, h * 0.75),
                    bg_color
                ) {
                    ggez::graphics::draw(ctx, &rect, DrawParam::default()
                        .dest(na::Point2::new(w * 0.25 * 0.5, h * 0.25 * 0.5)) );
                }

                if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
                    ggez::graphics::DrawMode::Stroke(stroke_options),
                    ggez::graphics::Rect::new(0.0, 0.0, w * 0.75, h * 0.75),
                    border_color
                ) {
                    ggez::graphics::draw(ctx, &rect, DrawParam::default()
                        .dest(na::Point2::new(w * 0.25 * 0.5, h * 0.25 * 0.5)) );
                }

                let dialog_content = msg.clone();
                //level_name_content.pusgame_state.level.name.clone();
                let mut dialog_text = ggez::graphics::Text::new(dialog_content);
                dialog_text.set_font(game_state.font, Scale { x: 20.0, y: 20.0 });
                let text_w = dialog_text.width(ctx);
                let text_h = dialog_text.height(ctx);
                if let Err(_) = graphics::draw(ctx, &dialog_text,
                    DrawParam::new()
                    .dest(na::Point2::new(cent_x-(text_w as f32 / 2.0),cent_y-(text_h as f32 / 2.0)))
                    //.scale(na::Vector2::new(2.0,2.0))
                ) {
                    draw_ok = false;
                }
                if !draw_ok {
                    println!("Draw error occurred");
                }

            },
            _ => {}
        }

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
                }
                if let Err(_) = graphics::draw(ctx, &game_state.paused_text, //(na::Point2::new(cent_x,cent_y),
                        //Color::new(0.8,0.85,1.0,1.0)) ) 
                        DrawParam::new()
                        .dest(na::Point2::new(cent_x-(text_w as f32 / 2.0),cent_y-(text_h as f32 / 2.0)))
                        .color(Color::new(0.8,0.85,1.0,1.0))
                        ) {
                    draw_ok = false;
                }

                let level_name_y = 4.0 * h as f32 / 5.0;
                let level_name_content = String::from(format!("Level \"{}\"", &game_state.level.name));
                //level_name_content.pusgame_state.level.name.clone();
                let mut level_name_text = ggez::graphics::Text::new(level_name_content);
                level_name_text.set_font(game_state.font, Scale { x: 20.0, y: 20.0 });
                let text_w = level_name_text.width(ctx);
                let text_h = level_name_text.height(ctx);
                if let Err(_) = graphics::draw(ctx, &level_name_text,
                    DrawParam::new()
                    .dest(na::Point2::new(cent_x-(text_w as f32 / 2.0),level_name_y-(text_h as f32 / 2.0)))
                    //.scale(na::Vector2::new(2.0,2.0))
                ) {
                    draw_ok = false;
                }
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

    fn call_renderer(ctx: &mut Context, world: &World, entity: Entity, pt: &na::Point2<f32>, item_index: u32) {
        
        {
            // Try reading CharacterDisplayComponent to render
            let ch_disp_comp = world.read_storage::<CharacterDisplayComponent>();
            let ch_disp_comp_res = ch_disp_comp.get(entity);
            if let Some(res) = ch_disp_comp_res {
                // Call component render method
                Self::render_item(ctx, &world, entity, pt, item_index, res);
                //res.draw(ctx, &world, Some(entity.id()), pt.clone(), item_index);
            }
            else {
                let sprite_disp_comp = world.read_storage::<SpriteComponent>();
                let sprite_disp_comp_res = sprite_disp_comp.get(entity);
                if let Some(res) = sprite_disp_comp_res {
                    // Call component render method
                    Self::render_item(ctx, &world, entity, pt, item_index, res);
                    //res.draw(ctx, &world, Some(entity.id()), pt.clone(), item_index);
                }
                else {
                    let sprite_disp_comp = world.read_storage::<MultiSpriteComponent>();
                    let sprite_disp_comp_res = sprite_disp_comp.get(entity);
                    if let Some(res) = sprite_disp_comp_res {
                        // Call component render method
                        Self::render_item(ctx, &world, entity, pt, item_index, res);
                        //res.draw(ctx, &world, Some(entity.id()), pt.clone(), item_index);
                    }
                }
            }
        }
        
    }

    fn render_item(ctx: &mut Context, world: &World, entity: Entity, pt: &na::Point2<f32>, item_index: u32, render_item: &RenderTrait) {
        render_item.draw(ctx, &world, Some(entity.id()), pt.clone(), item_index);
    }

    // pub fn get_draw_offset(ctx: &mut Context) -> na::Point2<f32> {
    //     na::Point2::new(0.0, 0.0)
    // }

    fn pre_render_list(&self, game_state: &GameState, ctx: &mut Context, world: &World) {
        let (width, height) = ggez::graphics::size(ctx);
        let scale = game_state.display_scale;
        //let 
        let mut dp = DrawParam::new().dest(na::Point2::new(self.display_offset.x * scale + (width / 2.0), self.display_offset.y * scale + (height / 2.0)));
        dp = dp.scale(na::Vector2::new(scale,scale));
        let transform = dp.to_matrix();
        graphics::push_transform(ctx, Some(transform));

        graphics::apply_transformations(ctx);

    }

    fn post_render_list(&self, ctx: &mut Context, world: &World) {

        graphics::pop_transform(ctx);
        graphics::apply_transformations(ctx);
    }

}
