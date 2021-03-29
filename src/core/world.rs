use ggez::{Context};
use specs::prelude::*;
use specs::{World, WorldExt, Entity, Builder}; // Builder, Component, ReadStorage, System, VecStorage, RunNow
use specs::shred::{Dispatcher, DispatcherBuilder};
use rand::prelude::*;

use crate::resources::{add_resources,GameStateResource};
use crate::components::{Position, Velocity, LevelSource, register_components};
use crate::components::sprite::{SpriteLayer,SpriteConfig};
use crate::components::player::{CharacterDisplayComponent};
use crate::entities::platform::{PlatformBuilder};
use crate::entities::player::{CharacterBuilder};
use crate::entities::ghost::{GhostBuilder};
use crate::entities::ui::{UIBuilder};
use crate::systems::*;
use crate::systems::interactor::{InterActorSys};
use crate::core::{PhysicsWorld};

// Initialize world entities and state
fn init_world(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld) {
    let mut rng = rand::thread_rng();
    const POSX_RANGE: f32 = 7000.0;
    const POSY_RANGE: f32 = 12000.0;
    const VELX_RANGE: f32 = 395.0;
    const VELY_RANGE: f32 = 375.0;

    // UIBuilder::build_icon(world, ctx);

    // let mut sprite = SpriteConfig::create_from_config(world, ctx, "entities/electric-bg".to_string());
    // // sprite.scale.x = 3.0;
    // // sprite.scale.y = 3.0;
    // sprite.z_order = SpriteLayer::BG.to_z();
    
    // world.create_entity()
    //     .with(Position { x: POSX_RANGE / 2.0, y: POSY_RANGE / 2.0 })
    //     .with(DisplayComp { circle: false, display_type: DisplayCompType::DrawSelf })
    //     .with(sprite) //SpriteComponent::new(ctx, &sprite.pa, 1000.0))
    //     //.with(Collision::new_circle(20.0))
    //     .build();

    // // PlatformBuilder::build(world, ctx, physics_world, 1000.0, 0.0, 1000.0, 50.0, SpriteLayer::World.to_z());
    // // PlatformBuilder::build(world, ctx, physics_world, 0.0, 600.0, 50.0, 600.0, SpriteLayer::World.to_z());
    // // PlatformBuilder::build(world, ctx, physics_world, 1000.0, 1200.0, 1000.0, 50.0, SpriteLayer::World.to_z());
    // // PlatformBuilder::build(world, ctx, physics_world, 2000.0, 600.0, 50.0, 600.0, SpriteLayer::World.to_z());

    // CharacterBuilder::build(world, ctx, physics_world, 2000.0, 2000.0);

    // for i in 0..120 {
    //     let x: f32 = ((-500.0 + rng.gen::<f32>() * POSX_RANGE) / 100.0).round() * 100.0;
    //     let y: f32 = ((100.0 + rng.gen::<f32>() * 1500.0) / 100.0).round() * 100.0;
    //     let ang: f32 = rng.gen::<f32>() * 2.0 - 1.0;

    //     if rng.gen::<f32>() < 0.7 {
    //         PlatformBuilder::build(world, ctx, physics_world, x, y, 100.0 + rng.gen::<f32>() * 500.0, 10.0 + rng.gen::<f32>() * 20.0, 0.0, SpriteLayer::BGNear.to_z());
    //     }
    //     else {
    //         PlatformBuilder::build_dynamic(world, ctx, physics_world, x, y, 100.0 + rng.gen::<f32>() * 500.0, 10.0 + rng.gen::<f32>() * 20.0, (3.14158 / 2.0) + ang, SpriteLayer::BGNear.to_z());
    //     }
    // }

    // for i in 0..70 {
    //     let x: f32 = ((100.0 + rng.gen::<f32>() * 2000.0) / 100.0).round() * 100.0;
    //     let y: f32 = ((100.0 + rng.gen::<f32>() * POSY_RANGE) / 100.0).round() * 100.0;
    //     let ang: f32 = rng.gen::<f32>() * 2.0 - 1.0;

    //     if rng.gen::<f32>() < 0.7 {
    //         PlatformBuilder::build(world, ctx, physics_world, x, y, 100.0 + rng.gen::<f32>() * 500.0, 10.0 + rng.gen::<f32>() * 20.0, 0.0, SpriteLayer::BGNear.to_z());
    //     }
    //     else {
    //         PlatformBuilder::build_dynamic(world, ctx, physics_world, x, y, 100.0 + rng.gen::<f32>() * 500.0, 10.0 + rng.gen::<f32>() * 20.0, (3.14158 / 2.0) + ang, SpriteLayer::BGNear.to_z());
    //     }
    // }

    // for i in 0..70 {
    //     let x: f32 = ((POSX_RANGE - 2000.0 + rng.gen::<f32>() * 2000.0) / 100.0).round() * 100.0;
    //     let y: f32 = ((100.0 + rng.gen::<f32>() * POSY_RANGE) / 100.0).round() * 100.0;
    //     let ang: f32 = rng.gen::<f32>() * 2.0 - 1.0;

    //     if rng.gen::<f32>() < 0.7 {
    //         PlatformBuilder::build(world, ctx, physics_world, x, y, 100.0 + rng.gen::<f32>() * 500.0, 10.0 + rng.gen::<f32>() * 20.0, 0.0, SpriteLayer::BGNear.to_z());
    //     }
    //     else {
    //         PlatformBuilder::build_dynamic(world, ctx, physics_world, x, y, 100.0 + rng.gen::<f32>() * 500.0, 10.0 + rng.gen::<f32>() * 20.0, (3.14158 / 2.0) + ang, SpriteLayer::BGNear.to_z());
    //     }
    // }

    // for i in 0..70 {
    //     let x: f32 = ((100.0 + rng.gen::<f32>() * POSX_RANGE) / 100.0).round() * 100.0;
    //     let y: f32 = ((10100.0 + rng.gen::<f32>() * 1500.0) / 100.0).round() * 100.0;
    //     let ang: f32 = rng.gen::<f32>() * 2.0 - 1.0;

    //     if rng.gen::<f32>() < 0.7 {
    //         PlatformBuilder::build(world, ctx, physics_world, x, y, 100.0 + rng.gen::<f32>() * 500.0, 10.0 + rng.gen::<f32>() * 20.0, 0.0, SpriteLayer::BGNear.to_z());
    //     }
    //     else {
    //         PlatformBuilder::build_dynamic(world, ctx, physics_world, x, y, 100.0 + rng.gen::<f32>() * 500.0, 10.0 + rng.gen::<f32>() * 20.0, (3.14158 / 2.0) + ang, SpriteLayer::BGNear.to_z());
    //     }
    // }


    // for i in 0..415 {
    //     let x: f32 = ((100.0 + rng.gen::<f32>() * POSX_RANGE) / 100.0).round() * 100.0;
    //     let y: f32 = ((100.0 + rng.gen::<f32>() * POSY_RANGE) / 100.0).round() * 100.0;
    //     let vx: f32 = (rng.gen::<f32>() * VELX_RANGE) - (VELX_RANGE / 2.0);
    //     let vy: f32 = (rng.gen::<f32>() * VELY_RANGE) - (VELY_RANGE / 2.0);
    //     let ang: f32 = rng.gen::<f32>() * 0.1 - 0.05;
    //     // build ball entity and add to world
    //     if i % 11 < 5 {
    //         if i % 2 == 0 {
    //         //     BallBuilder::build(world, ctx, x, y, vx, vy);
    //             PlatformBuilder::build(world, ctx, physics_world, x, y, 100.0, 100.0, ang, SpriteLayer::BGNear.to_z());
    //         }
    //         else if i % 3 == 0 {
    //             PlatformBuilder::build(world, ctx, physics_world, x, y, 100.0, 50.0, ang, SpriteLayer::BGNear.to_z());
    //         }
    //         else {
    //         //GhostBuilder::build_collider(world, ctx, physics_world, x, y, vx, vy, 20.0, 0.15, 20.0, 20.0);
    //             PlatformBuilder::build_dynamic(world, ctx, physics_world, x, y, 50.0, 50.0, 0.0, SpriteLayer::BGNear.to_z());
    //         }
    //     }
    //     else {
    //         //if i % 2 == 0 {
    //             //PlatformBuilder::build(world, ctx, physics_world, x, y, 25.0, 25.0, ang, SpriteLayer::World.to_z());
    //         //}
    //         //else {
    //         //    GhostBuilder::build_static_collider(world, ctx, physics_world, x, y, 20.0, 0.15, 20.0, 20.0);
    //         //}
    //         GhostBuilder::build_collider(world, ctx, physics_world, x, y, vx, vy, 20.0, 0.15,  20.0, 20.0);
            
    //         //GhostBuilder::build_static_collider(world, ctx, physics_world, x, y, 20.0, 0.15, 25.0, 25.0);
    //         // GhostBuilder::build_static_collider(world, ctx, x-35.0, y, 20.0, 0.15);
    //         // GhostBuilder::build_static_collider(world, ctx, x+35.0, y, 20.0, 0.15);
    //     }
        
    // }

}

pub fn empty_world(ctx: &mut Context, world: &mut World, physics_world: &mut PhysicsWorld) {
    


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
// pub fn create_dispatcher<'a>() -> Dispatcher<'a,'a> {
//     // build disptacher by including all needed systems
//     let dispatcher = DispatcherBuilder::new()
//     // apply inputs to entities that are player controlled
//     .with(InputSystem::new(), "input_system", &[])
//     // apply gravity to entities
//     //.with(GravSys, "grav_sys", &["input_system"])
//     // handle entity interactions
//     //.with(InterActorSys::new::<'a>(physics_world), "interactor_sys", &["grav_sys"])
//     // update entity positions by velocity
//     //.with(UpdatePos { t_delta: core::time::Duration::new(1,0) }, "update_pos", &["grav_sys"])
//     // reports entity status
//     //.with(SumSys, "sum_sys", &["update_pos"])
//     .build();

//     dispatcher
// }

pub trait SuriWorld {
    // Find the player which is controlled or the first controllable player
    fn get_player(&self) -> Option<Entity>;
    // Get Item_index from level if it was created from the level (or -1)
    fn get_level_source(&self, entity: &Entity) -> i32;
}

impl SuriWorld for World {
    fn get_player(&self) -> Option<Entity> {
        let char_res = self.read_storage::<CharacterDisplayComponent>();
        let ent_res = self.entities();

        for (character, ent) in (&char_res, &ent_res).join() {
            if character.is_controllable && character.is_controlled {
                return Some(ent.clone());
            }
        }

        for (character, ent) in (&char_res, &ent_res).join() {
            if character.is_controllable {
                return Some(ent.clone());
            }
        }

        None
    }
    fn get_level_source(&self, entity: &Entity) -> i32 {
        let lvl_src_res = self.read_storage::<LevelSource>();

        if let Some(lvl_src) = lvl_src_res.get(*entity) {
            return lvl_src.item_index;
        }
        
        return -1;
    }
}
