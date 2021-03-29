
use ggez::{Context};
use ggez::nalgebra as na;
use specs::{ Component, DenseVecStorage, VecStorage, NullStorage, World, WorldExt, Entity };

use specs_derive::*;


use crate::resources::{GameStateResource};
use crate::core::game_state::{GameState};
use crate::core::physics::{PhysicsWorld};
use crate::components::collision::{Collision};
use crate::components::player::{CharacterDisplayComponent};
use crate::components::npc::{NpcComponent};
use crate::components::sprite::{SpriteComponent,MultiSpriteComponent,ParallaxSpriteComponent};
use crate::components::anim_sprite::{AnimSpriteComponent};
use crate::components::particle_sys::{ParticleSysComponent};

pub trait RenderItemTarget {
    fn render_item(game_state: &GameState, ctx: &mut Context, entity: &Entity,
        pos: &na::Point2<f32>, item_index: usize);
}

// Defines the types to call on to render
#[derive(Debug,Clone,PartialEq)]
pub enum RenderLayerType {
    WindowBackground,
    LevelLayer,
    DialogLayer,
    PausedLayer,
    UILayer
}

// Defines the types to call on to render
#[derive(Debug,Clone,PartialEq)]
pub enum RenderFlagType {
    Sprite,
    MultiSprite,
    AnimSprite,
    ParallaxSprite,
    Character,
    ParticleSys,
    Unimplemented
}

impl Default for RenderFlagType {
    fn default() -> Self {
        RenderFlagType::Unimplemented
    }
}

#[derive(Default,Component)]
#[storage(DenseVecStorage)]
pub struct RenderFlag {
    pub layers: Vec::<RenderLayerType>
}

impl RenderFlag {
    pub fn in_layer(&self, layer_type: RenderLayerType) -> bool {
        for self_layer_type in self.layers.iter() {
            if &layer_type == self_layer_type {
                return true;
            }
        }
        return false;
    }

    pub fn from_layer(layer_type: RenderLayerType) -> Self {
        Self {
            layers: vec![layer_type],
        }
    }

    pub fn from_layers(layer_types: &Vec::<RenderLayerType>) -> Self {
        let mut layers_copy = Vec::<RenderLayerType>::new();
        for layer_type in layer_types.iter() {
            layers_copy.push(layer_type.clone());
        }

        Self {
            layers: layers_copy,
        }
    }
}

#[derive(Debug)]
pub struct RenderCallInfo {
    pub entity: Entity,
    pub pos: na::Point2<f32>,
    pub item_index: usize,
    pub z_order: f32,
    pub render_type: RenderFlagType
}

impl RenderCallInfo {
    // Call render_item on any component class w/ RenderItemTarget implementation
    // Classes passed in as T type will be checked for RenderItemTarget trait at compile
    pub fn call_render_item<T>(&self, game_state: &GameState, ctx: &mut Context)
        where T: Component + RenderItemTarget {
        T::render_item(game_state, ctx, &self.entity, &self.pos, self.item_index);        
    }

    pub fn render_item(&self, game_state: &GameState, ctx: &mut Context) {
        //println!("Found RenderCallInfo {:?}", self);
        match &self.render_type {
            RenderFlagType::Sprite => {
                //SpriteComponent::render_item(game_state, ctx, &self.entity, &self.pos, self.item_index);
                self.call_render_item::<SpriteComponent>(game_state, ctx);
            },
            RenderFlagType::MultiSprite => {
                //MultiSpriteComponent::render_item(game_state, ctx, &self.entity, &self.pos, self.item_index);
                self.call_render_item::<MultiSpriteComponent>(game_state, ctx);
            },
            RenderFlagType::AnimSprite => {
                //AnimSpriteComponent::render_item(game_state, ctx, &self.entity, &self.pos, self.item_index);
                self.call_render_item::<AnimSpriteComponent>(game_state, ctx);
            },
            RenderFlagType::ParallaxSprite => {
                self.call_render_item::<ParallaxSpriteComponent>(game_state, ctx);
            },
            RenderFlagType::Character => {
                //println!("Found Character RenderCallInfo");
                //CharacterDisplayComponent::render_item(game_state, ctx, &self.entity, &self.pos, self.item_index);
                self.call_render_item::<CharacterDisplayComponent>(game_state, ctx);
            },
            RenderFlagType::ParticleSys => {
                //ParticleSysComponent::render_item(game_state, ctx, &self.entity, &self.pos, self.item_index);
                self.call_render_item::<ParticleSysComponent>(game_state, ctx);
            },
            
            _ => {}
        }
    }

}

#[derive(Default,Component)]
#[storage(NullStorage)]
pub struct RenderSpriteFlag;
impl RenderSpriteFlag {
    pub fn render_item(game_state: &GameState, ctx: &mut Context, entity: &Entity,
        pos: &na::Point2<f32>, item_index: usize) {
        SpriteComponent::render_item(game_state, ctx, entity, pos, item_index);
    }
}

#[derive(Default,Component)]
#[storage(NullStorage)]
pub struct RenderMultiSpriteFlag;
impl RenderMultiSpriteFlag {
    pub fn render_item(game_state: &GameState, ctx: &mut Context, entity: &Entity,
        pos: &na::Point2<f32>, item_index: usize) {
        MultiSpriteComponent::render_item(game_state, ctx, entity, pos, item_index);
    }
}

#[derive(Default,Component)]
#[storage(NullStorage)]
pub struct RenderAnimSpriteFlag;
impl RenderAnimSpriteFlag {
    pub fn render_item(game_state: &GameState, ctx: &mut Context, entity: &Entity,
        pos: &na::Point2<f32>, item_index: usize) {
        AnimSpriteComponent::render_item(game_state, ctx, entity, pos, item_index);
    }
}

#[derive(Default,Component)]
#[storage(NullStorage)]
pub struct RenderCharacterFlag;
impl RenderCharacterFlag {
    pub fn render_item(game_state: &GameState, ctx: &mut Context, entity: &Entity,
        pos: &na::Point2<f32>, item_index: usize) {
        CharacterDisplayComponent::render_item(game_state, ctx, entity, pos, item_index);
    }
}

#[derive(Default,Component)]
#[storage(NullStorage)]
pub struct RenderParticleSysFlag;
impl RenderParticleSysFlag {
    pub fn render_item(game_state: &GameState, ctx: &mut Context, entity: &Entity,
        pos: &na::Point2<f32>, item_index: usize) {
        ParticleSysComponent::render_item(game_state, ctx, entity, pos, item_index);
    }
}


// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    world.register::<RenderFlag>();
    world.register::<RenderSpriteFlag>();
    world.register::<RenderMultiSpriteFlag>();
    world.register::<RenderAnimSpriteFlag>();
    world.register::<RenderCharacterFlag>();
    world.register::<RenderParticleSysFlag>();
}
