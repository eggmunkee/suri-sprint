use ggez::{Context, GameResult, GameError};
use ggez::graphics;
use ggez::graphics::{Image};
use ggez::nalgebra as na;
use specs::{Builder,Entity,EntityBuilder,World,WorldExt};
use wrapped2d::user_data::*;

use crate::conf::*;
use crate::game_state::{GameState};
use crate::resources::{GameStateResource,ImageResources};
use crate::components::{Position, Velocity,DisplayComp,DisplayCompType};
use crate::components::sprite::{SpriteComponent,SpriteConfig,SpriteLayer};
use crate::components::collision::{Collision};
use crate::components::ball::{BallDisplayComponent};
use crate::components::player::{CharacterDisplayComponent};
use crate::systems::*;
use crate::physics::{PhysicsWorld,CollisionCategory};

pub struct PlatformBuilder;

impl PlatformBuilder {

    // pub fn get_sprite_paths() -> Vec<String> {
    //     vec!["/dirty-box-1.png".to_string()]
    // }

    pub fn build(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, x: f32, y: f32,
        width: f32, height: f32, z_order: f32) -> Entity {

        // Create sprite from config
        let mut sprite = SpriteConfig::create_from_config(world, ctx, "entities/box".to_string());

        sprite.z_order = z_order;
        sprite.scale.x *= width / 24.0;
        sprite.scale.y *= height / 24.0;

        let mut collision = Collision::new_specs(5.0,0.02, width, height);
        // collision.dim_1 = width;
        // collision.dim_2 = height;
        collision.pos.x = x;
        collision.pos.y = y;
        collision.angle = 0.0;
        collision.collision_category = CollisionCategory::Level;
        collision.collision_mask.clear();
        collision.collision_mask.push(CollisionCategory::Level);
        collision.collision_mask.push(CollisionCategory::Player);
        collision.collision_mask.push(CollisionCategory::Ghost);

        collision.create_static_body_box(physics_world);
        let body_handle_clone = collision.body_handle.clone();

        let entity = world.create_entity()
        .with(Position { x: x, y: y })
        .with(DisplayComp { circle: false, display_type: DisplayCompType::DrawSelf })
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

    pub fn build_dynamic(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, x: f32, y: f32,
        width: f32, height: f32, z_order: f32) -> Entity {

        // Create sprite from config
        let mut sprite = SpriteConfig::create_from_config(world, ctx, "entities/box".to_string());

        //let mut sprite = SpriteComponent::new(ctx, &"/dirty-box-1.png".to_string(), z_order);
        sprite.scale.x *= width / 24.0;
        sprite.scale.y *= height / 24.0;

        let mut collision = Collision::new_specs(3.0,0.25, width, height);
        // collision.dim_1 = width;
        // collision.dim_2 = height;
        collision.pos.x = x;
        collision.pos.y = y;
        collision.collision_category = CollisionCategory::Level;
        collision.collision_mask.clear();
        collision.collision_mask.push(CollisionCategory::Level);
        collision.collision_mask.push(CollisionCategory::Player);
        collision.collision_mask.push(CollisionCategory::Ghost);
        collision.create_dynamic_body_box_rotable(physics_world);

        let body_handle_clone = collision.body_handle.clone();

        let entity = world.create_entity()
        .with(Position { x: x, y: y })
        .with(DisplayComp { circle: false, display_type: DisplayCompType::DrawSelf })
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
