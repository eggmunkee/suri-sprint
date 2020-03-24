use ggez::nalgebra as na;
use ggez::{Context};
use specs::prelude::*;

use crate::resources::{InputResource,WorldAction};
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

    fn handle_player_list<'a>(&mut self, mut v: Vec<(&mut Velocity, &mut Collision, &PlayerComponent, Option<&mut CharacterDisplayComponent>, Entity)>, input: &InputResource,
        ent: &Entities, lazy: &Read<'a, LazyUpdate>) {

        // handle each input applicable entity
        for inn_v in v.iter_mut() { 
            let (vel, coll, _player, _display, _e) = inn_v;      
            self.handle_player_input(inn_v, input, ent, lazy);
        }
    }

    // handle input updates from an entity
    fn handle_player_input<'a>(&mut self, v: &mut (&mut Velocity, &mut Collision, &PlayerComponent, Option<&mut CharacterDisplayComponent>, Entity), input: &InputResource,
        ent: &Entities, lazy: &Read<'a, LazyUpdate>) {
        let (vel, coll, _player, _display, _e) = v;

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

        if input.fire_pressed {
            //lazy.create_entity(ent: &EntitiesRes);
            let x = coll.pos.x;
            let y = coll.pos.y;

            self.meows.push(na::Point2::new(x, y));

            // let mut collision = Collision::new_specs(0.1,0.72, 30.0, 30.0);
            // // collision.dim_1 = width;
            // // collision.dim_2 = height;
            // collision.pos.x = x;
            // collision.pos.y = y;
            // collision.collision_category = CollisionCategory::Meow;
            // collision.collision_mask.clear();
            // collision.collision_mask.push(CollisionCategory::Ghost);
    
            // collision.create_dynamic_body_circle(physics_world);
    
            // let entity = lazy.create_entity(ent)
            // .with(Position { x: x, y: y })
            // .with(DisplayComp { circle: false, display_type: DisplayCompType::DrawSelf })
            // .with(BallDisplayComponent::new(ctx, &"/dirty-box-1.png".to_string(), false))
            // //.with(collision)
            // .build();

        }

        if let Some(display) = _display {
            display.going_up = up_pressed;
            display.going_left = left_pressed;
            display.going_right = right_pressed;
            display.going_down = down_pressed;

            display.update(coll, 0.15);

            up_pressed = display.going_up;
            left_pressed = display.going_left;
            right_pressed = display.going_right;
            down_pressed = display.going_down;
        }

        

    }
}
impl<'a> System<'a> for InputSystem {
    type SystemData = (WriteStorage<'a, Velocity>,
                        WriteStorage<'a, Collision>,
                        ReadStorage<'a, PlayerComponent>,
                        WriteStorage<'a, CharacterDisplayComponent>,
                        Read<'a, InputResource>,
                        Entities<'a>,
                        Read<'a, LazyUpdate>);

    fn run(&mut self, (mut vel, mut coll, player, mut char_display, mut input, mut ent, lazy): Self::SystemData) {

        // tests collecting storage into vector
        let mut list = (&mut vel, &mut coll, &player, (&mut char_display).maybe(), &ent).join().collect::<Vec<_>>();

        if list.len() > 1 {
            println!("More than one player!");
        }
        else if list.len() == 0 {
            println!("No players found!");
        }

        //let new_ent = ent.create();

        self.handle_player_list(list, &*input, &ent, &lazy);

        // iterator over velocities with player components and input
        //for (vel, _player, _e) in list.iter_mut() {        
            //println!("Input proc for player {}", &player.player_name);    

            
        //}
    }
}

// handle ai sim input state to control Npcs
pub struct NpcInputSystem;