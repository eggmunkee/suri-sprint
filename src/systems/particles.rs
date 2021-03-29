

use ggez::nalgebra as na;
use ggez::{Context};
use specs::prelude::*;
use rand::prelude::*;

use crate::core::{GameState,RunningState};
use crate::resources::{GameStateResource};
use crate::components::*;
use crate::components::particle_sys::{ParticleSysComponent};

pub struct ParticleSystem {
    
}

impl ParticleSystem {
    pub fn new() -> Self {
        ParticleSystem {
        }
    }
}


impl<'a> System<'a> for ParticleSystem {
    type SystemData = (WriteStorage<'a, ParticleSysComponent>,
                        ReadStorage<'a, Position>,
                        ReadStorage<'a, Velocity>,
                        Write<'a, GameStateResource>,
                        Entities<'a>,
                        Read<'a, LazyUpdate>);

    fn run(&mut self, (mut particle_systems, position, velocity, mut game_state, mut entities, lazy): Self::SystemData) {
        use specs::Join;

        let time_delta = game_state.delta_seconds;

        for (mut particle_sys, pos, opt_vel, ent) in (&mut particle_systems, &position, (&velocity).maybe(), &entities).join() {

            // Update system velocity
            if let Some(vel) = opt_vel {
                if particle_sys.world_positions {
                    println!("ParticleSys has Velocity {:?}", &vel);
                    particle_sys.world_vel.0 = vel.x;
                    particle_sys.world_vel.1 = vel.y;
                }
            }

            // Update system position from pos
            if particle_sys.world_positions {
                particle_sys.world_offset.0 = pos.x;
                particle_sys.world_offset.1 = pos.y;
            }
            particle_sys.update(time_delta, false);

        }
    }
}