

use ggez::{Context};
use ggez::event::{KeyCode,KeyMods,Axis,Button,GamepadId};
use specs::{World, WorldExt};

use crate::resources::{InputResource,WorldAction};
use crate::entities::meow::{MeowBuilder};

#[derive(Debug)]
pub enum MouseInput {
    Left, Middle, Right
}
#[derive(Debug)]
pub enum InputKey {
    Left,
    Right,
    Up,
    Down,
    SpaceAction,
    AddCircle,
    Pause,
    Exit
}


pub struct InputMap;

impl InputMap {
    pub fn map_mouse_input(button_index: &usize) -> Option<MouseInput> {
        //Some(InputKey::Left)

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
    pub fn map_keycode(keycode: &KeyCode) -> Option<InputKey> {
        //Some(InputKey::Left)

        match keycode {
            KeyCode::A => {
                Some(InputKey::Left)
            },
            KeyCode::D => {
                Some(InputKey::Right)
            },
            KeyCode::W | KeyCode::LShift | KeyCode::LControl => {
                Some(InputKey::Up)
            },
            KeyCode::S => {
                Some(InputKey::Down)
            },
            KeyCode::P | KeyCode::Return => {
                Some(InputKey::Pause)
            },
            KeyCode::Space => {
                Some(InputKey::SpaceAction)
            },
            KeyCode::J => {
                Some(InputKey::AddCircle)
            },
            KeyCode::Escape => {
                Some(InputKey::Exit)
            },
            _ => None
        }
    }
    pub fn map_gamepadcode(button: &Button) -> Option<InputKey> {
        //Some(InputKey::Left)

        match button {
            Button::DPadLeft => {
                Some(InputKey::Left)
            },
            Button::DPadRight => {
                Some(InputKey::Right)
            },
            Button::DPadUp | Button::South => {
                Some(InputKey::Up)
            },
            Button::DPadDown => {
                Some(InputKey::Down)
            },
            Button::West => {
                Some(InputKey::SpaceAction)
            },
            Button::North | Button::East => {
                Some(InputKey::AddCircle)
            },
            Button::Start => {
                Some(InputKey::Pause)
            },
            Button::Select => {
                Some(InputKey::Exit)
            },
            _ => None
        }
    }

    pub fn key_down(world: &mut World,
        ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,) -> Option<InputKey> {

        match Self::map_keycode(&keycode) {
            Some(key) => match key {
                InputKey::Left => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_left(true);
                    Some(InputKey::Left)
                },
                InputKey::Right => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_right(true);
                    Some(InputKey::Right)
                },
                InputKey::Up => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_up(true);
                    Some(InputKey::Up)
                },
                InputKey::Down => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_down(true);
                    Some(InputKey::Down)
                },
                InputKey::SpaceAction => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_fire(true);
                    Some(InputKey::SpaceAction)

                    //MeowBuilder::build(world, ctx, physics_world);
                },
                other => {
                    Some(other)
                }
            },
            _ => { None }
        }
    }

    pub fn gamepad_button_down(world: &mut World,
        ctx: &mut Context,
        btn: Button,
        id: GamepadId) -> Option<InputKey> {

        match Self::map_gamepadcode(&btn) {
            Some(key) => match key {
                InputKey::Left => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_left(true);
                    Some(InputKey::Left)
                },
                InputKey::Right => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_right(true);
                    Some(InputKey::Right)
                },
                InputKey::Up => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_up(true);
                    Some(InputKey::Up)
                },
                InputKey::Down => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_down(true);
                    Some(InputKey::Down)
                },
                InputKey::SpaceAction => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_fire(true);
                    Some(InputKey::SpaceAction)

                    //MeowBuilder::build(world, ctx, physics_world);
                },
                other => { Some(other) }
            },
            _ => { None }
        }
    }

    pub fn key_up(world: &mut World,
        ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,) -> Option<InputKey> {

        match Self::map_keycode(&keycode) {
            Some(key) => match key {
                InputKey::Left => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_left(false);
                    Some(InputKey::Left)
                },
                InputKey::Right => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_right(false);
                    Some(InputKey::Right)
                },
                InputKey::Up => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_up(false);
                    Some(InputKey::Up)
                },
                InputKey::Down => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_down(false);
                    Some(InputKey::Down)
                },
                InputKey::SpaceAction => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_fire(false);
                    Some(InputKey::SpaceAction)
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

    pub fn gamepad_button_up(world: &mut World,
        ctx: &mut Context,
        btn: Button,
        id: GamepadId) -> Option<InputKey> {

        match Self::map_gamepadcode(&btn) {
            Some(key) => match key {
                InputKey::Left => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_left(false);
                    Some(InputKey::Left)
                },
                InputKey::Right => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_right(false);
                    Some(InputKey::Right)
                },
                InputKey::Up => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_up(false);
                    Some(InputKey::Up)
                },
                InputKey::Down => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_down(false);
                    Some(InputKey::Down)
                },
                InputKey::SpaceAction => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_fire(false);
                    Some(InputKey::SpaceAction)
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

    pub fn mouse_button_down(world: &mut World, _ctx: &mut Context, button_index: usize) {

        match Self::map_mouse_input(&button_index) {
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

    pub fn mouse_button_up(world: &mut World, _ctx: &mut Context, button_index: usize) {

        match Self::map_mouse_input(&button_index) {
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

    pub fn mouse_set_pos(world: &mut World, _ctx: &mut Context, x: f32, y: f32) {
        let mut input = world.fetch_mut::<InputResource>();
        input.set_mouse_pos(x, y);
        //println!("Mouse motion: x: {}, y: {}", &input.mouse_x, &input.mouse_y);
    }
}