
use ggez::nalgebra as na;
use na::{Point2,Vector2,distance_squared,distance};

use specs::{World, WorldExt, Entity};
use specs::Join;
use wrapped2d::b2;
use wrapped2d::user_data::*;
use wrapped2d::dynamics::body::{MetaBody};
use wrapped2d::dynamics::contacts::{Contact};

//======================
use crate::resources::{GameStateResource};
use crate::components::{Position,CharLevelInteractor};
use crate::components::collision::{Collision};
use crate::components::exit::{ExitComponent};
use crate::components::portal::{PortalComponent};
use crate::components::player::{CharacterDisplayComponent};
use crate::components::npc::{NpcComponent};

#[derive(Default,Copy,Clone)]
pub struct GameStateBodyData {
    pub entity_id: u32,
    pub collider_type: CollisionCategory,
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
    Other,
}

#[derive(Copy,Clone,Debug,PartialEq)]
pub enum CollisionCategory {
    Level = 1, // Physical level objects
    Player = 2, // Player collision type
    Etherial = 4, // ghosts plane of existence
    Sound = 8, // sound plane
    Portal = 16, // portal & exit colliders
    Unused = 128,
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


pub fn create_physics_world() -> PhysicsWorld {

    let gravity = PhysicsVec { x: 0.0, y: 25.0};
    let world = PhysicsWorld::new(&gravity);

    world

}

pub fn dot_product(v1: &PhysicsVec, v2: &PhysicsVec) -> f32 {
    v1.x * v2.x + v1.y * v2.y
}

pub fn get_contact_floor_dot(contact: &Contact) -> f32 {
    let manifold = contact.world_manifold();
    let contact_normal = manifold.normal;
    let down_normal = b2::Vec2{  x:0.0, y:1.0 };
    let dot = self::dot_product(&contact_normal,&down_normal);

    dot
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


pub fn create_body(world: &mut PhysicsWorld, body_type: PhysicsBodyType, pos: &Point2<f32>, angle: f32,
    collision_category: CollisionCategory, fixed_rot: bool) -> PhysicsBodyHandle {

    let def = b2::BodyDef {
        body_type: body_type,
        position: self::create_pos(pos),
        angle: angle,
        linear_velocity: b2::Vec2 { x: 0.0, y: 0.0 },
        linear_damping: 0.8,
        fixed_rotation: fixed_rot,
        .. b2::BodyDef::new()
    };

    let body_data = GameStateBodyData { entity_id: 0, collider_type: collision_category };

    // create body - getting handle
    let b_handle = world.create_body_with(&def, body_data);

    b_handle
}

pub fn add_kinematic_body_circle(world: &mut PhysicsWorld, pos: &Point2<f32>, vel: &Vector2<f32>, 
    body_radius: f32,     
    density: f32, restitution: f32,
    collision_category: CollisionCategory, collision_mask: &Vec<CollisionCategory>, fixed_rot: bool, is_sensor: bool) 
        -> b2::BodyHandle {
    let def = b2::BodyDef {
        body_type: PhysicsBodyType::Kinematic,
        position: self::create_pos(pos),
        linear_velocity: b2::Vec2 { x: vel.x, y: vel.y},
        linear_damping: 0.8,
        fixed_rotation: fixed_rot,
        .. b2::BodyDef::new()
    };

    let body_data = GameStateBodyData { entity_id: 0, collider_type: collision_category };

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


pub fn add_kinematic_body_box(world: &mut PhysicsWorld, pos: &Point2<f32>, vel: &Vector2<f32>, 
    body_width: f32, body_height: f32,    
    density: f32, restitution: f32,
    collision_category: CollisionCategory, collision_mask: &Vec<CollisionCategory>, fixed_rot: bool, is_sensor: bool) 
        -> b2::BodyHandle {
    let def = b2::BodyDef {
        body_type: PhysicsBodyType::Kinematic,
        position: self::create_pos(pos),
        linear_velocity: b2::Vec2 { x: vel.x, y: vel.y},
        linear_damping: 0.8,
        fixed_rotation: fixed_rot,
        .. b2::BodyDef::new()
    };

    let body_data = GameStateBodyData { entity_id: 0, collider_type: collision_category };

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


pub fn add_dynamic_body_box(world: &mut PhysicsWorld, pos: &Point2<f32>, body_width: f32, body_height: f32,
    density: f32, restitution: f32,
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
    let b_handle = create_body(world, PhysicsBodyType::Dynamic, pos, 0.0, collision_category, fixed_rot);

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
        density: f32, restitution: f32,
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
    let b_handle = create_body(world, PhysicsBodyType::Dynamic, pos, 0.0, collision_category, fixed_rot);
    
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
        density: f32, restitution: f32,
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
    let b_handle = create_body(world, PhysicsBodyType::Static, pos, angle, collision_category, fixed_rot);
    
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
    density: f32, restitution: f32,
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
    let b_handle = create_body(world, PhysicsBodyType::Static, pos, 0.0, collision_category, fixed_rot);
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



pub fn advance_physics(world: &mut World, physics_world: &mut PhysicsWorld, delta_seconds: f32) {

    // Run Physics setup process - address any inputs to physics system
    self::pre_advance_physics(world, physics_world, delta_seconds);

    //println!("Running physics engine... delta={}", delta_seconds);
    self::advance_physics_system(world, physics_world, delta_seconds);

    // Run Physics post-run process - address any outputs of physics system to game world
    self::post_advance_physics(world, physics_world, delta_seconds);

}

// Handle any component state which affects the physics - ex. player input applied forces
fn pre_advance_physics(world: &mut World, physics_world: &mut PhysicsWorld, delta_seconds: f32) {
    let state_reader = world.fetch::<GameStateResource>();
    let mut phys_writer = world.write_storage::<Collision>();
    let mut char_writer = world.write_storage::<CharacterDisplayComponent>();
    let mut npc_writer = world.write_storage::<NpcComponent>();
    let entities = world.entities();

    let level_bounds = &state_reader.level_bounds;
    //println!("Pre-advance-physics, level bounds: {:?}", level_bounds);

    // Make sure collision body has update itself from game loop
    for (mut collision, mut character, mut npc, ent) in (&mut phys_writer, (&mut char_writer).maybe(),(&mut npc_writer).maybe(), &entities).join() {
        
        // update collision body from character
        collision.pre_physics_hook(physics_world, delta_seconds, character, npc, level_bounds);

    }
}

pub fn handle_contact(coll_type_1: &CollisionCategory, coll_type_2: &CollisionCategory) -> Option<CollideType> {
    // flip order if player id #2
    if coll_type_1 != &CollisionCategory::Player && coll_type_2 == &CollisionCategory::Player {
        return handle_contact(coll_type_2, coll_type_1);
    }
    match coll_type_1 {
        CollisionCategory::Player => match coll_type_2 {
            CollisionCategory::Portal => Some(CollideType::Collider_Portal),
            CollisionCategory::Level => Some(CollideType::Player_Level),
            _ => None
        },
        CollisionCategory::Etherial => match coll_type_2 {
            CollisionCategory::Portal => Some(CollideType::Collider_Portal),
            CollisionCategory::Sound => Some(CollideType::Ghost_Meow),
            _ => None,
        },
        CollisionCategory::Level => match coll_type_2 {
            CollisionCategory::Portal => {
                //println!("Got level-portal collide");
                Some(CollideType::Collider_Portal)
            },
            _ => None,
        },
        CollisionCategory::Portal => match coll_type_2 {
            CollisionCategory::Player => Some(CollideType::Collider_Portal),
            CollisionCategory::Etherial => Some(CollideType::Collider_Portal),
            CollisionCategory::Level => {
                //println!("Got level-portal collide");
                Some(CollideType::Collider_Portal)
            },
            _ => None,
        },
        CollisionCategory::Sound => match coll_type_2 {
            CollisionCategory::Etherial => Some(CollideType::Ghost_Meow),
            _ => None,
        }
        _ => None
    }
}

pub fn advance_physics_system(world: &mut World, physics_world: &mut PhysicsWorld, delta_seconds: f32) {

    // update the physics world
    physics_world.step(delta_seconds, 5, 5);

    // Keep list of collider entities that need to be destroyed
    let mut delete_entity_list : Vec::<u32> = Vec::new();



    //println!("After physics world step ---------------------------------------------");

    // iterate bodies
    for (body_handle, _) in physics_world.bodies() {
        // get physics body
        let body = physics_world.body(body_handle);
        // get physics body type
        let body_type = body.body_type();

        if body_type == PhysicsBodyType::Static { continue; }

        // get body metadata
        let body_data = &*body.user_data();
        // get game collider type
        let primary_collider_type = body_data.collider_type;

        // get world entity id
        let primary_id = body_data.entity_id;
        // get world entity
        let entity_1 = world.entities().entity(primary_id);

        // Get world data writers - Collision, Character, etc.
        let mut coll_res = world.write_storage::<Collision>();
        let mut char_disp_comp_res = world.write_storage::<CharacterDisplayComponent>();
        let mut npc_comp_res = world.write_storage::<NpcComponent>();

        // extract body 1 position from collision component
        let mut existing_portal = -1;
        let mut body_1_pos : na::Point2::<f32> = na::Point2::new(0.0,0.0);
        if let Some(collision) = coll_res.get_mut(entity_1) {
            body_1_pos.x = collision.pos.x;
            body_1_pos.y = collision.pos.y;
            existing_portal = collision.portal_id;
        }
        
        let mut any_stand_contact = false;

        for (other_body_handle, contact) in body.contacts() {

            if contact.is_touching() == false { continue; }

            let dot = get_contact_floor_dot(&contact);

            let other_body = physics_world.body(other_body_handle);
            let other_body_data = &*other_body.user_data();
                
            //let b = other_meta_body.body;
            //let other_body_data = (21,); //other_meta_body.();
            //let otherbody = &mut *other_meta_body;
            let other_id = other_body_data.entity_id;
            let other_collider_type = other_body_data.collider_type;

            // Handle entity 
            let entity_2 = world.entities().entity(other_id);

            let collide_type = handle_contact(&primary_collider_type, &other_collider_type);

            // Handle contact collide type info
            match &collide_type {
                Some(collide_t) => {

                    // HANDLE SPECIAL COLLIDE TYPES HERE IF NEEDED
                    // Handle ghost meow collide
                    if collide_t == &CollideType::Ghost_Meow {
                        if primary_collider_type == CollisionCategory::Etherial {
                            delete_entity_list.push(primary_id);
                        }
                        else {
                            delete_entity_list.push(other_id);
                        }
                    }                        
                    else if collide_t == &CollideType::Collider_Portal {
                        match primary_collider_type {
                            CollisionCategory::Etherial | CollisionCategory::Player 
                            | CollisionCategory::Level => {
                                //let portal_res = world.read_storage::<PortalComponent>();
                                if let Some(collision) = coll_res.get_mut(entity_1) {
                                    if primary_collider_type == CollisionCategory::Level {
                                        println!("Level collider portal collision update");
                                    }
                                    
                                    //if let Some(portal) = portal_res.get(entity_2) {
                                    let portal_id = other_id as i32;
                                    //if let Some(collision) = coll_res.get_mut(entity_1) {
                                    collision.in_portal = true;
                                    collision.portal_id = portal_id;
                                    //}
                                }
                            },
                            _ => {}
                        }

                        match other_collider_type {
                            CollisionCategory::Etherial | CollisionCategory::Player
                            | CollisionCategory::Level => {
                                // let portal_res = world.read_storage::<PortalComponent>();
                        
                                if let Some(collision) = coll_res.get_mut(entity_2) {
                                    if other_collider_type == CollisionCategory::Level {
                                        println!("Level collider portal collision update");
                                    }
                                    // if let Some(portal) = portal_res.get(entity_1) {
                                    let portal_id = primary_id as i32;
                                    //if let Some(collision) = coll_res.get_mut(entity_2) {
                                    collision.in_portal = true;
                                    collision.portal_id = portal_id;
                                    //}
                                }
                            },
                            _ => {}
                        }

                        // if primary_collider_type == CollisionCategory::Etherial 
                        //     || primary_collider_type == CollisionCategory::Level {

                        //     let mut portal_res = world.read_storage::<PortalComponent>();
                        //     let mut portal_id = -1;
                    
                        //     if let Some(portal) = portal_res.get(entity_2) {
                        //         portal_id = other_id as i32;
                        //         //if let Some(collision) = coll_res.get_mut(entity_1) {
                        //         collision.in_portal = true;
                        //         collision.portal_id = portal_id;
                        //         //}
                        //     }

                            
                        // }
                        // else if other_collider_type == CollisionCategory::Etherial 
                        // || other_collider_type == CollisionCategory::Level  {

                        //     let mut portal_res = world.read_storage::<PortalComponent>();
                        //     let mut portal_id = -1;
                    
                        //     if let Some(portal) = portal_res.get(entity_1) {
                        //         portal_id = other_id as i32;
                        //         //if let Some(collision) = coll_res.get_mut(entity_2) {
                        //         collision.in_portal = true;
                        //         collision.portal_id = portal_id;
                        //         //}
                        //     }

                            
                        // }
                    }

                    // Add generic body contact to collider
                    if let Some(collision) = coll_res.get_mut(entity_1) {
                        collision.body_contacts.push((other_id as i32, collide_t.clone()));                           
                    }
        

                    //println!("PLAYER PORTAL !");
                },
                // Some(CollideType::Player_Level) => {
                //     //println!("... player level contact ...");


                // },
                // Some(x) => {
                //     println!("Player collide type: {:?}", &x);
                // },
                _ => {}
            }

            //}


            //let mut other_body_pos = na::Point2::new(0.0, 0.0);
            //let mut has_portal = false;

            // // Get Other item collision component
            // if let Some(other_coll) = coll_res.get_mut(entity_2) {

            //     if other_collider_type == CollisionCategory::Portal {
            //         //println!("Body 1 collider type: {:?} -- Body 2 collider type: {:?}", primary_collider_type, other_collider_type);
            //         //println!("Player hit portal");
            //         //println!("Collision points: {:?} - {:?}", &body_1_pos, &other_coll.pos);
                    
            //         other_body_pos.x = other_coll.pos.x;
            //         other_body_pos.y = other_coll.pos.y;
            //         has_portal = true;
            //     }
            // }

            let mut exit_res = world.read_storage::<ExitComponent>();
            let mut portal_res = world.read_storage::<PortalComponent>();
            let mut exit_id = -1;
            let mut portal_id = -1;
    
            // handle character exit =====================================================
            if let Some(character) = char_disp_comp_res.get_mut(entity_1) {
                //println!("Character 1 {:?}", &entity_1);
                if let Some(exit) = exit_res.get(entity_2) {
                    exit_id = other_id as i32;
                    character.in_exit = true;
                    character.exit_id = exit_id;
                }

            }
            // handle portal contact
            // if let Some(portal) = portal_res.get(entity_2) {
            //     println!("Collider touched portal: {}", &other_id);
            //     portal_id = other_id as i32;
            //     //character.in_portal = true;
            //     //character.portal_id = portal_id;

            //     if let Some(collider) = coll_res.get_mut(entity_1) {
            //         println!("Collider collider get_mut...");
            //         collider.in_portal = true;
            //         collider.portal_id = portal_id;
            //     }
            // }


            // if primary_collider_type == CollisionCategory::Sound && other_collider_type == CollisionCategory::Etherial {
            //     delete_entity_list.push(other_id);
            // }

            //println!("Character {:?} {:?} - Body 2 {:?} ", &entity_1, &character, &entity_2);

            if dot > 0.2 {
                //println!("Character {:?} stood on Body 2 {:?}, contact normal: {:?}", &entity_1, &entity_2, &contact_normal);
                any_stand_contact = true;
            }
                
            //}
        }

        //println!("Update character body status.");
        //character.update_body_status(any_stand_contact);
        if let Some(character) = char_disp_comp_res.get_mut(entity_1) {
            character.set_standing(any_stand_contact);
        }
        else if let Some(npc) = npc_comp_res.get_mut(entity_1) {
            npc.set_standing(any_stand_contact);
        }


        // }
        // // Handle Non-Character Bodies ===========================================================================
        // else {

            // let mut any_stand_contact = false;

            // if body_type == b2::BodyType::Static ||  body_type == b2::BodyType::Dynamic
            //     || body_type == b2::BodyType::Kinematic {


            //     for (other_body_handle, contact) in body.contacts() {
            //         // Skip non-touching contacts
            //         if contact.is_touching() == false { continue; }

            //         let dot = get_contact_floor_dot(&contact);

            //         // Get body data for collider type
            //         let other_body = physics_world.body(other_body_handle);
            //         let other_body_data = &*other_body.user_data();
            //         let other_collider_type = other_body_data.collider_type;
            //         // Get contact type
            //         let collide_type = handle_contact(&primary_collider_type, &other_collider_type);

            //         // Get other entity id and world entity
            //         let other_id = other_body_data.entity_id;
            //         let entity_2 = world.entities().entity(other_id);

            //         if let Some(collision) = coll_res.get_mut(entity_1) {

            //             match &collide_type {
            //                 Some(collide_t) => {
            //                     // HANDLE SPECIAL COLLIDE TYPES HERE IF NEEDED
            //                     // Handle ghost meow collide
            //                     if collide_t == &CollideType::Ghost_Meow {
            //                         if primary_collider_type == CollisionCategory::Etherial {
            //                             delete_entity_list.push(primary_id);
            //                         }
            //                         else {
            //                             delete_entity_list.push(other_id);
            //                         }
            //                     }
            //                     else if collide_t == &CollideType::Collider_Portal {
            //                         if primary_collider_type == CollisionCategory::Etherial {

            //                             let mut portal_res = world.read_storage::<PortalComponent>();
            //                             let mut portal_id = -1;
                                
            //                             if let Some(portal) = portal_res.get(entity_2) {
            //                                 portal_id = other_id as i32;
            //                                 //if let Some(collision) = coll_res.get_mut(entity_1) {
            //                                 collision.in_portal = true;
            //                                 collision.portal_id = portal_id;
            //                                 //}
            //                             }
        
                                        
            //                         }
            //                         else if other_collider_type == CollisionCategory::Etherial {
        
            //                             let mut portal_res = world.read_storage::<PortalComponent>();
            //                             let mut portal_id = -1;
                                
            //                             if let Some(portal) = portal_res.get(entity_1) {
            //                                 portal_id = other_id as i32;
            //                                 //if let Some(collision) = coll_res.get_mut(entity_2) {
            //                                 collision.in_portal = true;
            //                                 collision.portal_id = portal_id;
            //                                 //}
            //                             }
        
                                        
            //                         }
            //                     }
            //                     // push all body contacts to collision component
            //                     collision.body_contacts.push((other_id as i32, collide_t.clone()));                           
            //                 },
            //                 _ => {}
            //             }
        
            //         }

            //         if dot > 0.2 {
            //             //println!("Character {:?} stood on Body 2 {:?}, contact normal: {:?}", &entity_1, &entity_2, &contact_normal);
            //             any_stand_contact = true;
            //         }

            //         // match &collide_type {
            //         //     Some(CollideType::Ghost_Meow) => {
            //         //         if primary_collider_type == CollisionCategory::Etherial {
            //         //             delete_entity_list.push(primary_id);
            //         //         }
            //         //         else {
            //         //             delete_entity_list.push(other_id);
            //         //         }
            //         //         println!("GHOST MEOW !");
            //         //     },
            //         //     Some(CollideType::Collider_Portal) => {
            //         //         println!("GHOST PORTAL !");

            //         //         if primary_collider_type == CollisionCategory::Etherial {

            //         //             let mut portal_res = world.read_storage::<PortalComponent>();
            //         //             let mut portal_id = -1;
                        
            //         //             if let Some(portal) = portal_res.get(entity_2) {
            //         //                 portal_id = other_id as i32;
            //         //                 if let Some(collision) = coll_res.get_mut(entity_1) {
            //         //                     collision.in_portal = true;
            //         //                     collision.portal_id = portal_id;
            //         //                 }
            //         //             }

                                
            //         //         }
            //         //         else if other_collider_type == CollisionCategory::Etherial {

            //         //             let mut portal_res = world.read_storage::<PortalComponent>();
            //         //             let mut portal_id = -1;
                        
            //         //             if let Some(portal) = portal_res.get(entity_1) {
            //         //                 portal_id = other_id as i32;
            //         //                 if let Some(collision) = coll_res.get_mut(entity_2) {
            //         //                     collision.in_portal = true;
            //         //                     collision.portal_id = portal_id;
            //         //                 }
            //         //             }

                                
            //         //         }

            //         //     },
            //         //     Some(CollideType::Other) => {
            //         //         collision.body_contacts.push((other_id as i32, collide_t.clone()));    
            //         //     },
            //         //     _ => {}
            //         // }
            //     }

            // }

            // if let Some(npc) = npc_comp_res.get_mut(entity_1) {
            //     if !npc.going_up {
            //         npc.set_standing(any_stand_contact);
            //     }

            // }

        //}
    }

    // Delete any entities on the list
    for &entity_id in &delete_entity_list {
        let entity = world.entities().entity(entity_id);

        if entity.gen().is_alive() {

            // Call destroy body on any collision component of entity

            let mut collision_res = world.write_storage::<Collision>();
            if let Some(collision) = collision_res.get_mut(entity) {
                //entry.
                //collision.body_handle = None;
                println!("Destroying body for entity: {:?}", entity_id);
                collision.destroy_body(physics_world);

            }

            world.entities().delete(entity);

        }
    }

    // for body_handle in delete_body_list {
    //     physics_world.destroy_body(body_handle);
    // }

}

// Handle physics changes by updating component state
fn post_advance_physics(world: &mut World, physics_world: &mut PhysicsWorld, delta_seconds: f32) {
    let mut phys_writer = world.write_storage::<Collision>();
    let mut pos_writer = world.write_storage::<Position>();
    let entities = world.entities();

    // Update collision components after physics runs
    for (mut collision, mut pos, ent) in (&mut phys_writer, &mut pos_writer, &entities).join() {
        collision.post_physics_hook(physics_world);
        // update position from collision position
        pos.x = collision.pos.x;
        pos.y = collision.pos.y;
    }
}

