
use ggez::nalgebra as na;
use na::{Point2,Vector2,distance_squared,distance};

use specs::{World, WorldExt, Entity};
use specs::Join;
use wrapped2d::b2;
use wrapped2d::user_data::*;
use wrapped2d::dynamics::body::{MetaBody};

//======================
use crate::components::{Position};
use crate::components::collision::{Collision};
use crate::components::player::{CharacterDisplayComponent};

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
pub enum CollisionCategory {
    Level = 1,
    Player = 2,
    Ghost = 4,
    Meow = 8,
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
    let def = b2::BodyDef {
        body_type: PhysicsBodyType::Dynamic,
        position: self::create_pos(pos),
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
    let def = b2::BodyDef {
        body_type: PhysicsBodyType::Dynamic,
        position: self::create_pos(pos),
        fixed_rotation: fixed_rot,
        linear_damping: 0.8,
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
        collision_category: CollisionCategory, collision_mask: &Vec<CollisionCategory>) 
        -> b2::BodyHandle {
    let def = b2::BodyDef {
        body_type: b2::BodyType::Static,
        position: self::create_pos(pos),
        angle: angle,
        fixed_rotation: true,
        .. b2::BodyDef::new()
    };
    let body_data = GameStateBodyData { entity_id: 0, collider_type: collision_category };
    
    // create body - getting handle
    let b_handle = world.create_body_with(&def, body_data);
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


pub fn add_static_body_circle(world: &mut PhysicsWorld, pos: &Point2<f32>, body_radius: f32,
    density: f32, restitution: f32,
    collision_category: CollisionCategory, collision_mask: &Vec<CollisionCategory>) 
    -> b2::BodyHandle {
    let mut def = b2::BodyDef {
        body_type: b2::BodyType::Static,
        position: self::create_pos(pos),
        fixed_rotation: true,
        .. b2::BodyDef::new()
    };
    let body_data = GameStateBodyData { entity_id: 0, collider_type: collision_category };

    // create body - getting handle
    let b_handle = world.create_body_with(&def, body_data);
    // get mut ref to body
    let mut body = world.body_mut(b_handle);

    let shape = b2::CircleShape::new_with(PhysicsVec { x: 0.0, y: 0.0 }, create_size(body_radius));

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
    let mut phys_writer = world.write_storage::<Collision>();
    let mut char_writer = world.write_storage::<CharacterDisplayComponent>();
    let entities = world.entities();

    // Make sure collision body has update itself from game loop
    for (mut collision, mut character, ent) in (&mut phys_writer, (&mut char_writer).maybe(), &entities).join() {
        
        // update collision body from character
        collision.pre_physics_hook(physics_world, character);

    }
}

pub fn advance_physics_system(world: &mut World, physics_world: &mut PhysicsWorld, delta_seconds: f32) {

    // update the physics world
    physics_world.step(delta_seconds, 5, 5);

    let mut delete_entity_list : Vec::<u32> = Vec::new();

    //println!("After physics world step ---------------------------------------------");

    // iterate bodies
    for (body_handle, meta) in physics_world.bodies() {
        let body = physics_world.body(body_handle);
        let body_data = &*body.user_data();
        let body_type = body.body_type();
        //let meta_data = &*meta_ref;
        //let e = meta_ref.user_data();
        let primary_id = body_data.entity_id;
        let primary_collider_type = body_data.collider_type;
        let entity_1 = world.entities().entity(primary_id);

        //let char_disp_comp_res = world.write_storage::<CharacterDisplayComponent>();
        let mut char_disp_comp_res = world.write_storage::<CharacterDisplayComponent>();

        if let Some(character) = char_disp_comp_res.get_mut(entity_1) {
            //println!("Character 1 {:?}", &entity_1);

            let mut any_stand_contact = false;

            for (other_body_handle, contact) in body.contacts() {

                if contact.is_touching() == false { continue; }

                let manifold = contact.world_manifold();
                let contact_normal = manifold.normal;
                let up_normal = b2::Vec2{  x:0.0, y:1.0 };
                let dot = self::dot_product(&contact_normal,&up_normal);

                //println!("contact normal: {:?} dot: {}", &contact_normal, &dot);

                let other_body = physics_world.body(other_body_handle);
                if other_body.body_type() == b2::BodyType::Static ||  other_body.body_type() == b2::BodyType::Dynamic {
                    //println!("Contact with static body {:?} by {:?}", &other_body, &body_handle); 


                    let other_body_data = &*other_body.user_data();
                    
                    //let b = other_meta_body.body;
                    //let other_body_data = (21,); //other_meta_body.();
                    //let otherbody = &mut *other_meta_body;
                    let other_id = other_body_data.entity_id;
                    let other_collider_type = other_body_data.collider_type;
    
                    let entity_2 = world.entities().entity(other_id);
                    if primary_collider_type == CollisionCategory::Ghost || other_collider_type == CollisionCategory::Ghost {
                        //println!("Body 1 collider type: {:?} -- Body 2 collider type: {:?}", primary_collider_type, other_collider_type);
                    }

                    // if primary_collider_type == CollisionCategory::Meow && other_collider_type == CollisionCategory::Ghost {
                    //     delete_entity_list.push(other_id);
                    // }

                    //println!("Character {:?} {:?} - Body 2 {:?} ", &entity_1, &character, &entity_2);

                    if dot > 0.2 && !character.going_up  {
                        //println!("Character {:?} stood on Body 2 {:?}, contact normal: {:?}", &entity_1, &entity_2, &contact_normal);
                        any_stand_contact = true;
                    }
                    
                }
            }

            //println!("Update character body status.");
            character.update_body_status(any_stand_contact);


        }
        else {

            if body_type == b2::BodyType::Static ||  body_type == b2::BodyType::Dynamic
                || body_type == b2::BodyType::Kinematic {



                for (other_body_handle, contact) in body.contacts() {

                    if contact.is_touching() == false { continue; }

                    let manifold = contact.world_manifold();
                    let contact_normal = manifold.normal;
                    let up_normal = b2::Vec2{  x:0.0, y:1.0 };
                    let dot = self::dot_product(&contact_normal,&up_normal);

                    //println!("contact normal: {:?} dot: {}", &contact_normal, &dot);
                
                    let other_body = physics_world.body(other_body_handle);
                    //if other_body.body_type() == b2::BodyType::Dynamic {
                        //println!("Contact with dynamic body {:?} by {:?}", &other_body_handle, &body_handle); 

                        let other_body_data = &*other_body.user_data();
                        
                        //let b = other_meta_body.body;
                        //let other_body_data = (21,); //other_meta_body.();
                        //let otherbody = &mut *other_meta_body;
                        let other_id = other_body_data.entity_id;
                        let other_collider_type = other_body_data.collider_type;

                        let entity_2 = world.entities().entity(other_id);

                        if primary_collider_type == CollisionCategory::Ghost && other_collider_type == CollisionCategory::Meow {
                            println!("Body 1 collider type: {:?} -- Body 2 collider type: {:?}", primary_collider_type, other_collider_type);
                        }
                        if other_collider_type == CollisionCategory::Ghost && primary_collider_type == CollisionCategory::Meow {
                            println!("Body 2 collider type: {:?} -- Body 1 collider type: {:?}", other_collider_type, primary_collider_type);
                        }

                        if other_collider_type == CollisionCategory::Meow && primary_collider_type == CollisionCategory::Ghost {
                            delete_entity_list.push(primary_id);
                        }
                        if primary_collider_type == CollisionCategory::Meow && other_collider_type == CollisionCategory::Ghost {
                            delete_entity_list.push(other_id);
                        }


                        

                    //}
                }


            }
        }
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

