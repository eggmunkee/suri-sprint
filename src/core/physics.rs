
use ggez::nalgebra as na;
use na::{Point2,Vector2,distance_squared,distance};

use specs::{World, WorldExt, Entity};
use specs::Join;
use wrapped2d::b2;
use wrapped2d::user_data::*;
use wrapped2d::dynamics::body::{MetaBody};
use wrapped2d::dynamics::contacts::{Contact};

//======================
// use crate::resources::{GameStateResource};
// use crate::components::{Position,CharLevelInteractor};
// use crate::components::collision::{Collision};
// use crate::components::logic::{LogicComponent};
// use crate::components::exit::{ExitComponent};
// use crate::components::portal::{PortalComponent};
// use crate::components::pickup::{PickupComponent};
// use crate::components::player::{CharacterDisplayComponent};
// use crate::components::npc::{NpcComponent};
// use crate::entities::level_builder::{LevelType};

#[derive(Default,Copy,Clone)]
pub struct GameStateBodyData {
    pub entity_id: u32,
    pub collider_type: CollisionCategory,
    pub entity_type: EntityType,
}

#[derive(Default)]
pub struct GameStatePhysicsData;

impl UserDataTypes for GameStatePhysicsData {
    type BodyData = GameStateBodyData;
    type JointData = ();
    type FixtureData = ();
}

pub type PhysicsWorld = b2::World<GameStatePhysicsData>;
pub type PhysicsBody = b2::Body;
pub type PhysicsBodyType = b2::BodyType;
pub type PhysicsBodyHandle = b2::BodyHandle;
pub type PhysicsVec = b2::Vec2;

pub const WORLD_SCALE : f32 = 50.0;

#[derive(Copy,Clone,Debug,PartialEq)]
pub enum CollideType {
    Player_Level,
    Player_Ghost,
    Player_Portal,
    Npc_Level,
    Npc_Portal,
    Collider_Portal,
    Ghost_Meow,
    Meow_Level,
    Player_Point,
    Collider_Collider, //generic physical touch
    Other,
}

#[derive(Copy,Clone,Debug,PartialEq)]
pub enum PickupItemType {
    Point,
}


#[derive(Copy,Clone,Debug,PartialEq)]
pub enum EntityType {
    Player,
    Platform,
    EmptyBox,
    Ghost,
    Meow,
    Portal,
    Exit,
    Button,
    PickupItem(PickupItemType),
    None
}

impl Default for EntityType {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Copy,Clone,Debug,PartialEq)]
pub enum CollisionCategory {
    Level = 1, // Physical level objects
    Player = 2, // Player collision type
    Etherial = 4, // ghosts plane of existence
    Sound = 8, // sound plane
    Portal = 16, // portal & exit colliders
    Unused = 32,
    Unused1 = 64,
    Unused2 = 128,
}

impl Default for CollisionCategory {
    fn default() -> Self {
        CollisionCategory::Unused
    }
}

// Trait for making an object a u16 bit value
pub trait CollisionBit {
    fn to_bits(&self) -> u16;
}

// converting collision category to bits
impl CollisionBit for CollisionCategory {
    fn to_bits(&self) -> u16 {
        *self as u16
    }
}

// converting vec of categories to bits by OR-ing
impl CollisionBit for Vec::<CollisionCategory> {
    fn to_bits(&self) -> u16 {
        //let vec = *self;
        let mut combined = 0u16;
        for &category in self {
            combined |= category.to_bits();
        }
        combined
    }
}

// Struct which holds body/fixture physics query results
pub struct PhysicsQueryInfo {
    pub hit_info: Vec::<(b2::BodyHandle,b2::FixtureHandle)>,
}

impl PhysicsQueryInfo {
    pub fn new() -> Self {
        Self {
            hit_info: vec![],
        }
    }
}
impl b2::QueryCallback for PhysicsQueryInfo {

    fn report_fixture(
        &mut self, 
        body: b2::BodyHandle, 
        fixture: b2::FixtureHandle
    ) -> bool {
        //println!()

        self.hit_info.push((body.clone(), fixture.clone()));

        true
    }
}

pub struct ContactFilterConfig {
    pub ghost_player_contact: bool
}

impl ContactFilterConfig {
    pub fn new(ghost_player_contact: bool) -> Self {
        Self {
            ghost_player_contact: ghost_player_contact
        }
    }
}

impl wrapped2d::dynamics::world::callbacks::ContactFilter<GameStatePhysicsData> for ContactFilterConfig {
    fn should_collide(
        &mut self,
        body_a: wrapped2d::dynamics::world::callbacks::BodyAccess<'_, GameStatePhysicsData>,
        fixture_a: wrapped2d::dynamics::world::callbacks::FixtureAccess<'_, GameStatePhysicsData>,
        body_b: wrapped2d::dynamics::world::callbacks::BodyAccess<'_, GameStatePhysicsData>,
        fixture_b: wrapped2d::dynamics::world::callbacks::FixtureAccess<'_, GameStatePhysicsData>
    ) -> bool {

        let a_phys_type = body_a.body_type();
        let b_phys_type = body_a.body_type();
        let a_type = &body_a.user_data().entity_type;
        let b_type = &body_b.user_data().entity_type;
        let filt_a = fixture_a.filter_data();
        let fixt_a_cat = filt_a.category_bits;
        let fixt_a_mask = filt_a.mask_bits;
        let filt_b = fixture_b.filter_data();
        let fixt_b_cat = filt_b.category_bits;
        let fixt_b_mask = filt_b.mask_bits;
        let cat_mask_match = (fixt_a_cat & fixt_b_mask) | (fixt_b_cat & fixt_b_mask);

        if a_phys_type == PhysicsBodyType::Dynamic || a_phys_type == PhysicsBodyType::Kinematic ||
            b_phys_type == PhysicsBodyType::Dynamic || b_phys_type == PhysicsBodyType::Kinematic {
            // println!("Should collide?");
            // println!(" A: {:?}, body type: {:?}", a_type, &a_phys_type);
            // println!(" B: {:?}, body type: {:?}", b_type, &b_phys_type);
            // println!(" A cat/mask ({}/{})", &fixt_a_cat, &fixt_a_mask);
            // println!(" B cat/mask ({}/{})", &fixt_b_cat, &fixt_b_mask);
            // println!(" Cat/Mask match: {}", &cat_mask_match);
        }

        

        let collide = match a_type {
            EntityType::Button => match b_type {
                EntityType::Ghost => false,
                EntityType::Platform => true,
                _ => true
            },
            EntityType::Platform => match b_type {
                EntityType::Button => true,
                _ => true
            },
            EntityType::Ghost => match b_type {
                EntityType::Player => self.ghost_player_contact,
                EntityType::Button => false,
                _ => true
            },
            EntityType::Player => match b_type {
                EntityType::Ghost => self.ghost_player_contact,
                _ => true
            },
            _ => true
        };

        if a_phys_type == PhysicsBodyType::Dynamic || a_phys_type == PhysicsBodyType::Kinematic ||
            b_phys_type == PhysicsBodyType::Dynamic || b_phys_type == PhysicsBodyType::Kinematic {
            //println!(" Collide: {}", &collide);
        }

        cat_mask_match > 0 && collide
    }
}

pub fn create_physics_world(gravity_amount: f32) -> PhysicsWorld {

    let gravity = PhysicsVec { x: 0.0, y: gravity_amount}; //25.0
    let world = PhysicsWorld::new(&gravity);
    //world.set_contact_filter(Box<>)

    world
}

pub fn create_physics_world_2d_grav(gravity_amount: (f32, f32)) -> PhysicsWorld {

    let gravity = PhysicsVec { x: gravity_amount.0, y: gravity_amount.1 }; //25.0
    let world = PhysicsWorld::new(&gravity);

    world
}


pub fn update_world_gravity(phys_world: &mut PhysicsWorld, gravity: f32) {
    let gravity_vec = PhysicsVec { x: 0.0, y: gravity}; //25.0
    phys_world.set_gravity(&gravity_vec);
}

pub fn update_world_gravity_2d(phys_world: &mut PhysicsWorld, gravity: (f32, f32)) {
    let gravity_vec = PhysicsVec { x: gravity.0, y: gravity.1}; //25.0
    phys_world.set_gravity(&gravity_vec);
}

pub fn dot_product(v1: &PhysicsVec, v2: &PhysicsVec) -> f32 {
    v1.x * v2.x + v1.y * v2.y
}

pub fn get_contact_floor_dot(contact: &Contact, flip: bool) -> f32 {
    let manifold = contact.world_manifold();
    let contact_normal = manifold.normal;
    let down_normal = match flip {
        false => b2::Vec2{  x:0.0, y:1.0 },
        true => b2::Vec2{  x:0.0, y:-1.0 },
    };
    let dot = self::dot_product(&contact_normal,&down_normal);

    dot
}

pub fn debug_contact_floor_dot(contact: &Contact, flip: bool) {
    let l_manifold = contact.manifold();
    let manifold = contact.world_manifold();
    let norm_c_normal = l_manifold.local_normal;
    let contact_normal = manifold.normal;
    let down_normal = match flip {
        false => b2::Vec2{  x:0.0, y:1.0 },
        true => b2::Vec2{  x:0.0, y:-1.0 },
    };
    let dot = self::dot_product(&contact_normal,&down_normal);

    let local_dot = self::dot_product(&norm_c_normal, &down_normal);
    //println!("Flipped? {}", &flip);
    //println!("W: Dot product of {:?} {:?} = {}", &contact_normal, &down_normal, &dot);
    //println!("L: Dot product of {:?} {:?} = {}", &norm_c_normal, &down_normal, &local_dot);
}


pub fn create_pos(pos: &Point2<f32>) -> PhysicsVec {
    let x = pos.x / WORLD_SCALE;
    let y = pos.y / WORLD_SCALE;

    PhysicsVec { x: x, y: y}
}

pub fn get_pos(phys_pos: &PhysicsVec) -> Point2<f32> {
    let x = phys_pos.x * WORLD_SCALE;
    let y = phys_pos.y * WORLD_SCALE;

    Point2::new(x, y)
}

pub fn create_size(world_size: f32) -> f32 {
    world_size / WORLD_SCALE
}

pub fn get_size(phys_size: f32) -> f32 {
    phys_size * WORLD_SCALE
}


pub fn update_body_entity_data(entity: &Entity, physics_world: &mut PhysicsWorld, body_handle: Option<b2::BodyHandle>) {
    let entity_id = entity.id();
    if let Some(handle) = body_handle {
        let mut collision_body = physics_world.body_mut(handle);

        let body_data = &mut *collision_body.user_data_mut();
        //let data = &*data_ref;
        body_data.entity_id = entity_id;            
    }

}


pub fn create_body(world: &mut PhysicsWorld, body_type: PhysicsBodyType, pos: &Point2<f32>, angle: f32, entity_type: EntityType,
    collision_category: CollisionCategory, fixed_rot: bool) -> PhysicsBodyHandle {

    let def = b2::BodyDef {
        body_type: body_type,
        position: self::create_pos(pos),
        angle: angle,
        linear_velocity: b2::Vec2 { x: 0.0, y: 0.0 },
        linear_damping: 0.8,
        angular_damping: 0.8,
        fixed_rotation: fixed_rot,
        .. b2::BodyDef::new()
    };

    let body_data = GameStateBodyData { entity_id: 0, collider_type: collision_category, entity_type: entity_type };

    // create body - getting handle
    let b_handle = world.create_body_with(&def, body_data);

    b_handle
}

pub fn add_kinematic_body_circle(world: &mut PhysicsWorld, pos: &Point2<f32>, vel: &Vector2<f32>, 
    body_radius: f32,     
    density: f32, restitution: f32, entity_type: EntityType,
    collision_category: CollisionCategory, collision_mask: &Vec<CollisionCategory>, fixed_rot: bool, is_sensor: bool) 
        -> b2::BodyHandle {
    let def = b2::BodyDef {
        body_type: PhysicsBodyType::Kinematic,
        position: self::create_pos(pos),
        linear_velocity: b2::Vec2 { x: vel.x, y: vel.y},
        linear_damping: 0.8,
        angular_damping: 0.8,
        fixed_rotation: fixed_rot,
        .. b2::BodyDef::new()
    };

    let body_data = GameStateBodyData { entity_id: 0, collider_type: collision_category, entity_type: entity_type };

    // create body - getting handle
    let b_handle = world.create_body_with(&def, body_data);
    // get mut ref to body
    let mut body = world.body_mut(b_handle);
    
    let shape = b2::CircleShape::new_with(PhysicsVec { x: 0.0, y: 0.0 }, create_size(body_radius));

    //let fixture_handle = body.create_fast_fixture(&shape, 2.);
    let mut fixture_def = b2::FixtureDef {
        is_sensor: is_sensor,
        density: density,
        restitution: restitution,
        filter: b2::Filter {
            category_bits: collision_category.to_bits(),
            mask_bits: collision_mask.to_bits(),
            group_index: 0,
        },
        .. b2::FixtureDef::new()
    };

    let fixture_handle = body.create_fixture(&shape, &mut fixture_def);
    let fixture = body.fixture(fixture_handle);

    b_handle
}


pub fn add_kinematic_body_box(world: &mut PhysicsWorld, pos: &Point2<f32>, vel: &Vector2<f32>, angle: f32,
    body_width: f32, body_height: f32,    
    density: f32, restitution: f32, entity_type: EntityType,
    collision_category: CollisionCategory, collision_mask: &Vec<CollisionCategory>, fixed_rot: bool, is_sensor: bool) 
        -> b2::BodyHandle {
    let def = b2::BodyDef {
        body_type: PhysicsBodyType::Kinematic,
        position: self::create_pos(pos),
        angle: angle,
        linear_velocity: b2::Vec2 { x: vel.x, y: vel.y},
        linear_damping: 0.8,
        angular_damping: 0.8,
        fixed_rotation: fixed_rot,
        .. b2::BodyDef::new()
    };

    let body_data = GameStateBodyData { entity_id: 0, collider_type: collision_category, entity_type: entity_type };

    // create body - getting handle
    let b_handle = world.create_body_with(&def, body_data);
    // get mut ref to body
    let mut body = world.body_mut(b_handle);
    
    let shape = b2::PolygonShape::new_box(create_size(body_width), create_size(body_height));
    // HOW TO DO CIRCLE SHAPE FIXTURE
    //let shape = b2::CircleShape::new_with(PhysicsVec { x: 0.0, y: 0.0 }, create_size(body_width));
    
    let mut fixture_def = b2::FixtureDef {
        density: density,
        restitution: restitution,
        is_sensor: is_sensor,
        filter: b2::Filter {
            category_bits: collision_category.to_bits(),
            mask_bits: collision_mask.to_bits(),
            group_index: 0,
        },
        .. b2::FixtureDef::new()
    };

    body.create_fixture(&shape, &mut fixture_def);

    b_handle
}


pub fn add_dynamic_body_box(world: &mut PhysicsWorld, pos: &Point2<f32>, angle: f32, body_width: f32, body_height: f32,
    density: f32, restitution: f32, entity_type: EntityType,
    collision_category: CollisionCategory, collision_mask: &Vec<CollisionCategory>, fixed_rot: bool) 
        -> b2::BodyHandle {
    // let def = b2::BodyDef {
    //     body_type: PhysicsBodyType::Dynamic,
    //     position: self::create_pos(pos),
    //     linear_damping: 0.8,
    //     fixed_rotation: fixed_rot,
    //     .. b2::BodyDef::new()
    // };

    // let body_data = GameStateBodyData { entity_id: 0, collider_type: collision_category };

    // create body - getting handle
    let b_handle = create_body(world, PhysicsBodyType::Dynamic, pos, angle, entity_type, collision_category, fixed_rot);

    //world.create_body_with(&def, body_data);
    // get mut ref to body
    let mut body = world.body_mut(b_handle);
    
    let shape = b2::PolygonShape::new_box(create_size(body_width), create_size(body_height));

    let mut fixture_def = b2::FixtureDef {
        density: density,
        restitution: restitution,
        filter: b2::Filter {
            category_bits: collision_category.to_bits(),
            mask_bits: collision_mask.to_bits(),
            group_index: 0,
        },
        .. b2::FixtureDef::new()
    };

    body.create_fixture(&shape, &mut fixture_def);

    b_handle
}


pub fn add_dynamic_body_circle(world: &mut PhysicsWorld, pos: &Point2<f32>, body_radius: f32,
        density: f32, restitution: f32, entity_type: EntityType,
        collision_category: CollisionCategory, collision_mask: &Vec<CollisionCategory>, fixed_rot: bool)  
        -> b2::BodyHandle {
    // let def = b2::BodyDef {
    //     body_type: PhysicsBodyType::Dynamic,
    //     position: self::create_pos(pos),
    //     fixed_rotation: fixed_rot,
    //     linear_damping: 0.8,
    //     .. b2::BodyDef::new()
    // };
    
    // let body_data = GameStateBodyData { entity_id: 0, collider_type: collision_category };
    
    // // create body - getting handle
    // let b_handle = world.create_body_with(&def, body_data);
    // create body - getting handle
    let b_handle = create_body(world, PhysicsBodyType::Dynamic, pos, 0.0, entity_type, collision_category, fixed_rot);
    
    // get mut ref to body
    let mut body = world.body_mut(b_handle);
    
    let shape = b2::CircleShape::new_with(PhysicsVec { x: 0.0, y: 0.0 }, create_size(body_radius));
    
    //let fixture_handle = body.create_fast_fixture(&shape, 2.);
    let mut fixture_def = b2::FixtureDef {
        density: density,
        restitution: restitution,
        filter: b2::Filter {
            category_bits: collision_category.to_bits(),
            mask_bits: collision_mask.to_bits(),
            group_index: 0,
        },
        .. b2::FixtureDef::new()
    };

    body.create_fixture(&shape, &mut fixture_def);

    b_handle
}


pub fn add_static_body_box(world: &mut PhysicsWorld, pos: &Point2<f32>, angle: f32, body_width: f32, body_height: f32,
        density: f32, restitution: f32, entity_type: EntityType,
        collision_category: CollisionCategory, collision_mask: &Vec<CollisionCategory>, is_sensor: bool, fixed_rot: bool) 
        -> b2::BodyHandle {
    // let def = b2::BodyDef {
    //     body_type: b2::BodyType::Static,
    //     position: self::create_pos(pos),
    //     angle: angle,
    //     fixed_rotation: true,
    //     .. b2::BodyDef::new()
    // };
    // let body_data = GameStateBodyData { entity_id: 0, collider_type: collision_category };
    
    // // create body - getting handle
    // let b_handle = world.create_body_with(&def, body_data);
    // create body - getting handle
    let b_handle = create_body(world, PhysicsBodyType::Static, pos, angle, entity_type, collision_category, fixed_rot);
    
    // get mut ref to body
    let mut body = world.body_mut(b_handle);
    
    let shape = b2::PolygonShape::new_box(create_size(body_width), create_size(body_height));

    let mut fixture_def = b2::FixtureDef {
        density: density,
        restitution: restitution,
        is_sensor: is_sensor,
        filter: b2::Filter {
            category_bits: collision_category.to_bits(),
            mask_bits: collision_mask.to_bits(),
            group_index: 0,
        },
        .. b2::FixtureDef::new()
    };
    
    body.create_fixture(&shape, &mut fixture_def);

    b_handle
}


pub fn add_static_body_circle(world: &mut PhysicsWorld, pos: &Point2<f32>, body_radius: f32,
    density: f32, restitution: f32, entity_type: EntityType,
    collision_category: CollisionCategory, collision_mask: &Vec<CollisionCategory>, is_sensor: bool, fixed_rot: bool) 
    -> b2::BodyHandle {
    // let mut def = b2::BodyDef {
    //     body_type: b2::BodyType::Static,
    //     position: self::create_pos(pos),
    //     fixed_rotation: true,
    //     .. b2::BodyDef::new()
    // };
    // let body_data = GameStateBodyData { entity_id: 0, collider_type: collision_category };

    // // create body - getting handle
    // let b_handle = world.create_body_with(&def, body_data);
    let b_handle = create_body(world, PhysicsBodyType::Static, pos, 0.0, entity_type, collision_category, fixed_rot);
    // get mut ref to body
    let mut body = world.body_mut(b_handle);

    let shape = b2::CircleShape::new_with(PhysicsVec { x: 0.0, y: 0.0 }, create_size(body_radius));

    let mut fixture_def = b2::FixtureDef {
        density: density,
        restitution: restitution,
        is_sensor: is_sensor,
        filter: b2::Filter {
            category_bits: collision_category.to_bits(),
            mask_bits: collision_mask.to_bits(),
            group_index: 0,
        },
        .. b2::FixtureDef::new()
    };

    body.create_fixture(&shape, &mut fixture_def);

    b_handle
}


