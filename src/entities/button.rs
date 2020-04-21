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
use crate::components::button::{ButtonComponent,ButtonTriggerComponent};
use crate::components::sprite::{SpriteComponent,SpriteConfig,SpriteLayer,MultiSpriteComponent};
use crate::components::collision::{Collision};
use crate::components::npc::{NpcComponent};
use crate::components::player::{CharacterDisplayComponent};
use crate::systems::*;
use crate::physics::*;

pub struct ButtonBuilder;

impl ButtonBuilder {

    // pub fn get_sprite_paths() -> Vec<String> {
    //     vec!["/dirty-box-1.png".to_string()]
    // }

    pub fn build(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, x: f32, y: f32,
        width: f32, height: f32, angle: f32, name: String) -> (Entity, Entity) {

        // BUTTON ENTITY
        let mut button = ButtonComponent::new(name);
        
        // Create sprite from config
        let mut sprite = SpriteConfig::create_from_config(world, ctx, "entities/button-base".to_string());
        sprite.z_order = SpriteLayer::World.to_z() - 5.0;
        //sprite.rotation = angle;
        sprite.scale.x *= width / 24.0;
        sprite.scale.y *= height / 3.0;
        sprite.alpha = 0.95;

        let mut collision = Collision::new_specs(10.0,0.25, width, height);
        collision.pos.x = x;
        collision.pos.y = y;
        collision.angle = angle;
        collision.collision_category = CollisionCategory::Level;
        collision.collision_mask.clear();
        collision.collision_mask.push(CollisionCategory::Level);
        collision.collision_mask.push(CollisionCategory::Player);
        collision.collision_mask.push(CollisionCategory::Etherial);
        
        collision.body_handle = Some(Self::build_button_body(physics_world, &mut collision, 0.25, 0.4));

        let body_handle_clone = collision.body_handle.clone();

        let button_entity = world.create_entity()
        .with(button)
        .with(Position { x: x, y: y })
        .with(sprite)
        .with(collision)
        .build();

        let button_id = button_entity.id();
        if let Some(body_handle) = body_handle_clone {
            let mut collision_body = physics_world.body_mut(body_handle);
            let body_data = &mut *collision_body.user_data_mut();
            //let data = &*data_ref;
            body_data.entity_id = button_id;
        }

        // BUTTON TRIGGER ENTITY
        let mut button_trigger = ButtonTriggerComponent::new();
        // link trigger to button entity
        button_trigger.set_button(button_id as i32);

        // Create sprite from config
        let mut sprite = SpriteConfig::create_from_config(world, ctx, "entities/button-trigger".to_string());
        sprite.z_order = SpriteLayer::World.to_z() - 10.0;
        //sprite.rotation = angle;
        sprite.scale.x *= (width * 0.75) / 18.0;
        sprite.scale.y *= (height * 0.6) / 3.0;
        

        let mut collision = Collision::new_specs(10.0,0.25, width * 0.75, height * 0.6);
        collision.pos.x = x;
        collision.pos.y = y + height * 0.25;
        collision.angle = angle;
        collision.collision_category = CollisionCategory::Level;
        collision.collision_mask.clear();
        collision.collision_mask.push(CollisionCategory::Level);
        collision.collision_mask.push(CollisionCategory::Player);
        
        if let Some(body_handle) = body_handle_clone {
            let trigger_handle = Self::build_trigger_body(physics_world, &mut collision, 0.25, 0.4);
            
            let joint_handle = Self::build_joint(physics_world, &mut collision, 0.25, 0.4, height, &body_handle, &trigger_handle);
            
            collision.body_handle = Some(trigger_handle);
        }

        let body_handle_clone = collision.body_handle.clone();

        let trigger_entity = world.create_entity()
        .with(button_trigger)
        .with(Position { x: x, y: y })
        .with(sprite)
        .with(collision)
        .build();

        let trigger_id = trigger_entity.id();
        if let Some(body_handle) = body_handle_clone {
            let mut collision_body = physics_world.body_mut(body_handle);
            let body_data = &mut *collision_body.user_data_mut();
            //let data = &*data_ref;
            body_data.entity_id = trigger_id;
        }

        // link button to trigger entity
        //button.set_trigger(trigger_id as i32);

        (button_entity, trigger_entity)
    }

    pub fn build_button_body(world: &mut PhysicsWorld, collision: &mut Collision, density: f32, restitution: f32) 
            -> b2::BodyHandle {
        // create body - getting handle
        let fixed_rot = true;
        let b_handle = create_body(world, PhysicsBodyType::Static, &collision.pos, collision.angle, EntityType::Button, collision.collision_category, fixed_rot);
        
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

        let bottom_offset = na::Point2::new(0.0, 0.0); //collision.dim_2 * 0.2);
        let coll_h = create_size(collision.dim_2);
        let coll_w = create_size(collision.dim_1);
        let off_y = create_size(bottom_offset.y);

        let collision_shape_points : Vec::<b2::Vec2> = vec![
                // b2::Vec2 { x: 0.5 * coll_w, y: 0.5 * coll_h + off_y },
                // b2::Vec2 { x: -0.5 * coll_w, y: 0.5 * coll_h + off_y },
                // b2::Vec2 { x: -0.5 * coll_w, y: -0.5 * coll_h + off_y },
                // b2::Vec2 { x: 0.5 * coll_w, y: -0.5 * coll_h + off_y },

                b2::Vec2 { x: 0.65 * coll_w, y: -0.1 * coll_h },
                b2::Vec2 { x: 0.1 * coll_w, y: -coll_h },
                b2::Vec2 { x: -0.1 * coll_w, y: -coll_h },
                b2::Vec2 { x: -0.65 * coll_w, y: -0.1 * coll_h },
                //b2::Vec2 { x: -0.1 * coll_w, y: 0.5 * coll_h },
                b2::Vec2 { x: -1.0 * coll_w, y: coll_h },
                b2::Vec2 { x: 1.0 * coll_w, y: coll_h },
            ];

        // Bottom of box shape and fixture
        let bottom_shape = b2::PolygonShape::new_with(&collision_shape_points);
        //b2::PolygonShape::new_oriented_box(create_size(collision.dim_1), create_size(collision.dim_2),
            //&create_pos(&bottom_offset), 0.0); // offset and angle
        body.create_fixture(&bottom_shape, &mut fixture_def);

        b_handle
    }

    pub fn build_trigger_body(world: &mut PhysicsWorld, collision: &mut Collision, density: f32, restitution: f32) 
            -> b2::BodyHandle {

        // create body - getting handle
        let fixed_rot = true;
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

            let bottom_offset = na::Point2::new(0.0, 0.0);

            let coll_h = create_size(collision.dim_2);
            let coll_w = create_size(collision.dim_1);
            let off_y = create_size(bottom_offset.y);
            let collision_shape_points : Vec::<b2::Vec2> = vec![
                // b2::Vec2 { x: 0.5 * coll_w, y: 0.5 * coll_h + off_y },
                // b2::Vec2 { x: -0.5 * coll_w, y: 0.5 * coll_h + off_y },
                // b2::Vec2 { x: -0.5 * coll_w, y: -0.5 * coll_h + off_y },
                // b2::Vec2 { x: 0.5 * coll_w, y: -0.5 * coll_h + off_y },

                b2::Vec2 { x: 0.25 * coll_w, y: -0.2 * coll_h },
                b2::Vec2 { x: 0.05 * coll_w, y: -1.2 * coll_h },
                b2::Vec2 { x: -0.05 * coll_w, y: -1.2 * coll_h },
                b2::Vec2 { x: -0.25 * coll_w, y: -0.2 * coll_h },
                //b2::Vec2 { x: -0.1 * coll_w, y: 0.5 * coll_h },
                b2::Vec2 { x: -0.6 * coll_w, y: 1.1 * coll_h },
                b2::Vec2 { x: 0.6 * coll_w, y: 1.1 * coll_h },
            ];

            // Bottom of box shape and fixture
            let trigger_shape = b2::PolygonShape::new_with(&collision_shape_points);
                //create_size(collision.dim_1), create_size(collision.dim_2),
                //&create_pos(&bottom_offset), 0.0); // offset and angle
            body.create_fixture(&trigger_shape, &mut fixture_def);
        }

        b_handle
    }

    pub fn build_joint(world: &mut PhysicsWorld, collision: &mut Collision, density: f32, restitution: f32, height: f32,
        button_body_handle: &b2::BodyHandle, trigger_body_handle: &b2::BodyHandle) 
        -> b2::JointHandle {
        
        let joint_def = b2::PrismaticJointDef {
            enable_limit: true,
            lower_translation: create_size(height* 0.0),
            upper_translation: create_size(height*1.1),
            local_axis_a: b2::Vec2 { x: 0.0, y: -1.0 }, 
            enable_motor: true,
            max_motor_force: 5.0,
            motor_speed: 10.0,
            .. b2::PrismaticJointDef::new(button_body_handle.clone(), trigger_body_handle.clone())
        };
        let joint_handle = world.create_joint_with(&joint_def, ());

        joint_handle
    
    }

}

