use ggez::{Context, GameResult, GameError};
use ggez::graphics;
use ggez::graphics::{Image};
use ggez::nalgebra as na;
use specs::{Builder,Entity,Entities,EntityBuilder,World,WorldExt};
use wrapped2d::user_data::*;
use serde::{Deserialize,Serialize};

use crate::core::{GameState};
use crate::resources::{GameStateResource};
use crate::components::{Position};
//use crate::components::sprite::{SpriteLayer,SpriteConfig};
use crate::components::anim_sprite::{AnimSpriteConfig};
use crate::components::collision::{Collision};
use crate::components::meow::{MeowComponent};
use crate::components::logic::{ConnectionType};

use crate::components::sensor_area::{SensorAreaComponent};
use crate::systems::*;
use crate::core::physics::{PhysicsWorld,CollisionCategory,EntityType,PickupItemType};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum EffectAreaType {
    SwitchArea {
        switch_type: ConnectionType,
    },
    GravityArea {
        gravity_multiplier: f32,
    }
}

pub struct EffectAreaBuilder;

impl EffectAreaBuilder {
    pub fn build(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, x: f32, y: f32, z: f32,
        width: f32, height: f32, effect_area_type: EffectAreaType) -> Entity {

        let mut sensor_area = SensorAreaComponent::new();
        //pickup.sens = effect_area_type;

        // Create Collision component with Sensor flag - only detects without collide
        let mut collision = Collision::new_specs(0.1,0.72, width, height);
        // collision.dim_1 = width;
        // collision.dim_2 = height;
        collision.pos.x = x;
        collision.pos.y = y;
        collision.is_sensor = true;
        collision.entity_type = EntityType::SensorArea;
        collision.collision_category = CollisionCategory::Level;
        collision.collision_mask.clear();
        collision.collision_mask.push(CollisionCategory::Level);
        collision.collision_mask.push(CollisionCategory::Player);
        collision.create_static_body_circle(physics_world, true);

        let body_handle_clone = collision.body_handle.clone();

        let entity = world.create_entity()
        .with(sensor_area)
        .with(Position { x: x, y: y })
        //.with(DisplayComp { circle: false, display_type: DisplayCompType::DrawSelf })
        //.with(sprite)
        .with(collision)
        .build();

        let entity_id = entity.id();
        if let Some(body_handle) = body_handle_clone {
            let mut collision_body = physics_world.body_mut(body_handle);
            let body_data = &mut *collision_body.user_data_mut();
            //let data = &*data_ref;
            body_data.entity_id = entity_id;
        }

        entity
    }
}