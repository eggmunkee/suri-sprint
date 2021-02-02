
use ggez::nalgebra as na;
use specs::{Component, DenseVecStorage, World, WorldExt};
use specs_derive::*;
use wrapped2d::b2;
use rand::prelude::*;
use serde::{Deserialize,de::DeserializeOwned,Serialize};

use crate::components::sprite::{SpriteComponent};
use crate::components::collision::{Collision};
use crate::core::physics;
use crate::core::physics::{PhysicsWorld};
use crate::entities::level_builder::{ItemLogic};

#[derive(Debug,Clone,Deserialize,Serialize)]
pub enum ConnectionType {
    SwitchOnce,
    Switch,
    NotSwitch,
}

impl Default for ConnectionType {
    fn default() -> Self {
        Self::Switch
    }
}

#[derive(Debug,Copy,Clone,Deserialize,Serialize)]
pub enum LogicOpType {
    And,
    Or,
    Odd,
    Even,
}

impl Default for LogicOpType {
    fn default() -> Self {
        Self::And
    }
}


#[derive(Debug,Default,Deserialize,Serialize)]
pub struct LogicConnection {
    pub from: String,
    pub to: String,
    pub conn_type: ConnectionType,
}

impl LogicConnection {
    pub fn new(frm: String, t: String, cntype: ConnectionType) -> Self {
        LogicConnection {
            from: frm,
            to: t,
            conn_type: cntype,
        }
    }
}


#[derive(Debug,Component)]
#[storage(DenseVecStorage)]
pub struct LogicComponent {
    pub id: String,
    // Static Logic Inputs
    pub initial_value: bool, // base value
    pub logic_op: LogicOpType,
    pub logic_type: ConnectionType,

    // Dynamic Values
    pub input_value: Option<bool>, // If the external input has a signal and its value
    pub is_active: bool, // If this node is generating an active signal
    pub value: bool, // the node's output result value

    // History keeping
    pub last_value: bool,
    pub last_input: Option<bool>,
    pub last_active: bool,
    pub change_count: i32,
    pub frozen: bool,
}

impl LogicComponent {
    pub fn new(id: String, is_enabled: bool, logic_op_opt: Option<LogicOpType>) -> LogicComponent {

        let mut logic = LogicComponent {
            id: id,
            initial_value: is_enabled,
            logic_op: match logic_op_opt {
                Some(logic_op_val) => logic_op_val,
                _ => LogicOpType::And, // default to and
            },
            logic_type: ConnectionType::default(),
            input_value: None,
            value: is_enabled,
            is_active: false,
            last_value: is_enabled,
            last_active: false,
            last_input: None,
            change_count: 0,
            frozen: false,
        };

        logic
    }

    pub fn new_logic(id: String, is_enabled: bool, logic_opt: Option<ItemLogic>) -> LogicComponent {

        let mut logic = LogicComponent {
            id: id,
            initial_value: is_enabled,
            logic_op: match &logic_opt {
                Some(logic) => match &logic.logic_op {
                    Some(logic_op_value) => logic_op_value.clone(),
                    _ => LogicOpType::And,
                },
                _ => LogicOpType::And, // default to and
            },
            logic_type: match &logic_opt {
                Some(logic) => match &logic.logic_type {
                    Some(logic_type_value) => logic_type_value.clone(),
                    _ => ConnectionType::Switch,
                },
                _ => ConnectionType::Switch, // default to and
            },
            input_value: None,
            value: is_enabled,
            is_active: false,
            last_value: is_enabled,
            last_active: false,
            last_input: None,
            change_count: 0,
            frozen: false,
        };

        logic
    }

    pub fn clear_input(&mut self) {
        self.input_value = None;
    }

    pub fn calc_value(&mut self) {
        let debug = false; //self.id.eq("btna");
        let mut calc_val = self.initial_value;
        if debug {
            println!("[LOGIC][{}] INITIAL, value: {}", &self.id, &calc_val);
        }
        if let Some(input) = self.input_value {
            if input {
                calc_val = !calc_val;
                if debug {
                    println!("[LOGIC][{}] FLIP BY INPUT VAL, value: {}", &self.id, &calc_val);
                }
            }
        }
        if self.is_active {
            calc_val = !calc_val;
            if debug {
                println!("[LOGIC][{}] FLIP BY ACTIVE VAL, value: {}", &self.id, &calc_val);
            }
        }
        self.value = calc_val;
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
        self.calc_value();
    }

    pub fn set_input_value(&mut self, input_value: bool) -> bool {
        //println!("Set input value on id {}, input_val: {}", &self.id, &input_value);
        
        self.input_value = Some(input_value);
        self.calc_value();
        //println!("Result value: {}", &self.value);
        
        self.value
    }

    pub fn update(&mut self, time_delta: f32) {
        if self.last_value != self.value {
            self.change_count += 1;
            if self.change_count > 2 {
                println!("[LOGIC] {} from {} => {} (change {})", &self.id, &self.last_value, &self.value, &self.change_count);
            }
        }
        else {
            self.change_count = 0;
        }

        self.last_value = self.value;

    }

}


// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    world.register::<LogicComponent>();
}