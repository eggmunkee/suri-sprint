use ggez::{Context, GameResult, GameError};
use ggez::graphics;
use ggez::graphics::{Image};
use ggez::nalgebra as na;
use specs::{Builder,Entity,Entities,EntityBuilder,World,WorldExt};
use wrapped2d::user_data::*;

use crate::core::{GameState};
use crate::resources::{GameStateResource};
use crate::components::{Position,RenderFlag,RenderLayerType};
use crate::components::sprite::{SpriteLayer,SpriteConfig};
use crate::components::collision::{Collision};
use crate::components::meow::{MeowComponent};
use crate::components::flags::{RenderSpriteFlag};
use crate::systems::*;
use crate::core::physics::{PhysicsWorld,CollisionCategory,EntityType};

pub struct MeowBuilder;

impl MeowBuilder {
    pub fn build(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, x: f32, y: f32,
        vx: f32, vy: f32,
        width: f32, height: f32) -> Entity {

        let mut meow = MeowComponent::new();
        meow.meow_time = 0.4;

        let mut sprite = SpriteConfig::create_from_config(world, ctx, "entities/meow".to_string());
        sprite.scale.x = width / 24.0;
        sprite.scale.y = height / 24.0;
        sprite.z_order = SpriteLayer::PlayerBehind.to_z();
        //sprite.shader = Some("meow_shader".to_string());

        let mut collision = Collision::new_specs(0.1,0.72, width, height);
        // collision.dim_1 = width;
        // collision.dim_2 = height;
        collision.pos.x = x;
        collision.pos.y = y;
        collision.vel.x = vx;
        collision.vel.y = vy;
        collision.is_sensor = true;
        collision.entity_type = EntityType::Meow;
        collision.collision_category = CollisionCategory::Sound;
        collision.collision_mask.clear();
        collision.collision_mask.push(CollisionCategory::Etherial);
        collision.create_kinematic_body_circle(physics_world, true);

        let body_handle_clone = collision.body_handle.clone();

        let entity = world.create_entity()
        .with(Position { x: x, y: y })
        //.with(DisplayComp { circle: false, display_type: DisplayCompType::DrawSelf })
        .with(sprite)
        .with(collision)
        .with(meow)
        .with(RenderFlag::from_layer(RenderLayerType::LevelLayer))
        .with(RenderSpriteFlag)
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
