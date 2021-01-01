
use ggez::{Context, GameResult, GameError};

use ggez::graphics;
use ggez::nalgebra as na;
use ggez::graphics::{Color,StrokeOptions,Rect,FillOptions,DrawParam,Scale,set_window_title};

use specs::{Entity,World,WorldExt,System,WriteStorage};
use specs::join::Join;
use rand::prelude::*;



pub mod level;
pub mod dialog;
pub mod paused;

use crate::resources::{GameStateResource,ShaderResources,ShaderInputs,ImageResources,InputResource};
use crate::components::{Position,Velocity,RenderTrait};
use crate::components::sprite::{SpriteComponent,SpriteLayer,MultiSpriteComponent};
use crate::components::anim_sprite::{AnimSpriteComponent};
use crate::components::particle_sys::{ParticleSysComponent};
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
    //pub snap_view: bool,
}

impl Renderer {
    pub fn new(paused_anim: f32) -> Renderer {
        Renderer {
            display_offset: na::Point2::new(0.0,0.0),
            paused_renderer: PausedRenderer {
                anim_position: paused_anim,
            },
            //snap_view: false
        }
    }

    pub fn render_frame(&mut self, game_state: &mut GameState, ctx: &mut Context) -> GameResult {
        let world: &World = &game_state.world;
        let mut render_objects : Vec<(u32,na::Point2<f32>,f32,u32)> = vec![];
        //let mut player_offset = na::Point2::<f32>::new(0.0,0.0);

        let mut char_in_warp = false;
        let mut char_in_portal = false;

        let mut level_run_time : f32 = 0.0;
        let mut game_run_time : f32 = 0.0;        

        let mut target_offset_x : f32 = game_state.current_offset.x;
        let mut target_offset_y : f32 = game_state.current_offset.y;

        let mut move_camera : bool = true;

        match &game_state.running_state {
            // DISPLAY DIALOG TEXT
            RunningState::Dialog(_) => {
                move_camera = false;
            },
            _ => {}
        }
        match &game_state.current_state {
            State::Paused => {
                move_camera = false;
            },
            _ => {}
        }

        self.display_offset.x = target_offset_x;
        self.display_offset.y = target_offset_y;

        // BUILD RENDER OBJECTS LIST -----------------------------------------------------------------
        {
            let gs_res = game_state.world.fetch::<GameStateResource>();

            let current_player_num = gs_res.player_1_char_num;
            level_run_time = gs_res.level_world_seconds;
            game_run_time = gs_res.game_run_seconds;

            let pos = game_state.world.read_storage::<Position>();
            //let char_disp = game_state.world.read_storage::<CharacterDisplayComponent>();
            let entities = game_state.world.entities();

            // Get read storage for all display components
            let sprite_disp = game_state.world.read_storage::<SpriteComponent>();
            let multi_sprite_disp = game_state.world.read_storage::<MultiSpriteComponent>();
            let anim_sprite_disp = game_state.world.read_storage::<AnimSpriteComponent>();
            let particle_sys_disp = game_state.world.read_storage::<ParticleSysComponent>();
            let char_disp = game_state.world.read_storage::<CharacterDisplayComponent>();
            for (opt_sprite_disp,opt_char_disp,opt_multi_sprite,opt_anim_sprite,opt_particle_sys,pos,ent) in 
                ((&sprite_disp).maybe(), (&char_disp).maybe(), (&multi_sprite_disp).maybe(), 
                (&anim_sprite_disp).maybe(), (&particle_sys_disp).maybe(),                
                &pos,&entities).join() {
                // Check for any of the display components
                match opt_char_disp {
                    Some(character) => {
                        if character.player_number == current_player_num {
                            target_offset_x = -pos.x;
                            target_offset_y = -pos.y;
                        }
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
                            _ => match opt_anim_sprite {
                                Some(anim_sprite) => {
                                    let z_order = anim_sprite.z_order;
                                    render_objects.push(
                                        (ent.id(),na::Point2::new(pos.x, pos.y), z_order, 0)
                                    );
                                },
                                _ => match opt_particle_sys {
                                    Some(particle_sys) => {
                                        let z_order = particle_sys.z_order;
                                        render_objects.push(
                                            (ent.id(),na::Point2::new(pos.x, pos.y), z_order, 0)
                                        );
                                    },
                                    _ => {}
                                }
                            }
                        }
                    }
                };

            }

        }

        let targ_x_mag = (target_offset_x - self.display_offset.x).abs();
        let targ_y_mag = (target_offset_y - self.display_offset.y).abs();
        let targ_axes_sum = targ_x_mag * targ_x_mag + targ_y_mag * targ_y_mag;
        if !game_state.snap_view && move_camera == true {

            if targ_axes_sum < 10000.0 {
            }
            else if targ_axes_sum < 200000.0 {
                // let midpoint = (self.display_offset.x - target_offset_x) * (targ_x_mag / 20.0);
                // self.display_offset.x -= midpoint;
                let midpoint_x = (self.display_offset.x - target_offset_x) * (targ_x_mag / 8000.0);
                self.display_offset.x -= midpoint_x;
                let midpoint_y = (self.display_offset.y - target_offset_y) * (targ_y_mag / 2500.0);
                self.display_offset.y -= midpoint_y;
            }
            else {
                //self.display_offset.x = target_offset_x;
                //let midpoint = (self.display_offset.x - target_offset_x) * 0.95;
                self.display_offset.x = target_offset_x; //midpoint;
                self.display_offset.y = target_offset_y;
            }
        }
        else if game_state.snap_view {
            self.display_offset.x = target_offset_x; //midpoint;
            self.display_offset.y = target_offset_y;
            game_state.snap_view = false;
        }

        // if char_in_portal {
        //     graphics::clear(ctx, [0.5, 0.5, 0.6, 0.0].into());
        // }
        // else {
        //     graphics::clear(ctx, [0.2, 0.2, 0.25, 1.0].into());
        // }
        // Clear background
        graphics::clear(ctx, [0.2, 0.2, 0.25, 1.0].into());
        


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

        {
            self.post_render_list(ctx, world);
        }

        {
            // Over-scene overlays for warping
            let mut warping_in = false;
            let mut warping_in_time : f32 = 0.0;

            {
                let game_res = world.fetch::<GameStateResource>();
                
                if game_res.level_world_seconds < 0.75 {
                    warping_in = true;
                    warping_in_time = game_res.level_world_seconds;
                }
            }

            {
                if game_state.level_warping {
                    if game_state.level_warp_timer < 0.5 {

                    }
                }
            }

            if warping_in {
                //graphics::clear(ctx, [0.5, 0.5, 0.5, (1.5 - game_state.level_warp_timer) * 0.1 ].into());
                let mut images = game_state.world.fetch_mut::<ImageResources>();
                let texture_ref = images.image_ref("/warp-overlay-grey.png".to_string());

                let (scrw, scrh) = (game_state.window_w, game_state.window_h);

                if let Ok(mut texture) = texture_ref {

                    let w = texture.width();
                    let h = texture.height();
                    let scale_x = scrw / w as f32;
                    let scale_y = scrh / h as f32;
                    if let Err(_) = ggez::graphics::draw(ctx, texture, DrawParam::new()
                            .dest(na::Point2::new(0.0,0.0))
                            //.offset(na::Point2::new(0.5f32,0.5f32))
                            .scale(na::Vector2::new(scale_x, scale_y))
                            .color(Color::new(1.0,1.0,1.0,(0.75 - warping_in_time) / 1.5))) { 
                        //_draw_ok = false;
                        println!("Failed to render overlay image");
                    }
                }
                else {
                    println!("Failed to get overlay texture");
                }
            }

            if game_state.level_warping {
                //graphics::clear(ctx, [0.5, 0.5, 0.5, (1.5 - game_state.level_warp_timer) * 0.1 ].into());
                let mut images = game_state.world.fetch_mut::<ImageResources>();
                let texture_ref = images.image_ref("/warp-overlay-grey.png".to_string());

                let (scrw, scrh) = (game_state.window_w, game_state.window_h);

                if let Ok(mut texture) = texture_ref {

                    let w = texture.width();
                    let h = texture.height();
                    let scale_x = scrw / w as f32;
                    let scale_y = scrh / h as f32;
                    if let Err(_) = ggez::graphics::draw(ctx, texture, DrawParam::new()
                            .dest(na::Point2::new(0.0,0.0))
                            //.offset(na::Point2::new(0.5f32,0.5f32))
                            .scale(na::Vector2::new(scale_x, scale_y))
                            .color(Color::new(1.0,1.0,1.0,game_state.level_warp_timer * 0.25))) { 
                        //_draw_ok = false;
                        println!("Failed to render overlay image");
                    }
                }
                else {
                    println!("Failed to get overlay texture");
                }
            }
        }

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
                {
                    self.paused_renderer.render(&game_state, ctx);
                }

                // World overlay shader
                {
                    let mut images = game_state.world.fetch_mut::<ImageResources>();
                    let texture_ref = images.image_ref("/overlay.png".to_string());
                    let (scrw, scrh) = (game_state.window_w, game_state.window_h);

                    if let Ok(mut texture) = texture_ref {
                        let mut shader_res = world.fetch_mut::<ShaderResources>();

                        let mut _lock : Option<ggez::graphics::ShaderLock> = None;
                        if let Ok(shader_ref) = shader_res.shader_ref("overlay".to_string()) {
                            let mut dim = shader_ref.send(ctx, ShaderInputs {game_time: game_run_time});

                            _lock = Some(ggez::graphics::use_shader(ctx, shader_ref));
                        }

                        let w = texture.width();
                        let h = texture.height();
                        let scale_x = scrw / w as f32;
                        let scale_y = scrh / h as f32;
                        if let Err(_) = ggez::graphics::draw(ctx, texture, DrawParam::new()
                                .dest(na::Point2::new(0.0,0.0))
                                //.offset(na::Point2::new(0.5f32,0.5f32))
                                .scale(na::Vector2::new(scale_x, scale_y))
                                .color(Color::new(1.0,1.0,1.0,1.0))) { 
                            //_draw_ok = false;
                            println!("Failed to render overlay image");
                        }
                    }
                    else {
                        println!("Failed to get overlay texture");
                    }

                    // let fill_opt = ggez::graphics::FillOptions::DEFAULT.clone();
                    // let (w, h) = (game_state.window_w, game_state.window_h);
                    // if let Ok(overlay_rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
                    //     ggez::graphics::DrawMode::Fill(fill_opt),
                    //     ggez::graphics::Rect::new(0.0, 0.0, w, h),
                    //     ggez::graphics::Color::new(1.0, 1.0, 1.0, 0.25)
                    // ) {
                        
                    //     let mut shader_res = world.fetch_mut::<ShaderResources>();

                    //     let mut _lock : Option<ggez::graphics::ShaderLock> = None;
                    //     if let Ok(shader_ref) = shader_res.shader_ref("overlay".to_string()) {
                    //         _lock = Some(ggez::graphics::use_shader(ctx, shader_ref));
                    //     }

                    //     ggez::graphics::draw(ctx, &overlay_rect, DrawParam::default() );
                    // }
                }
                
            },
            _ => {}
        }

        {
            let mut input = world.fetch_mut::<InputResource>();

            //if let Some(input_res) = input {
            let mx = input.mouse_x;
            let my = input.mouse_y;
            //}
            
            //let mut stroke_opt = ggez::graphics::StrokeOptions::DEFAULT.clone();
            //stroke_opt.line_width = 4.0;

            if let Ok(circle) = ggez::graphics::Mesh::new_circle(ctx, ggez::graphics::DrawMode::Fill(FillOptions::default()),
                na::Point2::new(mx, my), 10.0, 0.7, Color::new(0.8, 0.8, 0.0, 0.3) 
                // ggez::graphics::DrawMode::Stroke(stroke_opt),
                // ggez::graphics::Rect::new(0.0, 0.0, width, height),
                // ggez::graphics::Color::new(0.0, 0.0, 0.0, 0.5)
            ) {
                ggez::graphics::draw(ctx, &circle, DrawParam::default() );
                    //.dest(na::Point2::new(mx, my)) );
            }


        }


        {
            let gs_res = game_state.world.fetch::<GameStateResource>();
            let game_time = gs_res.game_run_seconds;
            DialogRenderer::render_at(game_state, ctx, format!("{}", &game_time), 
                0.075, 0.05, 0.15, 0.1, Color::new(0.0, 0.0, 0.0, 0.1), Color::new(0.5, 0.5, 0.5, 0.2) );

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
                    else {
                        let anim_sprite_comp = world.read_storage::<AnimSpriteComponent>();
                        let anim_sprite_comp_res = anim_sprite_comp.get(entity);
                        if let Some(res) = anim_sprite_comp_res {
                            // Call component render method
                            Self::render_item(ctx, &world, entity, pt, item_index, res);
                            //res.draw(ctx, &world, Some(entity.id()), pt.clone(), item_index);
                        }
                        else {
                            let p_sys_comp = world.read_storage::<ParticleSysComponent>();
                            let p_sys_comp_res = p_sys_comp.get(entity);
                            if let Some(res) = p_sys_comp_res {
                                // Call component render method
                                Self::render_item(ctx, &world, entity, pt, item_index, res);
                                //res.draw(ctx, &world, Some(entity.id()), pt.clone(), item_index);
                            }
                        }
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
