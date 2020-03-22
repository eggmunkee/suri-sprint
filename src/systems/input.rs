
use specs::prelude::*;

use crate::resources::{InputResource,WorldAction};
use crate::components::*;
use crate::components::player::*;

// handle input state to control Players
// every frame, operate on velocity of player components
//  based on InputResource
pub struct InputSystem;
impl InputSystem {
    pub fn new() -> InputSystem {
        InputSystem
    }

    fn handle_player_list(mut v: Vec<(&mut Velocity, &PlayerComponent, Option<&mut CharacterDisplayComponent>, Entity)>, input: &InputResource) {

        // handle each input applicable entity
        for inn_v in v.iter_mut() { 
            let (vel, _player, _display, _e) = inn_v;      
            Self::handle_player_input(inn_v, input);
        }
    }

    // handle input updates from an entity
    fn handle_player_input(v: &mut (&mut Velocity, &PlayerComponent, Option<&mut CharacterDisplayComponent>, Entity), input: &InputResource) {
        let (vel, _player, _display, _e) = v;

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

        if let Some(display) = _display {
            display.going_up = up_pressed;
            display.going_left = left_pressed;
            display.going_right = right_pressed;
            display.going_down = down_pressed;

            display.update(vel, 0.15);

            up_pressed = display.going_up;
            left_pressed = display.going_left;
            right_pressed = display.going_right;
            down_pressed = display.going_down;
        }

        

    }
}
impl<'a> System<'a> for InputSystem {
    type SystemData = (WriteStorage<'a, Velocity>,
                        ReadStorage<'a, PlayerComponent>,
                        WriteStorage<'a, CharacterDisplayComponent>,
                        Read<'a, InputResource>,
                        Entities<'a>);

    fn run(&mut self, (mut vel, player, mut char_display, mut input, ent): Self::SystemData) {

        // tests collecting storage into vector
        let mut list = (&mut vel, &player, (&mut char_display).maybe(), &ent).join().collect::<Vec<_>>();

        if list.len() > 1 {
            println!("More than one player!");
        }
        else if list.len() == 0 {
            println!("No players found!");
        }

        Self::handle_player_list(list, &*input);

        // iterator over velocities with player components and input
        //for (vel, _player, _e) in list.iter_mut() {        
            //println!("Input proc for player {}", &player.player_name);    

            
        //}
    }
}

// handle ai sim input state to control Npcs
pub struct NpcInputSystem;