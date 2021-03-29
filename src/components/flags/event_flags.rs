
use ggez::{Context};
use ggez::nalgebra as na;
use specs::{ Component, DenseVecStorage, VecStorage, NullStorage, World, WorldExt, Entity };

use specs_derive::*;


use crate::resources::{GameStateResource};
use crate::core::game_state::{GameState};
use crate::core::physics::{PhysicsWorld};
use crate::components::{RenderTrait};
use crate::components::collision::{Collision};
use crate::components::player::{CharacterDisplayComponent};
use crate::components::npc::{NpcComponent};
use crate::components::sprite::{SpriteComponent,MultiSpriteComponent};
use crate::components::anim_sprite::{AnimSpriteComponent};
use crate::components::particle_sys::{ParticleSysComponent};

// Defines the types to call on to render
#[derive(Debug,Clone,PartialEq)]
pub enum EventFlagType {
    Sprite,
    MultiSprite,
    AnimSprite,
    Character,
    ParticleSys,
    Unimplemented
}

#[derive(Default,Component)]
#[storage(DenseVecStorage)]
pub struct EventsFlag {    
    pub layers: Vec::<EventFlagType>
}

impl EventsFlag {
    pub fn in_layer(&self, layer_type: EventFlagType) -> bool {
        for self_layer_type in self.layers.iter() {
            if &layer_type == self_layer_type {
                return true;
            }
        }
        return false;
    }

    pub fn from_layer(layer_type: EventFlagType) -> Self {
        Self {
            layers: vec![layer_type],
        }
    }

    pub fn from_layers(layer_types: &Vec::<EventFlagType>) -> Self {
        let mut layers_copy = Vec::<EventFlagType>::new();
        for layer_type in layer_types.iter() {
            layers_copy.push(layer_type.clone());
        }

        Self {
            layers: layers_copy,
        }
    }

    pub fn update_item(&self, game_state: &GameState, ctx: &mut Context, event_type: EventFlagType) {
        //println!("Found RenderCallInfo {:?}", self);
        match &event_type {
            /*EventFlagType::Sprite => {
                //SpriteComponent::render_item(game_state, ctx, &self.entity, &self.pos, self.item_index);
            },
            EventFlagType::MultiSprite => {
                //MultiSpriteComponent::render_item(game_state, ctx, &self.entity, &self.pos, self.item_index);
            },
            EventFlagType::AnimSprite => {
                //AnimSpriteComponent::render_item(game_state, ctx, &self.entity, &self.pos, self.item_index);
            },
            EventFlagType::Character => {
                //println!("Found Character RenderCallInfo");
                //CharacterDisplayComponent::render_item(game_state, ctx, &self.entity, &self.pos, self.item_index);
            },
            EventFlagType::ParticleSys => {
                //ParticleSysComponent::render_item(game_state, ctx, &self.entity, &self.pos, self.item_index);
            },*/            
            _ => {}
        }
    }
}

#[derive(Default,Component)]
#[storage(DenseVecStorage)]
pub struct PhysicsEventsFlag {
    
}

impl PhysicsEventsFlag {
    pub fn pre_physics_update(game_state: &GameState, entity: &Entity) {

    }
    pub fn post_physics_update(game_state: &GameState, entity: &Entity) {

    }
}




// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    world.register::<EventsFlag>();
    world.register::<PhysicsEventsFlag>();
    /*world.register::<RenderMultiSpriteFlag>();
    world.register::<RenderAnimSpriteFlag>();
    world.register::<RenderCharacterFlag>();
    world.register::<RenderParticleSysFlag>();*/
}
