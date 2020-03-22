

use ggez::{Context};
use ggez::event::{KeyCode,KeyMods};
use specs::{World, WorldExt};

use crate::resources::{InputResource,WorldAction};

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
            KeyCode::W => {
                Some(InputKey::Up)
            },
            KeyCode::S => {
                Some(InputKey::Down)
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

    pub fn key_down(world: &mut World,
        ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,) {

        match Self::map_keycode(&keycode) {
            Some(key) => match key {
                InputKey::Left => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_left(true);
                },
                InputKey::Right => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_right(true);
                },
                InputKey::Up => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_up(true);
                },
                InputKey::Down => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_down(true);
                },
                InputKey::SpaceAction => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_jump(true);
                },
                InputKey::Exit => {
                    ggez::event::quit(ctx);
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn key_up(world: &mut World,
        ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,) {

        match Self::map_keycode(&keycode) {
            Some(key) => match key {
                InputKey::Left => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_left(false);
                },
                InputKey::Right => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_right(false);
                },
                InputKey::Up => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_up(false);
                },
                InputKey::Down => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_down(false);
                },
                InputKey::SpaceAction => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_jump(false);
                },
                // InputKey::AddCircle => {
                //     let mut input = world.fetch_mut::<InputResource>();
                //     input.add_action(WorldAction::AddCircle);
                // },
                _ => {}
            },
            _ => {}
        }
    }

    pub fn mouse_button_down(world: &mut World, _ctx: &mut Context, button_index: usize) {

        match Self::map_mouse_input(&button_index) {
            Some(inp) => match inp {
                MouseInput::Left => {
                    let mut input = world.fetch_mut::<InputResource>();
                    input.set_mouse_down(true, 0);
                    println!("Mouse button pressed: {:?}, x: {}, y: {}", &inp, &input.mouse_x, &input.mouse_y);
                },
                MouseInput::Middle => {
                    let mut input = world.fetch_mut::<InputResource>();
                    //input.set;
                    input.set_mouse_down(true, 1);
                    println!("Mouse button pressed: {:?}, x: {}, y: {}", &inp, &input.mouse_x, &input.mouse_y);
                    //_ctx.current_state;
                },
                MouseInput::Right => {
                    let mut input = world.fetch_mut::<InputResource>();
                    //input.set_jump(false);
                    input.set_mouse_down(true, 2);
                    println!("Mouse button pressed: {:?}, x: {}, y: {}", &inp, &input.mouse_x, &input.mouse_y);
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
                    println!("Mouse button released: {:?}, x: {}, y: {}", &inp, &input.mouse_x, &input.mouse_y);
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
                    println!("Mouse button released: {:?}, x: {}, y: {}", &inp, &input.mouse_x, &input.mouse_y);
                },
                _ => {}
            },
            _ => {}
        }
    }

    pub fn mouse_set_pos(world: &mut World, _ctx: &mut Context, x: f32, y: f32) {
        let mut input = world.fetch_mut::<InputResource>();
        input.set_mouse_pos(x, y);
        println!("Mouse motion: x: {}, y: {}", &input.mouse_x, &input.mouse_y);
    }
}