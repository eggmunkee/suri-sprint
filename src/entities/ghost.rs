use ggez::{Context};
use specs::{Builder,Entity,World,WorldExt};

use crate::components::{Position, Velocity, DisplayComp, DisplayCompType};
use crate::components::ball::*;
use crate::components::collision::{Collision};

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
        .with(BallDisplayComponent::new(ctx, &"/ghost-1-r.png".to_string(), false))
        //.with(Collision::new_circle(20.0))
        .build()
    }

    pub fn build_collider(world: &mut World, ctx: &mut Context, x: f32, y: f32, vx: f32, vy: f32, m: f32, fric: f32) -> Entity {
        world.create_entity()
        .with(Position { x: x, y: y })
        .with(Velocity { x: vx, y: vy, gravity: true, frozen: false })
        .with(DisplayComp { circle: true, display_type: DisplayCompType::DrawCircle })
        .with(BallDisplayComponent::new(ctx, &"/ghost-1-r.png".to_string(), true))
        .with(Collision::new_specs(m,fric))
        .build()
    }

    pub fn build_static_collider(world: &mut World, ctx: &mut Context, x: f32, y: f32, m: f32, fric: f32) -> Entity {
        world.create_entity()
        .with(Position { x: x, y: y })
        .with(Velocity { x: 0.0, y: 0.0, gravity: true, frozen: true })
        .with(DisplayComp { circle: true, display_type: DisplayCompType::DrawCircle })
        .with(BallDisplayComponent::new(ctx, &"/ghost-1-r.png".to_string(), false))
        .with(Collision::new_specs(m,fric))       //Collision::new_circle(20.0))
        .build()
    }
}