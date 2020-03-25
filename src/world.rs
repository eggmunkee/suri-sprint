use ggez::{Context};
use specs::{World, WorldExt, Entity, Builder}; // Builder, Component, ReadStorage, System, VecStorage, RunNow
use specs::shred::{Dispatcher, DispatcherBuilder};
use rand::prelude::*;

use crate::resources::{add_resources,GameStateResource};
use crate::components::{Position, Velocity,register_components, DisplayComp, DisplayCompType}; // 
use crate::components::sprite::{SpriteLayer,SpriteConfig};
use crate::entities::platform::{PlatformBuilder};
use crate::entities::suri::{SuriBuilder};
use crate::entities::ghost::{GhostBuilder};
use crate::entities::ui::{UIBuilder};
use crate::systems::*;
use crate::systems::interactor::{InterActorSys};
use crate::physics::{PhysicsWorld};

// Initialize world entities and state
fn init_world(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld) {
    let mut rng = rand::thread_rng();
    const POSX_RANGE: f32 = 1800.0;
    const POSY_RANGE: f32 = 1000.0;
    const VELX_RANGE: f32 = 395.0;
    const VELY_RANGE: f32 = 375.0;

    UIBuilder::build_icon(world, ctx);

    let mut sprite = SpriteConfig::create_from_config(world, ctx, "entities/pyramids".to_string());
    // sprite.scale.x = 3.0;
    // sprite.scale.y = 3.0;
    sprite.z_order = SpriteLayer::BG.to_z();
    
    world.create_entity()
        .with(Position { x: POSX_RANGE / 2.0, y: POSY_RANGE / 2.0 })
        .with(DisplayComp { circle: false, display_type: DisplayCompType::DrawSelf })
        .with(sprite) //SpriteComponent::new(ctx, &sprite.pa, 1000.0))
        //.with(Collision::new_circle(20.0))
        .build();

    PlatformBuilder::build(world, ctx, physics_world, 1000.0, 0.0, 1000.0, 50.0, SpriteLayer::World.to_z());
    PlatformBuilder::build(world, ctx, physics_world, 0.0, 600.0, 50.0, 600.0, SpriteLayer::World.to_z());
    PlatformBuilder::build(world, ctx, physics_world, 1000.0, 1200.0, 1000.0, 50.0, SpriteLayer::World.to_z());
    PlatformBuilder::build(world, ctx, physics_world, 2000.0, 600.0, 50.0, 600.0, SpriteLayer::World.to_z());

    SuriBuilder::build(world, ctx, physics_world, 250.0, 120.0);

    for i in 0..115 {
        let x: f32 = ((100.0 + rng.gen::<f32>() * POSX_RANGE) / 50.0).round() * 50.0;
        let y: f32 = ((100.0 + rng.gen::<f32>() * POSY_RANGE) / 50.0).round() * 50.0;
        let vx: f32 = (rng.gen::<f32>() * VELX_RANGE) - (VELX_RANGE / 2.0);
        let vy: f32 = (rng.gen::<f32>() * VELY_RANGE) - (VELY_RANGE / 2.0);
        // build ball entity and add to world
        if i % 11 < 1 {
            // if i % 11 == 0 {
            //     BallBuilder::build(world, ctx, x, y, vx, vy);
            // }
            // else {
            GhostBuilder::build_collider(world, ctx, physics_world, x, y, vx, vy, 20.0, 0.15, 20.0, 20.0);
            //}
        }
        else {
            if i % 2 == 0 {
                PlatformBuilder::build(world, ctx, physics_world, x, y, 25.0, 25.0, SpriteLayer::World.to_z());
            }
            else {
                GhostBuilder::build_static_collider(world, ctx, physics_world, x, y, 20.0, 0.15, 20.0, 20.0);
            }
            
            //GhostBuilder::build_static_collider(world, ctx, physics_world, x, y, 20.0, 0.15, 25.0, 25.0);
            // GhostBuilder::build_static_collider(world, ctx, x-35.0, y, 20.0, 0.15);
            // GhostBuilder::build_static_collider(world, ctx, x+35.0, y, 20.0, 0.15);
        }
        
    }
}

// Build world by loading resources, components, and calling init_world
pub fn create_world<'a>(ctx: &mut Context, game_state_res: GameStateResource, physics_world: &mut PhysicsWorld) -> World {
    let mut world = World::new();
    
    world.insert(game_state_res);

    add_resources(&mut world, ctx);

    // Register all components
    register_components(&mut world);

    // Create initial world entities
    init_world(&mut world, ctx, physics_world);

    world
}

// Create the dispatcher for the world systems
pub fn create_dispatcher<'a>() -> Dispatcher<'a,'a> {
    // build disptacher by including all needed systems
    let dispatcher = DispatcherBuilder::new()
    // apply inputs to entities that are player controlled
    .with(InputSystem::new(), "input_system", &[])
    // apply gravity to entities
    //.with(GravSys, "grav_sys", &["input_system"])
    // handle entity interactions
    //.with(InterActorSys::new::<'a>(physics_world), "interactor_sys", &["grav_sys"])
    // update entity positions by velocity
    //.with(UpdatePos { t_delta: core::time::Duration::new(1,0) }, "update_pos", &["grav_sys"])
    // reports entity status
    //.with(SumSys, "sum_sys", &["update_pos"])
    .build();

    dispatcher
}