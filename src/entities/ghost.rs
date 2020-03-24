use ggez::{Context};
use specs::{Builder,Entity,World,WorldExt};

use crate::components::sprite::*;
use crate::components::{Position, Velocity, DisplayComp, DisplayCompType};
use crate::components::ball::*;
use crate::components::collision::{Collision};
use crate::resources::{ImageResources};
use crate::physics::{PhysicsWorld,CollisionCategory};


pub struct GhostBuilder;

impl GhostBuilder {

    pub fn get_sprite_paths() -> Vec<String> {
        vec!["/ghost-1-r.png".to_string(), "/dirty-box-1.png".to_string()]
    }

    pub fn init_images(world: &mut World, ctx: &mut Context) {
        if let Some(mut images) = world.get_mut::<ImageResources>() {

            for path in Self::get_sprite_paths() {
                let has_image = images.has_image(path.clone());
                if (!has_image) {
                    images.load_image(path.clone(), ctx);
                }
            }
            
        }
    }

    pub fn build(world: &mut World, ctx: &mut Context, x: f32, y: f32, vx: f32, vy: f32) -> Entity {

        Self::init_images(world, ctx);

        world.create_entity()
        .with(Position { x: x, y: y })
        .with(Velocity { x: vx, y: vy, gravity: true, frozen: false })
        .with(DisplayComp { circle: true, display_type: DisplayCompType::DrawCircle })
        .with(SpriteComponent::new(ctx, &"/ghost-1-r.png".to_string(), SpriteLayer::Entities.to_z()))
        //.with(Collision::new_circle(20.0))
        .build()
    }

    pub fn build_static(world: &mut World, ctx: &mut Context, x: f32, y: f32) -> Entity {

        Self::init_images(world, ctx);

        world.create_entity()
        .with(Position { x: x, y: y })
        .with(Velocity { x: 0.0, y: 0.0, gravity: true, frozen: true })
        .with(DisplayComp { circle: true, display_type: DisplayCompType::DrawCircle })
        .with(SpriteComponent::new(ctx, &"/dirty-box-1.png".to_string(), SpriteLayer::Entities.to_z()))
        //.with(Collision::new_circle(20.0))
        .build()
    }

    pub fn build_collider(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, x: f32, y: f32, vx: f32, vy: f32, m: f32, fric: f32, dim_1: f32, dim_2: f32) -> Entity {

        Self::init_images(world, ctx);

        let mut collision = Collision::new_specs(1.0,0.25, dim_1, dim_2);
        //collision.dim_1 = dim_1;
        //collision.dim_2 = dim_2;
        collision.pos.x = x;
        collision.pos.y = y;
        collision.vel.x = vx;
        collision.vel.y = vy;
        collision.collision_category = CollisionCategory::Ghost;
        collision.collision_mask.clear();
        collision.collision_mask.push(CollisionCategory::Level);
        collision.collision_mask.push(CollisionCategory::Ghost);
        collision.create_dynamic_body_circle(physics_world);

        world.create_entity()
        .with(Position { x: x, y: y })
        .with(Velocity { x: vx, y: vy, gravity: true, frozen: false })
        .with(DisplayComp { circle: true, display_type: DisplayCompType::DrawCircle })
        .with(SpriteComponent::new(ctx, &"/ghost-1-r.png".to_string(), SpriteLayer::Entities.to_z()))
        .with(collision)
        .build()
    }

    pub fn build_static_collider(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, x: f32, y: f32, m: f32, fric: f32, dim_1: f32, dim_2: f32) -> Entity {

        Self::init_images(world, ctx);

        let mut collision = Collision::new_specs(5.0,0.25, dim_1, dim_2);
        //collision.dim_1 = dim_1;
        //collision.dim_2 = dim_2;
        collision.pos.x = x;
        collision.pos.y = y;
        collision.collision_category = CollisionCategory::Level;
        collision.collision_mask.clear();
        collision.collision_mask.push(CollisionCategory::Ghost);
        collision.collision_mask.push(CollisionCategory::Player);
        collision.create_static_body(physics_world);

        world.create_entity()
        .with(Position { x: x, y: y })
        .with(Velocity { x: 0.0, y: 0.0, gravity: true, frozen: true })
        .with(DisplayComp { circle: true, display_type: DisplayCompType::DrawCircle })
        .with(SpriteComponent::new(ctx, &"/ghost-1-r.png".to_string(), SpriteLayer::Entities.to_z()))
        .with(collision)       //Collision::new_circle(20.0))
        .build()
    }
}