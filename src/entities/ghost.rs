use ggez::{Context};
use specs::{Builder,Entity,World,WorldExt};
use wrapped2d::user_data::*;

use crate::components::sprite::*;
use crate::components::anim_sprite::*;
use crate::components::{Position,RenderFlag,RenderLayerType};
//use crate::components::ball::*;
use crate::components::collision::{Collision};
use crate::components::npc::{NpcComponent};
use crate::resources::{ImageResources};
use crate::core::physics::{PhysicsWorld,CollisionCategory,EntityType};


pub struct GhostBuilder;

impl GhostBuilder {

    pub fn build_collider(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, x: f32, y: f32, vx: f32, vy: f32, m: f32, fric: f32, dim_1: f32, dim_2: f32) -> Entity {

        //Self::init_images(world, ctx);

        let mut sprite = AnimSpriteConfig::create_from_config(world, ctx, "entities/ghost_anim".to_string());
        sprite.z_order = SpriteLayer::Entities.to_z();

        //let mut sprite = AnimSpriteConfig::create_from_config(world, ctx, "entities/lemming".to_string());
        //sprite.z_order = SpriteLayer::Entities.to_z();

        let npc = NpcComponent::new();


        let mut collision = Collision::new_specs(1.0,0.5, dim_1, dim_2);
        //collision.dim_1 = dim_1;
        //collision.dim_2 = dim_2;
        collision.density = 0.05;
        collision.pos.x = x;
        collision.pos.y = y;
        collision.vel.x = vx;
        collision.vel.y = vy;
        collision.enable_warp = true;
        collision.entity_type = EntityType::Ghost;
        collision.collision_category = CollisionCategory::Etherial;
        collision.collision_mask.clear();
        collision.collision_mask.push(CollisionCategory::Level);
        collision.collision_mask.push(CollisionCategory::Player);
        collision.collision_mask.push(CollisionCategory::Etherial);
        collision.collision_mask.push(CollisionCategory::Portal);
        collision.collision_mask.push(CollisionCategory::Sound);
        collision.create_dynamic_body_circle_fixed(physics_world);
        // Set slight upward gravity scale (reversing and 1/10 power)
        collision.set_gravity_scale(physics_world, -0.1);

        let body_handle_clone = collision.body_handle.clone();

        let entity = world.create_entity()
        .with(npc)
        .with(Position { x: x, y: y })
        //.with(Velocity { x: vx, y: vy, gravity: true, frozen: false })
        //.with(DisplayComp { circle: true, display_type: DisplayCompType::DrawCircle })
        .with(sprite) // SpriteComponent::new(ctx, &"/ghost-1-r.png".to_string(), SpriteLayer::Entities.to_z()))
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