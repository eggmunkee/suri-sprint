
use ggez::{Context, GameResult, GameError};

use ggez::graphics;
use ggez::nalgebra as na;
use ggez::graphics::{Color,DrawParam,Scale,set_window_title};

use specs::{Entity,World,WorldExt,System,WriteStorage};
use specs::join::Join;
use rand::prelude::*;



pub mod level;
pub mod dialog;
pub mod paused;

use crate::resources::{GameStateResource};
use crate::components::{Position,Velocity,RenderTrait};
use crate::components::sprite::{SpriteComponent,SpriteLayer,MultiSpriteComponent};
use crate::components::player::{CharacterDisplayComponent};
use crate::game_state::{GameState,State,GameMode,RunningState};
use crate::entities::level_builder::{LevelItem};
use self::dialog::{DialogRenderer};
use self::level::{LevelRenderer};
use self::paused::{PausedRenderer};


// Render object type for an entity
pub enum RenderObjectType {
    Character, // complex suri animation display
    Sprite, // generic single texture display
    MultiSprite, // generic multi texture display for an entity
    Button,
}

pub struct Renderer {
    pub display_offset: na::Point2::<f32>,
    pub paused_renderer: PausedRenderer,
}

impl Renderer {
    pub fn new(paused_anim: f32) -> Renderer {
        Renderer {
            display_offset: na::Point2::new(0.0,0.0),
            paused_renderer: PausedRenderer {
                anim_position: paused_anim,
            },
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
            // Render Edit mode level setup            
            LevelRenderer::render(&game_state, ctx);
        }

        {
            // draw level bounds
            let game_res = game_state.world.fetch::<GameStateResource>();
            let level_bounds = game_res.level_bounds.clone();
            let width = level_bounds.max_x - level_bounds.min_x;
            let height = level_bounds.max_y - level_bounds.min_y;

            let mut stroke_opt = ggez::graphics::StrokeOptions::DEFAULT.clone();
            stroke_opt.line_width = 4.0;

            if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
                ggez::graphics::DrawMode::Stroke(stroke_opt),
                ggez::graphics::Rect::new(0.0, 0.0, width, height),
                ggez::graphics::Color::new(0.0, 0.0, 0.0, 0.5)
            ) {
                ggez::graphics::draw(ctx, &rect, DrawParam::default()
                    .dest(na::Point2::new(level_bounds.min_x, level_bounds.min_y)) );
            }

            stroke_opt.line_width = 2.0;
            if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
                ggez::graphics::DrawMode::Stroke(stroke_opt),
                ggez::graphics::Rect::new(0.0, 0.0, width, height),
                ggez::graphics::Color::new(1.0, 0.0, 0.0, 0.5)
            ) {
                ggez::graphics::draw(ctx, &rect, DrawParam::default()
                    .dest(na::Point2::new(level_bounds.min_x, level_bounds.min_y)) );
            }

        }


        self.post_render_list(ctx, world);

        // RENDER UI STATE --------------------------------------------------------------------------
        match &game_state.current_state {
            // RUNNING - check for dialog
            State::Running => {
                match &game_state.running_state {
                    // DISPLAY DIALOG TEXT
                    RunningState::Dialog(msg) => {
                        DialogRenderer::render(&game_state, ctx, msg.clone());
                    },
                    _ => {}
                }
            },

            // PAUSED STATE -----------------------------------------------------------------
            State::Paused => {
                // DRAW PAUSED DISPLAY
                self.paused_renderer.render(&game_state, ctx);
                
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
