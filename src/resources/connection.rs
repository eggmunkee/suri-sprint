
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
use crate::components::logic::{LogicOpType};


#[allow(dead_code)]
#[derive(Default)]
pub struct ConnectionResource {
    // map input variables to a list of output connections
    pub connections: HashMap::<String,Vec::<String>>,
    pub back_connections: HashMap::<String,Vec::<String>>,
    pub logic_ops: HashMap::<String,LogicOpType>,
    pub value_register: HashMap::<String,(Option<bool>, bool, bool, i32)>, // INPUT, OUTPUT, FLAG, INPUT COUNTER
}

impl ConnectionResource {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
            back_connections: HashMap::new(),
            logic_ops: HashMap::new(),
            value_register: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.connections.clear();
        self.back_connections.clear();
        self.logic_ops.clear();
        self.value_register.clear();
    }

    pub fn set_in_value_std(&mut self, input_key: &str, in_value: Option<bool>, set_flag: bool, logic_op: Option<LogicOpType>) -> bool {
        self.set_in_value(input_key, in_value, set_flag, logic_op, false)
    }

    pub fn set_in_value(&mut self, input_key: &str, set_value: Option<bool>, set_flag: bool, logic_op: Option<LogicOpType>, debug: bool) -> bool {

        let logic_op_real = self.get_logic_op(input_key.to_string());
        if debug {
            println!("    [SetInValue] [{}] Op: {:?} Override Op: {:?} Set: {:?}", &input_key, &logic_op_real, &logic_op, &set_value);
        }

        if let Some( (ref mut curr_input_value, ref mut val, ref mut flag, ref mut input_counter)) = self.value_register.get_mut(input_key) {
            let mut had_value = false;
            let mut old_value = false;
            if let (Some(current_value), Some(param_in_value)) = (&curr_input_value, set_value) {
                old_value = *current_value;
                had_value = true;
                if set_flag && *current_value != param_in_value {
                    *flag = true;
                }
            }
            else if let Some(current_value) = &curr_input_value {
                old_value = *current_value;
                had_value = true;
                if set_flag {
                    *flag = true;
                }
            }

            if had_value {
                *curr_input_value = match set_value {
                    Some(in_bool_value) => {                            
                        match &logic_op_real {
                            //Some(LogicOpType::And) => Some(old_value && in_some_value),
                            //Some(LogicOpType::Or) => Some(old_value || in_some_value),
                            LogicOpType::And => {
                                if debug {
                                    println!("    - [SetInValue] AND op - {} && {} = {}", &old_value, &in_bool_value, (old_value && in_bool_value));
                                }
                                Some(old_value && in_bool_value)
                            },
                            LogicOpType::Or => {
                                if debug {
                                    println!("    - [SetInValue] OR op  - {} && {} = {}", &old_value, &in_bool_value, (old_value || in_bool_value));
                                }
                                Some(old_value || in_bool_value)
                            },
                            _ => {
                                if debug {
                                    println!("    - [SetInValue] Unknown op - Set = {:?}", &set_value);
                                }
                                set_value
                            }
                        }
                    },
                    None => {
                        if debug {
                            println!("    - [SetInValue] Setting None - Set = {:?}", &set_value);
                        }
                        *input_counter = 0;
                        set_value
                    }
                };
            }
            else {
                *input_counter = 0;
                if debug {
                    println!("    - [SetInValue] Setting First Value - Set = {:?}", &set_value);
                }
                *curr_input_value = set_value;
            }
            
            //*in_val = in_value;
            *flag
        }
        else {
            println!("    New value for {} of {:?}", &input_key, &set_value);
            self.value_register.insert(input_key.to_string(), (set_value, false, set_flag, 0));
            set_flag
        }
    }

    pub fn get_in_value(&mut self, input_key: &str) -> Option<bool> {
        if let Some( (ref in_val, ref val, ref flag, ref in_count)) = self.value_register.get_mut(input_key) {
            if let Some(input_val) = in_val {
                Some(*input_val)
            }
            else {
                None
            }
        }
        else {
            None
        }
    }

    pub fn set_value(&mut self, input_key: &str, value: bool, set_flag: bool) -> bool {
        if let Some( (ref mut in_val, ref mut val, ref mut flag, ref in_count)) = self.value_register.get_mut(input_key) {
            if set_flag && *val != value {
                *flag = true;
            }
            *val = value;
            *flag
        }
        else {
            println!("  [CONN SET VALUE] New value for {} of {}", &input_key, &value);
            self.value_register.insert(input_key.to_string(), (None, value, set_flag, 0));
            set_flag
        }
    }

    pub fn get_value(&mut self, input_key: &str) -> bool {
        if let Some( (ref in_val, ref val, ref flag, ref in_count)) = self.value_register.get(input_key) {
            *val
        }
        else {
            //println!("Value not found {}", input_key );
            false
        }
    }

    pub fn set_flagged(&mut self, input_key: &str, flagged: bool) {
        if let Some( (ref mut in_val, ref mut val, ref mut flag, ref in_count)) = self.value_register.get_mut(input_key) {
            *flag = flagged;
        }
        else {
            self.value_register.insert(input_key.to_string(), (None, false, flagged, 0));
        }
    }
    pub fn get_flagged(&mut self, input_key: &str) -> bool {
        if let Some( (ref in_val, ref val, ref flag, ref in_count)) = self.value_register.get(input_key) {
            *flag
        }
        else {
            false
        }
    }

    pub fn set_logic_op(&mut self, key: String, logic_op: LogicOpType) {
        // Set Logic Op for the results of this
        if let Some(curr_logic_op) = self.logic_ops.get_mut(&key) {
            *curr_logic_op = logic_op;
        }
        else {
            self.logic_ops.insert(key, logic_op);
        }
    }

    pub fn get_logic_op(&mut self, key: String) -> LogicOpType {
        // Set Logic Op for the results of this
        if let Some(curr_logic_op) = self.logic_ops.get_mut(&key) {
            *curr_logic_op
        }
        else {
            LogicOpType::And
        }
    }


    pub fn add_connection(&mut self, input_key: String, output_key: String, logic_op: LogicOpType) {
        println!("[CONN] Adding connection for [{}] => [{}] (Logic Op: {:?})", &input_key, &output_key, &logic_op);

        // Setup In/Out values in value register
        self.set_in_value_std(&input_key, None, false, None);
        self.set_value(&input_key, false, false);

        // Add forward connection - conn[input_key] = [a,b,output_key]
        // memory: clone keys for first connection
        if let Some(ref mut outputs) = self.connections.get_mut(&input_key.clone()) {
            if let None = outputs.iter().position(|out_key| out_key == &output_key) {
                outputs.push(output_key.clone());
            }            
        }
        else {
            self.connections.insert(input_key.clone(), vec![output_key.clone()]);
        }
        // Add backwards connection - back_conn[output_key] = [a,b,input_key]
        // memory: move keys into back connection
        if let Some(ref mut inputs) = self.back_connections.get_mut(&output_key) {
            if let None = inputs.iter().position(|in_key| in_key == &input_key) {
                inputs.push(input_key.clone());
            }            
        }
        else {
            self.back_connections.insert(output_key, vec![input_key.clone()]);
        }

    }

    pub fn apply_value(&mut self, input_key: &str, debug: bool) -> i32 {
        let mut apply_count = 0;
        let mut output_vars : Vec::<String> = vec![];

        // OUTPUTS LIST
        // build list of output vars to update
        if let Some(outputs) = self.connections.get(input_key) {
            //let outputs = outputs.collect();            
            for out_key in outputs {
                output_vars.push(out_key.clone());
            }
        }
        // GET IN VALUE TO APPLY
        let mut out_value = false;
        if let Some( (_, ref out_val, _, _)) = self.value_register.get(input_key) {
            out_value = *out_val;
            if debug {
                println!("Applying {} output value {}", &input_key, &out_value);
            }
        }
        else {
            if debug {
                println!("No value to apply for {}", &input_key);
            }
            return 0;
        }
        // APPLY VALUE TO OUTPUT NODES
        for out_key in output_vars {
            if debug {
                println!(" - Applying {} output value {}", &out_key, &out_value);
            }
            let flagged = self.set_in_value(&out_key, Some(out_value), true, Some(LogicOpType::Or), debug);
            apply_count += 1;
        }

        apply_count
    }

    pub fn set_in_values(&mut self, input_key: &str, in_value: Option<bool>, debug: bool) -> i32 {

        if debug {
            println!("[CONN] Set In Values - Applying {} output value {:?}", &input_key, &in_value);
        }

        let mut apply_count = 0;
        let mut output_vars : Vec::<String> = vec![];
        // build list of output vars to update
        if let Some(outputs) = self.connections.get(input_key) {
            //let outputs = outputs.collect();            
            for out_key in outputs {
                output_vars.push(out_key.clone());
            }
        }

        // let mut out_value = false;
        // if let Some( (_, ref out_val, _)) = self.value_register.get(input_key) {
        //     out_value = *out_val;
        //     if debug {
        //         println!("Applying {} output value {}", &input_key, &out_value);
        //     }
        // }
        // else {
        //     if debug {
        //         println!("No value to apply for {}", &input_key);
        //     }
        //     return 0;
        // }

        for out_key in output_vars {
            if debug {
                println!(" - Applying {} output value {:?}", &out_key, &in_value);
            }
            let flagged = self.set_in_value(&out_key, in_value, true, Some(LogicOpType::Or), debug);
            apply_count += 1;
        }

        apply_count
    }

    // pub fn apply_values_to_output(&mut self, output_key: &str, debug: bool) {
    //     if debug {
    //         println!("Apply Values to Output for [{}]", &output_key);
    //     }

    //     // Save original output key value
    //     let mut in_value = false;
    //     if let Some( (ref val, _)) = self.value_register.get(output_key) {
    //         in_value = *val;
    //         //println!("Applying {} input value {}", &input_key, &in_value);
    //     }
    //     else {
    //         println!("No value to apply for {}", &output_key);
    //         return;
    //     }

    //     if debug {
    //         println!(" Orig Value: [{}]", &in_value);
    //     }

    //     // Gather inputs
    //     let mut input_vars : Vec::<String> = vec![];

    //     // build list of output vars to update
    //     if let Some(inputs) = self.back_connections.get(output_key) {
    //         //let outputs = outputs.collect();            
    //         for in_key in inputs {
    //             if debug {
    //                 println!(" Input value: {}", &in_key);
    //             }
        
    //             input_vars.push(in_key.clone());
    //         }
    //     }
    //     else {
    //         if debug {
    //             println!("No back connections for {}", &output_key);
    //         }
    //     }

    //     // abandon if no inputs
    //     if input_vars.len() == 0 {
    //         println!("No input vars for {}", &output_key);
    //         return;
    //     }

    //     // Set input value with AND operation for multiple
    //     let mut combo_val_opt : Option<bool> = None;
    //     for in_key in input_vars {
    //         //println!("Applying {} output value {}", &out_key, &in_value);
    //         let mut val = self.get_value(&in_key);

    //         if let Some(combo_value) = combo_val_opt {
    //             if debug {
    //                 println!(" Apply {}.[{}] AND value: {}", &in_key, &combo_value, &val);
    //             }                
    //             combo_val_opt = Some(val && combo_value);
    //         }
    //         else {
    //             if debug {
    //                 println!(" Apply {} value: {}", &in_key, &val);
    //             }                
    //             combo_val_opt = Some(val);
    //         }
    //     }
    //     if let Some(combo_value) = combo_val_opt {
    //         if combo_value {
    //             in_value = !in_value;
    //         }
    //         if debug {
    //             println!(" Setting {} comb_value: {} value: {}", &output_key, &combo_value, &in_value);
    //         }                
    //         self.set_value(&output_key, in_value, true);
    //     }

    // }

}