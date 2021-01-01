

use ggez::{Context};
use specs::{World,WorldExt,Builder};
use serde::{Deserialize,Serialize};


use crate::conf::*;
use crate::components::anim_sprite::*;
use crate::components::particle_sys::{ParticleSysConfig};
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
use crate::entities::mouse::{MouseBuilder};
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
pub struct ItemLogic {
    pub name: String,
    pub start_enabled: bool,
    pub logic_op: Option<LogicOpType>,
    pub logic_type: Option<ConnectionType>,
}

#[allow(dead_code)]
#[derive(Clone,Debug,Deserialize,Serialize)]
pub enum LevelItem {
    Player {
        x: f32, y: f32,
    },
    PlayerNamed {
        x: f32, y: f32, name: String,
    },
    Platform {
        x: f32, y: f32, w: f32, h: f32, ang: f32, z: Option<f32>, logic: Option<ItemLogic>
    },
    DynPlatform {
        x: f32, y: f32, w: f32, h: f32, ang: f32, 
    },
    StaticLevelProp {
        x: f32, y: f32, w: f32, h: f32, ang: f32, z: Option<f32>, 
        image: String, img_w: f32, img_h: f32,
        logic: Option<ItemLogic>,
    },
    DynStaticLevelProp {
        x: f32, y: f32, w: f32, h: f32, ang: f32, z: Option<f32>, image: String, img_w: f32, img_h: f32,
    },
    EmptyBox {
        x: f32, y: f32, w: f32, h: f32, ang: f32,
    },
    DynEmptyBox {
        x: f32, y: f32, w: f32, h: f32, ang: f32,
    },
    Button {
        x: f32, y: f32, w: f32, h: f32, ang: f32, name: String, start_enabled: bool,
    },
    Ghost {
        x: f32, y: f32,
    },
    Sprite {
        x: f32, y: f32, z: f32, sprite: String, angle: f32, src: (f32, f32, f32, f32), shader: Option<String>,
    },
    DynSprite {
        x: f32, y: f32, z: f32, sprite: String, angle: f32, src: (f32, f32, f32, f32), name: String, start_enabled: bool,
        logic_op: Option<LogicOpType>,
    },
    AnimSprite {
        x: f32, y: f32, z: f32, sprite: String, angle: f32, src: (f32, f32, f32, f32), shader: Option<String>,
    },
    Portal {
        x: f32, y: f32, w: f32, z: Option<f32>, name: String, destination: String, start_enabled: bool,
        logic_op: Option<LogicOpType>,
    },
    ParticleSys {
        x: f32, y: f32, z: f32, config: String,
    },
    Exit {
        x: f32, y: f32, w: f32, h: f32, z: Option<f32>, name: String, destination: String,
    },
    ExitCustom {
        x: f32, y: f32, w: f32, h: f32, z: Option<f32>, name: String, destination: String, image: String, img_w: f32, img_h: f32,
    },
    Bowl {
        x: f32, y: f32,z: Option<f32>, 
    },
    Mouse {
        x: f32, y: f32, z: Option<f32>, 
    },
    Connection {
        from: String, to: String, conn_type: ConnectionType,
    }
}


#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct LevelConfig {
    pub name: String,
    pub bounds: LevelBounds,
    pub soundtrack: String,
    pub items: Vec::<LevelItem>,
}

impl LevelConfig {
    pub fn new() -> Self {
        LevelConfig {
            name: "".to_string(),
            bounds: LevelBounds::new(0.0, 0.0, 800.0, 600.0),
            soundtrack: "".to_string(),
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

    pub fn build_level(&self, world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, entry_name: String) {
        
        for item in &self.items {
            match item {
                LevelItem::Player{ x, y } if entry_name.is_empty() => {
                    //SuriBuilder::build_npc(world, ctx, physics_world, *x+30.0, *y-30.0);
                    SuriBuilder::build(world, ctx, physics_world, *x, *y);
                },
                LevelItem::PlayerNamed{ x, y, name } if name == &entry_name => {
                    //SuriBuilder::build_npc(world, ctx, physics_world, *x+30.0, *y-30.0);
                    SuriBuilder::build(world, ctx, physics_world, *x, *y);
                },
                LevelItem::Platform{ x, y, w, h, ang, z, logic} => {
                    let mut z_value = SpriteLayer::World.to_z();
                    if let Some(z_cfg_val) = z {
                        z_value = *z_cfg_val;
                    }
                    PlatformBuilder::build_w_logic(world, ctx, physics_world, *x, *y, *w, *h, *ang, z_value, logic.clone());
                },
                LevelItem::DynPlatform{ x, y, w, h, ang} => {
                    PlatformBuilder::build_dynamic(world, ctx, physics_world, *x, *y, *w, *h, *ang, SpriteLayer::World.to_z());
                },
                LevelItem::StaticLevelProp{ x, y, w, h, ang, image, img_w, img_h, z, logic} => {
                    let mut z_value = SpriteLayer::World.to_z();
                    if let Some(z_cfg_val) = z {
                        z_value = *z_cfg_val;
                    }
                    PlatformBuilder::build_w_image_logic(world, ctx, physics_world, *x, *y, *w, *h, *ang, z_value, (*image).to_string(), *img_w, *img_h, logic.clone());
                },
                LevelItem::DynStaticLevelProp{ x, y, w, h, ang, image, img_w, img_h, z} => {
                    let mut z_value = SpriteLayer::World.to_z();
                    if let Some(z_cfg_val) = z {
                        z_value = *z_cfg_val;
                    }
                    PlatformBuilder::build_dynamic_w_image(world, ctx, physics_world, *x, *y, *w, *h, *ang, z_value, (*image).to_string(), *img_w, *img_h);
                },
                LevelItem::EmptyBox{ x, y, w, h, ang} => {
                    BoxBuilder::build(world, ctx, physics_world, *x, *y, *w, *h, *ang);
                },
                LevelItem::DynEmptyBox{ x, y, w, h, ang} => {
                    BoxBuilder::build_dynamic(world, ctx, physics_world, *x, *y, *w, *h, *ang, SpriteLayer::World.to_z());
                },
                LevelItem::Button{ x, y, w, h, ang, name, start_enabled } => {
                    ButtonBuilder::build(world, ctx, physics_world, *x, *y, *w, *h, *ang, (*name).to_string(), *start_enabled);
                },
                LevelItem::Ghost{ x, y } => {
                    GhostBuilder::build_collider(world, ctx, physics_world, *x, *y, 0.0, 0.0, 0.0, 0.0, 24.0, 24.0);  //(world, ctx, physics_world, *x, *y, *w, *h, *ang, SpriteLayer::BGNear.to_z());
                },
                LevelItem::Sprite{ x, y, z, sprite, angle, src, shader} => {
                    let sprite_path = &*sprite;
                    let mut sprite = SpriteConfig::create_from_config(world, ctx, sprite_path.clone());
                    sprite.angle = *angle;
                    sprite.z_order = *z;
                    sprite.set_src(&src); 
                    sprite.shader = shader.clone();

                    world.create_entity().with(sprite).with(Position { x: *x, y: *y }).build();
                },
                LevelItem::AnimSprite{ x, y, z, sprite, angle, src, shader} => {
                    let sprite_path = &*sprite;
                    let mut sprite = AnimSpriteConfig::create_from_config(world, ctx, sprite_path.clone());
                    sprite.angle = *angle;
                    sprite.z_order = *z;
                    sprite.set_src(&src); 
                    sprite.shader = shader.clone();

                    world.create_entity().with(sprite).with(Position { x: *x, y: *y }).build();
                },
                LevelItem::DynSprite{ x, y, z, sprite, angle, src, name, start_enabled, logic_op } => {
                    let sprite_path = &*sprite;
                    let mut sprite = SpriteConfig::create_from_config(world, ctx, sprite_path.clone());
                    sprite.angle = *angle;
                    sprite.z_order = *z;
                    sprite.toggleable = true;
                    sprite.set_src(&src); 

                    let logic_comp = LogicComponent::new((*name).to_string(), *start_enabled, *logic_op);
                    // set logic operation if specified
                    // if let Some(logic_operation) = &logic_op {
                    //     logic_comp.logic_op = *logic_operation;
                    // }
                    world.create_entity().with(sprite).with(logic_comp).with(Position { x: *x, y: *y }).build();
                },
                LevelItem::Portal { x, y, w, z, name, destination, start_enabled, logic_op } => {
                    let mut z_value = SpriteLayer::World.to_z();
                    if let Some(z_cfg_val) = z {
                        z_value = *z_cfg_val;
                    }
                    PortalBuilder::build(world, ctx, physics_world, *x, *y, *w, (*name).to_string(), (*destination).to_string(), *start_enabled, *logic_op);
                },
                LevelItem::ParticleSys { x, y, z, config } => {
                    let config_path = &*config;
                    let mut part_sys = ParticleSysConfig::create_from_config(world, ctx, config_path.clone());
                    part_sys.z_order = *z;

                    world.create_entity().with(part_sys).with(Position { x: *x, y: *y }).build();
                },
                LevelItem::Exit { x, y, w, h, z, name, destination } => {
                    let mut z_value = SpriteLayer::BGNear.to_z();
                    if let Some(z_cfg_val) = z {
                        z_value = *z_cfg_val;
                    }
                    ExitBuilder::build(world, ctx, physics_world, *x, *y, z_value, *w, *h, (*name).to_string(), (*destination).to_string());
                },
                LevelItem::ExitCustom { x, y, w, h, z, name, destination, image, img_w, img_h } => {
                    let mut z_value = SpriteLayer::BGNear.to_z();
                    if let Some(z_cfg_val) = z {
                        z_value = *z_cfg_val;
                    }
                    ExitBuilder::build_w_image(world, ctx, physics_world, *x, *y, z_value, *w, *h, (*name).to_string(), (*destination).to_string(),
                        (*image).to_string(), *img_w, *img_h);
                },
                LevelItem::Bowl { x, y, z } => {
                    let mut z_value = SpriteLayer::Entities.to_z();
                    if let Some(z_cfg_val) = z {
                        z_value = *z_cfg_val;
                    }
                    BowlBuilder::build(world, ctx, physics_world, *x, *y, z_value);
                },
                LevelItem::Mouse { x, y, z } => {
                    let mut z_value = 300.0;
                    if let Some(z_cfg_val) = z {
                        z_value = *z_cfg_val;
                    }
                    MouseBuilder::build(world, ctx, physics_world, *x, *y, 32.0, 8.0, 0.0, z_value);
                },
                LevelItem::Connection { from, to, conn_type } => {
                    let mut connection_res = world.fetch_mut::<ConnectionResource>();

                    let mut connection = &mut *connection_res;
                    connection.add_connection(from.clone(), to.clone(), LogicOpType::And);
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

