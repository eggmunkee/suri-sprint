
// Package includes
use ggez;
//use ggez::graphics;
use ggez::{Context, GameResult};
//use ggez::conf::{WindowMode};
use ggez::graphics::{Rect};
use ggez::event::{self, KeyCode, KeyMods, MouseButton};
use ggez::event::{GamepadId, Button, Axis};
use rand::prelude::*;

// Crate includes
use crate::core::game_state::{GameState,RunningState,DialogChoice};
//use crate::core::input::{InputMap,InputKey};
use crate::resources::{GameStateResource};
use crate::components::sprite::{SpriteLayer};
//use crate::entities::platform::{PlatformBuilder};


// Implementation of GGEZ EventHandler - handle or route to methods on Game State

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        //handle update event
        self.handle_update_event(ctx)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Handle render event
        self.handle_render_event(ctx)
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        let button_index = match button {
            MouseButton::Left => {
                Some(0usize)
            },
            MouseButton::Middle => {
                Some(1)
            },
            MouseButton::Right => {
                Some(2)
            }
            _ => None
        };
        if let Some(index) = button_index {
            self.input_map.mouse_set_pos(&mut self.world, ctx, x, y);
            self.input_map.mouse_button_down(&mut self.world, ctx, index.clone());
        }
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        let button_index = match button {
            MouseButton::Left => {
                Some(0usize)
            },
            MouseButton::Middle => {
                Some(1)
            },
            MouseButton::Right => {
                Some(2)
            }
            _ => None
        };
        if let Some(index) = button_index {
            self.input_map.mouse_set_pos(&mut self.world, ctx, x, y);
            self.input_map.mouse_button_up(&mut self.world, ctx, index.clone());
        }
        
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, _xrel: f32, _yrel: f32) {
        self.input_map.mouse_set_pos(&mut self.world, ctx, x, y);
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        println!("Mousewheel event, x: {}, y: {}", x, y);
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
        repeat: bool,
    ) {

        if repeat {
            if keycode == KeyCode::Subtract {
                if self.display_scale > 0.25 {
                    self.display_scale -= 0.05;
                }            
            }
            else if keycode == KeyCode::Equals {
                if self.display_scale < 4.75 {
                    self.display_scale += 0.05;
                }            
            }
            //
            else if keycode == KeyCode::RBracket {
                //let new_level = (self.audio.base_music_volume + 0.05).round().min(2.0);
                self.audio.incr_music_volume(); //  set_music_volume(new_level);
            }
            else if keycode == KeyCode::LBracket {
                //let new_level = (self.audio.base_music_volume - 0.05).max(0.0);
                self.audio.decr_music_volume(); //  set_music_volume(new_level);
                //self.audio.set_music_volume(new_level);
            }            
    
        }



        let key = self.input_map.key_down(&mut self.world, ctx, keycode, keymod);
        if let Some(i_key) = key {
            self.game_key_down(ctx, &i_key);
        }
    }

    fn key_up_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
    ) {

        // if keycode == KeyCode::P {
        //     match self.current_state {
        //         State::Paused => {
        //             self.play();
        //         },
        //         State::Running => {
        //             match self.running_state {
        //                 RunningState::Playing => {
        //                     self.pause();
        //                 },
        //                 _ => {} // don't pause on dialogs
        //             }
        //         }
        //     }
        // }
        if keycode == KeyCode::J {
            // Get world action if any
            //println!("Processing AddCircle action");
            let mut rng = rand::thread_rng();

            let test : u16 = rng.gen::<u16>();
            if test % 5 == 0 {
                let w = 50.0 + 0.001 * test as f32;
                let h = 10.0 + 0.00025 * test as f32;
                crate::entities::platform::PlatformBuilder::build_dynamic(&mut self.world, ctx, &mut self.phys_world, 200.0, 100.0,
                    w, h, 0.0, SpriteLayer::Entities.to_z());
            }
            else if test % 4 == 0 {
                let w = 10.0 + 0.001 * test as f32;
                crate::entities::empty_box::BoxBuilder::build_dynamic(&mut self.world, ctx, &mut self.phys_world, 200.0, 100.0,
                    w, w, rng.gen::<f32>() * 2.0 * 3.14159, SpriteLayer::Entities.to_z());
            }
            else {
                crate::entities::ghost::GhostBuilder::build_collider(&mut self.world, ctx, &mut self.phys_world, 100.0, 400.0, 0.0, 0.0,
                    30.0, 0.15, 25.0, 25.0);
            }
        }
        else if keycode == KeyCode::Subtract {
            if self.display_scale > 0.0125 {
                self.display_scale -= 0.05;
            }            
        }
        else if keycode == KeyCode::Equals {
            if self.display_scale < 4.75 {
                self.display_scale += 0.05;
            }            
        }
        // toggle edit mode - showing original level layout
        // else if keycode == KeyCode::F1 {
        //     if self.mode == GameMode::Play {
        //         self.mode = GameMode::Edit;
        //     }
        //     else {
        //         self.mode = GameMode::Play;
        //     }
        // }
        else if keycode == KeyCode::F11 {
            // let mut game_state_writer = self.world.fetch_mut::<GameStateResource>();

            // let mut new_fs_type : ggez::conf::FullscreenType = ggez::conf::FullscreenType::Windowed;
            // match game_state_writer.window_mode.fullscreen_type {
            //     ggez::conf::FullscreenType::Windowed => {
            //         new_fs_type = ggez::conf::FullscreenType::Desktop;
            //     },
            //     ggez::conf::FullscreenType::Desktop => {
            //         new_fs_type = ggez::conf::FullscreenType::True;
            //     },
            //     ggez::conf::FullscreenType::True => {
            //         new_fs_type = ggez::conf::FullscreenType::Windowed;
            //     }
            // }
            // game_state_writer.window_mode.fullscreen_type = new_fs_type;

            // ggez::graphics::set_fullscreen(ctx, new_fs_type);
        }
        // reload current level
        // else if keycode == KeyCode::R {
        //     //self.load_level(ctx, self.current_level_name.clone(), self.current_entry_name.clone());
        //     self.restart_level(ctx);

        // }        
        //
        if keycode == KeyCode::RBracket {
            // let new_level = (self.audio.base_music_volume + 0.05).min(2.0);
            // self.audio.set_music_volume(new_level);
            self.audio.incr_music_volume();
        }
        else if keycode == KeyCode::LBracket {
            // let new_level = (self.audio.base_music_volume - 0.05).max(0.0);
            // self.audio.set_music_volume(new_level);
            self.audio.decr_music_volume();
        }
        else if keycode == KeyCode::K {
                
            self.set_running_state(ctx, RunningState::world_dialog("K dialog".to_string(),
                Some(vec![
                    DialogChoice { key: "a".to_string(), message: "Choice A".to_string() },
                    DialogChoice { key: "b".to_string(), message: "Choice B".to_string() },
                    DialogChoice { key: "c".to_string(), message: "Choice C".to_string() },
                ]), "/images/dirty-box-1.png".to_string()));
        }
        else if keycode == KeyCode::M {
            self.open_menu();
        }
        else if keycode == KeyCode::L {
            println!("DEBUG LOGIC 3x -------------------------------------------------");
            self.debug_logic_frames = 3;
        }
        else if keycode == KeyCode::E {
            let mut game_state_writer = self.world.fetch_mut::<GameStateResource>();
            if game_state_writer.player_count > 0 {
                game_state_writer.player_1_char_num = (game_state_writer.player_1_char_num % game_state_writer.player_count) + 1;
                println!("New player number: {} / {}", &game_state_writer.player_1_char_num, &game_state_writer.player_count);
            }            
        }
        

        let key = self.input_map.key_up(&mut self.world, ctx, keycode, keymod);
        if let Some(i_key) = key {
            self.game_key_up(ctx, &i_key);
        }
        
    }

    fn gamepad_button_down_event(
        &mut self,
        ctx: &mut Context,
        btn: Button,
        id: GamepadId
    ) {
        //println!("gamepad_button_down: {:?}", &_btn);
        let key = self.input_map.gamepad_button_down(&mut self.world, ctx, btn, id);
        if let Some(i_key) = key {
            self.game_key_down(ctx, &i_key);
        }
    }

    fn gamepad_button_up_event(
        &mut self,
        ctx: &mut Context,
        btn: Button,
        id: GamepadId
    ) {
        //println!("gamepad_button_up: {:?}", &_btn);
        let key = self.input_map.gamepad_button_up(&mut self.world, ctx, btn, id);
        if let Some(i_key) = key {
            self.game_key_up(ctx, &i_key);
        }
    }

    fn gamepad_axis_event(
        &mut self,
        _ctx: &mut Context,
        _axis: Axis,
        _value: f32,
        _id: GamepadId
    ) {
        println!("gamepad_axis: {:?} {}", &_axis, &_value);

    }

    fn text_input_event(&mut self, _ctx: &mut Context, ch: char) {
        //println!("Text input: {}", ch);
        if self.terminal_open {
            self.input_map.text_typed(&mut self.world, ch);
        }
        
    }

    fn focus_event(&mut self, _ctx: &mut Context, gained: bool) {
        if gained {
            //println!("Focus gained");
        } else {
            //println!("Focus lost");
        }
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        //println!("Resized: {}, {}", &width, &height);

        // set game state w/h
        let mut game_state_writer = self.world.fetch_mut::<GameStateResource>();

        self.window_w = width;
        self.window_h = height;

        game_state_writer.window_w = width;
        game_state_writer.window_h = height;

        let mut mode = game_state_writer.window_mode;

        mode.width = width;
        mode.height = height;
        //println!("New window mode {:?}", &mode);

        ggez::graphics::set_screen_coordinates(ctx, Rect::new(0.0, 0.0, width, height)).expect("Failed to set coords on resize");

        drop(game_state_writer);
        
    }
}