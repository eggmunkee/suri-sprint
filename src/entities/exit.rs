use ggez::{Context, GameResult, GameError};
use ggez::graphics;
use ggez::graphics::{Image};
use ggez::nalgebra as na;
use specs::{Builder,Entity,Entities,EntityBuilder,World,WorldExt};
use wrapped2d::user_data::*;

use crate::game_state::{GameState};
use crate::resources::{GameStateResource};
use crate::components::{Position};
use crate::components::sprite::{SpriteLayer,SpriteConfig};
use crate::components::collision::{Collision};
use crate::components::exit::{ExitComponent};
use crate::physics::{PhysicsWorld,CollisionCategory};

pub struct ExitBuilder;

impl ExitBuilder {

    pub fn build(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, x: f32, y: f32,
        width: f32, height: f32, name: String, destination: String) -> Entity {

        ExitBuilder::build_w_image(world, ctx, physics_world, x, y, width, height, name, destination,
            "entities/exit".to_string(), 48.0, 48.0)
    }

    pub fn build_w_image(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, x: f32, y: f32,
        width: f32, height: f32, name: String, destination: String, image: String, img_w: f32, img_h: f32) -> Entity {

        let exit = ExitComponent::new(name, destination);

        let half_img_w = img_w / 2.0;
        let half_img_h = img_h / 2.0;

        let mut sprite = SpriteConfig::create_from_config(world, ctx, image);
        sprite.scale.x = width / half_img_w;
        sprite.scale.y = height / half_img_h;
        sprite.z_order = SpriteLayer::BGNear.to_z();

        let mut collision = Collision::new_specs(0.1,0.72, width * 0.7, height * 0.7);
        // collision.dim_1 = width;
        // collision.dim_2 = height;
        collision.pos.x = x;
        collision.pos.y = y;
        collision.vel.x = 0.0;
        collision.vel.y = 0.0;
        collision.is_sensor = true;
        collision.collision_category = CollisionCategory::Portal;
        collision.collision_mask.clear();
        collision.collision_mask.push(CollisionCategory::Level);
        collision.collision_mask.push(CollisionCategory::Etherial);
        collision.collision_mask.push(CollisionCategory::Player);
        //collision.create_kinematic_body_circle(physics_world);
        collision.create_static_body_circle(physics_world, true);

        let body_handle_clone = collision.body_handle.clone();

        let entity = world.create_entity()
        .with(exit)
        .with(Position { x: x, y: y })
        .with(sprite)
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
