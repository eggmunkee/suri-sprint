

use ggez::{Context};
use specs::{Component,World,WorldExt,Builder,Entity,EntityBuilder};
use serde::{Deserialize,Serialize};


use crate::conf::*;
use crate::components::anim_sprite::*;
use crate::components::particle_sys::{ParticleSysConfig};
use crate::components::sprite::*;
use crate::components::logic::*;
use crate::components::{Position,RenderFlag,RenderLayerType,LevelSource};
use crate::components::flags::{RenderSpriteFlag,RenderAnimSpriteFlag,RenderParticleSysFlag};
use crate::entities::platform::{PlatformBuilder};
use crate::entities::empty_box::{BoxBuilder};
use crate::entities::button::{ButtonBuilder};
use crate::entities::portal_area::{PortalBuilder};
use crate::entities::exit::{ExitBuilder};
use crate::entities::player::{CharacterBuilder,PlayerCharacter};
use crate::entities::ghost::{GhostBuilder};
use crate::entities::bowl::{BowlBuilder};
use crate::entities::ball::{BallBuilder};
use crate::entities::mouse::{MouseBuilder};
use crate::entities::point_pickup::{PickupBuilder};
use crate::components::collision::{Collision};
use crate::resources::{ImageResources};
use crate::resources::{ConnectionResource};
use crate::core::physics::{PhysicsWorld,CollisionCategory,PickupItemType};

pub use crate::entities::level::*;

pub fn add_render_flag(builder: EntityBuilder, layer: RenderLayerType) -> EntityBuilder {
    builder.with(RenderFlag::from_layer(layer))
}

#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct LevelConfig {
    pub name: String,
    pub bounds: LevelBounds,
    pub soundtrack: String,
    pub level_type: Option<LevelType>,
    pub items: Vec::<LevelItem>,
    #[serde(skip)]
    built_player: bool,
    #[serde(skip)]
    build_index: i32,
}

impl LevelConfig {
    pub fn new() -> Self {
        LevelConfig {
            name: "".to_string(),
            bounds: LevelBounds::new(0.0, 0.0, 800.0, 600.0),
            soundtrack: "".to_string(),
            level_type: Some(LevelType::default()),
            items: vec![],
            built_player: false,
            build_index: 0,
        }
    }

    pub fn load_level(path: &str) -> LevelConfig {
        println!("Loading level {}", path);
        let mut level_path = String::from(path);
        level_path.insert_str(0, "levels/");

        let opt_level = get_ron_config::<LevelConfig>(level_path);

        opt_level.expect(format!("Failed to load level {}", path).as_str())

    }

    pub fn get_level_type(&self) -> LevelType {
        match &self.level_type {
            Some(lt) => lt.clone(),
            None => LevelType::default(),
        }
    }

    pub fn build_item(&mut self, world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld,
        entry_name: &str, item: &LevelItem) -> (Option<Entity>, Option<Entity>) {
        let lvl_type = self.get_level_type();
        let mut entity : Option<Entity> = None;
        let mut entity2 : Option<Entity> = None;
        match item {
            LevelItem::Player{ x, y, player } if entry_name.is_empty() => {
                let mut player_val = PlayerCharacter::Suri;
                if let Some(plyr) = player {
                    player_val = plyr.clone();
                }
                let start_controlling_player = !self.built_player;
                //CharacterBuilder::build_npc(world, ctx, physics_world, *x+30.0, *y-30.0);
                entity = Some(
                    CharacterBuilder::build(world, ctx, physics_world, *x, *y, player_val, &lvl_type, start_controlling_player)
                );
                self.built_player = true;
            },
            LevelItem::PlayerNamed{ x, y, player, name } if name == &entry_name => {
                let mut player_val = PlayerCharacter::Suri;
                if let Some(plyr) = player {
                    player_val = plyr.clone();
                }
                let start_controlling_player = !self.built_player;
                //CharacterBuilder::build_npc(world, ctx, physics_world, *x+30.0, *y-30.0);
                entity = Some(CharacterBuilder::build(world, ctx, physics_world, *x, *y, player_val, &lvl_type, start_controlling_player));
                self.built_player = true;
            },
            LevelItem::PlayerNpc{ x, y, player } if entry_name.is_empty() => {
                let mut player_val = PlayerCharacter::Suri;
                if let Some(plyr) = player {
                    player_val = plyr.clone();
                }
                //CharacterBuilder::build_npc(world, ctx, physics_world, *x+30.0, *y-30.0);
                entity = Some(CharacterBuilder::build_npc(world, ctx, physics_world, *x, *y, player_val, &lvl_type));
            },
            LevelItem::Platform{ x, y, w, h, ang, z, logic} => {
                let mut z_value = SpriteLayer::World.to_z();
                if let Some(z_cfg_val) = z {
                    z_value = *z_cfg_val;
                }
                entity = Some(PlatformBuilder::build_w_logic(world, ctx, physics_world, *x, *y, *w, *h, *ang, z_value, logic.clone()));
            },
            LevelItem::DynPlatform{ x, y, w, h, ang} => {
                entity = Some(PlatformBuilder::build_dynamic(world, ctx, physics_world, *x, *y, *w, *h, *ang, SpriteLayer::World.to_z()));
            },
            LevelItem::StaticLevelProp{ x, y, w, h, ang, image, img_w, img_h, z, logic} => {
                let mut z_value = SpriteLayer::World.to_z();
                if let Some(z_cfg_val) = z {
                    z_value = *z_cfg_val;
                }
                entity = Some(PlatformBuilder::build_w_image_logic(world, ctx, physics_world, *x, *y, *w, *h, *ang, z_value, (*image).to_string(), *img_w, *img_h, logic.clone()));
            },
            LevelItem::DynStaticLevelProp{ x, y, w, h, ang, image, img_w, img_h, z} => {
                let mut z_value = SpriteLayer::World.to_z();
                if let Some(z_cfg_val) = z {
                    z_value = *z_cfg_val;
                }
                entity = Some(PlatformBuilder::build_dynamic_w_image(world, ctx, physics_world, *x, *y, *w, *h, *ang, z_value, (*image).to_string(), *img_w, *img_h));
            },
            LevelItem::EmptyBox{ x, y, w, h, ang} => {
                entity = Some(BoxBuilder::build(world, ctx, physics_world, *x, *y, *w, *h, *ang));
            },
            LevelItem::DynEmptyBox{ x, y, w, h, ang} => {
                entity = Some(BoxBuilder::build_dynamic(world, ctx, physics_world, *x, *y, *w, *h, *ang, SpriteLayer::World.to_z()));
            },
            LevelItem::Button{ x, y, w, h, ang, name, start_enabled } => {
                let (ent1, ent2) = ButtonBuilder::build(world, ctx, physics_world, *x, *y, *w, *h, *ang, (*name).to_string(), *start_enabled);
                entity = Some(ent1);
                entity2 = Some(ent2);
            },
            LevelItem::Ghost{ x, y } => {
                entity = Some(GhostBuilder::build_collider(world, ctx, physics_world, *x, *y, 0.0, 0.0, 0.0, 0.0, 24.0, 24.0));  //(world, ctx, physics_world, *x, *y, *w, *h, *ang, SpriteLayer::BGNear.to_z());
            },
            LevelItem::Sprite{ x, y, z, sprite, angle, src, shader} => {
                let sprite_path = &*sprite;
                let mut sprite = AnimSpriteConfig::create_from_sprite_config(world, ctx, sprite_path.clone());
                sprite.angle = *angle;
                sprite.z_order = *z;
                sprite.set_src(&src); 
                sprite.shader = shader.clone();

                entity = Some(
                    world.create_entity().with(sprite).with(Position { x: *x, y: *y })
                    .with(RenderFlag::from_layer(RenderLayerType::LevelLayer)).with(RenderSpriteFlag).build());
            },
            LevelItem::AnimSprite{ x, y, z, sprite, angle, src, shader} => {
                let sprite_path = &*sprite;
                let mut sprite = AnimSpriteConfig::create_from_config(world, ctx, sprite_path.clone());
                sprite.angle = *angle;
                sprite.z_order = *z;
                sprite.set_src(&src); 
                sprite.shader = shader.clone();

                entity = Some(world.create_entity().with(sprite).with(Position { x: *x, y: *y })
                    .with(RenderFlag::from_layer(RenderLayerType::LevelLayer)).with(RenderAnimSpriteFlag).build());
            },
            LevelItem::DynSprite{ x, y, z, sprite, angle, src, name, start_enabled, logic_op } => {
                let sprite_path = &*sprite;
                let mut sprite = SpriteConfig::create_from_config(world, ctx, sprite_path.clone());
                sprite.angle = *angle;
                sprite.z_order = *z;
                sprite.toggleable = true;
                sprite.set_src(&src); 

                let logic_comp = LogicComponent::new((*name).to_string(), *start_enabled, *logic_op);
                // set logic operation if specified
                // if let Some(logic_operation) = &logic_op {
                //     logic_comp.logic_op = *logic_operation;
                // }
                entity = Some(world.create_entity().with(sprite).with(logic_comp).with(Position { x: *x, y: *y })
                    .with(RenderFlag::from_layer(RenderLayerType::LevelLayer)).with(RenderSpriteFlag).build());
            },
            LevelItem::ParallaxSprite { x, y, sprites, scroll_factors } => {

                let mut plx_sprite = ParallaxSpriteComponent::new(ctx);

                let spr_len = sprites.len();
                let factor_len = scroll_factors.len();
                let mut i : usize = 0;
                while i < spr_len && i < factor_len {
                    // Get sprite and scroll factor item i
                    if let Some(sprite_config) = sprites.get(i) {
                        if let Some(factor) = scroll_factors.get(i) {
                            let sprite_comp = SpriteConfig::create_from_config(world, ctx, sprite_config.sprite.clone());
                            plx_sprite.add_sprite(ctx, sprite_comp, *factor);
                        }
                    }

                    i += 1;
                }
                
                entity = Some(
                    world.create_entity().with(plx_sprite).with(Position { x: *x, y: *y })
                    .with(RenderFlag::from_layer(RenderLayerType::LevelLayer)).build());
            },
            LevelItem::Portal { x, y, w, z, name, destination, start_enabled, logic } => {
                let mut z_value = SpriteLayer::World.to_z();
                if let Some(z_cfg_val) = z {
                    z_value = *z_cfg_val;
                }
                entity = Some(PortalBuilder::build(world, ctx, physics_world, *x, *y, z_value, *w, 
                    (*name).clone(), (*destination).to_string(), *start_enabled, logic.clone()));
            },
            LevelItem::PortalSide { x, y, ang, w, h, z, color, name, destination, start_enabled, logic, normal } => {
                let mut z_value = SpriteLayer::World.to_z();
                if let Some(z_cfg_val) = z {
                    z_value = *z_cfg_val;
                }
                entity = Some(PortalBuilder::build_side(world, ctx, physics_world, *x, *y, z_value, *ang, *w, *h, (*color).clone(),
                    (*name).clone(), (*destination).to_string(), *start_enabled, logic.clone(), (normal.0, normal.1)));
            },
            LevelItem::ParticleSys { x, y, z, config, logic } => {
                let config_path = &*config;
                let mut part_sys = ParticleSysConfig::create_from_config(world, ctx, config_path.clone(),
                    *x, *y, 0.0, 0.0, (0.0, 0.0));
                // part_sys.world_offset.0 = *x;
                // part_sys.world_offset.1 = *y;
                part_sys.z_order = *z;

                let mut builder = world.create_entity().with(Position { x: *x, y: *y });
                if let Some(ItemLogic{ name, start_enabled, logic_op, logic_type }) = logic {
                    part_sys.toggleable = true;
                    println!("ParticleSys has ItemLogic: name: {}, start_enabled: {}, logic_op: {:?}, logic_type: {:?}",
                        name, start_enabled, logic_op, logic_type);
                    builder = builder.with(LogicComponent::new_logic(name.clone(), *start_enabled, Some(ItemLogic { 
                        name: name.clone(), start_enabled: *start_enabled, logic_op: *logic_op, logic_type: logic_type.clone() }) ));
                }                
                builder = builder.with(part_sys);
                builder = add_render_flag(builder, RenderLayerType::LevelLayer);
                entity = Some(builder.with(RenderParticleSysFlag).build());
            },
            LevelItem::Exit { x, y, w, h, z, name, destination } => {
                let mut z_value = SpriteLayer::BGNear.to_z();
                if let Some(z_cfg_val) = z {
                    z_value = *z_cfg_val;
                }
                entity = Some(ExitBuilder::build(world, ctx, physics_world, *x, *y, z_value, *w, *h, (*name).to_string(), (*destination).to_string()));
            },
            LevelItem::ExitCustom { x, y, w, h, z, name, destination, image, img_w, img_h } => {
                let mut z_value = SpriteLayer::BGNear.to_z();
                if let Some(z_cfg_val) = z {
                    z_value = *z_cfg_val;
                }
                entity = Some(ExitBuilder::build_w_image(world, ctx, physics_world, *x, *y, z_value, *w, *h, (*name).to_string(), (*destination).to_string(),
                    (*image).to_string(), *img_w, *img_h));
            },
            LevelItem::Bowl { x, y, z } => {
                let mut z_value = SpriteLayer::Entities.to_z();
                if let Some(z_cfg_val) = z {
                    z_value = *z_cfg_val;
                }
                entity = Some(BowlBuilder::build(world, ctx, physics_world, *x, *y, z_value));
            },
            LevelItem::Mouse { x, y, z } => {
                let mut z_value = 300.0;
                if let Some(z_cfg_val) = z {
                    z_value = *z_cfg_val;
                }
                entity = Some(MouseBuilder::build(world, ctx, physics_world, *x, *y, 32.0, 12.0, 0.0, z_value));
            },
            LevelItem::Ball { x, y, z } => {
                let mut z_value = 300.0;
                if let Some(z_cfg_val) = z {
                    z_value = *z_cfg_val;
                }
                entity = Some(BallBuilder::build(world, ctx, physics_world, *x, *y, 24.0, 24.0, 0.0, z_value));
            },
            LevelItem::Pickup { x, y, z, pickup_type } => {
                let mut z_value = 300.0;
                if let Some(z_cfg_val) = z {
                    z_value = *z_cfg_val;
                }
                entity = Some(PickupBuilder::build(world, ctx, physics_world, *x, *y, z_value, 12.0, 12.0, *pickup_type ));
            },
            LevelItem::DynPickup { x, y, z, pickup_type } => {
                let mut z_value = 300.0;
                if let Some(z_cfg_val) = z {
                    z_value = *z_cfg_val;
                }
                entity = Some(PickupBuilder::build_dynamic(world, ctx, physics_world, *x, *y, z_value, 12.0, 12.0, *pickup_type ));
            },
            LevelItem::EffectArea { .. } => {

            },
            LevelItem::Connection { from, to, conn_type } => {
                let mut connection_res = world.fetch_mut::<ConnectionResource>();

                let mut connection = &mut *connection_res;
                connection.add_connection(from.clone(), to.clone(), LogicOpType::And);
            },
            _ => {
                // Player starts that don't apply on the current level entry - ignored
            }
        }


        return (entity, entity2);
    }

    pub fn build_level(&mut self, world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, entry_name: String) {
        // Clear built player flag when building the full level - will be set when player is built
        self.built_player = false;
        self.build_index = 0;

        let mut items : Vec<LevelItem> = vec![];
        // Get cloned level items
        for level_item in &self.items {
            items.push(level_item.clone());
        }
        // Built level items in order
        for item in items {
            let (ent1, ent2) = self.build_item(world, ctx, physics_world, &entry_name, &item);

            if let Some(entity1) = ent1 {
                let mut lvl_src_writer = world.write_storage::<LevelSource>();
                lvl_src_writer.insert(entity1, LevelSource { item_index: self.build_index })
                    .expect("Couldn't create LevelSource for entity.");
                
            }

            self.build_index += 1;
        }

        let border_thickness : f32 = 25.0;
        let dim_over = border_thickness * 0.7;
        if self.bounds.solid_sides[0] { // top
            let width = self.bounds.max_x - self.bounds.min_x;
            PlatformBuilder::build(world, ctx, physics_world, self.bounds.min_x + 0.5 * width, self.bounds.min_y + 1.0,
                width / 2.0 + dim_over, border_thickness, 0.0, SpriteLayer::BGNear.to_z());
        }
        if self.bounds.solid_sides[1] { // right
            let height = self.bounds.max_y - self.bounds.min_y;
            PlatformBuilder::build(world, ctx, physics_world, self.bounds.max_x - 1.0, self.bounds.min_y + 0.5 * height,
                border_thickness, height / 2.0 + dim_over, 0.0, SpriteLayer::BGNear.to_z());
        }
        if self.bounds.solid_sides[2] { // bottom
            let width = self.bounds.max_x - self.bounds.min_x;
            PlatformBuilder::build(world, ctx, physics_world, self.bounds.min_x + 0.5 * width, self.bounds.max_y - 1.0,
                width / 2.0 + dim_over, border_thickness, 0.0, SpriteLayer::BGNear.to_z());
        }
        if self.bounds.solid_sides[3] { // left
            let height = self.bounds.max_y - self.bounds.min_y;
            PlatformBuilder::build(world, ctx, physics_world, self.bounds.min_x + 1.0, self.bounds.min_y + 0.5 * height,
                border_thickness, height / 2.0 + dim_over, 0.0, SpriteLayer::BGNear.to_z());
        }

    }
}

