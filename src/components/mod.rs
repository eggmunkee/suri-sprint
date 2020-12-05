use ggez::{Context};
use ggez::nalgebra as na;
use specs::{ Component, DenseVecStorage, VecStorage, World, WorldExt};

use specs_derive::*;

pub mod logic;
pub mod sprite;
pub mod button;
pub mod player;
pub mod collision;
pub mod meow;
pub mod npc;
pub mod portal;
pub mod exit;
// DEFINE COMMON COMPONENTS

#[derive(Debug)]
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

pub trait CharLevelInteractor {
    fn set_standing(&mut self, is_standing: bool);
}


pub trait RenderTrait {    
    fn draw(&self, ctx: &mut Context, world: &World, ent: Option<u32>, pos: na::Point2::<f32>, item_index: u32);
}


impl RenderTrait for &dyn RenderTrait {
    fn draw(&self, _ctx: &mut Context, _world: &World, _ent: Option<u32>, _pos: na::Point2::<f32>, _item_index: u32) {

    }
}

// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    world.register::<Position>();
    world.register::<Velocity>();
    
    // sub-module components
    self::logic::register_components(world);
    self::sprite::register_components(world);
    self::collision::register_components(world);
    self::player::register_components(world);
    self::meow::register_components(world);
    self::npc::register_components(world);
    self::portal::register_components(world);
    self::button::register_components(world);
    self::exit::register_components(world);
}
