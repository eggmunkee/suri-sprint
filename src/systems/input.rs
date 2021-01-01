use ggez::nalgebra as na;
use ggez::{Context};
use specs::prelude::*;
use wrapped2d::b2;
use rand::prelude::*;

use crate::game_state::{GameState,RunningState};
use crate::resources::{InputResource,WorldAction,GameStateResource};
use crate::components::*;
use crate::components::collision::{Collision};
use crate::components::player::*;
use crate::components::npc::{NpcComponent};
use crate::physics::{CollisionCategory};

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
        ent: &Entities, lazy: &Read<'a, LazyUpdate>, time_delta: f32) {
        let (coll, npc, _e) = v;

        let x = coll.pos.x;
        let y = coll.pos.y;
        let body_movement = coll.get_movement();
        // Call npc update handler with current status
        npc.update(body_movement, time_delta, x, y);
        
    }

    // handle input updates from an entity
    fn handle_player_input<'a>(&mut self, v: &mut (&mut Collision, &mut CharacterDisplayComponent, Entity), input: &InputResource,
        ent: &Entities, lazy: &Read<'a, LazyUpdate>, time_delta: f32) {
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
       

        character.update(body_movement, time_delta);

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
    pub fn handle_dialog_input(input: &InputResource, game_state: &GameState, time_delta: f32) -> RunningState {

        let mut up_pressed = false;
        let mut left_pressed = false;
        let mut right_pressed = false;
        let mut down_pressed = false;
        let mut fire_pressed = false;

        // apply input status to player
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
        if input.fire_pressed {
            fire_pressed = true;

            RunningState::Playing

        }
        else {
            game_state.running_state.clone()
        }

    }
}


impl<'a> System<'a> for InputSystem {
    type SystemData = (WriteStorage<'a, Collision>,
                        WriteStorage<'a, CharacterDisplayComponent>,
                        WriteStorage<'a, NpcComponent>,
                        Write<'a, GameStateResource>,
                        Read<'a, InputResource>,
                        Entities<'a>,
                        Read<'a, LazyUpdate>);

    fn run(&mut self, (mut coll, mut char_display, mut npc, mut game_state, mut input, mut ent, lazy): Self::SystemData) {

        let time_delta = game_state.delta_seconds;

        let mut rng = rand::thread_rng();

        // Get vec of npc input components to handle       
        let mut list = (&mut coll, &mut npc, &ent).join().collect::<Vec<_>>();
        for inn_v in list.iter_mut() { 

            let mut target_x = game_state.player_target_loc.0;
            let mut target_y = game_state.player_target_loc.1;

            //let dec = rng.gen::<f32>();
            let new_target_x = (rng.gen::<f32>() * 800.0) - 400.0;
            let new_target_y = (rng.gen::<f32>() * 800.0) - 400.0;

            if new_target_x < target_x {
                (*inn_v.1).target_position.0 = (target_x + 10.0).min(target_x);
            }
            else if new_target_x > target_x {
                (*inn_v.1).target_position.0 = (target_x - 10.0).max(target_x);
            }
            if new_target_y < target_y {
                (*inn_v.1).target_position.1 = (target_y - 10.0).min(target_y);
            }
            else if new_target_y > target_y {
                (*inn_v.1).target_position.1 = (target_y + 10.0).max(target_y);
            }
            
            

            // call npc input step
            self.handle_npc_input(inn_v, &*input, &ent, &lazy, time_delta);
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

        // Pass vec of player input components to handler
        // handle each input applicable entity
        for inn_v in list.iter_mut() { 
            let char_player_num = (*inn_v.1).player_number;

            self.handle_player_input(inn_v, &*input, &ent, &lazy, time_delta);
            
            if player_1_char_num == char_player_num {

                //let (vel, coll, _display, _e) = inn_v;      

                char_x = (*inn_v.0).pos.x;
                char_y = (*inn_v.0).pos.y;

                game_state.player_target_loc.0 = char_x;
                game_state.player_target_loc.1 = char_y;

                (*inn_v.1).is_controlled = true;
            }
            else {
                (*inn_v.1).is_controlled = false;
            }
        }        

        drop(list);

       

    }
}
