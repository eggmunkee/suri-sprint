
// use std::fmt;
// use std::fmt::{Display};
use std::collections::{HashMap};
use std::collections::hash_map::{Entry};
use ggez::graphics;
use ggez::graphics::{Image,Font};
use ggez::{Context,GameResult,GameError};
use ggez::conf::{WindowMode};
use specs::{World};
// -------------------------

use crate::physics::{PhysicsWorld};


#[allow(dead_code)]
pub struct ImageResources {
    pub image_lookup: HashMap<String,usize>,
    pub images: Vec<Image>,
    pub font: Font,
}

impl ImageResources {
    #[allow(dead_code)]
    pub fn has_image(&mut self, path: String) -> bool {
        return self.image_lookup.contains_key(&path);
    }

    #[allow(dead_code)]
    pub fn load_image(&mut self, path:String, ctx: &mut Context) -> GameResult<()> {
        let entry = self.image_lookup.entry(path.clone());
        if let Entry::Vacant(_) = entry {
            let image = Image::new(ctx, path.clone())?;
            let new_idx = self.images.len();
            self.images.push(image);
            self.image_lookup.insert(path.clone(), new_idx);
            //()
        }
        Ok(()) // ok if already loaded
    }

    #[allow(dead_code)]
    pub fn image_ref<'a>(&mut self, path:String) -> GameResult<&mut Image> {
        
        //self.load_image(path.clone(), ctx)?;

        match self.image_lookup.entry(path.clone()) {
            Entry::Occupied(o) => {
                //let o = o;
                let index = o.get().clone();
                let image = &mut self.images[index];
                Ok(image)
            },
            _ => Err(GameError::ResourceLoadError("Got image_ref for missing image".to_string()))
        }
    }

    pub fn init_images(world: &mut World, ctx: &mut Context, paths: &Vec<String>) {
        if let Some(mut images) = world.get_mut::<ImageResources>() {

            for path in paths {
                let has_image = images.has_image(path.clone());
                if !has_image {
                    images.load_image(path.clone(), ctx);
                }
            }
            
        }
    }
}
