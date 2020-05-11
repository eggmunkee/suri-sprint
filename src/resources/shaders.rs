
// use std::fmt;
// use std::fmt::{Display};
use std::collections::{HashMap};
use std::collections::hash_map::{Entry};
use ggez::graphics::{Shader,EmptyConst,BlendMode};
use ggez::graphics::{Image,Font};
use ggez::{Context,GameResult,GameError};
use ggez::conf::{WindowMode};
use specs::{World};
// -------------------------

use crate::physics::{PhysicsWorld};
use crate::components::sprite::{ShaderConfig};
use crate::conf::{get_ron_config};


#[allow(dead_code)]
pub struct ShaderResources {
    pub shader_lookup: HashMap<String,usize>,
    pub shaders: Vec<Shader<EmptyConst>>,
}

impl ShaderResources {
    pub fn new() -> Self {
        ShaderResources {
            shader_lookup: HashMap::<String,usize>::new(),
            shaders: vec![],
        }
    }

    #[allow(dead_code)]
    pub fn has_shader(&mut self, path: String) -> bool {
        return self.shader_lookup.contains_key(&path);
    }

    pub fn shader_factory(&self, name: String, path: String, ctx: &mut Context) -> Option<Shader<EmptyConst>> {
        println!("SHADER=FACTORY$> {}, {}", &name, &path);
        if let Some(shader_config) = get_ron_config::<ShaderConfig>(path) {
            let data = ggez::graphics::EmptyConst {};
            let vert_path = shader_config.vert_path;
            let frag_path = shader_config.frag_path;
            println!("Shader Factory paths: {}, {}", &vert_path, &frag_path);

            if let Ok(shader) = Shader::<EmptyConst>::new(ctx, vert_path, frag_path, data, name, Some(&vec![BlendMode::Alpha]) ) {
                println!("Shader: {:?}", &shader);
                Some(shader)
            }
            else {
                println!("Shader could not be loaded.");
                ggez::event::quit(ctx);
                None
            }
        }
        else {
            None
        }

    }

    #[allow(dead_code)]
    pub fn load_shader(&mut self, name: String, path:String, ctx: &mut Context) -> GameResult<()> {
        let entry = self.shader_lookup.entry(name.clone());
        println!("Shader ref {}", &name);
        if let Entry::Vacant(_) = entry {
            //let image = Image::new(ctx, path.clone())?;
            //let new_idx = self.images.len();
            println!("Shader vacant");
            if let Some(shader) = self.shader_factory(name.clone(), path.clone(), ctx) {
                let new_idx = self.shaders.len();
                self.shaders.push(shader);
                println!("Shader added at index: {}", &new_idx);
                self.shader_lookup.insert(name.clone(), new_idx);
            }
            //()
        }
        Ok(()) // ok if already loaded
    }

    #[allow(dead_code)]
    pub fn shader_ref<'a>(&mut self, name:String) -> GameResult<&mut Shader<EmptyConst>> {
        
        //self.load_image(path.clone(), ctx)?;

        match self.shader_lookup.entry(name.clone()) {
            Entry::Occupied(o) => {
                //let o = o;
                let index = o.get().clone();
                let shader = &mut self.shaders[index];
                //println!("Shader: {:?}", &shader);
                Ok(shader)
            },
            _ => Err(GameError::ResourceLoadError("Got shader_ref for missing shader".to_string()))
        }
    }

    // pub fn init_images(world: &mut World, ctx: &mut Context, paths: &Vec<String>) {
    //     if let Some(mut shaders) = world.get_mut::<ShaderResources>() {
    //         let mut i : i32 = 0;
    //         for path in paths {
    //             let has_image = shaders.has_shader(path.clone());
    //             if !has_image {
    //                 shaders.load_shader(path.clone(), ctx);
    //             }
    //         }
            
    //     }
    // }
}