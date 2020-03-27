use ggez::{Context, GameResult, GameError};
use ggez::graphics;
use ggez::graphics::{Image};
use ggez::nalgebra as na;
use specs::{Builder,Entity,Entities,EntityBuilder,World,WorldExt};
use wrapped2d::user_data::*;

use crate::game_state::{GameState};
use crate::resources::{GameStateResource};
use crate::components::{Position, Velocity,DisplayComp,DisplayCompType};
use crate::components::sprite::{SpriteLayer,SpriteConfig};
use crate::components::collision::{Collision};
use crate::components::ball::{BallDisplayComponent};
use crate::components::meow::{MeowComponent};
use crate::systems::*;
use crate::physics::{PhysicsWorld,CollisionCategory};

pub struct MeowBuilder;

impl MeowBuilder {
    pub fn build(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, x: f32, y: f32,
        vx: f32, vy: f32,
        width: f32, height: f32) -> Entity {

        let mut meow = MeowComponent::new();
        meow.meow_time = 0.4;
        // let vx = (vx * 2.0).min(5.0).max(-5.0);
        // let vy = (vy * 2.0).min(5.0).max(-5.0);

        // if vx == 0.0 && vy == 0.0 {
        //     vx = 1.5; vy = -0.5;
        // }
        // vx = 0.0;
        // vy = 0.0;

        let mut sprite = SpriteConfig::create_from_config(world, ctx, "entities/meow".to_string());
        sprite.scale.x = width / 24.0;
        sprite.scale.x = width / 24.0;
        sprite.z_order = SpriteLayer::PlayerBehind.to_z();

        let mut collision = Collision::new_specs(0.1,0.72, width, height);
        // collision.dim_1 = width;
        // collision.dim_2 = height;
        collision.pos.x = x;
        collision.pos.y = y;
        collision.vel.x = vx;
        collision.vel.y = vy;
        collision.collision_category = CollisionCategory::Meow;
        collision.collision_mask.clear();
        collision.collision_mask.push(CollisionCategory::Ghost);
        collision.create_kinematic_body_circle(physics_world, true);

        let body_handle_clone = collision.body_handle.clone();

        let entity = world.create_entity()
        .with(Position { x: x, y: y })
        //.with(DisplayComp { circle: false, display_type: DisplayCompType::DrawSelf })
        .with(sprite)
        .with(collision)
        .with(meow)
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

    // pub fn build_from_ent<'a>(entities: Entities<'a>, ctx: &mut Context, physics_world: &mut PhysicsWorld, x: f32, y: f32,
    //     width: f32, height: f32) -> Entity {


    //     let mut collision = Collision::new_specs(0.1,0.72, width, height);
    //     // collision.dim_1 = width;
    //     // collision.dim_2 = height;
    //     collision.pos.x = x;
    //     collision.pos.y = y;
    //     collision.collision_category = CollisionCategory::Meow;
    //     collision.collision_mask.clear();
    //     collision.collision_mask.push(CollisionCategory::Ghost);

    //     collision.create_dynamic_body_circle(physics_world);

    //     let entity = entities.build_entity()
    //     .with(Position { x: x, y: y })
    //     .with(DisplayComp { circle: false, display_type: DisplayCompType::DrawSelf })
    //     .with(BallDisplayComponent::new(ctx, &"/dirty-box-1.png".to_string(), false))
    //     .with(collision)
    //     .build();

    //     //let entId = entity.id();

    //     entity
    // }
}
