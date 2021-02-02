

use ggez::{Context};
use specs::{Builder,Entity,World,WorldExt};

use crate::components::{Position};
//use crate::components::ball::*;
use crate::components::sprite::*;
use crate::components::collision::{Collision};
use crate::resources::{ImageResources};
use crate::core::physics::{PhysicsWorld,CollisionCategory};


pub struct UIBuilder;

impl UIBuilder {

    pub fn get_sprite_paths() -> Vec<String> {
        vec!["/icon.png".to_string()]
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

    pub fn build_icon(world: &mut World, ctx: &mut Context) {

        Self::init_images(world, ctx);

        world.create_entity()
        .with(Position { x: 50.0, y: 50.0 })
        //.with(DisplayComp { circle: false, display_type: DisplayCompType::DrawSelf })
        .with(SpriteComponent::new(ctx, &"/icon.png".to_string(), 1000.0))
        //.with(Collision::new_circle(20.0))
        .build();

        world.create_entity()
        .with(Position { x: 100.0, y: 50.0 })
        //.with(DisplayComp { circle: false, display_type: DisplayCompType::DrawSelf })
        .with(SpriteComponent::new(ctx, &"/icon.png".to_string(), 1000.0))
        //.with(Collision::new_circle(20.0))
        .build();

        world.create_entity()
        .with(Position { x: 150.0, y: 50.0 })
        //.with(DisplayComp { circle: false, display_type: DisplayCompType::DrawSelf })
        .with(SpriteComponent::new(ctx, &"/icon.png".to_string(), 1000.0))
        //.with(Collision::new_circle(20.0))
        .build();
    }

}