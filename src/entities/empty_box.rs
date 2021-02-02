use ggez::{Context, GameResult, GameError};
use ggez::graphics;
use ggez::graphics::{Image};
use ggez::nalgebra as na;
use specs::{Builder,Entity,EntityBuilder,World,WorldExt};
use wrapped2d::user_data::*;
use wrapped2d::b2;

use crate::conf::*;
use crate::game_state::{GameState};
use crate::resources::{GameStateResource,ImageResources};
use crate::components::{Position};
use crate::components::sprite::{SpriteComponent,SpriteConfig,SpriteLayer,MultiSpriteComponent};
use crate::components::collision::{Collision};
use crate::components::npc::{NpcComponent};
use crate::components::player::{CharacterDisplayComponent};
use crate::systems::*;
use crate::core::physics::*;

pub struct BoxBuilder;

impl BoxBuilder {

    // pub fn get_sprite_paths() -> Vec<String> {
    //     vec!["/dirty-box-1.png".to_string()]
    // }

    pub fn build(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, x: f32, y: f32,
        width: f32, height: f32, angle: f32) -> Entity {

        // Create sprite from config
        let mut multi_sprite = MultiSpriteComponent::new(ctx);
        let mut sprite = SpriteConfig::create_from_config(world, ctx, "entities/empty_box_bg".to_string());
        sprite.z_order = SpriteLayer::PlayerBehind.to_z();
        //sprite.rotation = angle;
        sprite.scale.x *= width / 24.0;
        sprite.scale.y *= height / 24.0;
        multi_sprite.sprites.push(sprite);

        let mut sprite = SpriteConfig::create_from_config(world, ctx, "entities/empty_box_fg".to_string());
        sprite.z_order = SpriteLayer::PlayerFront.to_z();
        //sprite.rotation = angle;
        sprite.scale.x *= width / 24.0;
        sprite.scale.y *= height / 24.0;
        multi_sprite.sprites.push(sprite);

        let mut collision = Collision::new_specs(8.0,0.02, width, height);
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

        collision.create_static_body_box(physics_world);
        let body_handle_clone = collision.body_handle.clone();

        let entity = world.create_entity()
        .with(Position { x: x, y: y })
        //.with(DisplayComp { circle: false, display_type: DisplayCompType::DrawSelf })
        .with(multi_sprite)
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
        width: f32, height: f32, angle: f32, z_order: f32) -> Entity {

        // Create sprite from config
        let mut multi_sprite = MultiSpriteComponent::new(ctx);
        let mut sprite = SpriteConfig::create_from_config(world, ctx, "entities/empty_box_bg".to_string());
        sprite.z_order = SpriteLayer::BGNear.to_z() + 5.0;
        //sprite.rotation = angle;
        sprite.scale.x *= width / 24.0;
        sprite.scale.y *= height / 24.0;
        multi_sprite.sprites.push(sprite);

        let mut sprite = SpriteConfig::create_from_config(world, ctx, "entities/empty_box_fg".to_string());
        sprite.z_order = SpriteLayer::PlayerFront.to_z();
        //sprite.rotation = angle;
        sprite.scale.x *= width / 24.0;
        sprite.scale.y *= height / 24.0;
        multi_sprite.sprites.push(sprite);

        let npc = NpcComponent::new();

        let mut collision = Collision::new_specs(3.0,0.25, width, height);
        // collision.dim_1 = width;
        // collision.dim_2 = height;
        collision.pos.x = x;
        collision.pos.y = y;
        collision.density = 0.25;
        collision.angle = angle;
        collision.collision_category = CollisionCategory::Level;
        collision.collision_mask.clear();
        collision.collision_mask.push(CollisionCategory::Level);
        collision.collision_mask.push(CollisionCategory::Player);
        collision.collision_mask.push(CollisionCategory::Portal);
        collision.collision_mask.push(CollisionCategory::Etherial);
        
        collision.body_handle = Some(Self::build_box_body(physics_world, &collision.pos, angle, width, height, 0.25, 0.4,
            collision.collision_category.clone(),
            &collision.collision_mask, false));

        //collision.create_dynamic_body_box_rotable(physics_world);

        let body_handle_clone = collision.body_handle.clone();

        let entity = world.create_entity()
        .with(Position { x: x, y: y })
        //.with(Velocity { x: 0.0, y: 0.0, gravity: true, frozen: false  })
        //.with(DisplayComp { circle: false, display_type: DisplayCompType::DrawSelf })
        .with(multi_sprite)
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


    pub fn build_box_body(world: &mut PhysicsWorld, pos: &na::Point2<f32>, angle: f32, body_width: f32, body_height: f32,
        density: f32, restitution: f32,
        collision_category: CollisionCategory, collision_mask: &Vec<CollisionCategory>, fixed_rot: bool) 
            -> b2::BodyHandle {
        // let def = b2::BodyDef {
        //     body_type: PhysicsBodyType::Dynamic,
        //     position: self::create_pos(pos),
        //     linear_damping: 0.8,
        //     fixed_rotation: fixed_rot,
        //     .. b2::BodyDef::new()
        // };

        // let body_data = GameStateBodyData { entity_id: 0, collider_type: collision_category };

        // create body - getting handle
        let b_handle = create_body(world, PhysicsBodyType::Dynamic, pos, angle, EntityType::EmptyBox, collision_category, fixed_rot);

        //world.create_body_with(&def, body_data);
        // get mut ref to body
        let mut body = world.body_mut(b_handle);

        // Common fixture definition
        let mut fixture_def = b2::FixtureDef {
            density: density,
            restitution: restitution,
            filter: b2::Filter {
                category_bits: collision_category.to_bits(),
                mask_bits: collision_mask.to_bits(),
                group_index: 0,
            },
            .. b2::FixtureDef::new()
        };

        let left_side_offset = na::Point2::new(-0.8 * body_width, -0.1);
        let right_side_offset = na::Point2::new(0.8 * body_width, -0.1);
        let bottom_offset = na::Point2::new(0.0, 0.8 * body_height);

        // Left Side shape and fixture
        let left_side_shape = b2::PolygonShape::new_oriented_box(create_size(body_width * 0.10), create_size(body_height * 0.9),
            &create_pos(&left_side_offset), 0.0); // offset and angle
        body.create_fixture(&left_side_shape, &mut fixture_def);

        // Right Side shape and fixture
        let right_side_shape = b2::PolygonShape::new_oriented_box(create_size(body_width * 0.10), create_size(body_height * 0.9),
            &create_pos(&right_side_offset), 0.0); // offset and angle
        body.create_fixture(&right_side_shape, &mut fixture_def);

        // Bottom of box shape and fixture
        let bottom_shape = b2::PolygonShape::new_oriented_box(create_size(body_width * 0.9), create_size(body_height * 0.10),
            &create_pos(&bottom_offset), 0.0); // offset and angle
        body.create_fixture(&bottom_shape, &mut fixture_def);

        b_handle
    }

}
