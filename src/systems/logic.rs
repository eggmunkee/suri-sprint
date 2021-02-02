


use specs::prelude::*;


//use crate::game_state::{GameState,RunningState};
use crate::resources::{InputResource,WorldAction,GameStateResource};
use crate::components::logic::{LogicComponent};
use crate::components::portal::{PortalComponent};
use crate::components::button::{ButtonComponent};
use crate::components::sprite::{SpriteComponent};
use crate::components::anim_sprite::{AnimSpriteComponent};
use crate::components::particle_sys::{ParticleSysComponent};
use crate::components::collision::{Collision};
use crate::components::logic::{ConnectionType};
use crate::resources::{ConnectionResource};

pub struct LogicSystem {
    pub show_debug_output : bool,
}

impl<'a> System<'a> for LogicSystem {
    type SystemData = (WriteStorage<'a, LogicComponent>,
                        WriteStorage<'a, PortalComponent>,
                        WriteStorage<'a, ButtonComponent>,
                        WriteStorage<'a, SpriteComponent>,
                        WriteStorage<'a, AnimSpriteComponent>,
                        WriteStorage<'a, ParticleSysComponent>,
                        WriteStorage<'a, Collision>,
                        Read<'a, GameStateResource>,
                        Write<'a, ConnectionResource>,
                        Entities<'a>);

    fn run(&mut self, (mut logic_res, mut portal_res, mut button_res, mut sprite_res,
            mut anim_sprite_res, mut particle_res,
            mut coll_res,
            game_state, mut connection, mut entities): Self::SystemData) {
        use specs::Join;

        let time_delta = game_state.delta_seconds;
        let debug = self.show_debug_output;

        // PORTALS ONLY - set enabled from portal value - initial value
        if debug {
            println!("[LOGIC ENTITY STEP] - Clear Input values, Calc Init Logic values");
        }
        for (mut logic, ent) in (&mut logic_res, &entities).join() {

            if logic.frozen {
                continue;
            }

            if debug {                
                println!("[LOGIC ENTITY] {} Pre-state - value: {}, active: {}", &logic.id, &logic.value, &logic.is_active);
            }
            //let enabled = portal.is_enabled;
            //let enabled = logic.last_value;
            //logic.set_active(enabled);
            //logic.value = enabled;
            //connection.set_flagged(&logic.id, false);
            // let mut input_val = false;
            // if let Some(in_val) = logic.last_input {
            //     input_val = in_val;
            // }

            logic.clear_input();

            // BUTTON
            if let Some(button) = button_res.get(ent) {
                let active = button.is_active();
                logic.set_active(active);
            }
            // NON-BUTTON
            else {

            }           

            logic.calc_value();
            if debug {
                println!("[LOGIC ENTITY] {} Post-init - value: {}, active: {}", &logic.id, &logic.value, &logic.is_active);
            }

            // Update connection Logic Ops for this node
            let logic_op = logic.logic_op;
            connection.set_logic_op(logic.id.clone(), logic_op);

            connection.set_in_value(&logic.id, None, false, None, debug);
            connection.set_value(&logic.id, logic.value, false);
        }

        // BUTTON ONLY INPUTS
        // Update active field of logic nodes
        // apply button is_active() to logic active status, and connection value
        // if debug {
        //     println!("[BUTTONS STEP] - Get Active value, set Logic Active flag");
        // }
        // for (mut logic, button, ent) in (&mut logic_res, &button_res, &entities).join() {
        //     let active = button.is_active();
        //     if debug {
        //         println!("[LOGIC] 1) Button {} is active: {}", &logic.id, &active);
        //     }
        //     //logic.clear_input();
        //     logic.set_active(active);
        //     if debug {
        //         println!("[LOGIC]   2) Button {} is active: {}, value: {}", &logic.id, &logic.is_active, &logic.value);
        //     }
        //     //connection.set_flagged(&logic.id, false);
        //     connection.set_value(&logic.id, active, false);
        // }

        for (output_key, inputs) in connection.back_connections.iter() {
            //if inputs.len() > 1 {
                for input_key in inputs.iter() {
                    if debug {
                        println!("Back connections from {} to [{}].", &output_key, &input_key);
                    }
                }
            //}
        }

        // apply logic component values into connection struct
        if debug {
            println!("[APPLY INPUT VALs STEP] - Conn Input Val -> Logic Input, Calc, Apply Inputs to Conns");
        }
        for (mut logic, ent) in (&mut logic_res, &entities).join() {

            if logic.frozen {
                continue;
            }

            //logic.set_active(active);
            let input_key = logic.id.clone();
            if debug {
                println!("[LOGIC] * Pre-set value: item {}, value: {}, active: {}", &input_key, &logic.value, &logic.is_active);
            }

            // Assign this in value to logic
            let in_value = connection.get_in_value(&input_key);
            //connection.set_in_value(&input_key, in_value, false);
            if debug {
                println!("[LOGIC] * Set In value: item {}, in_value: {:?}", &input_key, &in_value);
            }
            if let Some(input_value) = in_value {
                if debug {
                    if let None = logic.input_value {
                        println!("[LOGIC] ^^^ Setting First Value: item {}, in_value: {:?}", &input_key, &in_value);
                    }
                }
                logic.set_input_value(input_value);
            }
            logic.calc_value();
            let value_to_apply = logic.value;
            let init_active = logic.is_active;
            if debug {
                println!("[LOGIC] * Set value: item {}, in_value: {:?}, value: {}, active: {}", &input_key, &in_value, &value_to_apply, &init_active);
            }
            //connection.set_value(&input_key, init_val, true);
            //connection.set_flagged(&input_key, true);
            //connection.apply_value(&input_key, debug);
            connection.set_in_values(&input_key, Some(value_to_apply), debug);
            if debug {
                println!(" - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -");
            }
        }

        // Process connection values up to 3 times -
        if debug {
            println!("[FLAGGED INPUTS LOOP] - Process Flagged Inputs, Sync Conn & Logic state");
        }
        for i in 0..5 {
            if debug {
                println!("Connection apply loop {} == == == == == == == == == == == == == == ==", &i);
            }
            let mut flagged_ct : i32 = 0;

            if debug {
                println!("[LOGIC SYSTEM] - APPLY LOOP - - - - - - - - - - - - - - - - - - - - - -");
            }
            for (mut logic, ent) in (&mut logic_res, &entities).join() {
                //logic.set_active(active);
                //let input_key = &logic.id;
                if logic.frozen {
                    continue;
                }

                // Assign this in value to logic
                let in_value = connection.get_in_value(&logic.id);
                //connection.set_in_value(&input_key, in_value, false);
                if let Some(input_value) = in_value {
                    logic.set_input_value(input_value);
                }

                logic.calc_value();

                //connection.set_value(&input_key, logic.value, false);
                let input_key = &logic.id;
                let logic_value = logic.value;

                let flag = connection.get_flagged(input_key);
                let input_value = connection.get_in_value(input_key);
                if flag || i == 0 {
                    //println!("Logic item {} is FLAGGED", &input_key);
                    //connection.apply_values_to_output(input_key, true);
                    if debug {
                        println!("[LOGIC] Pre-apply for {} - Value {}, input val {:?}, active {}", &logic.id, &logic.value, &logic.input_value, &logic.is_active);
                        println!("[CONN] connection Value {:?}, flag {}", &input_value, &flag);
                    }

                    connection.set_in_values(&input_key, Some(logic_value), debug);

                    // let apply_count = connection.apply_value(input_key, debug);
                    // if apply_count > 0 {
                    //     flagged_ct += 1;
                    // }

                    connection.set_flagged(input_key, false); // clear flag
                }
                else {
                    println!("[LOGIC] Logic item {} NOT FLAGGED", &input_key);
                }
                if debug {
                    println!(" - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -");
                }
            }

            if debug {
                println!("[LOGIC SYSTEM] - Logic Entities & Final Input Values - - - - - - - - - - - - - - - - - - -");
            }
            for (mut logic, ent) in (&mut logic_res, &entities).join() {

                if logic.frozen {
                    continue;
                }

                // Set input of logic from connection
                let input_value = connection.get_in_value(&logic.id);

                if let Some(in_value) = input_value {
                    logic.set_input_value(in_value); // re-calculates value

                    if let ConnectionType::SwitchOnce = logic.logic_type {
                        //println!("!@!@!@!@!@!@!@!@!@!@!@!@!@!@!@!@!@!@!@");
                        //println!("Checking SWITCH ONCE NODE. [{}] Value {}", &logic.id, &in_value);
                        if in_value {
                            println!("!@!@!@!@!@!@!@!@!@!@!@!@!@!@!@!@!@!@!@");
                            println!("!@!@!@!@!@!@  [{}] FROZEN   @!@!@!@!@!@!@", &logic.id);
                            logic.frozen = true;
                        }
                    }
                }
                
                // Set output value back in connection node
                //connection.set_value(&logic.id, logic.value, true);
            }

            // let mut check_inputs : Vec::<String> = vec![];
            // for (output_key, inputs) in connection.back_connections.iter() {
            //     for input_key in inputs.iter() {
            //         check_inputs.push(input_key.clone());
            //     }
            // }

            if debug {
                println!("Flagged items: {}", &flagged_ct);
            }
            if flagged_ct == 0 { break; }
        }

        if debug {
            println!("[UPDATE LOGIC ENTITIES] - Calc Logic Value, Update Portal/Sprite/etc. state - - - - - - - - -");
        }
        for (mut logic, ent) in (&mut logic_res, &entities).join() {

            // if logic.frozen {
            //     continue;
            // }

            let input_key = logic.id.clone();
            logic.calc_value();

            // let in_val = connection.get_in_value(&input_key);
            // let val = connection.get_value(&input_key);
            // let flag = connection.get_flagged(&input_key);
            if debug {
                println!("[LOGIC] Logic {} - Input Value {:?} Value {} ", &logic.id, &logic.input_value, &logic.value);
            }

            let res_val = logic.value;
            
            // {
            //     logic.set_input_value(val);
            //     if debug {
            //         println!("Logic result value {} - Value {}, input val {:?}, active {}", &logic.id, &logic.value, &logic.input_value, &logic.is_active);
            //     }
            // }

            {
                logic.update(time_delta);
                if logic.change_count > 1 {
                    println!("Logic for {} changed more than once.", &input_key);
                }
            }

            // Handle entities that can be toggled by logic
            // Portal toggle
            if let Some(mut portal) = portal_res.get_mut(ent) {
                //println!("Updating portal enabled from logic {}", &val);
                if debug && portal.is_enabled != res_val {
                    println!("[LOGIC PORTAL UPDATE] - ENABLED: Old val {} - New val {}", &portal.is_enabled, &res_val);
                }
                portal.is_enabled = res_val;
            }
            // Sprite Toggle (dyn sprite)
            else {

                if let Some(mut sprite) = sprite_res.get_mut(ent) {
                    if sprite.toggleable {
                        if debug && sprite.visible != res_val {
                            println!("[LOGIC SPRITE UPDATE] - VISIBLE: Old val {} - New val {}", &sprite.visible, &res_val);
                        }
                        sprite.visible = res_val;
                    }
                }
                if let Some(mut anim_sprite) = anim_sprite_res.get_mut(ent) {
                    if anim_sprite.toggleable {
                        if debug && anim_sprite.visible != res_val {
                            println!("[LOGIC ANIM-SPRITE UPDATE] - VISIBLE: Old val {} - New val {}", &anim_sprite.visible, &res_val);
                        }
                        anim_sprite.visible = res_val;
                    }
                }
                if let Some(mut part_sys) = particle_res.get_mut(ent) {
                    if part_sys.toggleable {
                        if debug && part_sys.visible != res_val {
                            println!("[LOGIC P.SYS UPDATE] - VISIBLE: Old val {} - New val {}", &part_sys.visible, &res_val);
                        }
                        part_sys.visible = res_val;
                    }
                }
                if let Some(coll) = coll_res.get_mut(ent) {
                    if coll.toggleable {
                        coll.set_obstructing(res_val);
                    }                
                }
            } 

        }

    }

}