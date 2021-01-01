

use ggez::nalgebra as na;
use ggez::{Context};
use specs::prelude::*;
use rand::prelude::*;

use crate::game_state::{GameState,RunningState};
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
                        Write<'a, GameStateResource>,
                        Entities<'a>,
                        Read<'a, LazyUpdate>);

    fn run(&mut self, (mut particle_systems, mut game_state, mut entities, lazy): Self::SystemData) {
        use specs::Join;

        let time_delta = game_state.delta_seconds;

        for (mut particle_sys, ent) in (&mut particle_systems, &entities).join() {
            particle_sys.update(time_delta);

        }
    }
}