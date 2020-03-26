use ggez::nalgebra as na;
use ggez::{Context};
use specs::prelude::*;

use crate::resources::{InputResource,WorldAction,GameStateResource};
use crate::components::*;
use crate::components::collision::{Collision};
use crate::components::ball::{BallDisplayComponent};
use crate::components::player::*;
use crate::physics::{CollisionCategory};

// handle input state to control Players
// every frame, operate on velocity of player components
//  based on InputResource
pub struct InputSystem {
    pub meows: Vec::<na::Point2<f32>>,
}
impl InputSystem {
    pub fn new() -> InputSystem {
        InputSystem {
            meows: vec![],
        }
    }

    fn handle_player_list<'a>(&mut self, mut v: Vec<(&mut Velocity, &mut Collision, &mut CharacterDisplayComponent, Entity)>, input: &InputResource,
        ent: &Entities, lazy: &Read<'a, LazyUpdate>, time_delta: f32) {

        // handle each input applicable entity
        for inn_v in v.iter_mut() { 
            //let (vel, coll, _display, _e) = inn_v;      
            self.handle_player_input(inn_v, input, ent, lazy, time_delta);
        }
    }

    // handle input updates from an entity
    fn handle_player_input<'a>(&mut self, v: &mut (&mut Velocity, &mut Collision, &mut CharacterDisplayComponent, Entity), input: &InputResource,
        ent: &Entities, lazy: &Read<'a, LazyUpdate>, time_delta: f32) {
        let (vel, coll, display, _e) = v;

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
        display.going_up = up_pressed;
        display.going_left = left_pressed;
        display.going_right = right_pressed;
        display.going_down = down_pressed;
        display.meowing = input.fire_pressed;

        display.update(coll, time_delta);

        if display.meowing {
            let x = coll.pos.x;
            let y = coll.pos.y;

            self.meows.push(na::Point2::new(x, y));
            display.since_meow = 0.0;

        }

    }
}
impl<'a> System<'a> for InputSystem {
    type SystemData = (WriteStorage<'a, Velocity>,
                        WriteStorage<'a, Collision>,
                        WriteStorage<'a, CharacterDisplayComponent>,
                        Read<'a, GameStateResource>,
                        Read<'a, InputResource>,
                        Entities<'a>,
                        Read<'a, LazyUpdate>);

    fn run(&mut self, (mut vel, mut coll, mut char_display, game_state, mut input, mut ent, lazy): Self::SystemData) {

        let time_delta = game_state.delta_seconds;

        // tests collecting storage into vector
        let mut list = (&mut vel, &mut coll, &mut char_display, &ent).join().collect::<Vec<_>>();

        if list.len() > 1 {
            println!("More than one player!");
        }
        else if list.len() == 0 {
            println!("No players found!");
        }

        //let new_ent = ent.create();

        self.handle_player_list(list, &*input, &ent, &lazy, time_delta);

        // iterator over velocities with player components and input
        //for (vel, _player, _e) in list.iter_mut() {        
            //println!("Input proc for player {}", &player.player_name);    

            
        //}
    }
}

// handle ai sim input state to control Npcs
pub struct NpcInputSystem;