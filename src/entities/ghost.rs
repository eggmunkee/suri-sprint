use ggez::{Context};
use specs::{Builder,Entity,World,WorldExt};

use crate::components::{Position, Velocity, DisplayComp, DisplayCompType};
use crate::components::ball::*;
use crate::components::collision::{Collision};
use crate::physics::{PhysicsWorld};


pub struct GhostBuilder;

impl GhostBuilder {
    pub fn build(world: &mut World, ctx: &mut Context, x: f32, y: f32, vx: f32, vy: f32) -> Entity {
        world.create_entity()
        .with(Position { x: x, y: y })
        .with(Velocity { x: vx, y: vy, gravity: true, frozen: false })
        .with(DisplayComp { circle: true, display_type: DisplayCompType::DrawCircle })
        .with(BallDisplayComponent::new(ctx, &"/ghost-1-r.png".to_string(), true))
        //.with(Collision::new_circle(20.0))
        .build()
    }

    pub fn build_static(world: &mut World, ctx: &mut Context, x: f32, y: f32) -> Entity {
        world.create_entity()
        .with(Position { x: x, y: y })
        .with(Velocity { x: 0.0, y: 0.0, gravity: true, frozen: true })
        .with(DisplayComp { circle: true, display_type: DisplayCompType::DrawCircle })
        .with(BallDisplayComponent::new(ctx, &"/dirty-box-1.png".to_string(), false))
        //.with(Collision::new_circle(20.0))
        .build()
    }

    pub fn build_collider(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, x: f32, y: f32, vx: f32, vy: f32, m: f32, fric: f32, dim_1: f32, dim_2: f32) -> Entity {

        let mut collision = Collision::new_specs(25.0,0.02);
        collision.dim_1 = dim_1;
        collision.dim_2 = dim_2;
        collision.pos.x = x;
        collision.pos.y = y;
        collision.vel.x = vx;
        collision.vel.y = vy;
        collision.create_dynamic_body(world, physics_world);

        world.create_entity()
        .with(Position { x: x, y: y })
        .with(Velocity { x: vx, y: vy, gravity: true, frozen: false })
        .with(DisplayComp { circle: true, display_type: DisplayCompType::DrawCircle })
        .with(BallDisplayComponent::new(ctx, &"/ghost-1-r.png".to_string(), true))
        .with(collision)
        .build()
    }

    pub fn build_static_collider(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, x: f32, y: f32, m: f32, fric: f32, dim_1: f32, dim_2: f32) -> Entity {
        let mut collision = Collision::new_specs(25.0,0.02);
        collision.dim_1 = dim_1;
        collision.dim_2 = dim_2;
        collision.pos.x = x;
        collision.pos.y = y;
        collision.create_static_body(world, physics_world);

        world.create_entity()
        .with(Position { x: x, y: y })
        .with(Velocity { x: 0.0, y: 0.0, gravity: true, frozen: true })
        .with(DisplayComp { circle: true, display_type: DisplayCompType::DrawCircle })
        .with(BallDisplayComponent::new(ctx, &"/dirty-box-1.png".to_string(), false))
        .with(collision)       //Collision::new_circle(20.0))
        .build()
    }
}