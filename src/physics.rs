
use ggez::nalgebra as na;
use na::{Point2,Vector2,distance_squared,distance};

use specs::{World, WorldExt};
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

#[derive(Copy,Clone,Debug)]
pub enum CollisionCategory {
    Level = 1,
    Player = 2,
    Ghost = 4,
    Meow = 8,
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

    let body_data = GameStateBodyData { entity_id: 0 };

    // create body - getting handle
    let b_handle = world.create_body_with(&def, body_data);
    // get mut ref to body
    let mut body = world.body_mut(b_handle);
    
    let shape = b2::PolygonShape::new_box(create_size(body_width), create_size(body_height));
    // HOW TO DO CIRCLE SHAPE FIXTURE
    //let shape = b2::CircleShape::new_with(PhysicsVec { x: 0.0, y: 0.0 }, create_size(body_width));
    
    // let mut mask_bits = 0u16;
    // for &category in collision_mask {
    //     mask_bits |= category.to_bits();
    // }

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

    let fixture_handle = body.create_fixture(&shape, &mut fixture_def);
    let fixture = body.fixture(fixture_handle);

    b_handle
}


pub fn add_dynamic_body_circle(world: &mut PhysicsWorld, pos: &Point2<f32>, body_radius: f32,
        density: f32, restitution: f32,
        collision_category: CollisionCategory, collision_mask: &Vec<CollisionCategory>)  
        -> b2::BodyHandle {
    let def = b2::BodyDef {
        body_type: PhysicsBodyType::Dynamic,
        position: self::create_pos(pos),
        linear_damping: 0.8,
        .. b2::BodyDef::new()
    };
    
    let body_data = GameStateBodyData { entity_id: 0 };
    
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

    let fixture_handle = body.create_fixture(&shape, &mut fixture_def);
    //let fixture = body.fixture(fixture_handle);

    b_handle
}


pub fn add_static_body_box(world: &mut PhysicsWorld, pos: &Point2<f32>, body_width: f32, body_height: f32,
        density: f32, restitution: f32,
        collision_category: CollisionCategory, collision_mask: &Vec<CollisionCategory>) 
        -> b2::BodyHandle {
    let mut def = b2::BodyDef {
        body_type: b2::BodyType::Static,
        position: self::create_pos(pos),
        fixed_rotation: true,
        .. b2::BodyDef::new()
    };
    let body_data = GameStateBodyData { entity_id: 0 };
    
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
    
    let fixture_handle = body.create_fixture(&shape, &mut fixture_def);
    //let fixture = body.fixture(fixture_handle);

    b_handle
}

// Handle any component state which affects the physics - ex. player input applied forces
fn pre_advance_physics(world: &mut World, physics_world: &mut PhysicsWorld, delta_seconds: f32) {
    let mut phys_writer = world.write_storage::<Collision>();
    let mut char_writer = world.write_storage::<CharacterDisplayComponent>();
    let entities = world.entities();

    // Make sure collision body has update itself from game loop
    for (mut collision, mut character, ent) in (&mut phys_writer, &mut char_writer, &entities).join() {
        
        // update collision body from character
        collision.update_body(physics_world, character);

    }
}

pub fn advance_physics(world: &mut World, physics_world: &mut PhysicsWorld, delta_seconds: f32) {

    // Run Physics setup process - address any inputs to physics system
    self::pre_advance_physics(world, physics_world, delta_seconds);

    //println!("Running physics engine... delta={}", delta_seconds);
    // update the physics world
    physics_world.step(delta_seconds, 5, 5);

    let mut delete_entity_list : Vec::<u32> = Vec::new();
    let mut delete_body_list : Vec::<b2::BodyHandle> = Vec::new();

    // iterate bodies
    for (body_handle, meta) in physics_world.bodies() {
        let body = physics_world.body(body_handle);
        let body_data = &*body.user_data();
        //let meta_data = &*meta_ref;
        //let e = meta_ref.user_data();
        let primary_id = body_data.entity_id;
        let entity_1 = world.entities().entity(primary_id);

        //let char_disp_comp_res = world.write_storage::<CharacterDisplayComponent>();
        let mut char_disp_comp_res = world.write_storage::<CharacterDisplayComponent>();

        if let Some(character) = char_disp_comp_res.get_mut(entity_1) {
            println!("Character 1 {:?}", &entity_1);

            for (other_body_handle, contact) in body.contacts() {

                if contact.is_touching() == false { continue; }

                let manifold = contact.manifold();
                let contact_normal = manifold.local_normal;
                let up_normal = b2::Vec2{  x:0.0, y:1.0 };
                let dot = self::dot_product(&contact_normal,&up_normal);

                println!("contact normal: {:?},body_handle: {:?} other: {:?}, dot: {}", &contact_normal, &body_handle, &other_body_handle, &dot);

                let other_body = physics_world.body(other_body_handle);
                if other_body.body_type() == b2::BodyType::Static ||  other_body.body_type() == b2::BodyType::Dynamic {
                    //println!("Contact with static body {:?} by {:?}", &other_body, &body_handle); 
    
                    let other_body_data = &*other_body.user_data();
                    
                    //let b = other_meta_body.body;
                    //let other_body_data = (21,); //other_meta_body.();
                    //let otherbody = &mut *other_meta_body;
                    let other_id = other_body_data.entity_id;
    
                    let entity_2 = world.entities().entity(other_id);

                    if dot > 0.7 && !character.going_up  {
                        println!("Character {:?} stood on Body 2 {:?}, contact normal: {:?}", &entity_1, &entity_2, &contact_normal);
                        character.since_stand = 0.0;
                    }
                    
                    //delete_entity_list.push(other_id);
                    //world.entities().delete(entity_2);
                    //delete_body_list.push(other_body_handle.clone());
                }
            }

        }
        else {

            for (other_body_handle, contact) in body.contacts() {
            
                let other_body = physics_world.body(other_body_handle);
                if other_body.body_type() == b2::BodyType::Static {
                    //println!("Contact with static body {:?} by {:?}", &other_body, &body_handle); 
    
                    let other_body_data = &*other_body.user_data();
                    
                    //let b = other_meta_body.body;
                    //let other_body_data = (21,); //other_meta_body.();
                    //let otherbody = &mut *other_meta_body;
                    let other_id = other_body_data.entity_id;

                    
    
                    let entity_2 = world.entities().entity(other_id);
                    if let Some(character) = char_disp_comp_res.get(entity_2) {
                        
                        println!("Body 1 {:?} - Character {:?} ", &entity_1, &character);
                        
                    }
                    
    
                }
            }



        }

        
    }

    // Run Physics post-run process - address any outputs of physics system to game world
    self::post_advance_physics(world, physics_world, delta_seconds);

}

// Handle physics changes by updating component state
fn post_advance_physics(world: &mut World, physics_world: &mut PhysicsWorld, delta_seconds: f32) {
    let mut phys_writer = world.write_storage::<Collision>();
    let mut pos_writer = world.write_storage::<Position>();
    let entities = world.entities();

    // Update collision components after physics runs
    for (mut collision, mut pos, ent) in (&mut phys_writer, &mut pos_writer, &entities).join() {
        collision.update_component(physics_world);
        // update position from collision position
        pos.x = collision.pos.x;
        pos.y = collision.pos.y;
    }
}

// let gravity = b2::Vec2 { x: 0., y: -10. };
// let world = b2::World::<NoUserData>::new(&gravity);

// use cgmath::{Point2, Rad, Rotation2, Transform};
// use shrev::EventChannel;
// use specs::{RunNow, World};

// use rhusics::collide::prelude2d::{ BodyPose2,
//                                        BroadBruteForce2, CollisionMode, CollisionShape2,
//                                        CollisionStrategy, GJK2, Rectangle};

















// Check if two points are within a given radius using squared distances
//  If they are, it returns true and the actual distance
//  Otherwise, it returns false the radius value
pub fn radius_check_points(point: &Point2<f32>, check_point: &Point2<f32>, radius: f32) -> (bool, f32, Point2<f32>, Point2<f32>) {

    if distance_squared(point, check_point) > radius * radius {
        (false, radius, na::Point2::new(point.x, point.y), na::Point2::new(check_point.x, check_point.y))
    }
    else {
        let d = distance(point, check_point);
        (d < radius, d, na::Point2::new(point.x, point.y), na::Point2::new(check_point.x, check_point.y))
    }
}


pub fn radius_square_check_points(point: &Point2<f32>, check_point: &Point2<f32>, combined_radius: f32) -> (bool, f32, Point2<f32>, Point2<f32>) {

    let x_dif = point.x - check_point.x;
    let y_dif = point.y - check_point.y;
    let half_radius = combined_radius / 2.0;
    println!("=========== RAD SQUARE CHECK POINTS=================");
    // distance is min of either dimension given aligned squares
    // use only one axis for the return points
    let (x_dif_abs, y_dif_abs) = (x_dif.abs(), y_dif.abs());
    let (act_x_dif, act_y_dif) = (x_dif_abs - combined_radius, y_dif_abs - combined_radius);
    // pick farther away axis to just distance
    if act_x_dif > act_y_dif {
        let new_dist = act_x_dif;
        let new_pt1 = na::Point2::new(point.x, point.y);
        let mut xd = 0.0;
        if x_dif > 0.0 {
            xd = x_dif + combined_radius;
        }
        else {
            xd = x_dif - combined_radius;
        }
        let new_pt2 = na::Point2::new(point.x-xd, point.y);
        //let new_dist = dist; //distance(&new_pt1, &new_pt2);
        println!("radSqrChkPts - X orig: {:?} {:?}", &point, &check_point);
        println!("radSqrChkPts - X dist:{} {:?} {:?}", &new_dist, &new_pt1, &new_pt2);
        // return points vector only in this axis direction
        (new_dist < 0.0, new_dist, new_pt1, new_pt2)
    }
    else {
        let new_dist = act_y_dif;
        //println!("radSqrChkPts - Y orig: {:?} {:?}", &na::Point2::new(point.x, point.y), &na::Point2::new(check_point.x, check_point.y));
        //println!("radSqrChkPts - Y dist:{} {:?} {:?}", &dist, &na::Point2::new(point.x, point.y), &na::Point2::new(point.x, point.y-y_dif));
        let new_pt1 = na::Point2::new(point.x, point.y);
        let mut yd = 0.0;
        if y_dif > 0.0 {
            yd = y_dif + combined_radius;
        }
        else {
            yd = y_dif - combined_radius;
        }

        let new_pt2 = na::Point2::new(point.x, point.y-yd);
        //let new_dist = dist; // distance(&new_pt1, &new_pt2);
        println!("radSqrChkPts - X orig: {:?} {:?}", &point, &check_point);
        println!("radSqrChkPts - X dist:{} {:?} {:?}", &new_dist, &new_pt1, &new_pt2);
        // return points vector only in this axis direction
        (new_dist < 0.0, new_dist, new_pt1, new_pt2)
    }
    
}


pub fn rect_check_points(point: &Point2<f32>, check_point: &Point2<f32>, dim_1: &[f32;2], dim_2: &[f32;2]) -> (bool, f32, Point2<f32>, Point2<f32>) {

    let x_radius = dim_1[0] + dim_2[0];
    let y_radius = dim_1[1] + dim_2[1];
    let x_dif = point.x - check_point.x;
    let y_dif = point.y - check_point.y;
    // distance is min of either dimension given aligned squares
    // use only one axis for the return points
    let (x_dif_abs, y_dif_abs) = (x_dif.abs(), y_dif.abs());
    let (eff_x_dif, eff_y_dif) = (x_dif_abs - x_radius, y_dif_abs - y_radius);

    // pick farther away axis to just distance
    if eff_x_dif > eff_y_dif {
        let dist = eff_x_dif;
        //println!("radSqrChkPts - X orig: {:?} {:?}", &na::Point2::new(point.x, point.y), &na::Point2::new(check_point.x, check_point.y));
        //println!("radSqrChkPts - X dist:{} {:?} {:?}", &dist, &na::Point2::new(point.x, point.y), &na::Point2::new(point.x-x_dif, point.y));

        // return points vector only in this axis direction
        (dist < x_radius, dist, na::Point2::new(point.x, point.y), na::Point2::new(point.x-x_dif, point.y))
    }
    else {
        let dist = eff_y_dif;
        //println!("radSqrChkPts - Y orig: {:?} {:?}", &na::Point2::new(point.x, point.y), &na::Point2::new(check_point.x, check_point.y));
        //println!("radSqrChkPts - Y dist:{} {:?} {:?}", &dist, &na::Point2::new(point.x, point.y), &na::Point2::new(point.x, point.y-y_dif));

        // return points vector only in this axis direction
        (dist < y_radius, dist, na::Point2::new(point.x, point.y), na::Point2::new(point.x, point.y-y_dif))
    }
    
}

// fn pt_vector_check(_check_point: &Point2<f32>, _vector: &Vector2<f32>) -> bool {
//     true
// }

pub fn actors_push(pos_i: &na::Point2<f32>, pos_j: &na::Point2<f32>, 
    svel_i : Option<&mut na::Vector2<f32>>, svel_j : Option<&mut na::Vector2<f32>>,
    mass_i: f32, mass_j: f32, friction_i: f32, friction_j: f32, combined_obj_radius: f32) {
    //let impulse : f32 = 220.0;
    let touch_dist : f32 = combined_obj_radius;
    //let friction_ratio : f32 = 0.90;
    //let pt = na::Point2::new(pos_i.x,pos_i.y);
    let (check, dist, pos_i, pos_j) = self::radius_check_points(&pos_i, &pos_j, touch_dist);
    if check {
        let mut imp = 0.0;
        if dist > 0.1 {
            let x_dif = (pos_j.x - pos_i.x) / dist;
            let y_dif = (pos_j.y - pos_i.y)  / dist;
            let overlap_len = dist;
            //imp = overlap_;
            //let overlap_ratio = overlap_len / touch_dist;
            imp = overlap_len * 2.0; //overlap_ratio * imp;
            let mut x_imp = imp * x_dif;
            let mut y_imp = imp * y_dif;
            //let inv_len = (touch_dist - dist).max(0.0).min(51.0);
            //let frac = 1.0 / (30.0 * dist); //inv_len / 51.0;
            if imp > 0.0 {
                //imp *= 1.0 - frac;

                // Check velocity status of two entities
                let no_i = match svel_i {
                    Some(_) => true,
                    _ => false
                };
                let no_j = match svel_j {
                    Some(_) => true,
                    _ => false
                };
                // Default to full strength impulses from mass
                let mut mass_frac_i = 0.5;
                let mut mass_frac_j = 0.5;
                // Double impulse per object if one doesn't have velocity
                if no_i || no_j {
                    x_imp *= 2.0;
                    y_imp *= 2.0;
                }

                mass_frac_i = mass_j / (mass_i + mass_j);
                mass_frac_j = mass_i / (mass_i + mass_j);

                let total_friction = (friction_i + friction_j);

                // if mass_i > 0.0 && mass_j > 0.0 {
                //     // having masses for both items, calc fraction of mass
                //     // for I and J, to multiply impulse by
                //     // frac is the magnitude of mass applied by the other object
                //     // i's mass frac is j's mass - J's mass applies in the force on I
                //     // j's mass frac is i's mass - I's mass applies in the force on J
                // }

                // If I has velocity to update, apply impulse
                if let Some(vel_i) = svel_i {
                    vel_i.x *= 1.0 - total_friction;
                    vel_i.y *= 1.0 - total_friction;

                    // apply impulse for
                    //if vel_i.x <
                    vel_i.x += x_imp * mass_frac_i;
                    vel_i.y += y_imp * mass_frac_i;
                    // vel_i.x = 0.0;
                    // vel_i.y = 0.0;
                }

                // If J has velocity to update, apply impulse
                if let Some(vel_j) = svel_j {
                    vel_j.x *= 1.0 - total_friction;
                    vel_j.y *= 1.0 - total_friction;

                    vel_j.x -= x_imp * mass_frac_j;
                    vel_j.y -= y_imp * mass_frac_j;
                    // vel_j.x = 0.0;
                    // vel_j.y = 0.0;
                }
                //}
            }
            // else {
            //     imp = 0.0;
            // }
            
            //println!("Impulse dist: {}, frac: {}, imp: {}", &dist, &frac, &imp);
        }
    }
}


// Actors push at each others boundaries based on two square shapes
pub fn actors_push_squares(pos_i: &na::Point2<f32>, pos_j: &na::Point2<f32>, 
    svel_i : Option<&mut na::Vector2<f32>>, svel_j : Option<&mut na::Vector2<f32>>,
    mass_i: f32, mass_j: f32, friction_i: f32, friction_j: f32, combined_obj_radius: f32) {
    //let impulse : f32 = 220.0;
    let touch_dist : f32 = combined_obj_radius;
    //let friction_ratio : f32 = 0.90;
    //let pt = na::Point2::new(pos_i.x,pos_i.y);
    //let (check, dist, pos_i, pos_j) = self::rect_check_points(&pos_i, &pos_j, &[touch_dist/2.0,touch_dist/2.0], &[touch_dist/2.0,touch_dist/2.0]);
    let (check, dist, pos_i, pos_j) = self::radius_square_check_points(&pos_i, &pos_j, touch_dist);
    if check {
        let mut imp = 0.0;
        let overlap_len = -dist;
        println!("check, dist {}, p1 {:?}, p2 {:?}, m1 = {}, m2 = {}", &dist, &pos_i, &pos_j, &mass_i, &mass_j);
        if overlap_len > 0.0 {
            let x_dif = (pos_i.x - pos_j.x) / overlap_len;
            let y_dif = (pos_i.y - pos_j.y)  / overlap_len;
            println!("overlap len: {}, touch dist: {}, dist: {}, xdif: {}, ydif: {}", &overlap_len, &touch_dist, &dist
                ,&x_dif, &y_dif);
            //imp = overlap_;
            let overlap_ratio = overlap_len / (touch_dist / 2.0);
            imp = overlap_len * overlap_ratio * 3.0; //overlap_ratio * imp;
            //imp = imp * 2.0;
            let x_imp = imp * x_dif;
            let y_imp = imp * y_dif;
            //let inv_len = (touch_dist - dist).max(0.0).min(51.0);
            //let frac = 1.0 / (30.0 * dist); //inv_len / 51.0;
            if imp > 0.0 {
                //imp *= 1.0 - frac;

                // Check velocity status of two entities
                let no_i = match svel_i {
                    Some(_) => true,
                    _ => false
                };
                let no_j = match svel_j {
                    Some(_) => true,
                    _ => false
                };
                // Default to full strength impulses from mass
                let mut mass_frac_i = 0.5;
                let mut mass_frac_j = 0.5;
                // Double impulse per object if one doesn't have velocity
                if no_i || no_j {
                    mass_frac_i *= 2.0;
                    mass_frac_j *= 2.0;
                }

                if !no_i && !no_j {
                    mass_frac_i = mass_j / (mass_i + mass_j);
                    mass_frac_j = mass_i / (mass_i + mass_j);
                }

                let total_friction = 0.0;// (friction_i + friction_j);

                // if mass_i > 0.0 && mass_j > 0.0 {
                //     // having masses for both items, calc fraction of mass
                //     // for I and J, to multiply impulse by
                //     // frac is the magnitude of mass applied by the other object
                //     // i's mass frac is j's mass - J's mass applies in the force on I
                //     // j's mass frac is i's mass - I's mass applies in the force on J
                // }

                let i_x_imp_final = x_imp * mass_frac_i;
                let i_y_imp_final = y_imp * mass_frac_i;
                let j_x_imp_final = x_imp * mass_frac_j;
                let j_y_imp_final = y_imp * mass_frac_j;

                if !no_i || !no_j {
                    println!("Impulse: {}, friction: {}, mass_frac_i: {}, mfj: {}, ({},{}), ({},{})", 
                    &imp, &total_friction, &mass_frac_i, &mass_frac_j,
                    &i_x_imp_final, &i_y_imp_final, &j_x_imp_final, &j_y_imp_final);
                }
                

                // If I has velocity to update, apply impulse
                if let Some(vel_i) = svel_i {
                    vel_i.x *= 1.0 - total_friction;
                    vel_i.y *= 1.0 - total_friction;

                    // apply impulse for
                    //if vel_i.x <
                    vel_i.x += i_x_imp_final;
                    vel_i.y += i_y_imp_final;
                    println!("Final i vel: {}, {}", &vel_i.x, &vel_i.y);
                    // vel_i.x = 0.0;
                    // vel_i.y = 0.0;
                }

                // If J has velocity to update, apply impulse
                if let Some(vel_j) = svel_j {
                    vel_j.x *= 1.0 - total_friction;
                    vel_j.y *= 1.0 - total_friction;

                    vel_j.x -= j_x_imp_final;
                    vel_j.y -= j_y_imp_final;
                    println!("Final j vel: {}, {}", &vel_j.x, &vel_j.y);
                    // vel_j.x = 0.0;
                    // vel_j.y = 0.0;
                }
                //}
            }
            // else {
            //     imp = 0.0;
            // }
            
            //println!("Impulse dist: {}, frac: {}, imp: {}", &dist, &frac, &imp);
        }
    }
}


