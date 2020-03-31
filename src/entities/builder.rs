

use ggez::{Context};
use specs::{World,WorldExt};
use serde::{Deserialize,de::DeserializeOwned};


use crate::conf::*;
use crate::components::sprite::*;
use crate::components::{Position, Velocity, DisplayComp, DisplayCompType};
use crate::entities::platform::{PlatformBuilder};
use crate::entities::suri::{SuriBuilder};
use crate::entities::ghost::{GhostBuilder};
use crate::components::collision::{Collision};
use crate::resources::{ImageResources};
use crate::physics::{PhysicsWorld,CollisionCategory};


#[derive(Copy,Clone,Debug,Deserialize)]
pub enum LevelItem {
    Player {
        x: f32, y: f32,
    },
    Platform {
        x: f32, y: f32, w: f32, h: f32, ang: f32,
    },
    Ghost {
        x: f32, y: f32,
    }
}


#[derive(Clone,Debug,Deserialize)]
pub struct LevelConfig {
    pub name: String,
    pub items: Vec::<LevelItem>,
}

impl LevelConfig {
    pub fn load_level(path: &str) -> LevelConfig {
        println!("Loading level {}", path);
        let mut level_path = String::from(path);
        level_path.insert_str(0, "levels/");

        let opt_level = get_ron_config::<LevelConfig>(level_path);

        opt_level.expect(format!("Failed to load level {}", path).as_str())

    }

    pub fn build_level(&self, world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld) {
        
        for item in &self.items {
            match item {
                LevelItem::Player{ x, y } => {
                    SuriBuilder::build(world, ctx, physics_world, *x, *y);
                },
                LevelItem::Platform{ x, y, w, h, ang} => {
                    PlatformBuilder::build(world, ctx, physics_world, *x, *y, *w, *h, *ang, SpriteLayer::BGNear.to_z());
                },
                LevelItem::Ghost{ x, y } => {
                    GhostBuilder::build_collider(world, ctx, physics_world, *x, *y, 0.0, 0.0, 0.0, 0.0, 24.0, 24.0);  //(world, ctx, physics_world, *x, *y, *w, *h, *ang, SpriteLayer::BGNear.to_z());
                },
                _ => {}
            }
        }
    }
}

