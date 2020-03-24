use ggez::{Context, GameResult, GameError};
use ggez::graphics;
use ggez::graphics::{Image};
use ggez::nalgebra as na;
use specs::{Builder,Entity,EntityBuilder,World,WorldExt};

use crate::game_state::{GameState};
use crate::resources::{GameStateResource};
use crate::components::{Position, Velocity,DisplayComp,DisplayCompType};
use crate::components::collision::{Collision};
use crate::components::ball::{BallDisplayComponent};
use crate::components::player::{PlayerComponent,CharacterDisplayComponent};
use crate::systems::*;
use crate::physics::{PhysicsWorld,CollisionCategory};

pub struct PlatformBuilder;

impl PlatformBuilder {
    pub fn build(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, x: f32, y: f32,
        width: f32, height: f32) -> Entity {


        let mut collision = Collision::new_specs(5.0,0.72, width, height);
        // collision.dim_1 = width;
        // collision.dim_2 = height;
        collision.pos.x = x;
        collision.pos.y = y;
        collision.collision_category = CollisionCategory::Level;
        collision.collision_mask.clear();
        collision.collision_mask.push(CollisionCategory::Player);
        collision.collision_mask.push(CollisionCategory::Ghost);

        collision.create_static_body(physics_world);

        let entity = world.create_entity()
        .with(Position { x: x, y: y })
        .with(DisplayComp { circle: false, display_type: DisplayCompType::DrawSelf })
        .with(BallDisplayComponent::new(ctx, &"/dirty-box-1.png".to_string(), false))
        .with(collision)
        .build();

        //let entId = entity.id();

        entity
    }

}
