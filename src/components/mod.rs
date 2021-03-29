use ggez::{Context};
use ggez::nalgebra as na;
use specs::{ Component, DenseVecStorage, VecStorage, NullStorage, World, WorldExt, Entity };

use specs_derive::*;

pub mod flags;
//pub mod updates;
pub mod logic;
pub mod sprite;
pub mod button;
pub mod player;
pub mod collision;
pub mod meow;
pub mod npc;
pub mod portal;
pub mod exit;
pub mod anim_sprite;
pub mod particle_sys;
pub mod pickup;
pub mod sensor_area;
// DEFINE COMMON COMPONENTS

use crate::resources::{GameStateResource};
use crate::core::physics::{PhysicsWorld};
use crate::components::collision::{Collision};
use crate::components::player::{CharacterDisplayComponent};
use crate::components::npc::{NpcComponent};
// use crate::components::npc::{NpcComponent};
// use crate::components::npc::{NpcComponent};

pub use crate::components::flags::*;

#[derive(Debug)]
#[allow(dead_code)]
pub struct GridLoc {
    pub row: i32,
    pub col: i32,
}

#[derive(Debug,Copy,Clone,Component)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug,Copy,Clone,Component)]
#[storage(DenseVecStorage)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub gravity: bool,
    pub frozen: bool,
}

#[derive(Debug,Copy,Clone,Component)]
#[storage(DenseVecStorage)]
pub struct LevelSource {
    pub item_index: i32
}

pub trait WorldUpdateTrait {
    fn update(&mut self, delta_time: f32, collision: &mut Collision, physics_world: &mut PhysicsWorld);
}

pub trait PhysicsUpdateTrait {
    fn pre_physics_update(&mut self, world: &World,physics_world: &mut PhysicsWorld, time_delta: f32, 
        opt_collision: &mut Option<&mut Collision>,
        opt_character: &mut Option<&mut CharacterDisplayComponent>,
        opt_npc: &mut Option<&mut NpcComponent>,
        entity: &Entity);
    fn post_physics_update(&mut self, world: &World, physics_world: &mut PhysicsWorld, time_delta: f32, 
        opt_collision: &mut Option<&mut Collision>,
        opt_character: &mut Option<&mut CharacterDisplayComponent>,
        opt_npc: &mut Option<&mut NpcComponent>,
        entity: &Entity);
}

impl PhysicsUpdateTrait for Position {
    fn pre_physics_update(&mut self, world: &World, physics_world: &mut PhysicsWorld, time_delta: f32, 
        opt_collision: &mut Option<&mut Collision>,
        opt_character: &mut Option<&mut CharacterDisplayComponent>,
        opt_npc: &mut Option<&mut NpcComponent>,
        entity: &Entity) {

        }
    fn post_physics_update(&mut self,  world: &World, physics_world: &mut PhysicsWorld, time_delta: f32, 
        opt_collision: &mut Option<&mut Collision>,
        opt_character: &mut Option<&mut CharacterDisplayComponent>,
        opt_npc: &mut Option<&mut NpcComponent>,
        entity: &Entity) {
            if let Some(collision) = opt_collision {
                self.x = collision.pos.x;
                self.y = collision.pos.y;
            }
        }
}

pub trait CharLevelInteractor {
    fn set_standing(&mut self, is_standing: bool);
}

pub trait RenderTrait {    
    fn draw(&self, ctx: &mut Context, world: &World, ent: Option<u32>, pos: na::Point2::<f32>, item_index: usize);
}


impl RenderTrait for &dyn RenderTrait {
    fn draw(&self, _ctx: &mut Context, _world: &World, _ent: Option<u32>, _pos: na::Point2::<f32>, _item_index: usize) {

    }
}

// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<LevelSource>();
    
    // sub-module components
    self::flags::register_components(world);
    self::logic::register_components(world);
    self::sprite::register_components(world);
    self::collision::register_components(world);
    self::player::register_components(world);
    self::meow::register_components(world);
    self::npc::register_components(world);
    self::portal::register_components(world);
    self::button::register_components(world);
    self::exit::register_components(world);
    self::anim_sprite::register_components(world);
    self::particle_sys::register_components(world);
    self::pickup::register_components(world);
    self::sensor_area::register_components(world);
}
