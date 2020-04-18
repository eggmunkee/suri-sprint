use ggez::{Context, GameResult, GameError};
use ggez::graphics;
use ggez::graphics::{Image};
use ggez::nalgebra as na;
use specs::{Builder,Entity,EntityBuilder,World,WorldExt};
use wrapped2d::user_data::*;

use crate::conf::*;
use crate::game_state::{GameState};
use crate::resources::{GameStateResource,ImageResources};
use crate::components::{Position};
use crate::components::sprite::{SpriteComponent,SpriteConfig,SpriteLayer};
use crate::components::collision::{Collision};
use crate::components::player::{CharacterDisplayComponent};
use crate::systems::*;
use crate::physics::{PhysicsWorld,CollisionCategory};

pub struct MouseBuilder;

impl MouseBuilder {

    // pub fn get_sprite_paths() -> Vec<String> {
    //     vec!["/dirty-box-1.png".to_string()]
    // }

    pub fn build(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, x: f32, y: f32,
        width: f32, height: f32, angle: f32, z_order: f32) -> Entity {

        // Create sprite from config
        let mut sprite = SpriteConfig::create_from_config(world, ctx, "entities/mouse".to_string());

        //let mut sprite = SpriteComponent::new(ctx, &"/dirty-box-1.png".to_string(), z_order);
        sprite.scale.x *= width / 24.0;
        sprite.scale.y *= height / 12.0;

        let avg_rad = (width + height) / 4.0;
        let mut collision = Collision::new_specs(1.0,0.75, avg_rad, avg_rad);
        // collision.dim_1 = width;
        // collision.dim_2 = height;
        collision.pos.x = x;
        collision.pos.y = y;
        collision.angle = angle;
        collision.collision_category = CollisionCategory::Level;
        collision.collision_mask.clear();
        collision.collision_mask.push(CollisionCategory::Level);
        collision.collision_mask.push(CollisionCategory::Player);
        collision.collision_mask.push(CollisionCategory::Etherial);
        collision.create_dynamic_body_circle(physics_world);

        let body_handle_clone = collision.body_handle.clone();

        let entity = world.create_entity()
        .with(Position { x: x, y: y })
        //.with(DisplayComp { circle: false, display_type: DisplayCompType::DrawSelf })
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
