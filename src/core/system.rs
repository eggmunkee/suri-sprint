
use ggez::nalgebra as na;
use ggez::{Context};
use specs::{World, WorldExt, RunNow, Entity, Builder}; // Builder, Component, ReadStorage, System, VecStorage, RunNow
use specs::Join;
use wrapped2d::b2;
use wrapped2d::user_data::*;
use std::collections::{HashMap};

use crate::core::game_state::{GameState,State,RunningState,GameMode};
use crate::core::world::{SuriWorld};
use crate::core::input::{InputKey};
use crate::core::physics::{BoxQueryInfo};
use crate::resources::{InputResource,GameStateResource,ConnectionResource,WorldAction,Camera,GameLog};
use crate::components::{Position,WorldUpdateTrait,Velocity};
use crate::components::collision::{Collision};
use crate::components::sprite::{SpriteLayer,SpriteComponent};
use crate::components::anim_sprite::{AnimSpriteComponent};
use crate::components::meow::{MeowComponent};
use crate::components::pickup::{PickupComponent};
use crate::components::exit::{ExitComponent};
use crate::components::portal::{PortalComponent};
use crate::components::button::{ButtonComponent};
use crate::components::player::{CharacterDisplayComponent};
use crate::components::npc::{NpcComponent};
use crate::entities::meow::{MeowBuilder};
use crate::systems::{InputSystem};
use crate::systems::logic::{LogicSystem};
use crate::systems::animation::{AnimationSystem};
use crate::systems::particles::{ParticleSystem};
use crate::systems::physics::{PhysicsSystem};

use crate::core::physics;
use crate::core::physics::{PhysicsWorld,RayCastQueryInfo};

pub struct CoreSystem {
}

impl CoreSystem {

    pub fn run_frame_time_update(game_state: &mut GameState, ctx: &mut Context, time_delta: f32, advance_world: bool) -> f32 {
        // Register actual pre-processed time (application time)
        game_state.update_run_time(time_delta);

        // Pre-process frame time (for simulation time)
        let time_delta = game_state.process_time_delta(ctx, time_delta, advance_world);
    
        // Save frame time
        game_state.set_frame_time(time_delta, advance_world);

        time_delta
    }

    pub fn clear_inputs(game_state: &mut GameState) {
        let mut input = game_state.world.fetch_mut::<InputResource>();
        input.keys_pressed.clear();
        drop(input);
    }

    pub fn run_terminal_step(game_state: &mut GameState, ctx: &mut Context, time_delta: f32) {
        Self::run_frame_time_update(game_state, ctx, time_delta, false);


        // InputSystem::handle_terminal_input(game_state, time_delta);

        // let mut input = game_state.world.fetch_mut::<InputResource>();
        // let wactions = input.actions.drain(0..).collect::<Vec<_>>();
        // input.clear_actions();
        // drop(input);

        // for world_action in &wactions {
        //     match world_action {
        //         WorldAction::ToggleConsole => {
        //             //println!("Open Menu");
        //             game_state.set_terminal_open(false);
        //             println!("[term step] Terminal Open set to {}", &game_state.terminal_open);
        //         },
        //         _ => {}
        //     }
        // }
        // if input.exit_flag {
        //     ggez::event::quit(ctx);
        // }


        //Self::clear_inputs(game_state);


    }

    // Process Menu update - check for unpause trigger
    pub fn run_menu_step(game_state: &mut GameState, ctx: &mut Context, time_delta: f32) {
        Self::run_frame_time_update(game_state, ctx, time_delta, false);

        let exiting_menu = game_state.menu_stack.len() == 0;

        if exiting_menu {
            if game_state.ui_game_display_zoom < 1.0 {
                if game_state.ui_game_display_zoom < 0.3 {
                    game_state.ui_game_display_zoom += 0.7 * time_delta;
                }
                else if game_state.ui_game_display_zoom < 0.6 {
                    game_state.ui_game_display_zoom += 1.0 * time_delta;
                }
                else {
                    game_state.ui_game_display_zoom += 1.5 * time_delta;
                }
                if game_state.ui_game_display_zoom >= 1.0 {
                    game_state.ui_game_display_zoom = 1.0;
                }
            }
        }
        else {
            //println!("Running menu step");
            if game_state.ui_game_display_zoom > 0.0 {
                game_state.ui_game_display_zoom -= 0.15 * time_delta;
                if game_state.ui_game_display_zoom < 0.0 {
                    game_state.ui_game_display_zoom = 0.0;
                }
            }
        }

        //CoreSystem::run_menu_update(game_state, ctx, time_delta);
        InputSystem::handle_menu_input(game_state, time_delta);

        let mut input = game_state.world.fetch_mut::<InputResource>();

        let mut wactions = input.actions.drain(0..).collect::<Vec<_>>();

        if input.exit_flag {
            ggez::event::quit(ctx);
        }

        input.clear_actions();
        drop(input);

        if !exiting_menu {

            for world_action in &wactions {
                match world_action {
                    WorldAction::CloseAllMenus => {
                        println!("Close All Menus");
                        game_state.close_all_menus();
                    },
                    WorldAction::CloseMenu => {
                        println!("Close Menu");
                        game_state.close_menu();
                        
                    },
                    WorldAction::OpenSubMenu(name) => {
                        println!("Open SubMenu {}", &name);
                        game_state.open_submenu(name.clone());
                    },
                    WorldAction::ExitGame => {
                        println!("Exit Game");
                        ggez::event::quit(ctx);
                    },
                    WorldAction::RestartLevel => {
                        game_state.close_all_menus();
                        game_state.restart_level(ctx);
                    },
                    WorldAction::NewGame => {
                        game_state.close_all_menus();
                        game_state.load_level(ctx, game_state.start_level_name.clone(), "".to_string());
                    },
                    WorldAction::ToggleFullscreen => {
                        game_state.toggle_fullscreen_mode(ctx);
                    },
                    // WorldAction::ToggleConsole => {
                    //     game_state.toggle_terminal();
                    //     println!("[menu step - toggle] Terminal Open set to {}", &game_state.terminal_open);
                    // },
                    _ => {}
                }
            }
        }
        else {
            for world_action in &wactions {
                match world_action {
                    WorldAction::CloseMenu => {
                        println!("Open Menu");
                        game_state.open_menu();
                    },
                    // WorldAction::ToggleConsole => {
                    //     game_state.toggle_terminal();
                    //     println!("[menu step (exiting) - toggle] Terminal Open set to {}", &game_state.terminal_open);
                    // },
                    _ => {}
                }
            }
        }

        Self::clear_inputs(game_state);
    }

    // Run non-paused update of world - Edit mode, Play mode (Playing/Dialog)
    pub fn run_update_step(game_state: &mut GameState, ctx: &mut Context, time_delta: f32) {

        // // Register actual pre-processed time (application time)
        // game_state.update_run_time(time_delta);

        // // Pre-process frame time (for simulation time)
        // let time_delta = game_state.process_time_delta(ctx, time_delta);
    
        // // Save frame time
        // game_state.set_frame_time(time_delta);        
        let time_delta = Self::run_frame_time_update(game_state, ctx, time_delta, true);

        let mut new_state = game_state.running_state.clone();
        let menu_lvls = game_state.menu_stack.len();
        let mut state_change = false;
        match &game_state.mode {
            GameMode::Edit => {

            },
            GameMode::Play => {
                if menu_lvls > 0 {
                    //let input_res = self.world.fetch::<InputResource>();
                    //CoreSystem::run_menu_update(game_state, ctx, time_delta);
                    println!("ERROR MENU WITHIN RUN UPDATE STEP #(*&#(*& $*#($(*# &*#( *&$");
                }
                else {
                    match &game_state.running_state { 
                        RunningState::Playing => {

                            if game_state.game_frame_count % 60 == 0 {
                                println!("A) Run Play Update ==================================");
                            }
                            // Update components in play mode
                            CoreSystem::run_play_update(game_state, ctx, time_delta);
            
                            if game_state.game_frame_count % 60 == 0 {
                                println!("   World Maintain  ==================================");
                            }
                            // Cleanup the world state after changes
                            game_state.world.maintain();

                            if game_state.game_frame_count % 60 == 0 {
                                println!("B) Run Physics Update ==================================");
                            }
                            // Run physics simulation for frame
                            PhysicsSystem::run_physics_update(&mut game_state.world, ctx, &mut game_state.phys_world, time_delta);
            
                            if game_state.game_frame_count % 60 == 0 {
                                println!("C) Run Post Physics Update ==================================");
                            }
                            // Update components after physics
                            //self.run_post_physics_update(ctx, time_delta);
                            CoreSystem::run_post_physics_update(game_state, ctx, time_delta);

                            if game_state.game_frame_count % 60 == 0 {
                                println!("   World Maintain  ==================================");
                            }
                            game_state.world.maintain();

                            // Tell game state a game frame finished its cycle
                            game_state.game_frame_completed();
                        },
                        RunningState::Dialog{..} => {
                            //let input_res = self.world.fetch::<InputResource>();
                            new_state = CoreSystem::run_dialog_update(game_state, ctx, time_delta);
                            //InputSystem::handle_dialog_input(&input_res, &self, time_delta);
                            if new_state == RunningState::Playing {
                                println!("Dialog > Playing");
                                state_change = true;
                            }
                        }
                    }
                }
            }
        }

        let mut start_pause = false;
        let mut open_menu = false;
        //let mut open_terminal = false;
        let mut slow_mode = false;
        let mut go_anywhere_mode = false;
        let mut toggle_fullscreen = false;
        {
            let input = game_state.world.fetch_mut::<InputResource>();
            if input.keys_pressed.len() > 0 {
                //println!("Frame Key Presses: - - - - - -");
                for key in &input.keys_pressed {
                    //println!("InputKey Pressed: {:?}", &key);
                    if menu_lvls == 0 {
                        if key == &InputKey::Pause {
                            // match game_state.current_state {
                            //     State::Paused => {
                            //         // println!("Pause activated -------------------------");
                            //         // start_play = true;
                            //     },
                            //     State::Running => {
                            if game_state.running_state == RunningState::Playing {
                                //RunningState::Playing => {
                                //println!("Pause activated -------------------------");
                                start_pause = true;
                                //},
                                //_ => {} // don't pause on dialogs
                            }
                            //     }
                            // }
                        }
                        else if key == &InputKey::Exit {
                            open_menu = true;
                        }
                        else if key == &InputKey::SlowMode {
                            slow_mode = true;
                        }
                        else if key == &InputKey::CheatGoAnywhere {
                            go_anywhere_mode = true;
                        }
                        else if key == &InputKey::Fullscreen {
                            toggle_fullscreen = true;
                        }
                        // else if key == &InputKey::ConsoleKey {
                        //     open_terminal = true;
                        // }
                    } 
                }
                //println!(" - - - - - - - - - - - - - - -");
            }

            if input.exit_flag {
                ggez::event::quit(ctx);
            }

            drop(input);
        }

        Self::clear_inputs(game_state);

        {
            //let curr_game_time = game_state.world.fetch::<GameStateResource>().game_run_seconds;
            //let curr_game_time = 
            let mut log = game_state.world.fetch_mut::<GameLog>();

            let mut delete_to_index = -1;
            for entry in log.entries.iter_mut() {
                entry.time_left -= time_delta;
                if entry.time_left <= 0.0 {
                    delete_to_index += 1; // -1 to 0 for 1st, 0 to 1 for 2nd
                }
            }
            // remove first entry for [delete_to_index] times
            while delete_to_index >= 0 && log.entries.len() > 0 {
                log.entries.remove(0);
                delete_to_index -= 1;
            }
            
        }

        if start_pause {
            game_state.pause();
        }
        else if open_menu {
            game_state.open_menu();
        }
        // else if open_terminal {
        //     game_state.set_terminal_open(true);
        //     println!("[run update step] Terminal Open set to {}", &game_state.terminal_open);
        // }
        else if slow_mode {
            let curr_scale = game_state.play_time_scale;
            if curr_scale >= 1.0 {
                game_state.set_timescale(0.5);
            }
            else if curr_scale >= 0.5 {
                game_state.set_timescale(0.25);
            }
            else {
                game_state.set_timescale(1.0);
            }
        }
        else if go_anywhere_mode {
            if let Some(player_entity) = game_state.world.get_player() {
                if player_entity.gen().is_alive() {
                    // Get character component for world player
                    let mut player_res = game_state.world.write_storage::<CharacterDisplayComponent>();
                    if let Some(ref mut player) = player_res.get_mut(player_entity) {
                        player.go_anywhere_mode = !player.go_anywhere_mode;
                        let mut log = game_state.world.fetch_mut::<GameLog>();
                        log.add_entry(true, format!("Go Anywhere: {:?}!", &player.go_anywhere_mode), None,
                            game_state.world.fetch_mut::<GameStateResource>().game_run_seconds);
                    }
                }
            }
        }
        else if toggle_fullscreen {
            game_state.toggle_fullscreen_mode(ctx);
        }
        
        // Update state if any step changed the game running state
        if state_change {
            game_state.set_running_state(ctx, new_state); //running_state = new_state;
        }

        {
            // TEST GAME GEOMETRY MORPH
            // if game_state.game_frame_count % 5 == 0 {
            //     let geometry = &mut game_state.level_geometry.patches;

            //     for patch in geometry {                
            //         patch.modify_random_many(3);
            //     }
    
            // }
        }

    }

    pub fn run_pause_step(game_state: &mut GameState, ctx: &mut Context, time_delta: f32) {

        let time_delta = Self::run_frame_time_update(game_state, ctx, time_delta, false);
        game_state.paused_anim += time_delta;

        // Check Input Resources for Pause Key Presses
        //let mut start_play = false;
        //let mut input = game_state.world.fetch_mut::<InputResource>();

        InputSystem::handle_paused_input(game_state, time_delta);
    }

    // Process Dialog update - check for unpause trigger
    pub fn run_dialog_update(game_state: &mut GameState, _ctx: &mut Context, time_delta: f32) -> RunningState {
        
        // {
        //     let world = &mut game_state.world;
        //     let mut input_sys = InputSystem::new();
        //     input_sys.run_now(&world);
        // }

        let mut input_res = game_state.world.fetch_mut::<InputResource>();
        let new_state = InputSystem::handle_dialog_input(&mut input_res, &game_state, time_delta);

        Camera::update(game_state, time_delta);

        new_state
    }

    pub fn run_logic_update(game_state: &mut GameState) {
        let mut logic_sys = LogicSystem {
            show_debug_output: game_state.debug_logic_frames > 0 
        };

        logic_sys.run_now(&game_state.world);
    }

    // Process Playing update - run all game systems
    pub fn run_play_update(game_state: &mut GameState, ctx: &mut Context, time_delta: f32) {
        let world = &mut game_state.world;

        // RUN LOGIC ---------------------------------------
        {            
            let mut logic_sys = LogicSystem {
                show_debug_output: game_state.debug_logic_frames > 0 
            };

            if logic_sys.show_debug_output {
                println!(" - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -");
                println!("Starting logic pass {} =============================================================", &game_state.debug_logic_frames);
            }
            logic_sys.run_now(&world);

            if game_state.debug_logic_frames > 0 {
                game_state.debug_logic_frames -= 1;
            }
        }

        // Run Input System - Mainly player/npc inputs, process meows, process input clicks
        {
            let camera = world.fetch::<Camera>();
            let mut display_offset = na::Point2::new(camera.display_offset.0, camera.display_offset.1);
            drop(camera);


            if game_state.game_frame_count % 60 == 0 {
                println!("Run InputSystem =======================");
            }
            // Run InputSystem on world
            // outputs: meow locations and clicked entity info
            let mut input_sys = InputSystem::new();
            //let physics_world = self.phys_world;
            //input_sys.phys_world = Some(Box::pi(physics_world));
            input_sys.run_now(&world);

            if game_state.game_frame_count % 60 == 0 {
                println!("   Build new Meows ====================");
            }
            // Process meow creation
            let mut meow_count : i32 = 0;
            for m in &input_sys.meows {
                // Create a meow bubble
                MeowBuilder::build(world, ctx, &mut game_state.phys_world, m.0.x, m.0.y, m.1.x, m.1.y, 20.0, 20.0);
                meow_count += 1;
            }
            if meow_count > 0 {
                game_state.audio.play_meow();
            }

            // CLICK - COLLIDER HANDLING CODE - in testing =========================
            if game_state.game_frame_count % 60 == 0 {
                println!("Run Click Check Code ==================");
            }
            // Get display size for click position calculations
            let dim = ggez::graphics::drawable_size(ctx);
            //let mut display_offset = game_state.current_offset;
            display_offset.x += dim.0 as f32 / 2.0;
            display_offset.y += dim.1 as f32 / 2.0;

            // list for entities found at click
            let mut entity_clicked : Vec<u32> = vec![];

            let mut click_items : i32 = 0;

            // Iterate through any click_info item from input system
            for click in &input_sys.click_info {
                // get center x/y based on click and display offset
                // to get from screen coordinates to game object coords
                let center_x = click.x - display_offset.x;
                let center_y = click.y - display_offset.y;
                click_items += 1;
                // println!("Click position: {}, {}", &click.x, &click.y);
                // println!("Display offset: {}, {}", &display_offset.x, &display_offset.y);
                // println!("Click game pos: {}, {}", &(click.x-1.0-display_offset.x), &(click.y-1.0-display_offset.y));

                // create bounding box for click position to check colliders
                // very small rectangle around the cursor position translated into world coords
                // let mut aabb = b2::AABB::new();
                // // create physics-scale positions for bounding box
                // aabb.lower = physics::create_pos(&na::Point2::new(center_x-0.5, center_y-0.5));
                // aabb.upper = physics::create_pos(&na::Point2::new(center_x+0.5, center_y+0.5));
        
                {
                    let physics = &game_state.phys_world;
                    // create object which received click collide info
                    let mut query_results = physics::box_query(physics, center_x, center_y, 1.0, 1.0);//BoxQueryInfo::new();
                    // query physics world with aabb, updating click_info
                    //physics.query_aabb(&mut query_results, &aabb);
        
                    // go through click info from query
                    for (b, f) in &query_results.hit_info {
                        println!("Clicked {:?},{:?}", &b, &f);
                        // get physics body
                        let body = game_state.phys_world.body(*b);
                        // get body user data with entity id
                        let body_data = &*body.user_data();
                        let ent_id = body_data.entity_id;
                        // add entity id to vector
                        entity_clicked.push(ent_id);
                    }
        
                    // clear click info from results object
                    query_results.hit_info.clear();
                    drop(query_results);
        
                }
            }

            let mut ent_click_ct : i32 = 0;
            for ent in &entity_clicked {
                println!("Entity {:?} clicked", &ent);
                ent_click_ct += 1;
            }
            if click_items > 0 {
                if ent_click_ct == 0 {
                    //println!("No entity clicked");
                }
                else {
                    game_state.audio.play_meow();
                }
            }

            input_sys.meows.clear();
            input_sys.click_info.clear();
            drop(input_sys);
        }


        // Non-collider velocity system
        {
            if game_state.game_frame_count % 60 == 0 {
                println!("Run Velocity Update loop ==================");
            }
            // Operator on meows, collisions and sprite components
            let mut pos_writer = world.write_storage::<Position>();
            let collision_reader = world.read_storage::<Collision>();
            let velocity_reader = world.read_storage::<Velocity>();
            let entities = world.entities();                

            for (pos, vel, coll_opt, ent) in (&mut pos_writer, &velocity_reader, (&collision_reader).maybe(), &entities).join() {
                let mut no_collision = true;
                if let Some(_) = coll_opt {
                    no_collision = false;
                }
                if game_state.game_frame_count % 60 == 0 {
                    println!(" Vel for {:?} - No collision? {}", &ent, &no_collision);
                }

                if no_collision {
                    println!(" From Pos: {:?}", &pos);
                    pos.x += vel.x * time_delta;
                    pos.y += vel.y * time_delta;
                    println!(" .. To Pos: {:?}", &pos);
                }
            }
        }

        // Meow "system" - updates meow state and components
        //  This could be moved into a system as long as the physics data was accessible & writable from the system class
        {
            if game_state.game_frame_count % 60 == 0 {
                println!("Run Meow Update loop ==================");
            }
            // Operator on meows, collisions and sprite components
            let mut meow_writer = world.write_storage::<MeowComponent>();
            let mut collision_writer = world.write_storage::<Collision>();
            let mut sprite_writer = world.write_storage::<SpriteComponent>();
            let entities = world.entities();                

            for (meow, coll, sprite, ent) in (&mut meow_writer, &mut collision_writer, &mut sprite_writer, &entities).join() {
                // update meow components
                meow.update(time_delta, coll, sprite, &mut game_state.phys_world);

                if meow.meow_time < 0.0 {

                    // Destroy physics body of collision component
                    coll.destroy_body(&mut game_state.phys_world);
                    // Delete entity from ecs world
                    entities.delete(ent).expect("Failed to delete meow");
                }
            }
        }

        // Portal "system"
        {
            if game_state.game_frame_count % 60 == 0 {
                println!("Run Portal update loop ==================");
            }
            // Operator on meows, collisions and sprite components
            let mut portal_writer = world.write_storage::<PortalComponent>();
            let mut collision_writer = world.write_storage::<Collision>();
            let mut sprite_writer = world.write_storage::<SpriteComponent>();
            let mut anim_sprite_writer = world.write_storage::<AnimSpriteComponent>();
            let entities = world.entities();                

            for (portal, coll, sprite, anim_sprite, _ent) in 
                (&mut portal_writer, &mut collision_writer, (&mut sprite_writer).maybe(), (&mut anim_sprite_writer).maybe(), &entities).join() {
                // update portal components
                if let Some(sprite) = sprite {
                    portal.update_sprite(time_delta, coll, sprite, &mut game_state.phys_world);
                }
                else if let Some(sprite) = anim_sprite {
                    portal.update_anim_sprite(time_delta, coll, sprite, &mut game_state.phys_world);
                }
            }
        }

        // Animation system
        {
            if game_state.game_frame_count % 60 == 0 {
                println!("Run Animation update loop ==================");
            }
            //let mut world = &mut self.world;
            let mut anim_sys = AnimationSystem {
            };

            anim_sys.run_now(&world);

        }

        // Particle system
        {
            if game_state.game_frame_count % 60 == 0 {
                println!("Run ParticleSys update loop ==================");
            }
            //let mut world = &mut self.world;
            let mut particle_sys = ParticleSystem {};

            particle_sys.run_now(&world);

        }

        {
            if game_state.game_frame_count % 60 == 0 {
                println!("Run Pickup update loop ==================");
            }
            // Operator on meows, collisions and sprite components
            let mut pickup_writer = world.write_storage::<PickupComponent>();
            let mut collision_writer = world.write_storage::<Collision>();
            let mut anim_sprite_writer = world.write_storage::<AnimSpriteComponent>();
            let entities = world.entities();                

            for (pickup, coll, sprite, _ent) in (&mut pickup_writer, &mut collision_writer, &mut anim_sprite_writer, &entities).join() {
                // update pickup components
                pickup.update(time_delta, coll, sprite);
            }
        }

        // {
        //     // Operator on npcs, updating sprite x scale to match facing direction
        //     let mut npc_reader = world.read_storage::<NpcComponent>();
        //     let mut sprite_writer = world.write_storage::<SpriteComponent>();
        //     let mut anim_sprite_writer = world.write_storage::<AnimSpriteComponent>();
        //     let entities = world.entities();                

        //     for (npc, sprite_opt, anim_sprite_opt, _ent) in 
        //         (&npc_reader, (&mut sprite_writer).maybe(), (&mut anim_sprite_writer).maybe(), &entities).join() {
        //         // update portal components
        //         let is_moving = npc.going_right || npc.going_left;
        //         if npc.is_enabled && is_moving {
        //             if let Some(sprite) = sprite_opt {
        //                 let sx = sprite.scale.x;

        //                 if npc.going_right && sx < 0.0 {
        //                     sprite.scale.x = -sprite.scale.x;
        //                 }
        //                 if npc.going_left && sx > 0.0 {
        //                     sprite.scale.x = -sprite.scale.x;
        //                 }
        //             }
        //             if let Some(sprite) = anim_sprite_opt {
        //                 let sx = sprite.scale.x;

        //                 if npc.going_right && sx < 0.0 {
        //                     sprite.scale.x = -sprite.scale.x;
        //                 }
        //                 if npc.going_left && sx > 0.0 {
        //                     sprite.scale.x = -sprite.scale.x;
        //                 }
        //             }
        //         }
        //     }
        // }

        // Button "system"
        let mut spawn_ghost = false;
        let mut spawn_box = false;
        let mut spawn_platform = false;
        let mut spawn_mouse = false;
        let mut spawn_closed_box = false;
        {
            if game_state.game_frame_count % 60 == 0 {
                println!("Run Button update loop ==================");
            }
            // Operator on meows, collisions and sprite components
            let mut button_reader = world.write_storage::<ButtonComponent>();
            let mut collision_writer = world.write_storage::<Collision>();
            let entities = world.entities();                

            for (button, coll, _ent) in (&mut button_reader, &mut collision_writer, &entities).join() {
                // update button components
                button.update(time_delta, coll, &mut game_state.phys_world);
                // if button.triggered {
                //     //println!("Button {} triggered", &button.name);
                //     if button.name == "ghost" {
                //         spawn_ghost = true;
                //     //     let test : u16 = rng.gen::<u16>();
                //     //     crate::entities::ghost::GhostBuilder::build_collider(&mut self.world, ctx, &mut self.phys_world, 100.0, 400.0, 0.0, 0.0,
                //     //         30.0, 0.15, 25.0, 25.0);
                //     }
                //     else if button.name == "box" {
                //         spawn_box = true;
                //     //     let test : u16 = rng.gen::<u16>();
                //     //     let w = 10.0 + 0.001 * test as f32;
                //     //     crate::entities::empty_box::BoxBuilder::build_dynamic(&mut self.world, ctx, &mut self.phys_world, 200.0, 100.0,
                //     //         w, w, rng.gen::<f32>() * 2.0 * 3.14159, SpriteLayer::Entities.to_z());
                //     }
                //     else if button.name == "closed_box" {
                //         spawn_closed_box = true;
                //     }
                //     else if button.name == "platform" {
                //         spawn_platform = true;
                //     //     let test : u16 = rng.gen::<u16>();
                //     //     let w = 50.0 + 0.001 * test as f32;
                //     //     let h = 10.0 + 0.00025 * test as f32;
                //     //     crate::entities::platform::PlatformBuilder::build_dynamic(&mut self.world, ctx, &mut self.phys_world, 200.0, 100.0,
                //     //         w, h, 0.0, SpriteLayer::Entities.to_z());                            
                //     }
                //     else if button.name == "mouse" {
                //         spawn_mouse = true;
                //     }
                // }
            }
        }

        // if spawn_mouse {
        //     crate::entities::mouse::MouseBuilder::build(&mut self.world, ctx, &mut self.phys_world, 150.0, -50.0, 12.0, 6.0, 0.0, SpriteLayer::Entities.to_z() );
        // }
        // if spawn_ghost {
        //     //let mut rng = rand::thread_rng();
        //     //let test : u16 = rng.gen::<u16>();
        //     crate::entities::ghost::GhostBuilder::build_collider(&mut self.world, ctx, &mut self.phys_world, 100.0, 400.0, 0.0, 0.0,
        //         30.0, 0.15, 25.0, 25.0);
        // }
        // if spawn_box {
        //     let mut rng = rand::thread_rng();
        //     let test : u16 = rng.gen::<u16>();
        //     let w = 10.0 + 0.001 * test as f32;
        //     crate::entities::empty_box::BoxBuilder::build_dynamic(&mut self.world, ctx, &mut self.phys_world, 200.0, 100.0,
        //         w, w, rng.gen::<f32>() * 2.0 * 3.14159, SpriteLayer::Entities.to_z());
        // }
        // if spawn_closed_box {
        //     let mut rng = rand::thread_rng();
        //     let test : u16 = rng.gen::<u16>().min(20000);
        //     let w = 10.0 + 0.001 * test as f32;
        //     crate::entities::platform::PlatformBuilder::build_dynamic_w_image(&mut self.world, ctx, 
        //         &mut self.phys_world, 200.0, 100.0, w, w, 0.0, SpriteLayer::Entities.to_z(), "entities/closed_box".to_string(), 41.0, 41.0);

                
        // }
        // if spawn_platform {
        //     let mut rng = rand::thread_rng();
        //     let test : u16 = rng.gen::<u16>();
        //     let w = 50.0 + 0.001 * test as f32;
        //     let h = 10.0 + 0.00025 * test as f32;
        //     crate::entities::platform::PlatformBuilder::build_dynamic(&mut self.world, ctx, &mut self.phys_world, 200.0, 100.0,
        //         w, h, 0.0, SpriteLayer::Entities.to_z());   
        // }

        // Camera System
        {
            Camera::update(game_state, time_delta);
        }

    }

    // Run system update after physics system runs
    /*******************************************************
     * Builds portal list then iterates components with collision/pos
     * and optionial Character, Npc, Sprite or AnimSprite components
     * Component processing:
     * * Character-Exit condition - set exit info on game state
     * * Collider-Portal condition - check in/out direction, check validity,
     *     update pos/velocity of collider
     * * If valid warp & character - set facing_right and start fall if not falling
     * * If valid warp & npc - set facing_right from portal interaction
     * If exit flagged, update game_state with exit poral info

    *******************************************************/
    pub fn run_post_physics_update(game_state: &mut GameState, ctx: &mut Context, time_delta: f32) {
        //let world = &mut game_state.world;

        let mut exit_name = "".to_string();
        let mut exit_entry_name = "".to_string();
        //let mut portal_id = -1;

        {
            // get character entities, handle portal & exit statuses
            let entities = game_state.world.entities();
            // operate on optional characters or npcs
            let mut char_res = game_state.world.write_storage::<CharacterDisplayComponent>();
            let mut npc_res = game_state.world.write_storage::<NpcComponent>();
            // position/velocity components
            let mut pos_res = game_state.world.write_storage::<Position>();
            //let mut vel_res = game_state.world.write_storage::<Velocity>();
            // collision component
            let mut coll_res = game_state.world.write_storage::<Collision>();
            let mut sprite_res = game_state.world.write_storage::<SpriteComponent>();
            let mut anim_sprite_res = game_state.world.write_storage::<AnimSpriteComponent>();

            // BUILD INFO ON PORTALS (hash by name)
            // hash to store portal names and their positions - avoid needing to search for them later
            let mut portal_hash = HashMap::<String,(i32,f32,f32,bool,(f32,f32))>::new();
            let portal_res = game_state.world.read_storage::<PortalComponent>();
            // Insert portal information into hash
            for (portal, pos, _ent) in (&portal_res, &pos_res, &entities).join() {
                portal_hash.insert(portal.name.clone(), (_ent.id() as i32, pos.x, pos.y, portal.screen_facing, (portal.normal.x, portal.normal.y)));
            }

            // Iterate Position/Collision with possible Character or Npc, with possible Sprite or AnimSprite
            //  Process Exit interation, Portal interaction,
            // Join entities and their components to process physics update
            for (_ent, mut character_opt, mut npc_opt,  mut pos, mut coll, mut sprite, mut anim_sprite) in 
                (&entities, (&mut char_res).maybe(), (&mut npc_res).maybe(), &mut pos_res, &mut coll_res,
                    (&mut sprite_res).maybe(), (&mut anim_sprite_res).maybe()).join() {

                let mut facing_right = true;

                // CHARACTER - EXIT INTERACTION ---------------------------------------------------------------
                // Handle character-exit specially
                if let Some(ref mut character) = character_opt {
                    if character.since_warp < 0.5 { continue; }
                    // Handle character entered an exit and not already level warping
                    if character.is_controlled && character.in_exit && !game_state.level_warping {
                        // Get exit 
                        let exit_id = character.exit_id as i32;
                        facing_right = character.facing_right;
        
                        //println!("Character exiting..., {}", &exit_id);
                        let exit_ent = game_state.world.entities().entity(exit_id as u32);
                        let exit_res = game_state.world.read_storage::<ExitComponent>();
                        if let Some(exit) = exit_res.get(exit_ent) {
                            println!("Exit info {:?}", &exit);
                            let mut exit_dest = exit.destination.clone();

                            if exit_dest.is_empty() == false {
                                if let Some(index) = exit_dest.find(":") {
                                    let (just_exit_name, entry_name) = exit_dest.split_at_mut(index);
                                    let (_, entry_name) = entry_name.split_at(1);
                                    exit_name = just_exit_name.to_string();
                                    exit_entry_name = entry_name.to_string();
                                }
                                else {
                                    exit_name = exit_dest;
                                    exit_entry_name = "".to_string();
                                }


                            }                            
                        }
        
                    }

                }

                // COLLIDER - PORTAL INTERACTION ---------------------------------------------------------------
                // Handle Collider entered portal - generic portal behavior
                if coll.in_portal && coll.since_warp > 0.35 { // && coll.since_warp > 0.75
                    // get
                    let portal_id = coll.portal_id as i32;
                    //println!("Collider since warp: {}", &coll.since_warp);

                    //exit_id = character.exit_id as i32;

                    //let exit_res = world

                    //println!("Collider exiting..., {}", &portal_id);
                    let mut valid_warp = true;

                    // PORTAL VALIDITY CHECK - Use portal and character direction/velocity to check
                    let portal_ent = game_state.world.entities().entity(portal_id as u32);
                    let portal_res = game_state.world.read_storage::<PortalComponent>();
                    if let Some(portal) = portal_res.get(portal_ent) {
                        let source_normal = portal.normal;
                        //println!("Portal info {:?}", &portal);
                        let portal_dest = portal.destination.clone();
                        if portal_dest.is_empty() == false {
                            if let Some((new_portal_id, x, y, scr_facing, normal)) = portal_hash.get(&portal_dest) {

                                //println!("Portal at {}, {}", &x, &y);
                                let mut flip_x = true;
                                let mut flip_y = true;

                                if *scr_facing == false {
                                    if !(source_normal.x < 0.0 && normal.0 < 0.0) && !(source_normal.x > 0.0 && normal.0 > 0.0) {
                                        flip_x = false;
                                    }
                                    //flip_x = false;
                                    if !(source_normal.y < 0.0 && normal.1 < 0.0) && !(source_normal.y > 0.0 && normal.1 > 0.0) {
                                        flip_y = false;
                                    }
                                }

                                let mut nx = *x;
                                let mut ny = *y;
                                let mut nvx = coll.vel_last.x * 1.0;
                                let mut nvy = coll.vel_last.y * 1.0;
                                let mut avg_x = coll.get_avg_x(10);
                                if avg_x < 0.0 && nvx < avg_x {
                                    avg_x = nvx;
                                }
                                if avg_x > 0.0 && nvx > avg_x {
                                    avg_x = nvx;
                                }
                                let mut avg_y = -coll.get_avg_y(10);
                                if avg_y < 0.0 && -nvy < avg_y {
                                    avg_y = -nvy;
                                }
                                if avg_y > 0.0 && -nvy > avg_y {
                                    avg_y = -nvy;
                                }

                                let up_pos_y = -coll.vel_last.y;

                                if flip_x {
                                    nvx = -nvx;
                                }
                                if flip_y {
                                    nvy = -nvy;
                                }
                                //println!("Vel update from {},{} to {},{}", &vel.x, &vel.y, &nvx, &nvy);
                                if *scr_facing {
                                    if nvx > 0.0 {
                                        facing_right = true;
                                    }
                                    else if nvx < 0.0 {
                                        facing_right = false;
                                    }
                                }
                                else {
                                    // if a left/right normal, must be moving left or right to warp
                                    // left facing wall portal
                                    let movement_margin = 0.03;
                                    let movement_margin_y = 1.0;
                                    if source_normal.y == 0.0 && source_normal.x < 0.0 && avg_x < movement_margin {
                                        println!("LEFT facing wall portal - REJECTED ({}, {}), {}, {}", 
                                            &source_normal.x, &source_normal.y, &coll.vel_last.x, &movement_margin);
                                        valid_warp = false;
                                        println!(" -- Avg X: {}", &avg_x);
                                    }
                                    // right facing wall portal
                                    else if source_normal.y == 0.0 && source_normal.x > 0.0 && avg_x > -movement_margin {
                                        println!("Right facing wall portal - REJECTED ({}, {}), {}, {}", 
                                            &source_normal.x, &source_normal.y, &coll.vel_last.x, &-movement_margin);
                                        valid_warp = false;
                                        println!(" -- Avg X: {}", &avg_x);
                                    }
                                    // down facing wall portal
                                    if source_normal.y < 0.0 && source_normal.x == 0.0 && avg_y < movement_margin_y {
                                        // println!("Down facing wall portal - REJECTED ({}, {}), {}, {}", 
                                        //     &source_normal.x, &source_normal.y, &up_pos_y, &movement_margin);
                                        //valid_warp = false;
                                        //println!(" -- Avg Y: {}", &avg_y);
                                    }
                                    // up facing wall portal
                                    else if source_normal.y > 0.0 && source_normal.x == 0.0 && avg_y > -movement_margin_y {
                                        //println!("Up facing wall portal - REJECTED ({}, {}), {}, {}", 
                                            //&source_normal.x, &source_normal.y, &up_pos_y, &movement_margin);
                                        //valid_warp = false;
                                    }
                                    if source_normal.y < 0.0 && source_normal.x == 0.0 {
                                        println!("Down facing wall portal - Valid: {} ({}, {}), {}, {}", &valid_warp,
                                            &source_normal.x, &source_normal.y, &up_pos_y, &movement_margin);
                                        println!(" -- Avg Y: {}", &avg_y);
                                    }
                                    else if source_normal.y > 0.0 && source_normal.x == 0.0 {
                                        println!("Up facing wall portal - Valid: {} ({}, {}), {}, {}", &valid_warp,
                                            &source_normal.x, &source_normal.y, &up_pos_y, &movement_margin);
                                        println!(" -- Avg Y: {}", &avg_y);
                                    }
                                }

                                if valid_warp {
                                    if !scr_facing {
                                        // Use portal normal to place collider output location
                                        //  use collider width and height as base, 
                                        // if !flip_x {
                                        // }
                                        // else {
                                        //     nx -= 1.0 * coll.dim_1 * normal.0;
                                        // }
                                        if normal.0 < 0.0 {
                                            facing_right = false;
                                        }
                                        else if normal.0 > 0.0 {
                                            facing_right = true;
                                        }
    
                                        
                                        // move player x in normal's X direction
                                        nx += 2.0 * coll.dim_1 * normal.0;
                                        // move player y in normal's Y direction
                                        ny += 2.0 * coll.dim_2 * normal.1;

                                        // Up to ..
                                        if source_normal.y > 0.0 {
                                            // left/right
                                            if normal.1 == 0.0 {
                                                let down_comp = (-avg_y).max(0.0);
                                                let left_right_comp = avg_x;
                                                if normal.0 < 0.0 {
                                                    // translate y down vel to left vel
                                                    nvx = -down_comp;
                                                    nvy = -left_right_comp;
                                                }
                                                else if normal.0 > 0.0 {
                                                    // translate y vel to right vel
                                                    nvx = down_comp;
                                                    nvy = left_right_comp;
                                                }
                                            }
                                            else if normal.1 < 0.0 {
                                                nvy = nvy.max(-avg_y);
                                            }
                                            else if normal.1 > 0.0 {
                                                nvy = nvy.min(-avg_y);
                                            }
                                        }
                                        // Down to ..
                                        else if source_normal.y < 0.0 {
                                            // left/right
                                            if normal.1 == 0.0 {
                                                let up_comp = avg_y.min(0.0);
                                                let left_right_comp = avg_x;
                                                if normal.0 < 0.0 {
                                                    // translate y down vel to left vel
                                                    nvx = -up_comp;
                                                    nvy = -left_right_comp;
                                                }
                                                else if normal.0 > 0.0 {
                                                    // translate y vel to right vel
                                                    nvx = -up_comp;
                                                    nvy = left_right_comp;
                                                }
                                            }
                                        }
                                        // Left to ..
                                        if source_normal.x < 0.0 {
                                            // up/down
                                            if normal.0 == 0.0 {
                                                let down_up_comp = avg_y;
                                                let right_comp = avg_x.max(0.0);
                                                if normal.1 > 0.0 {
                                                    // translate x left vel to up vel
                                                    nvy -= right_comp;
                                                    nvx = down_up_comp;
                                                }
                                                else if normal.1 < 0.0 {
                                                    // translate x right vel to down vel
                                                    nvy += right_comp;
                                                    nvx = -down_up_comp;
                                                }
                                            }
                                        }
                                        // Right to ..
                                        else if source_normal.x > 0.0 {
                                            // up/down
                                            if normal.0 == 0.0 {
                                                let down_up_comp = avg_y;
                                                let left_comp = -(avg_x.min(0.0));
                                                if normal.1 > 0.0 {
                                                    // translate x left vel to up vel
                                                    nvy = -left_comp;
                                                    nvx = down_up_comp;
                                                }
                                                else if normal.1 < 0.0 {
                                                    // translate x right vel to down vel
                                                    nvy = left_comp;
                                                    nvx = -down_up_comp;
                                                }
                                            }
                                        }
                                    }
                                }
                                

                                //vel.x = nvx;
                                //vel.y = nvy;

                                // COMPLETE START WARP - UPDATE Position, Collision, Sprite/Facing
                                if valid_warp {
                                    println!("WARPING from {:?} to {:?}", &portal_id, new_portal_id);
                                    println!("Warp old pos: {:?} new pos: {:?}", &coll.pos, &(nx, ny));

                                    pos.x = nx;
                                    pos.y = ny;
    
                                    coll.pos.x = nx;
                                    coll.pos.y = ny;
                                    println!("Warp old vel: {:?} new vel: {:?}", &coll.vel, &(nvx, nvy));

                                    coll.vel.x = nvx;
                                    coll.vel.y = nvy;
                                    coll.since_warp = 0.0;
                                    coll.trigger_portal_warp = false;
                                    coll.portal_id = *new_portal_id;
                                    coll.last_portal_id = *new_portal_id;

       
                                    // Update Position and Velocity of collider body
                                    coll.update_body_transform(&mut game_state.phys_world, &na::Point2::<f32>::new(nx, ny));
                                    coll.update_body_velocity(&mut game_state.phys_world, &na::Vector2::<f32>::new(nvx, nvy));
    
                                    if let Some(ref mut sprite_comp) = sprite {
                                        //sprite
                                        //sprite_comp
                                        if sprite_comp.scale[0] > 0.0 && nvx < 0.1 {
                                            sprite_comp.scale[0] = -sprite_comp.scale[0];
                                        }
                                        else if sprite_comp.scale[0] < 0.0 && nvx > -0.1 {
                                            sprite_comp.scale[0] = -sprite_comp.scale[0];
                                        }
                                    }
                                    if let Some(ref mut anim_sprite_comp) = anim_sprite {
                                        //sprite
                                        //sprite_comp
                                        if anim_sprite_comp.scale[0] > 0.0 && nvx < 0.1 {
                                            anim_sprite_comp.scale[0] = -anim_sprite_comp.scale[0];
                                        }
                                        else if anim_sprite_comp.scale[0] < 0.0 && nvx > -0.1 {
                                            anim_sprite_comp.scale[0] = -anim_sprite_comp.scale[0];
                                        }
                                    }
                                }
    
                                
                            }
                        }
                        
                    }

                    // Update Facing flags based on portal change for Char/NPC
                    if valid_warp {
                        if let Some(ref mut character) = character_opt {
                            character.facing_right = facing_right;
                            if !character.in_fall {
                                character.start_fall();
                            }
                        }
                        if let Some(ref mut npc) = npc_opt {
                            npc.facing_right = facing_right;
                        }
                    }
                }
            }

        }

        // If Level Warp triggered - start warp within GameState
        if game_state.level_warping == false && exit_name.is_empty() == false {
            game_state.start_warp(exit_name, exit_entry_name);
        }

    }
}