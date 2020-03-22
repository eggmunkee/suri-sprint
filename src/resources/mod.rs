// use std::fmt;
// use std::fmt::{Display};
use std::collections::{HashMap};
use std::collections::hash_map::{Entry};
use ggez::graphics;
use ggez::graphics::{Image,Font};
use ggez::{Context,GameResult,GameError};
use ggez::conf::{WindowMode};
use specs::{World};



#[derive(Default,Debug)]
pub struct GameStateResource {
    pub window_w: f32,
    pub window_h: f32,
    pub window_mode: WindowMode,
    pub stop_double: bool,
}


#[allow(dead_code)]
pub struct ImageResources {
    pub image_lookup: HashMap<String,usize>,
    pub images: Vec<Image>,
    pub font: Font,
}

impl ImageResources {
    #[allow(dead_code)]
    pub fn has_image(&mut self, path:String) -> bool {
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
}

#[derive(Debug)]
pub enum WorldAction {
    AddCircle,
    None
}
impl Default for WorldAction {
    fn default() -> Self { WorldAction::None }
}

#[derive(Default,Debug)]
pub struct InputResource {
    pub dirs_pressed: [bool;4],
    pub jump_pressed: bool,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub mouse_down: [bool;3],
    pub actions: Vec::<WorldAction>,
}

impl InputResource {
    pub fn set_left(&mut self, press: bool) {
        self.dirs_pressed[0] = press;
    }
    pub fn set_right(&mut self, press: bool) {
        self.dirs_pressed[1] = press;
    }
    pub fn set_up(&mut self, press: bool) {
        self.dirs_pressed[2] = press;
    }
    pub fn set_down(&mut self, press: bool) {
        self.dirs_pressed[3] = press;
    }
    pub fn set_jump(&mut self, press: bool) {
        self.jump_pressed = press;
    }
    pub fn set_mouse_pos(&mut self, mouse_x: f32, mouse_y: f32) {
        self.mouse_x = mouse_x;
        self.mouse_y = mouse_y;
    }
    pub fn set_mouse_x(&mut self, mouse_x: f32) {
        self.mouse_x = mouse_x;
    }
    pub fn set_mouse_y(&mut self, mouse_y: f32) {
        self.mouse_y = mouse_y;
    }
    pub fn set_mouse_down(&mut self, mouse_down: bool, button_index: usize) {
        if button_index < 3 {
            self.mouse_down[button_index] = mouse_down;
        }
    }
    // pub fn clear_actions(&mut self) {
    //     self.actions.clear();
    // }
    // pub fn add_action(&mut self, action: WorldAction) {
    //     println!("Add action: {:?}", &action);
    //     match action {
    //         WorldAction::None => {},
    //         _a => { self.actions.push(_a); }
    //     }
    // }
    // pub fn unpop_action(&mut self) -> WorldAction {
    //     if self.actions.len() == 0 {
    //         //println!("UnPop action: NONE");
    //         return WorldAction::None;
    //     }
    //     let action_spl = self.actions.splice(1.., Vec::new());

    //     for action in action_spl {
    //         println!("UnPop action: {:?}", &action);
    //         return action;
    //     }

    //     WorldAction::None
    // }
}

pub fn add_resources(world: &mut World, ctx: &mut Context) {

    //let (win_w, win_h) = ggez::graphics::drawable_size(ctx);
    // let curr_win_mode = ggez::graphics::get_mode(ctx);
    // world.insert(GameStateResource {
    //     window_w: win_w, window_h: win_h,
    // });

    world.insert(InputResource { 
        dirs_pressed: [false,false,false,false],
        jump_pressed: false,
        mouse_x: 0.0,
        mouse_y: 0.0,
        mouse_down: [false,false,false],
        actions: Vec::new(),
    });


    let font = graphics::Font::new(ctx, "/FreeMonoBold.ttf").unwrap();

    world.insert(ImageResources {
        image_lookup: HashMap::new(),
        images: Vec::<Image>::new(),
        font: font,
    })
}