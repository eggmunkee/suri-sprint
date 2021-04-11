
use ggez::{Context};
use specs::{World,WorldExt,Builder};
use serde::{Deserialize,Serialize};

use crate::components::logic::*;
use crate::entities::player::{CharacterBuilder,PlayerCharacter};
use crate::entities::effect_area::{EffectAreaType};
use crate::core::physics::{PhysicsWorld,CollisionCategory,PickupItemType,EntityType};


#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct SpriteDesc {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub path: Option<String>,
    pub sprite: Option<String>,
    pub scale: (f32, f32),
    pub angle: f32, 
    pub alpha: f32,
    pub src: (f32, f32, f32, f32), 
    //pub shader: Option<String>,
}


#[allow(dead_code)]
#[derive(Clone,Debug,Deserialize,Serialize)]
pub enum LevelType {
    Platformer,
    Overhead,
    Space
}

impl Default for LevelType {
    fn default() -> Self {
        LevelType::Platformer
    }
}

#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct WarpInfo {
    level_name: String,
    level_entry: String
}

impl Default for WarpInfo {
    fn default() -> Self {
        Self {
            level_name: String::new(),
            level_entry: String::new()
        }
    }
}

#[allow(dead_code)]
#[derive(Clone,Debug,Deserialize,Serialize)]
pub enum LevelGoals {
    UseExits, // No goals other than leaving by an exit
    // Goal is a logic key being set to logic_value, which goes to a specified level
    LogicState{ key: String, logic_value: bool}, 
    EntityCount{ entity_type:EntityType},
    AllGoals(Vec::<LevelGoals>),
    OneGoal(Vec::<LevelGoals>),
    None
}

impl Default for LevelGoals {
    fn default() -> Self {
        LevelGoals::UseExits
    }
}


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
    // Player start varieties
    Player {
        x: f32, y: f32, player: Option<PlayerCharacter>,
    },
    PlayerNamed {
        x: f32, y: f32, player: Option<PlayerCharacter>, name: String,
    },
    PlayerNpc {
        x: f32, y: f32, player: Option<PlayerCharacter>,
    },
    // Common entities - boxes, buttons and ghosts
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
    // Level terrain - platforms and physical objects - static or dynamic
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
    // Sprite varieties
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
    ParallaxSprite {
        x: f32, y: f32, lvl_center: (f32, f32), sprites: Vec::<SpriteDesc>, scroll_factors: Vec::<f32>,
    },
    // Portals and Exits
    Portal {
        x: f32, y: f32, w: f32, z: Option<f32>, name: String, destination: String, start_enabled: bool,
        logic: Option<ItemLogic>,
    },
    PortalSide {
        x: f32, y: f32, ang: f32, w: f32, h: f32, z: Option<f32>, color: String, name: String, destination: String, start_enabled: bool,
        normal: (f32, f32), logic: Option<ItemLogic>,
    },
    ParticleSys {
        x: f32, y: f32, z: f32, config: String, logic: Option<ItemLogic>,
    },
    Exit {
        x: f32, y: f32, w: f32, h: f32, z: Option<f32>, name: String, destination: String,
    },
    ExitCustom {
        x: f32, y: f32, w: f32, h: f32, z: Option<f32>, name: String, destination: String, image: String, img_w: f32, img_h: f32,
    },
    // Misc. Level Items
    Bowl {
        x: f32, y: f32,z: Option<f32>, 
    },
    Mouse {
        x: f32, y: f32, z: Option<f32>, 
    },
    Ball {
        x: f32, y: f32, z: Option<f32>, 
    },
    Pickup {
        x: f32, y: f32, z: Option<f32>, pickup_type: PickupItemType,
    },
    DynPickup {
        x: f32, y: f32, z: Option<f32>, pickup_type: PickupItemType,
    },
    EffectArea {
        x: f32, y: f32, w: f32, h: f32, area_type: EffectAreaType
    },
    ImportSection {
        name: String, x: f32, y: f32,
    },
    // Logic Items
    Connection {
        from: String, to: String, conn_type: ConnectionType,
    }
}

#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct LevelConfig {
    pub name: String,
    pub bounds: LevelBounds,
    pub soundtrack: String,
    pub level_type: Option<LevelType>,
    pub items: Vec::<LevelItem>,
    #[serde(skip)]
    pub built_player: bool,
    #[serde(skip)]
    pub build_index: i32,
}

#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct LevelSectionConfig {
    pub items: Vec::<LevelItem>,
    #[serde(skip)]
    pub built_player: bool,
    #[serde(skip)]
    pub build_index: i32,
}