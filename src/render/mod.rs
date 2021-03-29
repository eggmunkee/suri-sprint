
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

use crate::resources::{GameStateResource,ShaderResources,ShaderInputs,ImageResources,InputResource,Camera,GameLog};
use crate::components::{Position,Velocity,RenderTrait,RenderFlag,RenderLayerType};
use crate::components::sprite::{SpriteComponent,SpriteLayer,MultiSpriteComponent,ParallaxSpriteComponent};
use crate::components::anim_sprite::{AnimSpriteComponent};
use crate::components::particle_sys::{ParticleSysComponent};
use crate::components::player::{CharacterDisplayComponent};
use crate::components::flags::{RenderCallInfo,RenderFlagType};
use crate::core::game_state::{GameState,State,GameMode,RunningState,MenuItem};
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
    //pub paused_renderer: PausedRenderer,
    //pub snap_view: bool,
    pub game_run_time: f32,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            display_offset: na::Point2::new(0.0,0.0),
            //paused_renderer: PausedRenderer {
            //},
            //snap_view: false
            game_run_time: 0.0,
        }
    }

    pub fn add_resources(world: &World, ctx: &mut Context) {

        let mut images = world.fetch_mut::<ImageResources>();

        // Paused overlay (borders)
        images.load_image("/overlay.png".to_string(), ctx).expect("MISSING REQUIREMENT");
        // Warp In / Warp Out overlays
        images.load_image("/warp-overlay-purple.png".to_string(), ctx).expect("MISSING REQUIREMENT");
        images.load_image("/warp-overlay-grey.png".to_string(), ctx).expect("MISSING REQUIREMENT");

        // Menu Images
        images.load_image("/purple-dialog-bg.png".to_string(), ctx).expect("MISSING REQUIREMENT");
        images.load_image("/purple-dialog-wide-bg.png".to_string(), ctx).expect("MISSING REQUIREMENT");
        images.load_image("/blue-dialog-bg.png".to_string(), ctx).expect("MISSING REQUIREMENT");
        images.load_image("/green-dialog-bg.png".to_string(), ctx).expect("MISSING REQUIREMENT");
        //images.load_image("/grey-dialog-bg.png".to_string(), ctx).expect("MISSING REQUIREMENT");
        //images.load_image("/green-eye-blob.png".to_string(), ctx).expect("MISSING REQUIREMENT");
        images.load_image("/dirty-box-1.png".to_string(), ctx).expect("MISSING REQUIREMENT");
        images.load_image("/dark_messy_tile.png".to_string(), ctx).expect("MISSING REQUIREMENT");
        images.load_image("/cloud-dialog.png".to_string(), ctx).expect("MISSING REQUIREMENT");
        images.load_image("/cloud-dialog-shadow.png".to_string(), ctx).expect("MISSING REQUIREMENT");
        images.load_image("/cloud-dialog-bordered.png".to_string(), ctx).expect("MISSING REQUIREMENT");
        images.load_image("/cloud-dialog-selected.png".to_string(), ctx).expect("MISSING REQUIREMENT");

        drop(images);

    }

    pub fn render_frame(&mut self, game_state: &mut GameState, ctx: &mut Context) -> GameResult {
        let world: &World = &game_state.world;
        // RENDER LIST (Entity ID, Position (world coords), Z order, sub-index of item (used in multi sprites))
        //let mut render_objects : Vec<(u32,na::Point2<f32>,f32,usize)> = vec![];
        //let mut render_objects : Vec<RenderCallInfo> = vec![];
        //let mut player_offset = na::Point2::<f32>::new(0.0,0.0);

        // let mut char_in_warp = false;
        // let mut char_in_portal = false;

        // let mut level_run_time : f32 = 0.0;
        // let mut game_run_time : f32 = 0.0;
        {
            let camera = world.fetch::<Camera>();
            self.display_offset.x = camera.display_offset.0;
            self.display_offset.y = camera.display_offset.1;
        }


        // let mut target_offset_x : f32 = game_state.current_offset.x;
        // let mut target_offset_y : f32 = game_state.current_offset.y;

        // let mut move_camera : bool = true;

        // // Set freeze-camera states - when camera should not move towards target
        // if game_state.menu_stack.len() > 0 {
        //     move_camera = false;
        // }
        // match &game_state.running_state {
        //     // DISPLAY DIALOG TEXT
        //     RunningState::Dialog{..} => {
        //         move_camera = false;
        //     },
        //     _ => {}
        // }
        // match &game_state.current_state {
        //     State::Paused => {
        //         move_camera = false;
        //     },
        //     _ => {}
        // }

        // self.display_offset.x = target_offset_x;
        // self.display_offset.y = target_offset_y;

        // BUILD RENDER OBJECTS LIST -----------------------------------------------------------------
        let render_objects : Vec<RenderCallInfo> = self.generate_render_list(game_state, ctx, world);

        // {
        //     if game_state.game_frame_count % 60 == 1 {
        //         println!(" Build Render Object list ------------------");
        //     }
        //     let gs_res = game_state.world.fetch::<GameStateResource>();

        //     let current_player_num = gs_res.player_1_char_num;
        //     level_run_time = gs_res.level_world_seconds;
        //     game_run_time = gs_res.game_run_seconds;

        //     let pos = game_state.world.read_storage::<Position>();
        //     //let char_disp = game_state.world.read_storage::<CharacterDisplayComponent>();
        //     let entities = game_state.world.entities();

        //     let render_res = game_state.world.read_storage::<RenderFlag>();

        //     // Get read storage for all display components
        //     let sprite_disp = game_state.world.read_storage::<SpriteComponent>();
        //     let multi_sprite_disp = game_state.world.read_storage::<MultiSpriteComponent>();
        //     let anim_sprite_disp = game_state.world.read_storage::<AnimSpriteComponent>();
        //     let particle_sys_disp = game_state.world.read_storage::<ParticleSysComponent>();
        //     let char_disp = game_state.world.read_storage::<CharacterDisplayComponent>();

        //     for (render_flag, character, pos, ent) in (&render_res, &char_disp, &pos, &entities).join() {
        //         // Only process Level layer
        //         if !render_flag.in_layer(RenderLayerType::LevelLayer) { continue; }

        //         if character.player_number == current_player_num {
        //             target_offset_x = -pos.x;
        //             target_offset_y = -pos.y;
        //         }
        //         let z_order = SpriteLayer::Player.to_z();
        //         char_in_portal = character.in_exit || character.in_portal;

        //         let call_info = RenderCallInfo {
        //             entity: ent.clone(),
        //             pos: na::Point2::new(pos.x, pos.y),
        //             z_order: z_order,
        //             item_index: 0,
        //             render_type: RenderFlagType::Character
        //         };

        //         render_objects.push(
        //             //(ent.id(),na::Point2::new(pos.x, pos.y), z_order, 0)
        //             call_info
        //         );
        //     }

        //     for (_, sprite, pos, ent) in (&render_res, &sprite_disp, &pos, &entities).join() {
        //         let z_order = sprite.z_order;

        //         let call_info = RenderCallInfo {
        //             entity: ent.clone(),
        //             pos: na::Point2::new(pos.x, pos.y),
        //             z_order: z_order,
        //             item_index: 0,
        //             render_type: RenderFlagType::Sprite
        //         };

        //         render_objects.push(
        //             //(ent.id(),na::Point2::new(pos.x, pos.y), z_order, 0)
        //             call_info
        //         );
        //     }

        //     for (_, multi_sprite, pos, ent) in (&render_res, &multi_sprite_disp, &pos, &entities).join() {
        //         let mut index : usize = 0;
        //         for sprite in &multi_sprite.sprites {
        //             let z_order = sprite.z_order;

        //             let call_info = RenderCallInfo {
        //                 entity: ent.clone(),
        //                 pos: na::Point2::new(pos.x, pos.y),
        //                 z_order: z_order,
        //                 item_index: index,
        //                 render_type: RenderFlagType::MultiSprite
        //             };

        //             render_objects.push(
        //                 //(ent.id(),na::Point2::new(pos.x, pos.y), z_order, index)
        //                 call_info
        //             );
        //             index += 1;
        //         }
        //     }

        //     for (_, anim_sprite, pos, ent) in (&render_res, &anim_sprite_disp, &pos, &entities).join() {
        //         let z_order = anim_sprite.z_order;

        //         let call_info = RenderCallInfo {
        //             entity: ent.clone(),
        //             pos: na::Point2::new(pos.x, pos.y),
        //             z_order: z_order,
        //             item_index: 0,
        //             render_type: RenderFlagType::AnimSprite
        //         };

        //         render_objects.push(
        //             //(ent.id(),na::Point2::new(pos.x, pos.y), z_order, 0)
        //             call_info
        //         );
        //     }

        //     for (_, particle_sys, pos, ent) in (&render_res, &particle_sys_disp, &pos, &entities).join() {
        //         let z_order = particle_sys.z_order;

        //         let call_info = RenderCallInfo {
        //             entity: ent.clone(),
        //             pos: na::Point2::new(pos.x, pos.y),
        //             z_order: z_order,
        //             item_index: 0,
        //             render_type: RenderFlagType::ParticleSys
        //         };

        //         render_objects.push(
        //             //(ent.id(),na::Point2::new(pos.x, pos.y), z_order, 0)
        //             call_info
        //         );
        //     }


        //     // for (
        //     //     opt_sprite_disp,
        //     //     opt_char_disp,
        //     //     opt_multi_sprite,
        //     //     opt_anim_sprite,
        //     //     opt_particle_sys,
        //     //     pos,
        //     //     ent) in 
        //     //     (
        //     //         (&sprite_disp).maybe(),
        //     //         (&char_disp).maybe(),
        //     //         (&multi_sprite_disp).maybe(), 
        //     //         (&anim_sprite_disp).maybe(),
        //     //         (&particle_sys_disp).maybe(),                
        //     //         &pos,&entities).join() {
        //     //     // Check for any of the display components
        //     //     match opt_char_disp {
        //     //         Some(character) => {
        //     //             if character.player_number == current_player_num {
        //     //                 target_offset_x = -pos.x;
        //     //                 target_offset_y = -pos.y;
        //     //             }
        //     //             let z_order = SpriteLayer::Player.to_z();
        //     //             char_in_portal = character.in_exit || character.in_portal;

        //     //             render_objects.push(
        //     //                 (ent.id(),na::Point2::new(pos.x, pos.y), z_order, 0)
        //     //             );
        //     //         },
        //     //         _ => match opt_sprite_disp {
        //     //             Some(sprite) => {
        //     //                 let z_order = sprite.z_order;

        //     //                 render_objects.push(
        //     //                     (ent.id(),na::Point2::new(pos.x, pos.y), z_order, 0)
        //     //                 );
        //     //             },
        //     //             _ => match opt_multi_sprite {
        //     //                 Some(multi_sprite) => {
        //     //                     let mut index : usize = 0;
        //     //                     for sprite in &multi_sprite.sprites {
        //     //                         let z_order = sprite.z_order;

        //     //                         render_objects.push(
        //     //                             (ent.id(),na::Point2::new(pos.x, pos.y), z_order, index)
        //     //                         );
        //     //                         index += 1;
        //     //                     }
        //     //                 },
        //     //                 _ => match opt_anim_sprite {
        //     //                     Some(anim_sprite) => {
        //     //                         let z_order = anim_sprite.z_order;
        //     //                         render_objects.push(
        //     //                             (ent.id(),na::Point2::new(pos.x, pos.y), z_order, 0)
        //     //                         );
        //     //                     },
        //     //                     _ => match opt_particle_sys {
        //     //                         Some(particle_sys) => {
        //     //                             let z_order = particle_sys.z_order;
        //     //                             render_objects.push(
        //     //                                 (ent.id(),na::Point2::new(pos.x, pos.y), z_order, 0)
        //     //                             );
        //     //                         },
        //     //                         _ => {}
        //     //                     }
        //     //                 }
        //     //             }
        //     //         }
        //     //     };

        //     // }

        // }

        // if game_state.game_frame_count % 60 == 1 {
        //     println!(" Calculate camera update ------------------");
        // }
        // let targ_x_mag = (target_offset_x - self.display_offset.x).abs();
        // let targ_y_mag = (target_offset_y - self.display_offset.y).abs();
        // let targ_axes_sum = targ_x_mag * targ_x_mag + targ_y_mag * targ_y_mag;
        // if !game_state.snap_view && move_camera == true {

        //     if targ_axes_sum < 10000.0 {
        //     }
        //     else if targ_axes_sum < 200000.0 {
        //         // let midpoint = (self.display_offset.x - target_offset_x) * (targ_x_mag / 20.0);
        //         // self.display_offset.x -= midpoint;
        //         let midpoint_x = (self.display_offset.x - target_offset_x) * (targ_x_mag / 8000.0);
        //         self.display_offset.x -= midpoint_x;
        //         let midpoint_y = (self.display_offset.y - target_offset_y) * (targ_y_mag / 2500.0);
        //         self.display_offset.y -= midpoint_y;
        //     }
        //     else {
        //         //self.display_offset.x = target_offset_x;
        //         //let midpoint = (self.display_offset.x - target_offset_x) * 0.95;
        //         self.display_offset.x = target_offset_x; //midpoint;
        //         self.display_offset.y = target_offset_y;
        //     }
        // }
        // else if game_state.snap_view {
        //     self.display_offset.x = target_offset_x; //midpoint;
        //     self.display_offset.y = target_offset_y;
        //     game_state.snap_view = false;
        // }

        // if char_in_portal {
        //     graphics::clear(ctx, [0.5, 0.5, 0.6, 0.0].into());
        // }
        // else {
        //     graphics::clear(ctx, [0.2, 0.2, 0.25, 1.0].into());
        // }
        if game_state.game_frame_count % 60 == 1 {
            println!(" Clear display area ------------------");
        }
        // Clear background
        graphics::clear(ctx, [0.2, 0.2, 0.25, 1.0].into());
        

        if game_state.game_frame_count % 60 == 1 {
            println!(" Z Sort display list ------------------");
        }
        

        // });

        let render_count = render_objects.len();

        if game_state.game_frame_count % 60 == 1 {
            println!("   Display list ({}) ------------------", &render_count);
            println!("   Pre-render List step ------------------");
        }


        self.render_level(game_state, ctx, world, &render_objects);

        /*
        self.pre_render_list(game_state, ctx, world);

        // RENDER OBJECT LIST -----------------------------------------------------------------
        if let State::Running | State::Paused = &game_state.current_state {
            if game_state.game_frame_count % 60 == 1 {
                println!("   Running/Paused - Render objects loop ---------------");
            }            
            // for (ent, pt, _, item_index) in render_objects.iter() {
            //     // Get entity by id
            //     let entity = game_state.world.entities().entity(ent.clone());
            //     // If entity is still alive, render it
            //     if entity.gen().is_alive() {
            //         // Call generic renderer, which calls on render component to draw
            //         Self::call_renderer(ctx, world, entity, pt, *item_index);
            //     }
            // }

            for call_info in render_objects.iter() {
                // Get entity by id
                let entity = call_info.entity; //game_state.world.entities().entity(ent.clone());
                // If entity is still alive, render it
                if entity.gen().is_alive() {
                    // Call generic renderer, which calls on render component to draw
                    //Self::call_renderer(ctx, world, entity, call_info.pos, call_info.item_index);
                    call_info.render_item(game_state, ctx);
                }
            }

            let gs_res = game_state.world.fetch::<GameStateResource>();

            // Render Target Location of game state
            let ptl_x = gs_res.player_target_loc.0;
            let ptl_y = gs_res.player_target_loc.1;

            DialogRenderer::render_cursor(ctx, ptl_x, ptl_y, Color::new(0.0, 1.0, 0.0, 1.0));

        }

        if game_state.mode == GameMode::Edit {
            if game_state.game_frame_count % 60 == 1 {
                println!("   Edit Mode - Render level ---------------");
            }
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
            if game_state.game_frame_count % 60 == 1 {
                println!("   Post Render List step ---------------");
            }
            self.post_render_list(ctx, world);
        }*/

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
                if game_state.game_frame_count % 60 == 1 {
                    println!("   Render Warp Into Level overlay ---------------");
                }
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
                if game_state.game_frame_count % 60 == 1 {
                    println!("   Render Warp out of Level overlay ---------------");
                }
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
                    RunningState::Dialog{msg, ..} => {
                        if game_state.game_frame_count % 60 == 1 {
                            println!("  Dialog Render step ---------------");
                        }

                        let bg_image = game_state.running_state.get_bg_image();
                        //DialogRenderer::render(&game_state, ctx, msg.clone());
                        // DialogRenderer::render_dialog_textured(&game_state, ctx, String::new(),
                        //     0.5, 0.25, 0.608, 0.418,
                        //     "/cloud-dialog-shadow.png".to_string(), Color::new(0.0, 0.0, 0.0, 1.0)); //(&game_state, ctx, msg.clone());

                        // DialogRenderer::render_dialog_textured(&game_state, ctx, msg.clone(),
                        //     0.5, 0.25, 0.6, 0.4,
                        //     "/cloud-dialog.png".to_string(), Color::new(0.2, 0.2, 0.4, 1.0)); //(&game_state, ctx, msg.clone());

                        DialogRenderer::render_dialog_textured(&game_state, ctx, msg.clone(),
                            0.5, 0.25, 0.6, 0.4,
                            bg_image, Color::new(0.2, 0.2, 0.4, 1.0)); //(&game_state, ctx, msg.clone());
                    },
                    _ => {}
                }
            },

            // PAUSED STATE -----------------------------------------------------------------
            State::Paused => {
                // DRAW PAUSED DISPLAY
                {
                    if game_state.game_frame_count % 60 == 1 {
                        println!("  Paused Render step ---------------");
                    }
                    PausedRenderer::render(&game_state, ctx);
                }

                // World overlay shader
                {
                    if game_state.game_frame_count % 60 == 1 {
                        println!("     Render Paused Overlay ------------");
                    }
                    let mut images = game_state.world.fetch_mut::<ImageResources>();
                    let texture_ref = images.image_ref("/overlay.png".to_string());
                    let (scrw, scrh) = (game_state.window_w, game_state.window_h);

                    if let Ok(mut texture) = texture_ref {
                        let mut shader_res = world.fetch_mut::<ShaderResources>();

                        let mut _lock : Option<ggez::graphics::ShaderLock> = None;
                        if let Ok(shader_ref) = shader_res.shader_ref("overlay".to_string()) {
                            let mut dim = shader_ref.send(ctx, ShaderInputs {game_time: self.game_run_time});

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

        // GAME IN game Menu LAYER (points)
        self.render_dialog(ctx, game_state);

        // MENU LAYER
        self.render_menus(ctx, game_state);

        // MOUSE / CURSOR LAYER
        self.render_cursor(ctx, game_state);

        // Update framerate on title every 5 frames
        if game_state.game_frame_count % 60 == 1 {
            println!("  Calc FPS for window Title ------------");
        }
        if ggez::timer::ticks(ctx) % 10 == 0 {
            let fps = ggez::timer::fps(ctx);
            set_window_title(ctx, format!("{} ({:.1} fps for {} render objs)", &game_state.window_title, &fps, &render_count).as_str());

        }

        if game_state.game_frame_count % 60 == 1 {
            println!("  Frame contents Presented to device ------------");
        }
        graphics::present(ctx)?;

        Ok(())
    }

    fn call_renderer(ctx: &mut Context, world: &World, entity: Entity, pt: &na::Point2<f32>, item_index: usize) {
        
        {
            let p_sys_comp = world.read_storage::<ParticleSysComponent>();

            // Try reading CharacterDisplayComponent to render
            let ch_disp_comp = world.read_storage::<CharacterDisplayComponent>();
            let ch_disp_comp_res = ch_disp_comp.get(entity);
            if let Some(res) = ch_disp_comp_res {
                // Call component render method
                Self::render_item(ctx, &world, entity, pt, item_index, res);
                //res.draw(ctx, &world, Some(entity.id()), pt.clone(), item_index);

                let p_sys_comp_res = p_sys_comp.get(entity);
                if let Some(psys_res) = p_sys_comp_res {
                    let feet_pt = na::Point2::<f32>::new(pt.x, pt.y + 20.0);  // { x: pt.x, y: pt.y };

                    Self::render_item(ctx, &world, entity, &feet_pt, item_index, psys_res);
                }
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

    fn render_item(ctx: &mut Context, world: &World, entity: Entity, pt: &na::Point2<f32>, item_index: usize, render_item: &RenderTrait) {
        render_item.draw(ctx, &world, Some(entity.id()), pt.clone(), item_index);
    }

    // pub fn get_draw_offset(ctx: &mut Context) -> na::Point2<f32> {
    //     na::Point2::new(0.0, 0.0)
    // }
    fn generate_render_list(&mut self, game_state: &GameState, ctx: &mut Context, world: &World) -> Vec<RenderCallInfo> {

        let mut render_objects = vec![];

        if game_state.game_frame_count % 60 == 1 {
            println!(" Build Render Object list ------------------");
        }
        let gs_res = game_state.world.fetch::<GameStateResource>();

        let current_player_num = gs_res.player_1_char_num;
        //level_run_time = gs_res.level_world_seconds;
        self.game_run_time = gs_res.game_run_seconds;

        let pos = game_state.world.read_storage::<Position>();
        //let char_disp = game_state.world.read_storage::<CharacterDisplayComponent>();
        let entities = game_state.world.entities();

        let render_res = game_state.world.read_storage::<RenderFlag>();

        // Get read storage for all display components
        let sprite_disp = game_state.world.read_storage::<SpriteComponent>();
        let multi_sprite_disp = game_state.world.read_storage::<MultiSpriteComponent>();
        let anim_sprite_disp = game_state.world.read_storage::<AnimSpriteComponent>();
        let plx_sprite_disp = game_state.world.read_storage::<ParallaxSpriteComponent>();
        let particle_sys_disp = game_state.world.read_storage::<ParticleSysComponent>();
        let char_disp = game_state.world.read_storage::<CharacterDisplayComponent>();

        for (render_flag, character, pos, ent) in (&render_res, &char_disp, &pos, &entities).join() {
            // Only process Level layer
            if !render_flag.in_layer(RenderLayerType::LevelLayer) { continue; }

            // if character.player_number == current_player_num {
            //     target_offset_x = -pos.x;
            //     target_offset_y = -pos.y;
            // }
            let z_order = SpriteLayer::Player.to_z();
            //char_in_portal = character.in_exit || character.in_portal;

            let call_info = RenderCallInfo {
                entity: ent.clone(),
                pos: na::Point2::new(pos.x, pos.y),
                z_order: z_order,
                item_index: 0,
                render_type: RenderFlagType::Character
            };

            render_objects.push(
                //(ent.id(),na::Point2::new(pos.x, pos.y), z_order, 0)
                call_info
            );
        }

        for (_, sprite, pos, ent) in (&render_res, &sprite_disp, &pos, &entities).join() {
            let z_order = sprite.z_order;

            let call_info = RenderCallInfo {
                entity: ent.clone(),
                pos: na::Point2::new(pos.x, pos.y),
                z_order: z_order,
                item_index: 0,
                render_type: RenderFlagType::Sprite
            };

            render_objects.push(
                //(ent.id(),na::Point2::new(pos.x, pos.y), z_order, 0)
                call_info
            );
        }

        for (_, multi_sprite, pos, ent) in (&render_res, &multi_sprite_disp, &pos, &entities).join() {
            let mut index : usize = 0;
            for sprite in &multi_sprite.sprites {
                let z_order = sprite.z_order;

                let call_info = RenderCallInfo {
                    entity: ent.clone(),
                    pos: na::Point2::new(pos.x, pos.y),
                    z_order: z_order,
                    item_index: index,
                    render_type: RenderFlagType::MultiSprite
                };

                render_objects.push(
                    //(ent.id(),na::Point2::new(pos.x, pos.y), z_order, index)
                    call_info
                );
                index += 1;
            }
        }

        for (_, anim_sprite, pos, ent) in (&render_res, &anim_sprite_disp, &pos, &entities).join() {
            let z_order = anim_sprite.z_order;

            let call_info = RenderCallInfo {
                entity: ent.clone(),
                pos: na::Point2::new(pos.x, pos.y),
                z_order: z_order,
                item_index: 0,
                render_type: RenderFlagType::AnimSprite
            };

            render_objects.push(
                //(ent.id(),na::Point2::new(pos.x, pos.y), z_order, 0)
                call_info
            );
        }

        for (_, plx_sprite, pos, ent) in (&render_res, &plx_sprite_disp, &pos, &entities).join() {
            let mut index : usize = 0;
            for sprite in &plx_sprite.sprites {
                let z_order = sprite.z_order;

                let call_info = RenderCallInfo {
                    entity: ent.clone(),
                    pos: na::Point2::new(pos.x, pos.y),
                    z_order: z_order,
                    item_index: index,
                    render_type: RenderFlagType::ParallaxSprite
                };

                render_objects.push(
                    //(ent.id(),na::Point2::new(pos.x, pos.y), z_order, index)
                    call_info
                );
                index += 1;
            }
        }

        for (_, particle_sys, pos, ent) in (&render_res, &particle_sys_disp, &pos, &entities).join() {
            let z_order = particle_sys.z_order;

            let call_info = RenderCallInfo {
                entity: ent.clone(),
                pos: na::Point2::new(pos.x, pos.y),
                z_order: z_order,
                item_index: 0,
                render_type: RenderFlagType::ParticleSys
            };

            render_objects.push(
                //(ent.id(),na::Point2::new(pos.x, pos.y), z_order, 0)
                call_info
            );
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
            let bz = &b.z_order;
            let az = &a.z_order;

            if az < bz {
                std::cmp::Ordering::Less
            }
            else if az > bz {
                std::cmp::Ordering::Greater
            }
            else {
                // Sort same-Z items by Y position (best default for overhead view)
                if &a.pos.y < &b.pos.y {
                    std::cmp::Ordering::Less
                }
                else if &a.pos.y > &b.pos.y {
                    std::cmp::Ordering::Greater
                }
                else {
                    std::cmp::Ordering::Equal
                }
            }
        });

        render_objects

    }

    fn render_level(&self, game_state: &GameState, ctx: &mut Context, world: &World, render_objects: &Vec<RenderCallInfo>) {
        self.pre_render_list(game_state, ctx, world);

        // RENDER OBJECT LIST -----------------------------------------------------------------
        if let State::Running | State::Paused = &game_state.current_state {
            if game_state.game_frame_count % 60 == 1 {
                println!("   Running/Paused - Render objects loop ---------------");
            }            
            // for (ent, pt, _, item_index) in render_objects.iter() {
            //     // Get entity by id
            //     let entity = game_state.world.entities().entity(ent.clone());
            //     // If entity is still alive, render it
            //     if entity.gen().is_alive() {
            //         // Call generic renderer, which calls on render component to draw
            //         Self::call_renderer(ctx, world, entity, pt, *item_index);
            //     }
            // }

            for call_info in render_objects.iter() {
                // Get entity by id
                let entity = call_info.entity; //game_state.world.entities().entity(ent.clone());
                // If entity is still alive, render it
                if entity.gen().is_alive() {
                    // Call generic renderer, which calls on render component to draw
                    //Self::call_renderer(ctx, world, entity, call_info.pos, call_info.item_index);
                    call_info.render_item(game_state, ctx);
                }
            }

            let gs_res = game_state.world.fetch::<GameStateResource>();

            // Render Target Location of game state
            let ptl_x = gs_res.player_target_loc.0;
            let ptl_y = gs_res.player_target_loc.1;

            DialogRenderer::render_cursor(ctx, ptl_x, ptl_y, Color::new(0.0, 1.0, 0.0, 1.0));

        }

        if game_state.mode == GameMode::Edit {
            if game_state.game_frame_count % 60 == 1 {
                println!("   Edit Mode - Render level ---------------");
            }
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
            if game_state.game_frame_count % 60 == 1 {
                println!("   Post Render List step ---------------");
            }
            self.post_render_list(ctx, world);
        }
    }

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

    fn render_dialog(&self, ctx: &mut Context, game_state: &GameState) {
        if game_state.game_frame_count % 60 == 1 {
            println!("  Render Points UI ------------");
        }
        let gs_res = game_state.world.fetch::<GameStateResource>();
        let points = gs_res.points;
        // DialogRenderer::render_at(game_state, ctx, format!("{} Pts", &points), 
        //     0.075, 0.05, 0.15, 0.1,
        //     Color::new(0.0, 0.0, 0.0, 1.0), Color::new(0.5, 0.5, 0.5, 0.8), Color::new(1.0, 1.0, 1.0, 1.0) );

        DialogRenderer::render_dialog_textured(game_state, ctx, format!("{} Pts", &points),
            0.075, 0.05, 0.15, 0.1, "/cloud-dialog-bordered.png".to_string(), Color::new(0.2, 0.2, 0.4, 1.0));


        // DialogRenderer::render_at(game_state, ctx, "Ghost: Hello, Suri.... \nalk f jfkj akjf kdkj".to_string(), 
        //     0.6, 0.2, 0.3, 0.1,
        //     Color::new(0.0, 0.0, 0.0, 1.0), Color::new(0.9, 0.9, 0.9, 0.25), Color::new(0.0, 0.0, 0.0, 1.0) );

        // DialogRenderer::render_at(game_state, ctx, "Ghost: Hello, Suri..... \nalk f jfkj".to_string(), 
        //     0.5, 0.7, 0.15, 0.1,
        //     Color::new(0.5, 0.5, 0.5, 1.0), Color::new(0.9, 0.9, 0.9, 0.25), Color::new(0.0, 0.0, 0.0, 1.0) );

        // DialogRenderer::render_at(game_state, ctx, "Ghost: Hello, Suri..... \nalk f jfkj".to_string(), 
        //     0.2, 0.5, 0.2, 0.2,
        //     Color::new(0.0, 0.0, 0.0, 1.0), Color::new(0.9, 0.9, 0.9, 0.25), Color::new(0.0, 0.0, 0.0, 1.0) );

        // DialogRenderer::render_at(game_state, ctx, "".to_string(), 
        //     0.5, 0.9, 0.98, 0.18,
        //     Color::new(0.0, 0.0, 0.0, 1.0), Color::new(0.9, 0.9, 0.9, 0.25), Color::new(0.0, 0.0, 0.0, 1.0) );


        let game_log = game_state.world.fetch::<GameLog>();

        //let (scrw, scrh) = (game_state.window_w, game_state.window_h);

        // if game_log.entries.len() == 0 {
        //     game_log.add_entry(true, "Test entry 1".to_string(), None);
        //     game_log.add_entry(true, "Test entry 2 23@#$@#376".to_string(), None);
        // }
        

        let max_entr = game_log.max_keep;
        let mut item_idx : i32 = 0;

        // DialogRenderer::render_dialog_textured(game_state, ctx, "Test entry 1".to_string(),
        //     0.1,
        //     (0.0 / max_entr as f32 * 0.8) + 0.1,
        //     0.15, 0.08, "/dark_messy_tile.png".to_string(), Color::new(0.2, 0.2, 0.4, 1.0));
        // DialogRenderer::render_dialog_textured(game_state, ctx, "Test entry 2 23@#$@#376".to_string(),
        //     0.1,
        //     (1.0 / max_entr as f32 * 0.8) + 0.1,
        //     0.15, 0.08, "/dark_messy_tile.png".to_string(), Color::new(0.2, 0.2, 0.4, 1.0));

        for entry in game_log.entries.iter() {
            let log_pos_x = 0.15;
            let log_pos_y = ((item_idx as f32) / max_entr as f32 * 0.7) + 0.15;
            DialogRenderer::render_dialog_textured(game_state, ctx, entry.msg.clone(),
                log_pos_x, log_pos_y, 0.2, 0.08, "/purple-dialog-wide-bg.png".to_string(), Color::new(0.8, 0.2, 0.2, 1.0));
            item_idx += 1;
        }

    }

    fn render_menus(&self, ctx: &mut Context, game_state: &GameState) {
        if game_state.game_frame_count % 60 == 1 {
            println!("  Render Menus stack UI ------------");
        }
        let mut menu_layer = 0;
        for menu in &game_state.menu_stack {

            let item_count = menu.items.len() as i32;

            if game_state.game_frame_count % 60 == 1 {
                println!("  - Render Menu ------------");
            }
            let w = match menu_layer {
                0 => 0.5,
                1 => 0.43,
                _ => 0.33
            };
            let h = match menu_layer {
                0 => 0.5f32,
                1 => 0.6,
                _ => 0.5
            }.min( (item_count as f32 * 0.05).max(item_count as f32 * 40.0) );
            let bg_alpha = match menu_layer {
                0 => 0.7,
                _ => 1.0,
            };
            let bg_color = match menu_layer {
                0 => Color::new(0.0, 0.7, 0.7, bg_alpha),
                1 => Color::new(0.7, 0.0, 0.7, bg_alpha),
                _ => Color::new(0.7, 0.5, 0.5, bg_alpha),
            };

            // DialogRenderer::render_at(game_state, ctx, String::new(), 
            // 0.5, 0.5, w, h,
            // Color::new(1.0, 0.0, 1.0, 1.0), bg_color, Color::new(1.0, 1.0, 1.0, 1.0) );

            // let mut big_msg = String::new();
            // for i in 1..15 {
            //     big_msg.push_str("Test one two three fill this thing with lots of text.");
            // }

            let dialog_bg_texture = match menu_layer {
                0 => "/purple-dialog-wide-bg.png".to_string(),
                _ => "/dark_messy_tile.png".to_string(),
                //_ => "/green-dialog-bg.png".to_string(),
            };

            let bg_alpha = match menu_layer {
                0 => 0.2,
                1 => 0.2,
                _ => 0.2
            };

            DialogRenderer::render_dialog_textured(game_state, ctx, String::new(),
                0.5, 0.5, w, h, dialog_bg_texture, Color::new(1.0, 1.0, 1.0, bg_alpha));
            //ggez::graphics::push_transform()

            let bg_color = Color::new(0.5, 0.5, 0.5, 0.25);

            let item_height_ratio = 0.9;
            let header_item_margin_ratio = 0.95;
            let item_margin_ratio = 0.8;
            let mut item_idx = 0;
            let start_y = 0.5 - (h * item_height_ratio / 2.0);
            for item in &menu.items {
                let selected = item_idx == menu.selected_index;
                let h_per_item = h * item_height_ratio / item_count as f32;
                let color = match selected {
                    true => Color::new(1.0, 1.0, 1.0, 1.0),
                    false => Color::new(0.7, 0.5, 0.7, 1.0),
                };
                let mut is_header = false;
                let mut is_range_item = false;
                let item_text = match &item {
                    MenuItem::Header(msg) => {
                        is_header = true;
                        msg.clone()                            
                    },
                    MenuItem::ButtonItem { name, .. } => {
                        name.clone()
                    },
                    MenuItem::ToggleItem { name, value, .. } => {
                        format!("{} ({})", &name, &(match value { true => "On", false => "Off" })).to_string()
                    },
                    MenuItem::RangeItem { name, value, max, .. } => {
                        format!("{} ({:1}/{:1})", &name, value, max).to_string()
                    },                        
                };


                let bg_image = match is_header {
                    false => match selected {
                        true => "/purple-dialog-bg.png".to_string(),
                        false => "/blue-dialog-bg.png".to_string()
                    },
                    true => match menu_layer {
                        _ => "/dirty-box-1.png".to_string()
                    }
                };

                if !is_header {
                
                    //};
                    // DialogRenderer::render_at(game_state, ctx, item_text, 
                    //     0.5, start_y + ((item_idx as f32 + 0.5) * h_per_item), w * 0.95, h_per_item * 0.95,
                    //     Color::new(1.0, 0.0, 1.0, 1.0), bg_color, color);
                    DialogRenderer::render_dialog_bg_textured(game_state, ctx, 
                        0.5, start_y + ((item_idx as f32 + 0.5) * h_per_item), w * item_height_ratio, h_per_item * item_margin_ratio,
                        bg_image);

                    // if selected {
                    //     DialogRenderer::render_dialog_bg_textured(game_state, ctx, 
                    //         0.5, start_y + ((item_idx as f32 + 0.5) * h_per_item), w * item_height_ratio, h_per_item * item_margin_ratio,
                    //         bg_image);
                    //     DialogRenderer::render_dialog_textured(game_state, ctx, item_text,
                    //         0.5, start_y + ((item_idx as f32 + 0.5) * h_per_item), w * item_height_ratio, h_per_item * item_margin_ratio,
                    //         bg_image, color);
                    // }
                    // else {
                    //     DialogRenderer::render_dialog_textured(game_state, ctx, item_text,
                    //         0.5, start_y + ((item_idx as f32 + 0.5) * h_per_item), w * item_height_ratio, h_per_item * item_margin_ratio,
                    //         bg_image, color);
                    // }

                    match &item {
                        MenuItem::ToggleItem { value, .. } => {
                            DialogRenderer::render_progress_area(game_state, ctx, 
                                0.5, start_y + ((item_idx as f32 + 0.5) * h_per_item),
                                w * item_height_ratio * 0.95, h_per_item * header_item_margin_ratio * 0.75,
                                Color::new(0.2, 0.2, 0.5, 0.5), Color::new(0.3, 0.3, 1.0, 0.75),
                                0.0, 1.0, match (*value) { true => 1.0, false => 0.0 }
                            );
                        },
                        MenuItem::RangeItem { value, min, max, .. } => {
                            DialogRenderer::render_progress_area(game_state, ctx, 
                                0.5, start_y + ((item_idx as f32 + 0.5) * h_per_item),
                                w * item_height_ratio * 0.95, h_per_item * header_item_margin_ratio * 0.75,
                                Color::new(0.2, 0.2, 0.5, 0.5), Color::new(0.3, 0.3, 1.0, 0.75),
                                *min, *max, *value
                            );
                        },
                        _ => {}
                    }

                    DialogRenderer::render_dialog_text(game_state, ctx, item_text,
                        0.5, start_y + ((item_idx as f32 + 0.5) * h_per_item), w * item_height_ratio, h_per_item * item_margin_ratio,
                        color);

                    
                    
                }
                else {
                    DialogRenderer::render_dialog_textured(game_state, ctx, item_text,
                        0.5, start_y + ((item_idx as f32 + 0.5) * h_per_item), w * item_height_ratio, h_per_item * header_item_margin_ratio,
                        bg_image, Color::new(1.0, 1.0, 1.0, 1.0));
                }

                /*match &item {
                    MenuItem::RangeItem { name, value, min, max, .. } => {
                        DialogRenderer::render_progress_area(game_state, ctx, 
                            0.5, start_y + ((item_idx as f32 + 0.5) * h_per_item), w * item_height_ratio, h_per_item * header_item_margin_ratio,
                            Color::new(0.2, 0.2, 0.2, 1.0), Color::new(1.0, 1.0, 1.0, 1.0),
                            *min, *max, *value
                        );
                    },
                    _ => {}
                }*/


                item_idx = item_idx + 1;
            }

            menu_layer = menu_layer + 1;
        }
    }

    fn render_cursor(&self, ctx: &mut Context, game_state: &GameState)
    {
        if game_state.game_frame_count % 60 == 1 {
            println!("  Render Mouse Cursor indicator ------------");
        }
        let mut input = game_state.world.fetch_mut::<InputResource>();

        //if let Some(input_res) = input {
        let mx = input.mouse_x;
        let my = input.mouse_y;
        //}
        
        //let mut stroke_opt = ggez::graphics::StrokeOptions::DEFAULT.clone();
        //stroke_opt.line_width = 4.0;

        // if let Ok(circle) = ggez::graphics::Mesh::new_circle(ctx, ggez::graphics::DrawMode::Fill(FillOptions::default()),
        //     na::Point2::new(0.0, 0.0), 10.0, 0.8, Color::new(1.0, 1.0, 1.0, 1.0) 
        //     // ggez::graphics::DrawMode::Stroke(stroke_opt),
        //     // ggez::graphics::Rect::new(0.0, 0.0, width, height),
        //     // ggez::graphics::Color::new(0.0, 0.0, 0.0, 0.5)
        // ) {
        //     ggez::graphics::draw(ctx, &circle, DrawParam::default()
        //         .dest(na::Point2::new(mx, my))
        //         .color(Color::new(0.8, 0.8, 0.0, 0.3)) );

        //     ggez::graphics::draw(ctx, &circle, DrawParam::default()
        //         .dest(na::Point2::new(mx, my))
        //         .scale(na::Vector2::<f32>::new(0.5, 0.5)) 
        //         .color(Color::new(0.9, 0.9, 0.25, 0.5)));

        //     ggez::graphics::draw(ctx, &circle, DrawParam::default()
        //         .dest(na::Point2::new(mx, my))
        //         .scale(na::Vector2::<f32>::new(0.25, 0.25))
        //         .color(Color::new(1.0, 1.0, 0.6, 1.0)) );
        //         //.dest(na::Point2::new(mx, my)) );
        // }

        drop(input);

        DialogRenderer::render_cursor(ctx, mx, my,Color::new(1.0, 1.0, 0.6, 1.0) );

    }

}
