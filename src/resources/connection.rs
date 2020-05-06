
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
#[derive(Default)]
pub struct ConnectionResource {
    // map input variables to a list of output connections
    pub connections: HashMap::<String,Vec::<String>>,
    pub value_register: HashMap::<String,(bool, bool)>,
}

impl ConnectionResource {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
            value_register: HashMap::new(),
        }
    }
    pub fn set_value(&mut self, input_key: &str, value: bool, flagged: bool) {
        if let Some( (ref mut val, ref mut flag)) = self.value_register.get_mut(input_key) {
            if flagged && *val != value {
                *flag = true;
            }
            *val = value;
        }
        else {
            self.value_register.insert(input_key.to_string(), (value, flagged));
        }
    }

    pub fn get_value(&mut self, input_key: &str) -> bool {
        if let Some( (ref val, ref flag)) = self.value_register.get(input_key) {
            *val
        }
        else {
            //println!("Value not found {}", input_key );
            false
        }
    }

    pub fn set_flagged(&mut self, input_key: &str, flagged: bool) {
        if let Some( (ref mut val, ref mut flag)) = self.value_register.get_mut(input_key) {
            *flag = flagged;
        }
        else {
            self.value_register.insert(input_key.to_string(), (false, flagged));
        }
    }
    pub fn get_flagged(&mut self, input_key: &str) -> bool {
        if let Some( (ref val, ref flag)) = self.value_register.get(input_key) {
            *flag
        }
        else {
            false
        }
    }

    pub fn add_connection(&mut self, input_key: String, output_key: String) {
        if let Some(ref mut outputs) = self.connections.get_mut(&input_key) {
            if let None = outputs.iter().position(|out_key| out_key == &output_key) {
                outputs.push(output_key);
            }            
        }
        else {
            self.connections.insert(input_key, vec![output_key]);
        }
    }

    pub fn apply_value(&mut self, input_key: &str) {
        let mut output_vars : Vec::<String> = vec![];
        // build list of output vars to update
        if let Some(outputs) = self.connections.get(input_key) {
            //let outputs = outputs.collect();            
            for out_key in outputs {
                output_vars.push(out_key.clone());
            }
        }

        let mut in_value = false;
        if let Some( (ref val, _)) = self.value_register.get(input_key) {
            in_value = *val;
            //println!("Applying {} input value {}", &input_key, &in_value);
        }
        else {
            //println!("No value to apply for {}", &input_key);
            return;
        }
        for out_key in output_vars {
            //println!("Applying {} output value {}", &out_key, &in_value);
            self.set_value(&out_key, in_value, true);
        }
    }

}