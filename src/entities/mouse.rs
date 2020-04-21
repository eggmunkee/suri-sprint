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
use crate::components::sprite::{SpriteComponent,SpriteConfig,SpriteLayer};
use crate::components::collision::{Collision};
use crate::physics::*;

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

        //let avg_rad = (width + height) / 4.0;
        let mut collision = Collision::new_specs(1.0,0.75, width*0.5, height*0.5);
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
        //collision.create_dynamic_body_circle(physics_world);
        collision.body_handle = Some(Self::build_mouse_body(physics_world, &mut collision, 0.25, 0.4));

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

    pub fn build_mouse_body(world: &mut PhysicsWorld, collision: &mut Collision, density: f32, restitution: f32) 
            -> b2::BodyHandle {

        // create body - getting handle
        let fixed_rot = false;
        let b_handle = create_body(world, PhysicsBodyType::Dynamic, &collision.pos, collision.angle, EntityType::Button, collision.collision_category, fixed_rot);

        //world.create_body_with(&def, body_data);
        {
            // get mut ref to body
            let mut body = world.body_mut(b_handle);

            // Common fixture definition
            let mut fixture_def = b2::FixtureDef {
                density: density,
                restitution: restitution,
                filter: b2::Filter {
                    category_bits: collision.collision_category.to_bits(),
                    mask_bits: collision.collision_mask.to_bits(),
                    group_index: 0,
                },
                .. b2::FixtureDef::new()
            };

            let coll_h = create_size(collision.dim_2);
            let coll_w = create_size(collision.dim_1);
            let collision_shape_points : Vec::<b2::Vec2> = vec![

                b2::Vec2 { x: 0.2 * coll_w, y: -1.0 * coll_h },
                b2::Vec2 { x: -1.0 * coll_w, y: 0.0 },
                b2::Vec2 { x: 0.2 * coll_w, y: 1.0 * coll_h },
                b2::Vec2 { x: 1.0 * coll_w, y: 0.0 },



                // b2::Vec2 { x: 0.25 * coll_w, y: -0.2 * coll_h },
                // b2::Vec2 { x: 0.05 * coll_w, y: -1.2 * coll_h },
                // b2::Vec2 { x: -0.05 * coll_w, y: -1.2 * coll_h },
                // b2::Vec2 { x: -0.25 * coll_w, y: -0.2 * coll_h },
                // //b2::Vec2 { x: -0.1 * coll_w, y: 0.5 * coll_h },
                // b2::Vec2 { x: -0.6 * coll_w, y: 1.1 * coll_h },
                // b2::Vec2 { x: 0.6 * coll_w, y: 1.1 * coll_h },
            ];

            // Bottom of box shape and fixture
            let trigger_shape = b2::PolygonShape::new_with(&collision_shape_points);
                //create_size(collision.dim_1), create_size(collision.dim_2),
                //&create_pos(&bottom_offset), 0.0); // offset and angle
            body.create_fixture(&trigger_shape, &mut fixture_def);
        }

        b_handle
    }

}
