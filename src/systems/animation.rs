

use ggez::nalgebra as na;
use ggez::{Context};
use specs::prelude::*;
use rand::prelude::*;

use crate::core::{GameState,RunningState};
use crate::resources::{GameStateResource};
use crate::components::*;
use crate::components::anim_sprite::{AnimSpriteComponent};

pub struct AnimationSystem {
    
}

impl AnimationSystem {
    pub fn new() -> Self {
        AnimationSystem {
        }
    }
}


impl<'a> System<'a> for AnimationSystem {
    type SystemData = (WriteStorage<'a, AnimSpriteComponent>,
                        Write<'a, GameStateResource>,
                        Entities<'a>,
                        Read<'a, LazyUpdate>);

    fn run(&mut self, (mut anim_sprites, mut game_state, mut entities, lazy): Self::SystemData) {
        use specs::Join;

        let time_delta = game_state.delta_seconds;

        for (mut anim_sprite, ent) in (&mut anim_sprites, &entities).join() {
            anim_sprite.update(time_delta);

        }
    }
}