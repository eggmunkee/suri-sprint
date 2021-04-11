
use ggez::nalgebra as na;
use ggez::{Context};
use na::{Point2,Vector2,distance_squared,distance};
use specs::{World, WorldExt, Entity, Builder};
use specs::Join;
use wrapped2d::b2;
use wrapped2d::user_data::*;
use rand::prelude::*;

use crate::core::physics::*;
use crate::resources::{GameStateResource,GameLog};
use crate::components::{Position,Velocity,RenderFlag,RenderLayerType,CharLevelInteractor,PhysicsUpdateTrait};
use crate::components::collision::{Collision};
use crate::components::logic::{LogicComponent};
use crate::components::exit::{ExitComponent};
use crate::components::portal::{PortalComponent};
use crate::components::pickup::{PickupComponent};
use crate::components::button::{ButtonComponent};
use crate::components::particle_sys::{ParticleSysComponent,ParticleSysConfig};
use crate::components::player::{CharacterDisplayComponent};
use crate::components::npc::{NpcComponent};
use crate::entities::level_builder::{LevelType};
use crate::entities::point_pickup::{PickupBuilder};
use crate::entities::ghost::{GhostBuilder};


pub struct PhysicsSystem {}

impl PhysicsSystem {

    pub fn run_physics_update(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, delta_seconds: f32) {

        // Run Physics setup process - address any inputs to physics system
        Self::pre_advance_physics(world, physics_world, delta_seconds);

        //println!("Running physics engine... delta={}", delta_seconds);
        Self::advance_physics_system(world, ctx, physics_world, delta_seconds);

        // Run Physics post-run process - address any outputs of physics system to game world
        Self::post_advance_physics(world, physics_world, delta_seconds);

    }

    // fn call_pre_physics_hook(physics_world: &mut PhysicsWorld, delta_seconds: f32,
    //     physic_update_object: &mut dyn PhysicsUpdateTrait,
    //     opt_character: Option<&mut CharacterDisplayComponent>,
    //     opt_npc: Option<&mut NpcComponent>,
    //     game_state: &GameStateResource,
    //     entity: &Entity) {

    //     physic_update_object.pre_physics_update(physics_world, delta_seconds, opt_character, opt_npc, game_state, entity);

    // }

    // fn call_post_physics_hook(physics_world: &mut PhysicsWorld, delta_seconds: f32,
    //     physic_update_object: &mut dyn PhysicsUpdateTrait,
    //     opt_character: Option<&mut CharacterDisplayComponent>,
    //     opt_npc: Option<&mut NpcComponent>,
    //     game_state: &GameStateResource,
    //     entity: &Entity) {

    //     physic_update_object.post_physics_update(physics_world, delta_seconds, opt_character, opt_npc, game_state, entity);

    // }

    // Handle any component state which affects the physics - ex. player input applied forces
    //  
    fn pre_advance_physics(world: &mut World, physics_world: &mut PhysicsWorld, delta_seconds: f32) {
        //let state_reader = world.fetch::<GameStateResource>();
        let mut phys_writer = world.write_storage::<Collision>();
        let mut pos_writer = world.write_storage::<Position>();
        let mut char_writer = world.write_storage::<CharacterDisplayComponent>();
        let mut npc_writer = world.write_storage::<NpcComponent>();
        let mut button_writer = world.write_storage::<ButtonComponent>();
        let mut pickup_writer = world.write_storage::<PickupComponent>();
        let mut portal_writer = world.write_storage::<PortalComponent>();
        let entities = world.entities();

        //let level_bounds = &state_reader.level_bounds;
        //println!("Pre-advance-physics");

        // Make sure collision body has update itself from game loop
        for (mut collision, mut pos, mut character, mut npc, ent) in (&mut phys_writer, &mut pos_writer, (&mut char_writer).maybe(),(&mut npc_writer).maybe(), &entities).join() {
            
            // update collision body from character
            //Self::call_pre_physics_hook(physics_world, delta_seconds, collision, character, npc, &state_reader, &ent);
            collision.pre_physics_update(world, physics_world, delta_seconds, &mut None, &mut character, &mut npc, &ent);

            if let Some(ref mut char_comp) = character {
                char_comp.pre_physics_update(world, physics_world, delta_seconds, &mut Some(collision), &mut None, &mut npc, &ent);
            }
            
            if let Some(ref mut npc_comp) = npc {
                //npc_comp.pre_physics_update(physics_world, delta_seconds, &mut Some(collision), &mut character, &mut None, &state_reader, &ent);
            }

        }

        for (mut collision, mut pos, mut button, ent) in (&mut phys_writer, &mut pos_writer, &mut button_writer, &entities).join() {
            //button.pre_physics_update(physics_world, delta_seconds, &mut Some(collision), &mut None, &mut None, &state_reader, &ent);
        }

        // let mut particle_writer = world.write_storage::<ParticleSysComponent>();
        // for (mut collision, mut pos, mut particle_sys, ent) in (&mut phys_writer, &mut pos_writer, &mut particle_writer, &entities).join() {
        //     //particle_sys.pre_physics_update(physics_world, delta_seconds, &mut Some(collision), &mut None, &mut None, &state_reader, &ent);
        // }
        // drop(particle_writer);

        for (mut collision, mut pos, mut pickup, ent) in (&mut phys_writer, &mut pos_writer, &mut pickup_writer, &entities).join() {
            //pickup.pre_physics_update(physics_world, delta_seconds, &mut Some(collision), &mut None, &mut None, &state_reader, &ent);
        }

        for (mut collision, mut pos, mut portal, ent) in (&mut phys_writer, &mut pos_writer, &mut portal_writer, &entities).join() {
            //portal.pre_physics_update(physics_world, delta_seconds, &mut Some(collision), &mut None, &mut None, &state_reader, &ent);
        }

    }

    // Handle physics changes by updating component state
    fn post_advance_physics(world: &mut World, physics_world: &mut PhysicsWorld, delta_seconds: f32) {
        //let state_reader = world.fetch::<GameStateResource>();
        let mut phys_writer = world.write_storage::<Collision>();
        let mut pos_writer = world.write_storage::<Position>();
        let mut char_writer = world.write_storage::<CharacterDisplayComponent>();
        let mut npc_writer = world.write_storage::<NpcComponent>();
        let logic_reader = world.read_storage::<LogicComponent>();
        let mut button_writer = world.write_storage::<ButtonComponent>();
        //let mut particle_writer = world.write_storage::<ParticleSysComponent>();
        let mut pickup_writer = world.write_storage::<PickupComponent>();
        let mut portal_writer = world.write_storage::<PortalComponent>();
        let entities = world.entities();

        // Update collision components after physics runs
        for (mut collision, mut pos,  mut character, mut npc, ent) in 
        (&mut phys_writer, &mut pos_writer, (&mut char_writer).maybe(),(&mut npc_writer).maybe(), &entities).join() {
            //collision.post_physics_hook(physics_world, delta_seconds, character, npc, &state_reader);

            //Self::call_post_physics_hook(physics_world, delta_seconds, collision, character, npc, &state_reader, &ent);
            collision.post_physics_update(world, physics_world, delta_seconds, &mut None, &mut character, &mut npc, &ent);
            // update position from collision position
            pos.post_physics_update(world, physics_world, delta_seconds, &mut Some(collision), &mut character, &mut npc, &ent);

            if let Some(ref mut char_comp) = character {
                char_comp.post_physics_update(world, physics_world, delta_seconds, &mut Some(collision), &mut None, &mut npc, &ent);
            }            
        }

        for (mut collision, logic, ent) in (&mut phys_writer, &logic_reader, &entities).join() {
            
            // check for logic value
            let active = logic.value;
            //collision.update_body_obstruction(physics_world, active);
            collision.set_obstructing(active);
        }

        // Update collision components after physics runs
        for (mut character, mut collision, mut pos, ent) in
            (&mut char_writer, &mut phys_writer, &mut pos_writer, &entities).join() {
            
            for (id, contact_type) in collision.body_contacts.iter() {

            }
        }

        for (mut collision, mut pos, mut button, ent) in (&mut phys_writer, &mut pos_writer, &mut button_writer, &entities).join() {
            //button.post_physics_update(physics_world, delta_seconds, &mut Some(collision), &mut None, &mut None, &state_reader, &ent);
        }

        // let mut particle_writer = world.write_storage::<ParticleSysComponent>();
        // for (mut collision, mut pos, mut particle_sys, ent) in (&mut phys_writer, &mut pos_writer, &mut particle_writer, &entities).join() {
        //     //particle_sys.post_physics_update(physics_world, delta_seconds, &mut Some(collision), &mut None, &mut None, &state_reader, &ent);
        // }
        // drop(particle_writer);

        for (mut collision, mut pos, mut pickup, ent) in (&mut phys_writer, &mut pos_writer, &mut pickup_writer, &entities).join() {
            //pickup.post_physics_update(physics_world, delta_seconds, &mut Some(collision), &mut None, &mut None, &state_reader, &ent);
        }

        for (mut collision, mut pos, mut portal, ent) in (&mut phys_writer, &mut pos_writer, &mut portal_writer, &entities).join() {
            //portal.post_physics_update(physics_world, delta_seconds, &mut Some(collision), &mut None, &mut None, &state_reader, &ent);
        }
    }    

    pub fn handle_contact(coll_type_1: &CollisionCategory, coll_type_2: &CollisionCategory,
        ent_type_1: &EntityType, ent_type_2: &EntityType) -> Option<CollideType> {
        // flip order if player id #2
        // if coll_type_1 != &CollisionCategory::Player && coll_type_2 == &CollisionCategory::Player {
        //     return handle_contact(coll_type_2, coll_type_1);
        // }

        // ENTITY TYPE DETERMINED CONTACTS
        match ent_type_2 {
            EntityType::SensorArea => {
                match coll_type_1 {
                    CollisionCategory::Player | CollisionCategory::Etherial | CollisionCategory::Level => {
                        return Some(CollideType::Collider_Sensor);
                    },
                    _ => {}
                }                
            },
            EntityType::PickupItem(_) => {
                match coll_type_1 {
                    CollisionCategory::Player => {
                        return Some(CollideType::Player_Pickup);
                    },
                    _ => {}
                }
            },
            _ => {}
        }

        match coll_type_1 {
            CollisionCategory::Player | CollisionCategory::Etherial | CollisionCategory::Level => {
                
            },
            _ => {}
        }

        // COLLISION CATEGORY DETERMINED CONTACTS OTHERWISE
        match coll_type_1 {
            CollisionCategory::Player => match coll_type_2 {
                CollisionCategory::Etherial => Some(CollideType::Player_Ghost),
                CollisionCategory::Portal => Some(CollideType::Collider_Portal),
                CollisionCategory::Level | CollisionCategory::Player
                    => Some(CollideType::Collider_Collider),
                _ => None
            },
            CollisionCategory::Etherial => match coll_type_2 {
                CollisionCategory::Player => Some(CollideType::Player_Ghost),
                CollisionCategory::Portal => Some(CollideType::Collider_Portal),
                CollisionCategory::Sound => Some(CollideType::Ghost_Meow),
                CollisionCategory::Level | CollisionCategory::Etherial => Some(CollideType::Collider_Collider),
                _ => None,
            },
            CollisionCategory::Level => match coll_type_2 {
                CollisionCategory::Portal => Some(CollideType::Collider_Portal),
                CollisionCategory::Level | CollisionCategory::Etherial | CollisionCategory::Player
                    => Some(CollideType::Collider_Collider),
                // CollisionCategory::Player => match ent_type_1 {
                //     EntityType::PickupItem(_) => Some(CollideType::Player_Pickup),
                //     _ => Some(CollideType::Collider_Collider),
                // },
                _ => None,
            },
            CollisionCategory::Portal => match coll_type_2 {
                CollisionCategory::Player | CollisionCategory::Etherial | CollisionCategory::Level
                    => Some(CollideType::Collider_Portal),
                _ => None,
            },
            CollisionCategory::Sound => match coll_type_2 {
                //CollisionCategory::Etherial => Some(CollideType::Ghost_Meow),
                //CollisionCategory::Level => Some(CollideType::Meow_Level),
                _ => None,
            },
            
            _ => None
        }
    }

    pub fn set_standing_status(interactor: &mut dyn CharLevelInteractor, is_standing: bool) {
        interactor.set_standing(is_standing);
    }

    pub fn advance_physics_system(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, delta_seconds: f32) {

        let mut rng = rand::thread_rng();

        let mut lvl_type : LevelType = LevelType::default();
        {
            let game_state_res = world.fetch::<GameStateResource>();
            lvl_type = game_state_res.level_type.clone();
        }

        // update the physics world
        physics_world.step(delta_seconds, 5, 5);

        // Keep list of collider entities that need to be destroyed
        let mut delete_entity_list : Vec::<u32> = Vec::new();

        let mut wake_body_list : Vec::<PhysicsBodyHandle> = Vec::new();



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
            let primary_entity_type = body_data.entity_type;
            let primary_collider_type = body_data.collider_type;

            // get world entity id
            let primary_id = body_data.entity_id;
            // get world entity
            let entity_1 = world.entities().entity(primary_id);

            // Get world data writers - Collision, Character, etc.
            let mut coll_res = world.write_storage::<Collision>();
            let mut char_disp_comp_res = world.write_storage::<CharacterDisplayComponent>();
            let mut npc_comp_res = world.write_storage::<NpcComponent>();
            let mut pickup_res = world.write_storage::<PickupComponent>();

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

                // Only consider touching contacts
                if contact.is_touching() == false { continue; }

                let contact_flipped = {
                    let (bh_a, fix_a) = contact.fixture_a();
                    bh_a != body_handle
                };

                let dot = get_contact_floor_dot(&contact, contact_flipped);

                let other_body = physics_world.body(other_body_handle);
                let other_body_data = &*other_body.user_data();
                    
                //let b = other_meta_body.body;
                //let other_body_data = (21,); //other_meta_body.();
                //let otherbody = &mut *other_meta_body;
                let other_id = other_body_data.entity_id;
                let other_entity_type = other_body_data.entity_type;
                let other_collider_type = other_body_data.collider_type;

                if primary_collider_type == other_collider_type && primary_collider_type == CollisionCategory::Player {
                    //let dot = get_contact_floor_dot(&contact);
                    let (bh_a, fix_a) = contact.fixture_a();
                    let (bh_b, fix_b) = contact.fixture_b();
                    //println!("Player-player From: {:?} ({:?}) To: {:?} ({:?}) Last=Last={}", &bh_a, &body_handle, &bh_b, &other_body_handle,
                    //    (bh_b == other_body_handle));
                    //debug_contact_floor_dot(&contact, contact_flipped);
                }

                // Handle entity 
                let entity_2 = world.entities().entity(other_id);

                let collide_type = Self::handle_contact(&primary_collider_type, &other_collider_type, &primary_entity_type, &other_entity_type);

                // Handle contact collide type info
                if let Some(collide_t) = &collide_type {

                    // HANDLE SPECIAL COLLIDE TYPES HERE IF NEEDED
                    // Handle ghost meow collide
                    if collide_t == &CollideType::Ghost_Meow {
                        if primary_collider_type == CollisionCategory::Etherial {
                            //delete_entity_list.push(primary_id);
                            if let Some(collision) = coll_res.get_mut(entity_1) {
                                collision.delete_flag = true;
                            }
                        }
                        else {
                            //delete_entity_list.push(other_id);
                            // if let Some(collision) = coll_res.get_mut(entity_2) {
                            //     collision.delete_flag = true;
                            // }
                        }
                    }                        
                    else if collide_t == &CollideType::Collider_Portal {
                        // DEAL WITH IF FIRST ITEM IS THE COLLIDER
                        match primary_collider_type {
                            CollisionCategory::Etherial | CollisionCategory::Player 
                            | CollisionCategory::Level => {
                                if let Some(collision) = coll_res.get_mut(entity_1) {
                                    //println!("PORTAL COLLISION - PRIMARY - {:?} ===============================================", &entity_1);
                                    //debug_contact_floor_dot(&contact, contact_flipped);
                                    let mut portal_enabled = false;
                                    let portal_res = world.read_storage::<PortalComponent>();
                                    if let Some(portal) = portal_res.get(entity_2) {
                                        portal_enabled = portal.is_enabled;
                                    }
                                    if portal_enabled {
                                        let portal_id = other_id as i32;
                                        
                                        collision.in_portal = true;
                                        collision.portal_id = portal_id;
                                    }

                                    if body.is_awake() == false {
                                        wake_body_list.push(body_handle);
                                    }
                                }
                            },
                            _ => {}
                        }
                        // DEAL WITH IF SCEOND ITEM IS THE COLLIDER
                        match other_collider_type {
                            CollisionCategory::Etherial | CollisionCategory::Player
                            | CollisionCategory::Level => {
                                if let Some(collision) = coll_res.get_mut(entity_2) {
                                    //println!("PORTAL COLLISION - SECONDARY - {:?} ===============================================", &entity_2);
                                    //debug_contact_floor_dot(&contact, contact_flipped);

                                    let mut portal_enabled = false;
                                    let portal_res = world.read_storage::<PortalComponent>();
                                    if let Some(portal) = portal_res.get(entity_1) {
                                        portal_enabled = portal.is_enabled;
                                    }
                                    if portal_enabled {
                                        let portal_id = primary_id as i32;

                                        collision.in_portal = true;
                                        collision.portal_id = portal_id;

                                    }

                                    if other_body.is_awake() == false {
                                        wake_body_list.push(other_body_handle);
                                    }
                                }
                            },
                            _ => {}
                        }

                    }
                    else if collide_t == &CollideType::Player_Pickup {
                        match primary_entity_type {
                            EntityType::PickupItem(_) => {
                                /*
                                let mut points_speed_lvl = 0;
                                // Get point item collision component
                                if let Some(pickup) = pickup_res.get_mut(entity_1) {
                                    if !pickup.picked_up {
                                        pickup.pickup();

                                        let mut game_state_res = world.fetch_mut::<GameStateResource>();
                                        game_state_res.points = game_state_res.points + 1;
                                        if game_state_res.points >= 100 {
                                            points_speed_lvl = 3;
                                        }
                                        else if game_state_res.points >= 25 {
                                            points_speed_lvl = 2;
                                        }
                                        else if game_state_res.points >= 1 {
                                            points_speed_lvl = 1;
                                        }
                                        {
                                            // let mut log = world.fetch_mut::<GameLog>();
                                            // log.add_entry(true, "Suri got a point.".to_string(), None, game_state_res.game_run_seconds);
                                        }
                                    }
                                }

                                if dot > 0.2 {
                                    any_stand_contact = true;
                                }

                                if let Some(player) = char_disp_comp_res.get_mut(entity_2) {
                                    if player.speed_level < points_speed_lvl {
                                        player.speed_level = points_speed_lvl;
                                        println!("PLAYER SPEED INCREASE ==== [{}] ==== ==== [{}] ==== ",
                                                    &points_speed_lvl, &points_speed_lvl);

                                        let mut log = world.fetch_mut::<GameLog>();
                                        log.add_entry(true, format!("SPEED BOOST LVL {}!", points_speed_lvl), None,
                                            world.fetch_mut::<GameStateResource>().game_run_seconds);
                                    }
                                }
                                */
                            },
                            _ => {
                                match other_entity_type {
                                    EntityType::PickupItem(_) => {
                                        let mut points_speed_lvl = 0;
                                        // Get point item collision component
                                        if let Some(pickup) = pickup_res.get_mut(entity_2) {
                                            if !pickup.picked_up {
                                                pickup.pickup();
                                                let mut game_state_res = world.fetch_mut::<GameStateResource>();
                                                game_state_res.points = game_state_res.points + 1;
                                                if game_state_res.points >= 100 {
                                                    points_speed_lvl = 3;
                                                }
                                                else if game_state_res.points >= 25 {
                                                    points_speed_lvl = 2;
                                                }
                                                else if game_state_res.points >= 1 {
                                                    points_speed_lvl = 1;
                                                }
                                                {
                                                    // let mut log = world.fetch_mut::<GameLog>();
                                                    // log.add_entry(true, "Suri got a point.".to_string(), None, game_state_res.game_run_seconds);
                                                }
                                            }
                                        }

                                        if dot > 0.2 {
                                            any_stand_contact = true;
                                        }

                                        if let Some(player) = char_disp_comp_res.get_mut(entity_1) {
                                            if player.speed_level < points_speed_lvl {
                                                player.speed_level = points_speed_lvl;
                                                println!("PLAYER SPEED INCREASE ==== [{}] ==== ==== [{}] ==== ",
                                                    &points_speed_lvl, &points_speed_lvl);
                                                
                                                let mut log = world.fetch_mut::<GameLog>();
                                                log.add_entry(true, format!("SPEED BOOST LVL {}!", points_speed_lvl), None,
                                                    world.fetch_mut::<GameStateResource>().game_run_seconds);
                                                
                                            }
                                        }
                                    },
                                    _ => {}
                                }                                
                            },
                            _ => {}
                        }
                    }
                    // generic physical contact - level-player, level-ghost, player-player
                    else if collide_t == &CollideType::Collider_Collider || collide_t == &CollideType::Player_Ghost {
                        if dot > 0.2 {
                            any_stand_contact = true;
                        }
                    }

                    // Add generic body contact to collider
                    if let Some(collision) = coll_res.get_mut(entity_1) {
                        collision.body_contacts.push((other_id as i32, collide_t.clone()));                           
                    }

                }

                // handle character exit =====================================================
                if let Some(character) = char_disp_comp_res.get_mut(entity_1) {
                    // If character is touching exit component, set exit flag and id
                    let exit_res = world.read_storage::<ExitComponent>();
                    if let Some(_) = exit_res.get(entity_2) {
                        //exit_id = other_id as i32;
                        character.in_exit = true;
                        character.exit_id = other_id as i32;
                    }

                }

            }

            // If entity
            if let Some(character) = char_disp_comp_res.get_mut(entity_1) {
                //character.set_standing(any_stand_contact);
                match lvl_type {
                    LevelType::Platformer | LevelType::Space => {
                        Self::set_standing_status(character, any_stand_contact);
                    },
                    LevelType::Overhead => {
                        Self::set_standing_status(character, true);
                    }
                }
            }
            else if let Some(npc) = npc_comp_res.get_mut(entity_1) {
                //npc.set_standing(any_stand_contact);
                //set_standing_status(npc, any_stand_contact);
                match lvl_type {
                    LevelType::Platformer | LevelType::Space => {
                        Self::set_standing_status(npc, any_stand_contact);
                    },
                    LevelType::Overhead => {
                        Self::set_standing_status(npc, true);
                    }
                }
            }


        }

        for &body_handle in &wake_body_list {
            let mut body = physics_world.body_mut(body_handle);
            body.set_awake(true);
        }


        // DELETE ENTITY/PHYSICS BODY system
        // Get read storage for all display components
        {
            let mut coll_res = world.write_storage::<Collision>();
            let entities = world.entities();
            for (coll, ent) in 
                (&mut coll_res, &entities).join() {
                if coll.delete_flag {
                    let delete_id = ent.id();
                    delete_entity_list.push(delete_id);
                }
            }
            drop(coll_res);
            //drop(entities);

            let mut particle_sys_res = world.write_storage::<ParticleSysComponent>();
            //let entities = world.entities();
            for (psys, ent) in 
                (&mut particle_sys_res, &entities).join() {
                if psys.delete_flag {                    
                    let delete_id = ent.id();
                    println!("Deleting particle system {:?}", &delete_id);
                    delete_entity_list.push(delete_id);
                }
            }
            drop(particle_sys_res);
            drop(entities);

            // Delete any entities on the list
            for &entity_id in &delete_entity_list {
                let entity = world.entities().entity(entity_id);

                // Call destroy body on any collision component of entity
                let mut spawn_coin = false;
                let mut spawn_ghost = false;
                let mut spawn_smoke = false;
                let mut spawn_energy_explosion = false;
                let mut coll_x = 0.0;
                let mut coll_y = 0.0;
                let mut coll_vx = 0.0;
                let mut coll_vy = 0.0;

                if entity.gen().is_alive() {



                    let mut collision_res = world.write_storage::<Collision>();
                    if let Some(collision) = collision_res.get_mut(entity) {

                        coll_x = collision.pos.x;
                        coll_y = collision.pos.y;
                        coll_vx = collision.vel.x * 30.0; // get_size(collision.vel.x);
                        coll_vy = collision.vel.y * 30.0; //get_size(collision.vel.y);

                        match collision.entity_type {
                            EntityType::PickupItem(PickupItemType::Point) => {
                                if rng.gen::<f32>() >= 0.6 {
                                    spawn_ghost = true;
                                    spawn_smoke = true;
                                }
                                else {
                                    spawn_energy_explosion = true;
                                }
                                
                            },
                            EntityType::Ghost => {
                                spawn_coin = true;
                                spawn_smoke = true;
                            },
                            _ => {}                            
                        }

                        // Destroy collision body
                        collision.destroy_body(physics_world);
                       
                    }

                    // Destroy world entity
                    if let Err(_) = world.entities().delete(entity) {
                        println!("Error deleting entity {}", &entity_id);
                    }
                }

                if spawn_coin {
                    PickupBuilder::build_dynamic(world, ctx, physics_world, coll_x, coll_y, 100.0, 12.0, 12.0, PickupItemType::Point);
                }

                if spawn_ghost {
                    GhostBuilder::build_collider(world, ctx, physics_world, coll_x, coll_y, coll_vx, coll_vy, 0.0, 0.0, 24.0, 24.0); 
                }

                if spawn_smoke {
                    let mut part_sys = ParticleSysConfig::create_from_config(world, ctx, "psys/smoke_2s".to_string(),
                        coll_x, coll_y, coll_vx, coll_vy, (0.0, 0.0));
                    // part_sys.world_offset.0 = coll_x;
                    // part_sys.world_offset.1 = coll_y;
                    part_sys.z_order = 100.0;

                    let velocity = Velocity {
                        x: coll_vx, y: coll_vy, frozen: false, gravity: true,
                    };

                    world.create_entity()
                        .with(part_sys)
                        .with(Position { x: coll_x, y: coll_y })
                        .with(velocity)
                        .with(RenderFlag::from_layer(RenderLayerType::LevelLayer))
                        .build();
                }

                if spawn_energy_explosion {
                    let mut part_sys = ParticleSysConfig::create_from_config(world, ctx, "psys/energy_explosion".to_string(),
                        coll_x, coll_y, coll_vx, coll_vy, (0.0, 0.0));
                    // part_sys.world_offset.0 = coll_x;
                    // part_sys.world_offset.1 = coll_y;
                    part_sys.z_order = 100.0;

                    let velocity = Velocity {
                        x: coll_vx, y: coll_vy, frozen: false, gravity: true,
                    };

                    world.create_entity()
                        .with(part_sys)
                        .with(Position { x: coll_x, y: coll_y })
                        .with(velocity)
                        .with(RenderFlag::from_layer(RenderLayerType::LevelLayer))
                        .build();
                }

            }
        }

        

    }

}
