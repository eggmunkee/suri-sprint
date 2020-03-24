// External crates
use core::time::{Duration};
use ggez::nalgebra as na;
use rand::prelude::*;
use specs::prelude::*;
use specs::{
    //Builder, DispatcherBuilder,
    Entities,
    ReadStorage, WriteStorage, System, //VecStorage, 
    Read,
};

// Internal refs
use crate::resources::{GameStateResource};
use crate::components::{Position,Velocity,GridLoc};
use crate::components::collision::{Collision};
use crate::components::player::{PlayerComponent};

// Input system - takes inputs and applies to player & game
pub mod input;
// InterActor system - handles all actor to actor interactions
//   including actor physics, damage, effects, etc.
pub mod interactor;

// Re-export the input and interactor system items into this module
pub use crate::systems::input::*;
pub use crate::systems::interactor::*;

/**** SYSTEMS *********************************/

// // UpdatePos system - can update the position of each entity by the velocity amount
// pub struct UpdatePos {
//     pub t_delta: Duration
// }

// impl UpdatePos {
//     fn box_pos(pos: &mut Position, vel: &mut Velocity, scr_w: f32, scr_h: f32) {
//         if vel.frozen { return; }
//         let padding = 50.0;
//         let (min_x, max_x, min_y, max_y) = (padding, scr_w - padding, padding, scr_h - padding);
//         let mut hit_wall = false;
//         if pos.x < min_x {
//             pos.x = min_x;
//             if vel.x < 0.0 { vel.x = 0.0; }
//             //vel.x *= 0.97;
//             hit_wall = true;
//         }
//         else if pos.x > max_x {
//             pos.x = max_x;
//             if vel.x > 0.0 { vel.x = 0.0; }
//             //vel.x *= 0.97;
//             hit_wall = true;
//         }
//         if pos.y < min_y {
//             pos.y = min_y;
//             if vel.y < 0.0 { vel.y = 0.0; }
//             //vel.y *= 0.97;
//             hit_wall = true;
//         }
//         else if pos.y > max_y {
//             pos.y = max_y;
//             if vel.y > 0.0 { vel.y = 0.0; }
//             //vel.y *= 0.97;
//             hit_wall = true;
//         }
//         if hit_wall {
//             vel.x *= 0.96;
//             vel.y *= 0.96;
//         }
//     }

//     #[allow(dead_code,unused_variables)]
//     pub fn get_grid_locs(pos: &mut Position) -> Vec<GridLoc> {
//         let gl_vec = Vec::<GridLoc>::new();



//         gl_vec
//     }
// }

// impl<'a> System<'a> for UpdatePos {
//     type SystemData = (WriteStorage<'a, Velocity>,
//                        WriteStorage<'a, Position>,
//                         Read<'a, GameStateResource>);

//     fn run(&mut self, (mut vel, mut pos, game_state): Self::SystemData) {
//         use specs::ParJoin;
        
//         let (scr_w, scr_h) = (game_state.window_w, game_state.window_h);

//         // Parallel join the velocity and position component data
//         (&mut vel, &mut pos).par_join()
//         // Run this method multi-threaded
//         .for_each(|(vel, pos)| {
//             // update positions by velocity vectors
//             pos.x += vel.x * 0.15;
//             pos.y += vel.y * 0.15;
//             // Constrain position to a box (hardcoded atm)
//             //  and reverse velocity when going out of bounds
//             self::UpdatePos::box_pos(pos, vel, scr_w, scr_h);
//             println!("Updated pos: {},{}", &pos.x, &pos.y);
//         });
//     }
// }


// // UpdatePos system - can update the position of each entity by the velocity amount
// pub struct GravSys;

// impl GravSys {
   
// }

// impl<'a> System<'a> for GravSys {
//     type SystemData = WriteStorage<'a, Velocity>;

//     fn run(&mut self, mut vel: Self::SystemData) {
//         use specs::Join;
//         for vel in (&mut vel).join() {
//             // update velocity if gravity is applied
//             if !vel.frozen && vel.gravity {
//                 vel.y += 9.8 * 0.15;
//             }
//         }
//     }
// }


// // SumSym - summarizes the pos & vel of all entities, which have both components
// pub struct SumSys;
// impl<'a> System<'a> for SumSys {
//     type SystemData = (ReadStorage<'a, Velocity>,
//                        ReadStorage<'a, Position>);

//     fn run(&mut self, (vel, pos): Self::SystemData) {
//         use specs::Join;
//         let mut i = 0;
//         for (vel, pos) in (&vel, &pos).join() {            
//             println!("Ent {}, Pos: {:?} Vel: {:?}", &i, &pos, &vel);
//             i += 1;
//         }
//     }
// }

// // handle game level commands
// #[allow(dead_code)]
// pub struct GameCommandSystem;
// // handle changes to the game state
// #[allow(dead_code)]
// pub struct GameStateSys;


// // handle updating actors based on velocity or controlled value
// #[allow(dead_code)]
// pub struct MoveActorsSys;


// // handle sorting the sprites for drawing
// #[allow(dead_code)]
// pub struct ZOrderSpriteSys;
