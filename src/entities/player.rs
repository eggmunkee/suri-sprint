use ggez::{Context, GameResult, GameError};
use ggez::graphics;
use ggez::graphics::{Image};
use ggez::nalgebra as na;
use specs::{Builder,Entity,EntityBuilder,World,WorldExt};
use wrapped2d::user_data::*;
use serde::{Deserialize,Serialize};

use crate::conf::*;
use crate::core::{GameState};
use crate::resources::{GameStateResource,ImageResources};
use crate::components::{Position,RenderFlag,RenderLayerType};
use crate::components::sprite::{SpriteComponent,SpriteConfig};
use crate::components::collision::{Collision};
use crate::components::particle_sys::{ParticleSysConfig,ParticleSysComponent};
use crate::components::player::{CharacterDisplayComponent};
use crate::components::npc::{NpcComponent};
use crate::components::flags::{RenderCharacterFlag};
use crate::entities::level_builder::{LevelType};
use crate::systems::*;
use crate::core::physics;
use crate::core::physics::{EntityType,PhysicsWorld,CollisionCategory};

#[derive(Debug,Clone,Deserialize,Serialize)]
pub enum PlayerCharacter {
    Suri,
    Milo
}

pub struct CharacterBuilder;

impl CharacterBuilder {

    // pub fn get_sprite_paths() -> Vec<String> {
    //     vec!["/suri-spritesheet.png".to_string()]
    // }

    pub fn build_npc(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, x: f32, y: f32, player: PlayerCharacter, level_type: &LevelType) -> Entity {

        let mut player_num = 1;
        {
            let mut game_state = world.fetch_mut::<GameStateResource>();
            let pc = game_state.player_count + 1;
            game_state.player_count = pc;
            //level_type = level_type.clone();
            player_num = pc;
            println!("NPC Player {} being created...", &player_num);
        }

        // Init Suri images from SpriteConfig
        let maybe_config = match &player {
            PlayerCharacter::Suri => {
                get_ron_config::<SpriteConfig>("entities/suri".to_string())
            },
            PlayerCharacter::Milo => {
                get_ron_config::<SpriteConfig>("entities/milo".to_string())
            }            
        };
        
        let sprite_config = maybe_config.unwrap();
        SpriteConfig::init_images(world, ctx, sprite_config.path.clone());

        let npc = NpcComponent::new();

        // Create collision component
        let mut collision = match &player {
            PlayerCharacter::Suri => {
                Collision::new_specs(3.0,0.001, 18.0, 18.0)
            },
            PlayerCharacter::Milo => {
                Collision::new_specs(3.0,0.001, 25.0, 20.0)
            }            
        };
        
        collision.pos.x = x;
        collision.pos.y = y;
        collision.density = 1.0;
        collision.entity_type = EntityType::Player;
        collision.collision_category = CollisionCategory::Player;
        collision.enable_warp = true;
        collision.collision_mask.clear();
        collision.collision_mask.push(CollisionCategory::Level);
        collision.collision_mask.push(CollisionCategory::Portal);
        collision.collision_mask.push(CollisionCategory::Etherial);
        collision.collision_mask.push(CollisionCategory::Player);

        // Create physics body from collision properties
        //collision.create_dynamic_body_box_upright(physics_world);
        collision.create_dynamic_body_box_upright(physics_world);
        // get body handle value
        let body_handle_clone = collision.body_handle.clone();

        let mut char_comp = CharacterDisplayComponent::new(ctx, &sprite_config.path, player);
        char_comp.is_controlled = false;
        char_comp.is_controllable = false;
        char_comp.player_number = player_num;

        // create particle system
        let mut feet_psys = ParticleSysConfig::create_from_config(world, ctx, "psys/foot_dust".to_string(),
            x, y, 0.0, 0.0, (0.0, 20.0));
        feet_psys.visible = false;
        // feet_psys.world_offset.0 = x;
        // feet_psys.world_offset.1 = y;
        //feet_psys.
        //ParticleSysComponent::new(ctx);


        // Create entity
        let entity = world.create_entity()
        .with(npc)
        .with(Position { x: x, y: y })
        //.with(Velocity { x: 0.0, y: 0.0, gravity: true, frozen: false })
        //.with(DisplayComp { circle: false, display_type: DisplayCompType::DrawSelf })
        .with(char_comp)
        .with(collision)
        .with(feet_psys)
        .with(RenderFlag::from_layer(RenderLayerType::LevelLayer))
        .with(RenderCharacterFlag)
        .build();

        // Store entity ID in physics body data
        physics::update_body_entity_data(&entity, physics_world, body_handle_clone);
        // let entity_id = entity.id();
        // if let Some(body_handle) = body_handle_clone {
        //     let mut collision_body = physics_world.body_mut(body_handle);

        //     let body_data = &mut *collision_body.user_data_mut();
        //     //let data = &*data_ref;
        //     body_data.entity_id = entity_id;            
        // }

        entity








        // let mut character = CharacterDisplayComponent::new(ctx, &sprite_config.path, player);
        // character.input_enabled = false;

        // // Create entity
        // let entity = world.create_entity()
        // .with(npc)
        // .with(Position { x: x, y: y })
        // //.with(Velocity { x: 0.0, y: 0.0, gravity: true, frozen: false })
        // //.with(DisplayComp { circle: false, display_type: DisplayCompType::DrawSelf })
        // .with(character)
        // .with(collision)
        // .build();

        // // Store entity ID in physics body data
        // physics::update_body_entity_data(&entity, physics_world, body_handle_clone);
        // // let entity_id = entity.id();
        // // if let Some(body_handle) = body_handle_clone {
        // //     let mut collision_body = physics_world.body_mut(body_handle);

        // //     let body_data = &mut *collision_body.user_data_mut();
        // //     //let data = &*data_ref;
        // //     body_data.entity_id = entity_id;            
        // // }

        // entity

    }

    pub fn build(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, x: f32, y: f32, player: PlayerCharacter, level_type: &LevelType,
        start_controlling: bool) -> Entity {


        //let mut level_type: LevelType = LevelType::default();
        let mut player_num = 1;
        {
            let mut game_state = world.fetch_mut::<GameStateResource>();
            let pc = game_state.player_count + 1;
            game_state.player_count = pc;
            //level_type = level_type.clone();
            player_num = pc;
            println!("Player {} being created...", &player_num);
        }
        
        let mut npc = NpcComponent::new();
        npc.is_enabled = false;

        // Init Suri images from SpriteConfig
        let maybe_config = match &player {
            PlayerCharacter::Suri => {
                get_ron_config::<SpriteConfig>("entities/suri".to_string())
            },
            PlayerCharacter::Milo => {
                get_ron_config::<SpriteConfig>("entities/milo".to_string())
            }            
        };
        //let maybe_config = get_ron_config::<SpriteConfig>("entities/milo".to_string());
        let sprite_config = maybe_config.unwrap();
        SpriteConfig::init_images(world, ctx, sprite_config.path.clone());

        // Create collision component
        let mut collision = match &player {
            PlayerCharacter::Suri => {
                Collision::new_specs(3.0,0.001, 18.0, 18.0)
            },
            PlayerCharacter::Milo => {
                Collision::new_specs(3.0,0.001, 25.0, 20.0)
            }            
        };
        //let mut collision = Collision::new_specs(3.0,0.001, 18.0, 18.0);
        collision.pos.x = x;
        collision.pos.y = y;
        collision.density = 1.0;
        collision.entity_type = EntityType::Player;
        collision.collision_category = CollisionCategory::Player;
        collision.enable_warp = true;
        collision.collision_mask.clear();
        collision.collision_mask.push(CollisionCategory::Level);
        collision.collision_mask.push(CollisionCategory::Portal);
        collision.collision_mask.push(CollisionCategory::Etherial);
        collision.collision_mask.push(CollisionCategory::Player);
        // Create physics body from collision properties
        collision.create_dynamic_body_box_upright(physics_world);
        // get body handle value
        let body_handle_clone = collision.body_handle.clone();

        let mut char_comp = CharacterDisplayComponent::new(ctx, &sprite_config.path, player);
        char_comp.is_controlled = start_controlling;
        char_comp.is_controllable = true;
        char_comp.player_number = player_num;

        // create particle system
        let mut feet_psys = ParticleSysConfig::create_from_config(world, ctx, "psys/foot_dust".to_string(),
            x, y, 0.0, 0.0, (0.0, 20.0));
        feet_psys.toggleable = true;      
        feet_psys.emitting = false;  
        //feet_psys.visible = false;
        // feet_psys.world_offset.0 = x;
        // feet_psys.world_offset.1 = y;
        //feet_psys.
        //ParticleSysComponent::new(ctx);


        // Create entity
        let entity = world.create_entity()
        .with(npc)
        .with(Position { x: x, y: y })
        //.with(Velocity { x: 0.0, y: 0.0, gravity: true, frozen: false })
        //.with(DisplayComp { circle: false, display_type: DisplayCompType::DrawSelf })
        .with(char_comp)
        .with(collision)
        .with(feet_psys)
        .with(RenderFlag::from_layer(RenderLayerType::LevelLayer))
        .with(RenderCharacterFlag)
        .build();

        // Store entity ID in physics body data
        physics::update_body_entity_data(&entity, physics_world, body_handle_clone);
        // let entity_id = entity.id();
        // if let Some(body_handle) = body_handle_clone {
        //     let mut collision_body = physics_world.body_mut(body_handle);

        //     let body_data = &mut *collision_body.user_data_mut();
        //     //let data = &*data_ref;
        //     body_data.entity_id = entity_id;            
        // }

        entity
    }

}
