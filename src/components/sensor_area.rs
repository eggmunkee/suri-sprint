
use ggez::nalgebra as na;
use specs::{Component, DenseVecStorage, World, WorldExt, Entity};
use specs_derive::*;
use wrapped2d::b2;


use crate::components::{PhysicsUpdateTrait};
use crate::components::player::{CharacterDisplayComponent};
use crate::components::npc::{NpcComponent};
use crate::components::sprite::{SpriteComponent};
use crate::components::anim_sprite::{AnimSpriteComponent};
use crate::components::collision::{Collision};
use crate::components::logic::{ConnectionType};
use crate::entities::effect_area::{EffectAreaType};
use crate::core::physics;
use crate::core::physics::{PhysicsWorld,PickupItemType,CollideType};

#[derive(Debug)]
//#[storage(DenseVecStorage)]
pub struct SensorAreaComponent {
    //pub effect_type: EffectAreaType,
}

impl Component for SensorAreaComponent {
    type Storage = DenseVecStorage<Self>;
}

impl SensorAreaComponent {
    pub fn new() -> Self {
        Self {
            // effect_type: EffectAreaType::SwitchArea {
            //     switch_type: 
            // }.
        }
    }

    pub fn update(&mut self, time_delta: f32, collision: &mut Collision) {
        

        // if let Some(body_handle) = collision.body_handle {
        //     let body = physics_world.body(body_handle);

        //     for (entity_id, collide_type) in &collision.body_contacts {
        //         //println!("Body contact: {} {:?} Self Type: {:?}", &entity_id, &collide_type, &self.entity_type);

        //         // match &collide_type {
        //         //     CollideType::Player_Portal
        //         // }

                
        //     }

    }

}

impl PhysicsUpdateTrait for SensorAreaComponent {
    fn pre_physics_update(&mut self, world: &World, physics_world: &mut PhysicsWorld, time_delta: f32, 
        opt_collision: &mut Option<&mut Collision>,
        opt_character: &mut Option<&mut CharacterDisplayComponent>,
        opt_npc: &mut Option<&mut NpcComponent>,
        //level_bounds: &LevelBounds,
        //game_state: &GameStateResource,
        entity: &Entity) {

    }

    fn post_physics_update(&mut self, world: &World, physics_world: &mut PhysicsWorld, time_delta: f32, 
        opt_collision: &mut Option<&mut Collision>,
        opt_character: &mut Option<&mut CharacterDisplayComponent>,
        opt_npc: &mut Option<&mut NpcComponent>,
        //game_state: &GameStateResource,
        entity: &Entity) {

            if let Some(collision) = opt_collision {
                if let Some(body_handle) = collision.body_handle {
                    let body = physics_world.body(body_handle);

                    for (entity_id, collide_type) in &collision.body_contacts {
                        println!("Sensor contact: {} {:?} Self Type: {:?}", &entity_id, &collide_type, &collision.entity_type);
        
                        match &collide_type {
                            CollideType::Player_Pickup => {},
                            _ => {}
                        }
        
                        
                    }
                }
            }
            
                //     let body = physics_world.body(body_handle);
        
                //     for (entity_id, collide_type) in &collision.body_contacts {
                //         //println!("Body contact: {} {:?} Self Type: {:?}", &entity_id, &collide_type, &self.entity_type);
        
                //         // match &collide_type {
                //         //     CollideType::Player_Portal
                //         // }
        
                        
                //     }

    }
}

// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    world.register::<SensorAreaComponent>();
}