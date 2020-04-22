

use ggez::{Context};
use specs::{World,WorldExt,Builder};
use serde::{Deserialize,Serialize};


use crate::conf::*;
use crate::components::sprite::*;
use crate::components::logic::*;
use crate::components::{Position};
use crate::entities::platform::{PlatformBuilder};
use crate::entities::empty_box::{BoxBuilder};
use crate::entities::button::{ButtonBuilder};
use crate::entities::portal_area::{PortalBuilder};
use crate::entities::exit::{ExitBuilder};
use crate::entities::suri::{SuriBuilder};
use crate::entities::ghost::{GhostBuilder};
use crate::entities::bowl::{BowlBuilder};
use crate::components::collision::{Collision};
use crate::resources::{ImageResources};
use crate::resources::{ConnectionResource};
use crate::physics::{PhysicsWorld,CollisionCategory};


#[allow(dead_code)]
#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct LevelBounds {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub solid_sides: [bool;4], // top, right, bottom, left
}

impl Default for LevelBounds {
    fn default() -> Self {
        LevelBounds {
            min_x: 0.0, min_y: 0.0, max_x: 1000.0, max_y: 800.0,
            solid_sides: [false, true, true, true],
        }
    }
}

impl LevelBounds {
    pub fn new(minx: f32, miny: f32, maxx: f32, maxy: f32) -> LevelBounds {
        LevelBounds {
            min_x: minx, min_y: miny, max_x: maxx, max_y: maxy,
            solid_sides: [false, true, true, true],
        }
    }
}

#[allow(dead_code)]
#[derive(Clone,Debug,Deserialize,Serialize)]
pub enum LevelItem {
    Player {
        x: f32, y: f32,
    },
    Platform {
        x: f32, y: f32, w: f32, h: f32, ang: f32,
    },
    DynPlatform {
        x: f32, y: f32, w: f32, h: f32, ang: f32,
    },
    EmptyBox {
        x: f32, y: f32, w: f32, h: f32, ang: f32,
    },
    DynEmptyBox {
        x: f32, y: f32, w: f32, h: f32, ang: f32,
    },
    Button {
        x: f32, y: f32, w: f32, h: f32, ang: f32, name: String,
    },
    Ghost {
        x: f32, y: f32,
    },
    Sprite {
        x: f32, y: f32, z: f32, sprite: String, angle: f32, src: (f32, f32, f32, f32),
    },
    Portal {
        x: f32, y: f32, w: f32, name: String, destination: String, enabled: bool,
    },
    Exit {
        x: f32, y: f32, w: f32, h: f32, name: String, destination: String,
    },
    Bowl {
        x: f32, y: f32,
    },
    Connection {
        from: String, to: String, conn_type: ConnectionType,
    }
}


#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct LevelConfig {
    pub name: String,
    pub bounds: LevelBounds,
    pub items: Vec::<LevelItem>,
}

impl LevelConfig {
    pub fn new() -> Self {
        LevelConfig {
            name: "".to_string(),
            bounds: LevelBounds::new(0.0, 0.0, 800.0, 600.0),
            items: vec![LevelItem::Player { x: 50.0, y: 50.0 }],
        }
    }

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
                    //SuriBuilder::build_npc(world, ctx, physics_world, *x+30.0, *y-30.0);
                },
                LevelItem::Platform{ x, y, w, h, ang} => {
                    PlatformBuilder::build(world, ctx, physics_world, *x, *y, *w, *h, *ang, SpriteLayer::World.to_z());
                },
                LevelItem::DynPlatform{ x, y, w, h, ang} => {
                    PlatformBuilder::build_dynamic(world, ctx, physics_world, *x, *y, *w, *h, *ang, SpriteLayer::World.to_z());
                },
                LevelItem::EmptyBox{ x, y, w, h, ang} => {
                    BoxBuilder::build(world, ctx, physics_world, *x, *y, *w, *h, *ang);
                },
                LevelItem::DynEmptyBox{ x, y, w, h, ang} => {
                    BoxBuilder::build_dynamic(world, ctx, physics_world, *x, *y, *w, *h, *ang, SpriteLayer::World.to_z());
                },
                LevelItem::Button{ x, y, w, h, ang, name } => {
                    ButtonBuilder::build(world, ctx, physics_world, *x, *y, *w, *h, *ang, (*name).to_string());
                },
                LevelItem::Ghost{ x, y } => {
                    GhostBuilder::build_collider(world, ctx, physics_world, *x, *y, 0.0, 0.0, 0.0, 0.0, 24.0, 24.0);  //(world, ctx, physics_world, *x, *y, *w, *h, *ang, SpriteLayer::BGNear.to_z());
                },
                LevelItem::Sprite{ x, y, z, sprite, angle, src } => {
                    let sprite_path = &*sprite;
                    let mut sprite = SpriteConfig::create_from_config(world, ctx, sprite_path.clone());
                    sprite.angle = *angle;
                    sprite.z_order = *z;
                    sprite.set_src(&src); 

                    world.create_entity().with(sprite).with(Position { x: *x, y: *y }).build();
                    //GhostBuilder::build_collider(world, ctx, physics_world, *x, *y, 0.0, 0.0, 0.0, 0.0, 24.0, 24.0);  //(world, ctx, physics_world, *x, *y, *w, *h, *ang, SpriteLayer::BGNear.to_z());
                },
                LevelItem::Portal { x, y, w, name, destination, enabled } => {
                    PortalBuilder::build(world, ctx, physics_world, *x, *y, *w, (*name).to_string(), (*destination).to_string(), *enabled);
                },
                LevelItem::Exit { x, y, w, h, name, destination } => {
                    ExitBuilder::build(world, ctx, physics_world, *x, *y, *w, *h, (*name).to_string(), (*destination).to_string());
                },
                LevelItem::Bowl { x, y } => {
                    BowlBuilder::build(world, ctx, physics_world, *x, *y);
                },
                LevelItem::Connection { from, to, conn_type } => {
                    let mut connection_res = world.fetch_mut::<ConnectionResource>();

                    let mut connection = &mut *connection_res;
                    connection.add_connection(from.clone(), to.clone());
                },
                _ => {}
            }
        }

        let border_thickness : f32 = 25.0;
        let dim_over = border_thickness * 0.7;
        if self.bounds.solid_sides[0] { // top
            let width = self.bounds.max_x - self.bounds.min_x;
            PlatformBuilder::build(world, ctx, physics_world, self.bounds.min_x + 0.5 * width, self.bounds.min_y + 1.0,
                width / 2.0 + dim_over, border_thickness, 0.0, SpriteLayer::BGNear.to_z());
        }
        if self.bounds.solid_sides[1] { // right
            let height = self.bounds.max_y - self.bounds.min_y;
            PlatformBuilder::build(world, ctx, physics_world, self.bounds.max_x - 1.0, self.bounds.min_y + 0.5 * height,
                border_thickness, height / 2.0 + dim_over, 0.0, SpriteLayer::BGNear.to_z());
        }
        if self.bounds.solid_sides[2] { // bottom
            let width = self.bounds.max_x - self.bounds.min_x;
            PlatformBuilder::build(world, ctx, physics_world, self.bounds.min_x + 0.5 * width, self.bounds.max_y - 1.0,
                width / 2.0 + dim_over, border_thickness, 0.0, SpriteLayer::BGNear.to_z());
        }
        if self.bounds.solid_sides[3] { // left
            let height = self.bounds.max_y - self.bounds.min_y;
            PlatformBuilder::build(world, ctx, physics_world, self.bounds.min_x + 1.0, self.bounds.min_y + 0.5 * height,
                border_thickness, height / 2.0 + dim_over, 0.0, SpriteLayer::BGNear.to_z());
        }

    }
}

