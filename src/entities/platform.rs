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
use crate::components::npc::{NpcComponent};
use crate::components::player::{CharacterDisplayComponent};
use crate::components::logic::{LogicComponent,LogicOpType};
use crate::entities::level_builder::{ItemLogic};
use crate::systems::*;
use crate::core::physics::{PhysicsWorld,CollisionCategory,EntityType};

pub struct PlatformBuilder;

impl PlatformBuilder {

    // pub fn get_sprite_paths() -> Vec<String> {
    //     vec!["/dirty-box-1.png".to_string()]
    // }

    pub fn build(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, x: f32, y: f32,
        width: f32, height: f32, angle: f32, z_order: f32) -> Entity {
        
        PlatformBuilder::build_w_image(world, ctx, physics_world, x, y, width, height, angle, z_order, 
            "entities/box".to_string(), 48.0, 48.0)
    }

    pub fn build_w_logic(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, x: f32, y: f32,
        width: f32, height: f32, angle: f32, z_order: f32, logic: Option<ItemLogic>) -> Entity {
        
        PlatformBuilder::build_w_image_logic(world, ctx, physics_world, x, y, width, height, angle, z_order, 
            "entities/box".to_string(), 48.0, 48.0, logic)
    }

    pub fn build_w_image(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, x: f32, y: f32,
        width: f32, height: f32, angle: f32, z_order: f32, image: String, img_w: f32, img_h: f32) -> Entity {
        PlatformBuilder::build_w_image_logic(world, ctx, physics_world, x, y, width, height, angle, z_order, image, img_w, img_h, None)
    }

    pub fn build_w_image_logic(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, x: f32, y: f32,
            width: f32, height: f32, angle: f32, z_order: f32, image: String, img_w: f32, img_h: f32, logic: Option<ItemLogic>) -> Entity {

        // Create sprite from config
        let mut sprite = SpriteConfig::create_from_config(world, ctx, image);
        if let Some(_) = logic {
            sprite.toggleable = true;
        }

        let half_img_w = img_w / 2.0;
        let half_img_h = img_h / 2.0;

        sprite.z_order = z_order;
        //sprite.rotation = angle;
        sprite.scale.x *= width / half_img_w;
        sprite.scale.y *= height / half_img_h;

        let mut collision = Collision::new_specs(5.0,0.02, width, height);
        // collision.dim_1 = width;
        // collision.dim_2 = height;
        collision.pos.x = x;
        collision.pos.y = y;
        collision.angle = angle;
        collision.entity_type = EntityType::Platform;
        collision.collision_category = CollisionCategory::Level;
        collision.collision_mask.clear();
        collision.collision_mask.push(CollisionCategory::Level);
        collision.collision_mask.push(CollisionCategory::Player);
        collision.collision_mask.push(CollisionCategory::Etherial);

        if let Some(_) = logic {
            collision.toggleable = true;
        }

        collision.create_static_body_box(physics_world);
        let body_handle_clone = collision.body_handle.clone();

        let mut entity_build = world.create_entity()
        .with(Position { x: x, y: y })
        //.with(DisplayComp { circle: false, display_type: DisplayCompType::DrawSelf })
        .with(sprite)
        .with(collision);

        if let Some(item_logic) = logic {
            let mut logic = LogicComponent::new(item_logic.name.clone(), item_logic.start_enabled, item_logic.logic_op);
            //logic.
            if let Some(log_type) = item_logic.logic_type {
                logic.logic_type = log_type;
            }
            
            entity_build = entity_build.with(logic);
        }

        let entity = entity_build.build();

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
        width: f32, height: f32, angle: f32, z_order: f32) -> Entity {

        PlatformBuilder::build_dynamic_w_image(world, ctx, physics_world, x, y, width, height, angle, z_order, 
            "entities/box".to_string(), 48.0, 48.0)
    }

    pub fn build_dynamic_w_image(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, x: f32, y: f32,
        width: f32, height: f32, angle: f32, z_order: f32, image: String, img_w: f32, img_h: f32) -> Entity {

        // Create sprite from config
        let mut sprite = SpriteConfig::create_from_config(world, ctx, image);

        let half_img_w = img_w / 2.0;
        let half_img_h = img_h / 2.0;

        sprite.z_order = z_order;
        //sprite.rotation = angle;
        sprite.scale.x *= width / half_img_w;
        sprite.scale.y *= height / half_img_h;

        //let npc = NpcComponent::new();

        let mut collision = Collision::new_specs(3.0,0.25, width, height);
        // collision.dim_1 = width;
        // collision.dim_2 = height;
        collision.pos.x = x;
        collision.pos.y = y;
        collision.density = 0.25;
        collision.angle = angle;
        collision.entity_type = EntityType::Platform;
        collision.collision_category = CollisionCategory::Level;
        collision.collision_mask.clear();
        collision.collision_mask.push(CollisionCategory::Level);
        collision.collision_mask.push(CollisionCategory::Player);
        collision.collision_mask.push(CollisionCategory::Portal);
        collision.collision_mask.push(CollisionCategory::Etherial);
        collision.create_dynamic_body_box_rotable(physics_world);

        let body_handle_clone = collision.body_handle.clone();

        let entity = world.create_entity()
        //.with(npc)
        .with(Position { x: x, y: y })
        //.with(Velocity { x: 0.0, y: 0.0, gravity: true, frozen: false  })
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
