
use std::convert::{From};
use ggez::{Context};
use ggez::event::{KeyCode,KeyMods,Axis,Button,GamepadId};
use specs::{World, WorldExt};
use serde::{Serialize,Deserialize,de::DeserializeOwned};


use crate::core::compat::{VKeyCode,VButton};
use crate::conf::{get_ron_config, save_ron_config};
use crate::resources::{InputResource,WorldAction};
use crate::entities::meow::{MeowBuilder};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum MouseInput {
    Left, Middle, Right
}
#[derive(Debug,Clone,PartialEq,Serialize,Deserialize)]
pub enum InputKey {
    P1Left,
    P1Right,
    P1Up,
    P1Down,
    P1PrimaryAction,
    P1UseAction,
    Pause,
    SlowMode,
    CheatGoAnywhere,
    Exit,
    VolumeDown,
    VolumeUp,
    ZoomOut,
    ZoomIn,
    EditMode,
    Fullscreen,
    ConsoleKey,
    RestartLevel,
    None
}

impl Default for InputKey {
    fn default() -> Self {
        InputKey::None
    }
}

// impl Serialize for KeyCode;
// impl Deserialize for KeyCode;

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct InputSetting {
    pub game_key: InputKey,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub code: Option<VKeyCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub button: Option<VButton>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub default_code: Option<VKeyCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub default_button: Option<VButton>,
}

impl InputSetting {
    pub fn code(keycode: KeyCode, game_key: InputKey) -> InputSetting {
        InputSetting {
            game_key: game_key,
            code: Some(VKeyCode::from(keycode)),
            button: None,
            default_code: Some(VKeyCode::from(keycode)),
            default_button: None,
        }
    }

    pub fn button(button: Button, game_key: InputKey) -> InputSetting {
        InputSetting {
            game_key: game_key,            
            code: None,
            button: Some(VButton::from(button)),
            default_code: None,
            default_button: Some(VButton::from(button)),
        }
    }
}

#[macro_export]
macro_rules! key_setting {
    ( $cls:ident, $raw:ident, $map:ident ) => {
        $cls.input_settings.push(InputSetting::code(KeyCode::$raw, InputKey::$map))
    };
}

#[macro_export]
macro_rules! button_setting {
    ( $cls:ident, $raw:ident, $map:ident ) => {
        $cls.input_settings.push(InputSetting::button(Button::$raw, InputKey::$map))
    };
}

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct InputMap {
    pub input_settings: Vec<InputSetting>,
}

impl InputMap {
    pub fn load_or_init_inputmap() -> Self {
        let im_result = get_ron_config::<InputMap>("controls".to_string());
        if im_result.is_some() {
            println!("Input map loaded...");
            im_result.unwrap()
        }   
        else {
            println!("Input map defaulting...");
            let default = Self::new();

            save_ron_config::<InputMap>("controls".to_string(), &default);

            default
        }     
    }

    pub fn new() -> Self {
        let mut input_map = Self { input_settings: vec![] };
    
        // Keyboard Codes
        key_setting!(input_map, W, P1Up);
        key_setting!(input_map, LControl, P1Up);
        key_setting!(input_map, A, P1Left);
        key_setting!(input_map, D, P1Right);
        key_setting!(input_map, S, P1Down);
        key_setting!(input_map, Up, P1Up);
        key_setting!(input_map, Down, P1Down);
        key_setting!(input_map, Right, P1Right);
        key_setting!(input_map, Left, P1Left);
        key_setting!(input_map, Space, P1PrimaryAction);
        key_setting!(input_map, E, P1UseAction);
        key_setting!(input_map, Escape, Exit);
        key_setting!(input_map, P, Pause);
        key_setting!(input_map, O, SlowMode);
        key_setting!(input_map, Return, Pause);
        key_setting!(input_map, F1, EditMode);
        key_setting!(input_map, Subtract, ZoomOut);
        key_setting!(input_map, Add, ZoomIn);
        key_setting!(input_map, RBracket, VolumeUp);
        key_setting!(input_map, LBracket, VolumeDown);
        key_setting!(input_map, F11, Fullscreen);
        key_setting!(input_map, G, CheatGoAnywhere);
        key_setting!(input_map, Grave, ConsoleKey);
        key_setting!(input_map, R, RestartLevel);
        

        // Gamepad Buttons
        button_setting!(input_map, DPadUp, P1Up);
        button_setting!(input_map, South, P1Up);
        button_setting!(input_map, DPadLeft, P1Left);
        button_setting!(input_map, DPadRight, P1Right);
        button_setting!(input_map, DPadDown, P1Down);
        button_setting!(input_map, Start, Pause);
        button_setting!(input_map, Select, Exit);
        button_setting!(input_map, West, P1PrimaryAction);
        button_setting!(input_map, North, P1UseAction);

        // input_map.input_settings.push(InputSetting::button(Button::DPadUp, InputKey::P1Up));
        // input_map.input_settings.push(InputSetting::button(Button::South, InputKey::P1Up));
        // input_map.input_settings.push(InputSetting::button(Button::DPadLeft, InputKey::P1Left));
        // input_map.input_settings.push(InputSetting::button(Button::DPadRight, InputKey::P1Right));
        // input_map.input_settings.push(InputSetting::button(Button::DPadDown, InputKey::P1Down));
        // input_map.input_settings.push(InputSetting::button(Button::Start, InputKey::Pause));
        // input_map.input_settings.push(InputSetting::button(Button::Select, InputKey::Exit));
        // input_map.input_settings.push(InputSetting::button(Button::West, InputKey::P1PrimaryAction));
        // input_map.input_settings.push(InputSetting::button(Button::North, InputKey::P1UseAction));

        input_map
    }

    pub fn map_mouse_input(&self, button_index: &usize) -> Option<MouseInput> {
        //Some(InputKey::P1Left)

        match &button_index {
            0 => {
                Some(MouseInput::Left)
            },
            1 => {
                Some(MouseInput::Middle)
            },
            2 => {
                Some(MouseInput::Right)
            },            
            _ => None
        }
    }
    pub fn map_keycode(&self, keycode: &KeyCode) -> Option<InputKey> {
        //Some(InputKey::P1Left)
        let vkeycode = VKeyCode::from(keycode);
        for setting in self.input_settings.iter() {
            if let Some(ref code) = setting.code {
                if code == &vkeycode {
                    return Some(setting.game_key.clone());
                }
            }
        }

        // match keycode {
        //     KeyCode::A => {
        //         Some(InputKey::P1Left)
        //     },
        //     KeyCode::D => {
        //         Some(InputKey::P1Right)
        //     },
        //     KeyCode::W | KeyCode::LShift | KeyCode::LControl => {
        //         Some(InputKey::P1Up)
        //     },
        //     KeyCode::S => {
        //         Some(InputKey::P1Down)
        //     },
        //     KeyCode::P | KeyCode::Return => {
        //         Some(InputKey::Pause)
        //     },
        //     KeyCode::Space => {
        //         Some(InputKey::P1PrimaryAction)
        //     },
        //     KeyCode::J => {
        //         Some(InputKey::AddCircle)
        //     },
        //     KeyCode::Escape => {
        //         Some(InputKey::Exit)
        //     },
        //     KeyCode::F1 => {
        //         Some(InputKey::EditMode)
        //     },
        //     _ => None
        // }

        None
    }
    pub fn map_gamepadcode(&self, button: &Button) -> Option<InputKey> {
        //Some(InputKey::P1Left)
        let vbutton = VButton::from(button);
        for setting in self.input_settings.iter() {
            if let Some(ref btn) = setting.button {
                if btn == &vbutton {
                    return Some(setting.game_key.clone());
                }
            }
        }

        // match button {
        //     Button::DPadLeft => {
        //         Some(InputKey::P1Left)
        //     },
        //     Button::DPadRight => {
        //         Some(InputKey::P1Right)
        //     },
        //     Button::DPadUp | Button::South => {
        //         Some(InputKey::P1Up)
        //     },
        //     Button::DPadDown => {
        //         Some(InputKey::P1Down)
        //     },
        //     Button::West => {
        //         Some(InputKey::P1PrimaryAction)
        //     },
        //     Button::North | Button::East => {
        //         Some(InputKey::AddCircle)
        //     },
        //     Button::Start => {
        //         Some(InputKey::Pause)
        //     },
        //     Button::Select => {
        //         Some(InputKey::Exit)
        //     },
        //     _ => None
        // }
        None
    }

    pub fn key_down(&self, world: &mut World,
        ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,) -> Option<InputKey> {

        match self.map_keycode(&keycode) {
            Some(key) => {

                let mut input = world.fetch_mut::<InputResource>();
                input.keys_pressed.push(key.clone());
                drop(input);

                match &key {
                    InputKey::P1Left => {
                        let mut input = world.fetch_mut::<InputResource>();
                        input.set_left(true);
                        //Some(InputKey::P1Left)
                    },
                    InputKey::P1Right => {
                        let mut input = world.fetch_mut::<InputResource>();
                        input.set_right(true);
                        //Some(InputKey::P1Right)
                    },
                    InputKey::P1Up => {
                        let mut input = world.fetch_mut::<InputResource>();
                        input.set_up(true);
                        //Some(InputKey::P1Up)
                    },
                    InputKey::P1Down => {
                        let mut input = world.fetch_mut::<InputResource>();
                        input.set_down(true);
                        //Some(InputKey::P1Down)
                    },
                    InputKey::P1PrimaryAction => {
                        let mut input = world.fetch_mut::<InputResource>();
                        input.set_fire(true);
                        //Some(InputKey::P1PrimaryAction)

                        //MeowBuilder::build(world, ctx, physics_world);
                    },
                    _ => {
                        // let mut input = world.fetch_mut::<InputResource>();
                        // Some(other)
                    }
                }

                Some(key)
            },
            _ => { None }
        }
    }

    pub fn gamepad_button_down(&self, world: &mut World,
        ctx: &mut Context,
        btn: Button,
        id: GamepadId) -> Option<InputKey> {

        match self.map_gamepadcode(&btn) {
            Some(key) => match key {
                InputKey::P1Left => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_left(true);
                    Some(InputKey::P1Left)
                },
                InputKey::P1Right => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_right(true);
                    Some(InputKey::P1Right)
                },
                InputKey::P1Up => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_up(true);
                    Some(InputKey::P1Up)
                },
                InputKey::P1Down => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_down(true);
                    Some(InputKey::P1Down)
                },
                InputKey::P1PrimaryAction => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_fire(true);
                    Some(InputKey::P1PrimaryAction)

                    //MeowBuilder::build(world, ctx, physics_world);
                },
                other => { Some(other) }
            },
            _ => { None }
        }
    }

    // pub fn clear_text(&self, world: &mut World) {
    //     //world.fetch_mut::<InputResource>().cmd_text.clear();
    // }

    pub fn text_typed(&self, world: &mut World, c: char) {
        
        let mut input = world.fetch_mut::<InputResource>();
        if input.cmd_text.len() < 380 {
            input.cmd_text.push(c);
        }
    }

    pub fn key_up(&self, world: &mut World,
        ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,) -> Option<InputKey> {

        match self.map_keycode(&keycode) {
            Some(key) => match key {
                InputKey::P1Left => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_left(false);
                    Some(InputKey::P1Left)
                },
                InputKey::P1Right => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_right(false);
                    Some(InputKey::P1Right)
                },
                InputKey::P1Up => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_up(false);
                    Some(InputKey::P1Up)
                },
                InputKey::P1Down => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_down(false);
                    Some(InputKey::P1Down)
                },
                InputKey::P1PrimaryAction => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_fire(false);
                    Some(InputKey::P1PrimaryAction)
                },
                // InputKey::AddCircle => {
                //     let mut input = world.fetch_mut::<InputResource>();
                //     input.add_action(WorldAction::AddCircle);
                // },
                other => { Some(other) }
            },
            _ => { None }
        }
    }

    pub fn gamepad_button_up(&self, world: &mut World,
        ctx: &mut Context,
        btn: Button,
        id: GamepadId) -> Option<InputKey> {

        match self.map_gamepadcode(&btn) {
            Some(key) => match key {
                InputKey::P1Left => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_left(false);
                    Some(InputKey::P1Left)
                },
                InputKey::P1Right => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_right(false);
                    Some(InputKey::P1Right)
                },
                InputKey::P1Up => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_up(false);
                    Some(InputKey::P1Up)
                },
                InputKey::P1Down => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_down(false);
                    Some(InputKey::P1Down)
                },
                InputKey::P1PrimaryAction => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_fire(false);
                    Some(InputKey::P1PrimaryAction)
                },
                // InputKey::AddCircle => {
                //     let mut input = world.fetch_mut::<InputResource>();
                //     input.add_action(WorldAction::AddCircle);
                // },
                other => { Some(other) }
            },
            _ => { None }
        }
    }

    pub fn mouse_button_down(&self, world: &mut World, _ctx: &mut Context, button_index: usize) {

        match self.map_mouse_input(&button_index) {
            Some(inp) => match inp {
                MouseInput::Left => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_mouse_down(true, 0);
                    //println!("Mouse button pressed: {:?}, x: {}, y: {}", &inp, &input.mouse_x, &input.mouse_y);
                },
                MouseInput::Middle => {
                    let mut input = world.fetch_mut::<InputResource>();
                    //input.set;
                    input.set_mouse_down(true, 1);
                    //println!("Mouse button pressed: {:?}, x: {}, y: {}", &inp, &input.mouse_x, &input.mouse_y);
                    //_ctx.current_state;
                },
                MouseInput::Right => {
                    let mut input = world.fetch_mut::<InputResource>();
                    //input.set_jump(false);
                    input.set_mouse_down(true, 2);
                    //println!("Mouse button pressed: {:?}, x: {}, y: {}", &inp, &input.mouse_x, &input.mouse_y);
                },
                _ => {}
            },
            _ => {}
        }
    }

    pub fn mouse_button_up(&self, world: &mut World, _ctx: &mut Context, button_index: usize) {

        match self.map_mouse_input(&button_index) {
            Some(inp) => match inp {
                MouseInput::Left => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_mouse_down(false, 0);
                    //println!("Mouse button released: {:?}, x: {}, y: {}", &inp, &input.mouse_x, &input.mouse_y);
                },
                MouseInput::Middle => {
                    let mut input = world.fetch_mut::<InputResource>();
                    //input.set;
                    input.set_mouse_down(false, 1);
                },
                MouseInput::Right => {
                    let mut input = world.fetch_mut::<InputResource>();
                    //input.set_jump(false);
                    input.set_mouse_down(false, 2);
                    //println!("Mouse button released: {:?}, x: {}, y: {}", &inp, &input.mouse_x, &input.mouse_y);
                },
                _ => {}
            },
            _ => {}
        }
    }

    pub fn mouse_set_pos(&self, world: &mut World, _ctx: &mut Context, x: f32, y: f32) {
        let mut input = world.fetch_mut::<InputResource>();
        input.set_mouse_pos(x, y);
        //println!("Mouse motion: x: {}, y: {}", &input.mouse_x, &input.mouse_y);
    }
}