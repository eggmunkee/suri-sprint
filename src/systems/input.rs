use ggez::nalgebra as na;
use ggez::{Context};
use specs::prelude::*;
use wrapped2d::b2;
use rand::prelude::*;

use crate::core::{GameState,RunningState};
use crate::core::game_state::{MenuItem,State};
use crate::core::input::{InputKey};
use crate::resources::{InputResource,WorldAction,GameStateResource,Camera};
use crate::components::*;
use crate::components::collision::{Collision};
use crate::components::player::*;
use crate::components::npc::{NpcComponent};
use crate::components::button::{ButtonComponent};
use crate::entities::level_builder::{LevelType};
use crate::core::physics::{CollisionCategory,PhysicsWorld};

// handle input state to control Players
// every frame, operate on velocity of player components
//  based on InputResource
pub struct InputSystem {
    pub meows: Vec::<(na::Point2<f32>,na::Vector2<f32>)>,

    //pub click_info: Vec::<(b2::BodyHandle,b2::FixtureHandle)>,
    pub click_info: Vec::<na::Point2<f32>>,
}
impl InputSystem {
    pub fn new() -> InputSystem {
        InputSystem {
            meows: vec![],
            click_info: vec![],
        }
    }

    fn handle_npc_input<'a>(&mut self, v: &mut (&mut Collision, &mut NpcComponent, Entity), input: &InputResource,
        ent: &Entities, lazy: &Read<'a, LazyUpdate>, time_delta: f32, level_type: &LevelType) {
        let (coll, npc, _e) = v;

        let x = coll.pos.x;
        let y = coll.pos.y;
        let body_movement = coll.get_movement();
        // Call npc update handler with current status
        //npc.update(body_movement, time_delta, x, y);

        match level_type {
            LevelType::Platformer => {
                npc.update_overhead(body_movement, time_delta, x, y);
            },
            LevelType::Overhead => {
                npc.update_overhead(body_movement, time_delta, x, y);
            },
        };
        
    }

    // handle input updates from an entity
    fn handle_player_input<'a>(&mut self, v: &mut (&mut Collision, &mut CharacterDisplayComponent, Entity), input: &InputResource,
        ent: &Entities, lazy: &Read<'a, LazyUpdate>, time_delta: f32, level_type: &LevelType) {
        let (coll, character, _e) = v;

        let body_movement = coll.get_movement();
        let char_x = coll.pos.x;
        let char_y = coll.pos.y;

        let mut up_pressed = false;
        let mut left_pressed = false;
        let mut right_pressed = false;
        let mut down_pressed = false;

        // apply input status to player
        if character.is_controlled {
            if input.dirs_pressed[0] {
                left_pressed = true;
            }
            else if input.dirs_pressed[1] {
                right_pressed = true;
            }
            // Apply vector length to velocity Y
            if input.dirs_pressed[2] || input.jump_pressed {
                up_pressed = true;
            }
            else if input.dirs_pressed[3] {
                down_pressed = true;
            }
    
            //if let Some(display) = _display {
            if character.input_enabled {
                character.going_up = up_pressed;
                character.going_left = left_pressed;
                character.going_right = right_pressed;
                character.going_down = down_pressed;
                character.meowing = input.fire_pressed;
            }
        }
        else {
            // Don't apply actual inputs, because player is not player controlled right now
        }
       
        match level_type {
            LevelType::Platformer => {
                character.update(body_movement, time_delta);
            },
            LevelType::Overhead => {
                character.update_overhead(body_movement, time_delta);
            },
        };
        

        if character.meowing {
            let mut x = coll.pos.x;
            let mut y = coll.pos.y;
            let mut vx = coll.vel.x;
            let mut vy = coll.vel.y;

            let meow_spd = 4.0f32;

            y -= 12.0;
            if right_pressed {
                vx = vx + meow_spd; //.max(vx * 1.1);
                x += 20.0;
            }
            else if left_pressed {
                vx = vx - meow_spd; //.min(vx * 1.1);
                x -= 20.0;
            }
            else if character.facing_right {
                vx = vx + meow_spd * 0.5;
                x += 20.0;
            }
            else {
                vx = vx -meow_spd * 0.5;
                x -= 20.0;
            }
            if up_pressed {
                vy = vy - meow_spd * 1.5;
            }
            else if down_pressed {
                vy = vy + meow_spd * 1.5;
            }

            self.meows.push((na::Point2::new(x, y),na::Vector2::new(vx, vy)));
            character.since_meow = 0.0;

        }

        if input.mouse_down[0] {
            self.click_info.push(na::Point2::new(input.mouse_x, input.mouse_y));
        }

    }

    // handle input updates from an entity
    pub fn handle_paused_input(game_state: &mut GameState, time_delta: f32) {

        // Check Input Resources for Pause Key Presses
        let mut start_play = false;
        let mut open_menu = false;
        let mut input = game_state.world.fetch_mut::<InputResource>();
        if input.keys_pressed.len() > 0 {
            println!("Paused Frame Key Presses: - - - - - -");
            for key in &input.keys_pressed {
                println!("InputKey Pressed: {:?}", &key);
                if key == &InputKey::Pause {
                    match game_state.current_state {
                        State::Paused => {
                            println!("Play activated -------------------------");
                            start_play = true;
                        },
                        _ => {}
                    }
                }
                if key == &InputKey::Exit {
                    open_menu = true;
                }
            }
            println!(" - - - - - - - - - - - - - - -");
        }

        input.keys_pressed.clear();
        drop(input);

        if start_play {
            game_state.play();
        }
        if open_menu {
            game_state.open_menu();
        }

    }

    // handle input updates from an entity
    pub fn handle_dialog_input(input: &mut InputResource, game_state: &GameState, time_delta: f32) -> RunningState {

        // let mut up_pressed = false;
        // let mut left_pressed = false;
        // let mut right_pressed = false;
        // let mut down_pressed = false;

        // // apply input status to player
        // if input.dirs_pressed[0] {
        //     left_pressed = true;
        // }
        // else if input.dirs_pressed[1] {
        //     right_pressed = true;
        // }
        // // Apply vector length to velocity Y
        // if input.dirs_pressed[2] || input.jump_pressed {
        //     up_pressed = true;
        // }
        // else if input.dirs_pressed[3] {
        //     down_pressed = true;
        // }

        // Apply vector length to velocity Y
        let mut exit_pressed = false;
                
        for key in &input.keys_pressed {
            match key {
                InputKey::Exit => {
                    exit_pressed = true;
                },
                _ => {}
            } 
        }

        if exit_pressed {
            input.add_action(WorldAction::OpenMenu);
        }
        
        if input.fire_pressed {
            // Clear fire pressed flag when dialog is closed
            input.fire_pressed = false;

            RunningState::Playing
        }
        else {
            game_state.running_state.clone()
        }

    }

    // handle input updates from an entity
    pub fn handle_menu_input(game_state: &mut GameState, time_delta: f32) {

        let mut input = game_state.world.fetch_mut::<InputResource>();

        let mut up_pressed = false;
        let mut left_pressed = false;
        let mut right_pressed = false;
        let mut down_pressed = false;
        let mut fire_pressed = false;
        let mut toggle_fullscreen = false;

        let curr_menu_lvl = game_state.menu_stack.len() - 1;
        let mut curr_menu = &mut game_state.menu_stack[curr_menu_lvl];

        // Apply vector length to velocity Y
        let mut exit_pressed = false;
        
        for key in &input.keys_pressed {
            match key {
                InputKey::P1Up => {
                    up_pressed = true;
                },
                InputKey::P1Down => {
                    down_pressed = true;
                },
                InputKey::P1Right => {
                    right_pressed = true;
                },
                InputKey::P1Left => {
                    left_pressed = true;
                },
                InputKey::Exit => {
                    exit_pressed = true;
                },
                InputKey::P1PrimaryAction | InputKey::Pause => {
                    fire_pressed = true;
                },
                InputKey::Fullscreen => {
                    toggle_fullscreen = true;
                },
                _ => {}
            } 
        }

        if exit_pressed {
            // if game_state.menu_stack.len() > 0 {
            //     game_state.menu_stack.pop();
            // }
            //input.add_action(WorldAction::CloseMenu);
            input.add_action(WorldAction::CloseMenu);
        }
        if toggle_fullscreen {
            input.add_action(WorldAction::ToggleFullscreen);
        }

        //let mut open_menu_name = String::new();
        if fire_pressed {
            match &curr_menu.items[curr_menu.selected_index as usize] {
                MenuItem::ButtonItem { key, .. } => {
                    println!("Button pressed: {:?}", &key);
                    if key == "resume" {
                        input.add_action(WorldAction::CloseAllMenus);
                    }
                    else if key == "new_game" {
                        input.add_action(WorldAction::NewGame);
                    }
                    else if key == "restart_level" {
                        input.add_action(WorldAction::RestartLevel);
                    }
                    else if key == "exit" {
                        input.add_action(WorldAction::ExitGame);
                    }
                    else if key == "options" || key == "advanced" {
                        //open_menu_name = key.to_string();
                        input.add_action(WorldAction::OpenSubMenu(key.to_string()));
                    }
                    else if key == "close_menu" {
                        //open_menu_name = key.to_string();
                        input.add_action(WorldAction::CloseMenu);
                    }
                    else if key == "fullscreen" {
                        input.add_action(WorldAction::ToggleFullscreen);
                    }
                },
                other => {
                    println!("Other menu item pressed: {:?}", &other);
                }
            }

            input.fire_pressed = false;
        }

        if up_pressed {

            if curr_menu.selected_index > 0 {
                let mut test_index = curr_menu.selected_index - 1;

                let mut found_valid_item = false;
                while (!found_valid_item && test_index >= 0) {
                    match &curr_menu.items[test_index as usize] {
                        MenuItem::Header(_) => {
    
                        },
                        _ => {
                            found_valid_item = true;
                            break;
                        }
                    }
                    test_index = test_index - 1;
                }

                if found_valid_item {
                    curr_menu.selected_index = test_index;
                }
                //curr_menu.selected_index = curr_menu.selected_index - 1;
            }
        }
        else if down_pressed {
            if curr_menu.selected_index + 1 < curr_menu.items.len() as i32 {
                curr_menu.selected_index = curr_menu.selected_index + 1;
            }
        }

        match &mut curr_menu.items[curr_menu.selected_index as usize] {
            MenuItem::ToggleItem { key, value, .. } => {
                if right_pressed {
                    *value = true;
                    println!("Toggle pressed: {:?} Value: {}", &key, &value);
                }
                else if left_pressed {
                    *value = false;
                    println!("Toggle pressed: {:?} Value: {}", &key, &value);
                }
            },
            MenuItem::RangeItem { key, value, min, max, incr, .. } => {
                //println!("Range pressed: {:?}", &key);
                let mut value_update = false;
                if right_pressed {
                    if value < max {
                        *value = (*value + *incr).min(*max);
                        value_update = true;
                    }
                    println!("Range pressed: {:?} Value: {}", &key, &value);
                }
                else if left_pressed {
                    if value > min {
                        *value = (*value - *incr).max(*min);
                        value_update = true;
                    }
                    println!("Range pressed: {:?} Value: {}", &key, &value);
                }

                if value_update {
                    if key == "audio_volume" {
                        game_state.audio.set_sfx_volume(*value);
                        *value = game_state.audio.base_sfx_volume;
                    }
                    else if key == "music_volume" {
                        game_state.audio.set_music_volume(*value);
                        *value = game_state.audio.base_music_volume;
                    }
                }
            },
            _ => {}
        }

        drop(curr_menu);


    }
}


impl<'a> System<'a> for InputSystem {
    type SystemData = (WriteStorage<'a, Collision>,
                        WriteStorage<'a, CharacterDisplayComponent>,
                        WriteStorage<'a, NpcComponent>,
                        WriteStorage<'a, ButtonComponent>,
                        Write<'a, GameStateResource>,
                        Write<'a, InputResource>,
                        Write<'a, Camera>,
                        Entities<'a>,
                        Read<'a, LazyUpdate>);

    fn run(&mut self, (mut coll, mut char_display, mut npc, mut buttons,
        mut game_state, mut input, mut camera, mut ent, lazy): Self::SystemData) {

        let time_delta = game_state.delta_seconds;
        let level_type = game_state.level_type.clone();

        let target_x = game_state.player_target_loc.0;
        let target_y = game_state.player_target_loc.1;

        let mut rng = rand::thread_rng();

        // Get vec of npc input components to handle       
        let mut list = (&mut coll, &mut npc, &ent).join().collect::<Vec<_>>();
        for inn_v in list.iter_mut() { 

            //let dec = rng.gen::<f32>();
            let curr_x = (*inn_v.0).pos.x;
            let curr_y = (*inn_v.0).pos.y;
            let mut new_target_x = (rng.gen::<f32>() * 800.0) - 400.0;
            let mut new_target_y = (rng.gen::<f32>() * 800.0) - 400.0;

            let mut dist_range = 0;

            if (new_target_x - curr_x).abs() > 1000.0 {
                dist_range += 3;
                //new_target_x = (new_target_x + curr_x) / 2.0 + (rng.gen::<f32>() * 5000.0) - 2500.0;
            }
            else if (new_target_x - curr_x).abs() > 600.0 {
                dist_range += 2;
                //new_target_x = (new_target_x + curr_x) / 2.0 + (rng.gen::<f32>() * 3000.0) - 1500.0;
            }
            else if (new_target_x - curr_x).abs() > 300.0 {
                dist_range += 1;
                //new_target_x = (new_target_x + curr_x) / 2.0 + (rng.gen::<f32>() * 2000.0) - 1000.0;
            }
            if (new_target_y - curr_y).abs() > 1000.0 {
                dist_range += 3;
                //new_target_y = (new_target_y + curr_y) / 2.0 + (rng.gen::<f32>() * 5000.0) - 2500.0;
            }
            else if (new_target_y - curr_y).abs() > 600.0 {
                dist_range += 2;
                //new_target_y = (new_target_y + curr_y) / 2.0 + (rng.gen::<f32>() * 3000.0) - 1500.0;
            }
            else if (new_target_y - curr_y).abs() > 300.0 {
                dist_range += 1;
                //new_target_y = (new_target_y + curr_y) / 2.0 + (rng.gen::<f32>() * 2000.0) - 1000.0;
            }

            if dist_range == 1 {
                new_target_x = (new_target_x + curr_x) / 2.0 + (rng.gen::<f32>() * 1000.0) - 500.0;
                new_target_y = (new_target_y + curr_y) / 2.0 + (rng.gen::<f32>() * 1000.0) - 500.0;
            }
            else if dist_range == 2 {
                new_target_x = (new_target_x + curr_x) / 2.0 + (rng.gen::<f32>() * 2000.0) - 1000.0;
                new_target_y = (new_target_y + curr_y) / 2.0 + (rng.gen::<f32>() * 2000.0) - 1000.0;
            }
            else if dist_range == 3 {
                new_target_x = (new_target_x + curr_x) / 2.0 + (rng.gen::<f32>() * 3000.0) - 1500.0;
                new_target_y = (new_target_y + curr_y) / 2.0 + (rng.gen::<f32>() * 3000.0) - 1500.0;
            }
            else if dist_range == 4 {
                new_target_x = (new_target_x + curr_x) / 2.0 + (rng.gen::<f32>() * 4000.0) - 2000.0;
                new_target_y = (new_target_y + curr_y) / 2.0 + (rng.gen::<f32>() * 4000.0) - 2000.0;
            }
            else if dist_range == 5 {
                new_target_x = (new_target_x + curr_x) / 2.0 + (rng.gen::<f32>() * 5000.0) - 2500.0;
                new_target_y = (new_target_y + curr_y) / 2.0 + (rng.gen::<f32>() * 5000.0) - 2500.0;
            }
            else if dist_range > 5 {
                new_target_x = (new_target_x + curr_x) / 2.0 + (rng.gen::<f32>() * 9000.0) - 4500.0;
                new_target_y = (new_target_y + curr_y) / 2.0 + (rng.gen::<f32>() * 9000.0) - 4500.0;
            }

            if new_target_x < target_x {
                (*inn_v.1).target_position.0 = (target_x + 3.0).min(target_x);
            }
            else if new_target_x > target_x {
                (*inn_v.1).target_position.0 = (target_x - 3.0).max(target_x);
            }
            if new_target_y < target_y {
                (*inn_v.1).target_position.1 = (target_y - 3.0).min(target_y);
            }
            else if new_target_y > target_y {
                (*inn_v.1).target_position.1 = (target_y + 3.0).max(target_y);
            }
            
            

            // call npc input step
            self.handle_npc_input(inn_v, &*input, &ent, &lazy, time_delta, &level_type);
        }  
        drop(list);

        let mut list = (&mut npc, &mut char_display).join().collect::<Vec::<_>>();
        for inn_v in list.iter_mut() { 
            let (npc_comp, char_comp) = inn_v;

            npc_comp.is_enabled = !char_comp.is_controlled;
            if npc_comp.is_enabled {
                char_comp.facing_right = npc_comp.facing_right;
                char_comp.going_left = npc_comp.going_left;
                char_comp.going_right = npc_comp.going_right;
                char_comp.going_up = npc_comp.going_up;
                char_comp.going_down = npc_comp.going_down;
            }
            
            //char_comp.meowing = npc_comp.fire_pressed;
        }        

        drop(list);


        let mut player_1_char_num = -1;
        {
            player_1_char_num = game_state.player_1_char_num;
            
        }

        // get vec of character components
        let mut list = (&mut coll, &mut char_display, &ent).join().collect::<Vec<_>>();

        let mut char_x : f32 = 0.0;
        let mut char_y : f32 = 0.0;

        let mut found_player_target = false;

        // Pass vec of player input components to handler
        // handle each input applicable entity
        for inn_v in list.iter_mut() { 
            let char_player_num = (*inn_v.1).player_number;

            self.handle_player_input(inn_v, &*input, &ent, &lazy, time_delta, &level_type);
            
            if (*inn_v.1).is_controllable {
                if player_1_char_num == char_player_num {

                    //let (vel, coll, _display, _e) = inn_v;  
                    found_player_target = true;    
    
                    char_x = (*inn_v.0).pos.x;
                    char_y = (*inn_v.0).pos.y;
    
                    game_state.player_target_loc.0 = char_x;
                    game_state.player_target_loc.1 = char_y;

                    // Update camera to follow this character
                    //camera.following = Some((inn_v.2).clone());
    
                    (*inn_v.1).is_controlled = true;
                }
                else {
                    (*inn_v.1).is_controlled = false;
                }
            } 
        }        

        if !found_player_target {
            println!("[Cursor player target] Disp Offset: [{},{}] Mouse Pos: [{},{}] Window Size [{},{}]",
                &game_state.display_offset.0, &game_state.display_offset.1,
                &input.mouse_x, &input.mouse_y, &game_state.window_w, &game_state.window_h);
            game_state.player_target_loc.0 = -game_state.display_offset.0 + (input.mouse_x - game_state.window_w * 0.5); // * game_state.display_scale.0;
            game_state.player_target_loc.1 = -game_state.display_offset.1 + (input.mouse_y - game_state.window_h * 0.5);

            println!(" Results: Target Location: [{},{}]",
                &game_state.player_target_loc.0, &game_state.player_target_loc.1);
        }

        drop(list);

        // Can't incorporate physics world into specs system so far
        // if let Some(ref mut phys_world) = self.phys_world {
        //     for (button, coll, _ent) in (&mut buttons, &mut coll, &ent).join() {
        //         // update button components
        //         button.update(time_delta, coll, phys_world);
        //     }
        // }

    }
}
