

use ggez::{Context};
use ggez::event::{KeyCode,KeyMods,Axis,Button,GamepadId};
use specs::{World, WorldExt};

use crate::resources::{InputResource,WorldAction};
use crate::entities::meow::{MeowBuilder};

#[derive(Debug,Clone)]
pub enum MouseInput {
    Left, Middle, Right
}
#[derive(Debug,Clone,PartialEq)]
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

pub struct InputSetting {
    pub game_key: InputKey,
    pub code: Option<KeyCode>,
    pub button: Option<Button>,
    pub default_code: Option<KeyCode>,
    pub default_button: Option<Button>,
}

impl InputSetting {
    pub fn code(keycode: KeyCode, game_key: InputKey) -> InputSetting {
        InputSetting {
            game_key: game_key,
            code: Some(keycode),
            button: None,
            default_code: Some(keycode),
            default_button: None,
        }
    }

    pub fn button(button: Button, game_key: InputKey) -> InputSetting {
        InputSetting {
            game_key: game_key,            
            code: None,
            button: Some(button),
            default_code: None,
            default_button: Some(button),
        }
    }
}

pub struct InputMap {
    pub input_settings: Vec<InputSetting>,
}

impl InputMap {
    pub fn new() -> Self {
        let mut input_map = Self { input_settings: vec![] };
    
        // Keyboard Codes
        input_map.input_settings.push(InputSetting::code(KeyCode::W, InputKey::P1Up));
        input_map.input_settings.push(InputSetting::code(KeyCode::LShift, InputKey::P1Up));
        input_map.input_settings.push(InputSetting::code(KeyCode::LControl, InputKey::P1Up));
        input_map.input_settings.push(InputSetting::code(KeyCode::A, InputKey::P1Left));
        input_map.input_settings.push(InputSetting::code(KeyCode::D, InputKey::P1Right));
        input_map.input_settings.push(InputSetting::code(KeyCode::S, InputKey::P1Down));
        input_map.input_settings.push(InputSetting::code(KeyCode::Up, InputKey::P1Up));
        input_map.input_settings.push(InputSetting::code(KeyCode::Down, InputKey::P1Down));
        input_map.input_settings.push(InputSetting::code(KeyCode::Right, InputKey::P1Right));
        input_map.input_settings.push(InputSetting::code(KeyCode::Left, InputKey::P1Left));
        input_map.input_settings.push(InputSetting::code(KeyCode::Space, InputKey::P1PrimaryAction));
        input_map.input_settings.push(InputSetting::code(KeyCode::E, InputKey::P1UseAction));
        input_map.input_settings.push(InputSetting::code(KeyCode::Escape, InputKey::Exit));
        input_map.input_settings.push(InputSetting::code(KeyCode::P, InputKey::Pause));
        input_map.input_settings.push(InputSetting::code(KeyCode::O, InputKey::SlowMode));
        input_map.input_settings.push(InputSetting::code(KeyCode::Return, InputKey::Pause));
        input_map.input_settings.push(InputSetting::code(KeyCode::F1, InputKey::EditMode));
        input_map.input_settings.push(InputSetting::code(KeyCode::Subtract, InputKey::ZoomOut));
        input_map.input_settings.push(InputSetting::code(KeyCode::Add, InputKey::ZoomIn));
        input_map.input_settings.push(InputSetting::code(KeyCode::RBracket, InputKey::VolumeUp));
        input_map.input_settings.push(InputSetting::code(KeyCode::LBracket, InputKey::VolumeDown));
        input_map.input_settings.push(InputSetting::code(KeyCode::F11, InputKey::Fullscreen));
        input_map.input_settings.push(InputSetting::code(KeyCode::G, InputKey::CheatGoAnywhere));
        input_map.input_settings.push(InputSetting::code(KeyCode::Grave, InputKey::ConsoleKey));
        input_map.input_settings.push(InputSetting::code(KeyCode::R, InputKey::RestartLevel));
        

        // Gamepad Buttons
        input_map.input_settings.push(InputSetting::button(Button::DPadUp, InputKey::P1Up));
        input_map.input_settings.push(InputSetting::button(Button::South, InputKey::P1Up));
        input_map.input_settings.push(InputSetting::button(Button::DPadLeft, InputKey::P1Left));
        input_map.input_settings.push(InputSetting::button(Button::DPadRight, InputKey::P1Right));
        input_map.input_settings.push(InputSetting::button(Button::DPadDown, InputKey::P1Down));
        input_map.input_settings.push(InputSetting::button(Button::Start, InputKey::Pause));
        input_map.input_settings.push(InputSetting::button(Button::Select, InputKey::Exit));
        input_map.input_settings.push(InputSetting::button(Button::West, InputKey::P1PrimaryAction));
        input_map.input_settings.push(InputSetting::button(Button::North, InputKey::P1UseAction));

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

        for setting in self.input_settings.iter() {
            if let Some(ref code) = setting.code {
                if code == keycode {
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

        for setting in self.input_settings.iter() {
            if let Some(ref btn) = setting.button {
                if btn == button {
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