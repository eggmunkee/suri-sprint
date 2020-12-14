
use ggez::nalgebra as na;
use specs::{Component, DenseVecStorage, World, WorldExt};
use specs_derive::*;
use wrapped2d::b2;
use rand::prelude::*;
use serde::{Deserialize,de::DeserializeOwned,Serialize};

use crate::components::sprite::{SpriteComponent};
use crate::components::collision::{Collision};
use crate::physics;
use crate::physics::{PhysicsWorld};

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

#[derive(Debug,Clone,Deserialize,Serialize)]
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

    // Dynamic Values
    pub input_value: Option<bool>, // If the external input has a signal and its value
    pub is_active: bool, // If this node is generating an active signal
    pub value: bool, // the node's output result value

    // History keeping
    pub last_value: bool,
    pub last_input: Option<bool>,
    pub last_active: bool,
    pub change_count: i32,
}

impl LogicComponent {
    pub fn new(id: String, is_enabled: bool) -> LogicComponent {

        let mut logic = LogicComponent {
            id: id,
            initial_value: is_enabled,
            logic_op: LogicOpType::And,
            input_value: None,
            value: is_enabled,
            is_active: false,
            last_value: is_enabled,
            last_active: false,
            last_input: None,
            change_count: 0,
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