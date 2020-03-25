use ggez::{Context, GameResult, GameError};
use ggez::graphics;
use ggez::graphics::{Image};
use ggez::nalgebra as na;
use specs::{Builder,Entity,EntityBuilder,World,WorldExt};
use wrapped2d::user_data::*;

use crate::conf::*;
use crate::game_state::{GameState};
use crate::resources::{GameStateResource,ImageResources};
use crate::components::{Position, Velocity,DisplayComp,DisplayCompType};
use crate::components::sprite::{SpriteComponent,SpriteConfig};
use crate::components::collision::{Collision};
use crate::components::player::{PlayerComponent,CharacterDisplayComponent};
use crate::systems::*;
use crate::physics::{PhysicsWorld,CollisionCategory};

pub struct SuriBuilder;

impl SuriBuilder {

    // pub fn get_sprite_paths() -> Vec<String> {
    //     vec!["/suri-spritesheet.png".to_string()]
    // }

    pub fn build(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, x: f32, y: f32) -> Entity {

        //ImageResources::init_images(world, ctx, &Self::get_sprite_paths());

        // Init Suri images from SpriteConfig
        let maybe_config = get_ron_config::<SpriteConfig>("entities/suri".to_string());
        let sprite_config = maybe_config.unwrap();
        SpriteConfig::init_images(world, ctx, sprite_config.path.clone());

        let mut player_comp = PlayerComponent::new();
        player_comp.player_name.clear();
        player_comp.player_name.push_str("Noah");

        let mut collision = Collision::new_specs(3.0,0.001, 18.0, 18.0);
        //collision.dim_1 = 18.0;
        //collision.dim_2 = 18.0;
        collision.pos.x = x;
        collision.pos.y = y;
        collision.collision_category = CollisionCategory::Player;
        collision.collision_mask.clear();
        collision.collision_mask.push(CollisionCategory::Level);
        collision.collision_mask.push(CollisionCategory::Ghost);
        collision.collision_mask.push(CollisionCategory::Player);

        collision.create_dynamic_body_box_fixed_rot(physics_world);

        let body_handle_clone = collision.body_handle.clone();

        let entity = world.create_entity()
        .with(Position { x: x, y: y })
        .with(Velocity { x: 0.0, y: 0.0, gravity: true, frozen: false })
        .with(DisplayComp { circle: false, display_type: DisplayCompType::DrawSelf })
        .with(CharacterDisplayComponent::new(ctx, &sprite_config.path))
        .with(collision)
        .with(player_comp)
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

/*
    // pub fn draw(game_state: &mut GameState, entity: &Entity, ctx: &mut Context) -> GameResult<()> {
    //     println!("PlayerEntity - draw()");
    //     //let mut image_res = game_state.world.fetch::<ImageResources>();
    //     let position_storage = game_state.world.read_storage::<Position>();

    //     let pos_entry = position_storage.get(*entity).unwrap();

    //     let mut draw_ok = true;
    //     if let Ok(image) = Image::new(ctx, "/icon.png".to_string()) {
    //         //let img_ref : Image = image;
    //         if let Err(_) = graphics::draw(ctx, &image, (na::Point2::new(pos_entry.x +15.0, pos_entry.y+15.0),)) {
    //             draw_ok = false;
    //         }
    //     }
    //     else {
    //         draw_ok = false;
    //     }

    //     if draw_ok {
    //         Ok(())
    //     }
    //     else {
    //         Err(GameError::RenderError("PlayerEntity render error".to_string()))
    //     }

    // }

// Load any player related images here
        // 
        // if let Some(img_res) = world.get_mut::<ImageResources>() {
        //     let img_path = String::from("/icon.png");
        //     // check for image existence
        //     //println!("ImageResources has {}? {}", &img_path, &(img_res.has_image(img_path.clone())) );

        //     // load image once (if not set)
        //     img_res.load_image(img_path.clone(), ctx);
        //     //println!("ImageResources has {}? {}", &img_path, &(img_res.has_image(img_path.clone())) );

        //     // get image reference from path
        //     let img : &Image = img_res.image_ref(img_path).unwrap();
        //     println!("Image: {:?}", img);
        //     drop(img);

        // }

*/

