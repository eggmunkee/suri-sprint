use ggez::{Context, GameResult, GameError};
use ggez::graphics;
use ggez::graphics::{Image};
use ggez::nalgebra as na;
use specs::{Builder,Entity,EntityBuilder,World,WorldExt};

use crate::game_state::{GameState};
//use crate::resources::{ImageResources};
use crate::components::{Position, Velocity,DisplayComp,DisplayCompType};
use crate::components::collision::{Collision};
use crate::components::player::{PlayerComponent,CharacterDisplayComponent};
use crate::systems::*;

pub struct SuriBuilder;

impl SuriBuilder {
    pub fn build(world: &mut World, ctx: &mut Context, x: f32, y: f32) -> Entity {

        let mut player_comp = PlayerComponent::new();
        player_comp.player_name.clear();
        player_comp.player_name.push_str("Noah");
        let entity = world.create_entity()
        .with(Position { x: x, y: y })
        .with(Velocity { x: 0.0, y: 0.0, gravity: true, frozen: false })
        .with(DisplayComp { circle: false, display_type: DisplayCompType::DrawSelf })
        .with(CharacterDisplayComponent::new(ctx, &"/suri-1-r.png".to_string()))
        .with(Collision::new_specs(75.0,0.02))
        .with(player_comp)
        .build();

        //let entId = entity.id();

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

