use ggez::nalgebra as na;
use ggez::{Context};
use specs::prelude::*;
use wrapped2d::b2;

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

        let body_movement = coll.get_movement();
        // Call npc update handler with current status
        npc.update(body_movement, time_delta);
        
    }

    // handle input updates from an entity
    fn handle_player_input<'a>(&mut self, v: &mut (&mut Collision, &mut CharacterDisplayComponent, Entity), input: &InputResource,
        ent: &Entities, lazy: &Read<'a, LazyUpdate>, time_delta: f32) {
        let (coll, character, _e) = v;

        let body_movement = coll.get_movement();

        let mut up_pressed = false;
        let mut left_pressed = false;
        let mut right_pressed = false;
        let mut down_pressed = false;

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

        //if let Some(display) = _display {
        if character.input_enabled {
            character.going_up = up_pressed;
            character.going_left = left_pressed;
            character.going_right = right_pressed;
            character.going_down = down_pressed;
            character.meowing = input.fire_pressed;
        }

        character.update(body_movement, time_delta);

        if character.meowing {
            let mut x = coll.pos.x;
            let mut y = coll.pos.y;
            let mut vx = coll.vel.x;
            let mut vy = 0.0f32;

            let meow_spd = 4.0f32;

            if right_pressed {
                vx = vx.max(0.0) + meow_spd; //.max(vx * 1.1);
                x += 20.0;
            }
            else if left_pressed {
                vx = vx.min(0.0) - meow_spd; //.min(vx * 1.1);
                x -= 20.0;
            }
            else if character.facing_right {
                vx = meow_spd;
                x += 20.0;
            }
            else {
                vx = -meow_spd;
                x -= 20.0;
            }
            if up_pressed {
                vy = -8.0;
            }
            else if down_pressed {
                vy = 8.0;
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
                        Read<'a, GameStateResource>,
                        Read<'a, InputResource>,
                        Entities<'a>,
                        Read<'a, LazyUpdate>);

    fn run(&mut self, (mut coll, mut char_display, mut npc, game_state, mut input, mut ent, lazy): Self::SystemData) {

        let time_delta = game_state.delta_seconds;

        // Get vec of npc input components to handle       
        let mut list = (&mut coll, &mut npc, &ent).join().collect::<Vec<_>>();
        for inn_v in list.iter_mut() { 
            // call npc input step
            self.handle_npc_input(inn_v, &*input, &ent, &lazy, time_delta);
        }  
        drop(list);

        let mut list = (&npc, &mut char_display).join().collect::<Vec::<_>>();
        for inn_v in list.iter_mut() { 
            let (npc_comp, char_comp) = inn_v;      
            char_comp.facing_right = npc_comp.facing_right;
            char_comp.going_left = npc_comp.going_left;
            char_comp.going_right = npc_comp.going_right;
            char_comp.going_up = npc_comp.going_up;
            char_comp.going_down = npc_comp.going_down;
            //char_comp.meowing = npc_comp.fire_pressed;
        }        

        drop(list);

        // get vec of character components
        let mut list = (&mut coll, &mut char_display, &ent).join().collect::<Vec<_>>();

        // Pass vec of player input components to handler
        // handle each input applicable entity
        for inn_v in list.iter_mut() { 
            //let (vel, coll, _display, _e) = inn_v;      
            self.handle_player_input(inn_v, &*input, &ent, &lazy, time_delta);
        }        

        drop(list);
        

    }
}
