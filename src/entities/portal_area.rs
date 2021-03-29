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
use crate::components::anim_sprite::{AnimSpriteConfig};
use crate::components::logic::{LogicComponent,LogicOpType};
use crate::entities::level_builder::{ItemLogic};
use crate::components::collision::{Collision};
use crate::components::portal::{PortalComponent};
use crate::core::physics::{PhysicsWorld,CollisionCategory,EntityType};

pub struct PortalBuilder;

impl PortalBuilder {
    pub fn build(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, x: f32, y: f32, z: f32,
        width: f32, name: String, destination: String, enabled: bool, logic_opt: Option<ItemLogic>) -> Entity {

        let mut portal = PortalComponent::new(name.clone(), destination);
        portal.is_enabled = enabled;

        // let mut sprite = SpriteConfig::create_from_config(world, ctx, "entities/portal".to_string());
        // sprite.scale.x = width / 24.0;
        // sprite.scale.y = width / 24.0;
        // sprite.z_order = z;

        let mut anim_name = "entities/portal-front-red".to_string();
        let mut sprite = AnimSpriteConfig::create_from_config(world, ctx, anim_name);
        sprite.scale.x = width / 18.0;
        sprite.scale.y = width / 18.0;
        sprite.z_order = z;

        let logic = LogicComponent::new_logic(name, enabled, logic_opt);

        let mut collision = Collision::new_specs(0.1,0.72, width * 0.5, width * 0.5);
        // collision.dim_1 = width;
        // collision.dim_2 = height;
        collision.pos.x = x;
        collision.pos.y = y;
        collision.vel.x = 0.0;
        collision.vel.y = 0.0;
        collision.is_sensor = true;
        collision.entity_type = EntityType::Portal;
        collision.collision_category = CollisionCategory::Portal;
        collision.collision_mask.clear();
        collision.collision_mask.push(CollisionCategory::Level);
        collision.collision_mask.push(CollisionCategory::Etherial);
        collision.collision_mask.push(CollisionCategory::Player);
        //collision.create_kinematic_body_circle(physics_world, true);
        collision.create_kinematic_body_circle(physics_world, false);

        let body_handle_clone = collision.body_handle.clone();

        let entity = world.create_entity()
        .with(portal)
        .with(logic)
        .with(Position { x: x, y: y })
        .with(sprite)
        .with(collision)
        .with(RenderFlag::from_layer(RenderLayerType::LevelLayer))
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


    pub fn build_side(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, x: f32, y: f32, z: f32, angle: f32,
        width: f32, height: f32, color: String, name: String, destination: String, enabled: bool, logic_opt: Option<ItemLogic>, norm_vector: (f32, f32)) -> Entity {

        //let height = width * 2.0;
        let act_width = width; // * 0.5;

        let mut portal = PortalComponent::new(name.clone(), destination);
        portal.is_enabled = enabled;
        portal.screen_facing = false;
        portal.normal.x = norm_vector.0;
        portal.normal.y = norm_vector.1;

        let mut anim_name = "entities/portal-side-".to_string();
        anim_name.push_str(&color);
        let mut sprite = AnimSpriteConfig::create_from_config(world, ctx, anim_name);
        sprite.scale.x = act_width / 24.0;
        sprite.scale.y = height / 48.0;
        sprite.angle = angle;
        sprite.z_order = z;

        let logic = LogicComponent::new_logic(name, enabled, logic_opt);

        let mut collision = Collision::new_specs(0.1,0.72, act_width * 0.35, height * 0.25);
        // collision.dim_1 = width;
        // collision.dim_2 = height;
        collision.pos.x = x;
        collision.pos.y = y;
        collision.vel.x = 0.0;
        collision.vel.y = 0.0;
        collision.angle = angle;
        collision.is_sensor = true;
        collision.collision_category = CollisionCategory::Portal;
        collision.collision_mask.clear();
        collision.collision_mask.push(CollisionCategory::Level);
        collision.collision_mask.push(CollisionCategory::Etherial);
        collision.collision_mask.push(CollisionCategory::Player);
        //collision.create_kinematic_body_circle(physics_world, true);
        collision.create_kinematic_body_box_upright(physics_world);

        let body_handle_clone = collision.body_handle.clone();

        let entity = world.create_entity()
        .with(portal)
        .with(logic)
        .with(Position { x: x, y: y })
        .with(sprite)
        .with(collision)
        .with(RenderFlag::from_layer(RenderLayerType::LevelLayer))
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
