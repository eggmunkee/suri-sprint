

use crate::core::input::{InputKey};

#[derive(Debug)]
pub enum WorldAction {
    OpenMenu,
    CloseMenu,
    CloseAllMenus,
    OpenSubMenu(String),
    ExitGame,
    RestartLevel,
    NewGame,
    ToggleFullscreen,
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
    pub fire_pressed: bool,
    pub use_pressed: bool,
    pub actions: Vec::<WorldAction>,
    pub keys_pressed: Vec::<InputKey>,
    pub exit_flag: bool,
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
    pub fn set_fire(&mut self, press: bool) {
        self.fire_pressed = press;
    }
    pub fn set_use(&mut self, press: bool) {
        self.use_pressed = press;
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
    pub fn clear_actions(&mut self) {
        self.actions.clear();
    }
    pub fn add_action(&mut self, action: WorldAction) {
        println!("Add action: {:?}", &action);
        match action {
            WorldAction::None => {},
            _a => { self.actions.push(_a); }
        }
    }
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