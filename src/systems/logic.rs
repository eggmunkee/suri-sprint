


use specs::prelude::*;


use crate::game_state::{GameState,RunningState};
use crate::resources::{InputResource,WorldAction,GameStateResource};
use crate::components::logic::{LogicComponent};
use crate::components::portal::{PortalComponent};
use crate::components::button::{ButtonComponent};
use crate::resources::{ConnectionResource};

pub struct LogicSystem {

}

impl<'a> System<'a> for LogicSystem {
    type SystemData = (WriteStorage<'a, LogicComponent>,
                        WriteStorage<'a, PortalComponent>,
                        WriteStorage<'a, ButtonComponent>,
                        Read<'a, GameStateResource>,
                        Write<'a, ConnectionResource>,
                        Entities<'a>);

    fn run(&mut self, (mut logic_res, mut portal_res, mut button_res, game_state, mut connection, mut entities): Self::SystemData) {
        use specs::Join;

        let time_delta = game_state.delta_seconds;

        for (mut logic, portal, ent) in (&mut logic_res, &portal_res, &entities).join() {
            let enabled = portal.is_enabled;
            println!("Portal {} is active: {}", &logic.id, &enabled);
            //logic.set_active(active);
            //connection.set_flagged(&logic.id, false);
            connection.set_value(&logic.id, enabled, false);
        }

        // Update active field of logic nodes
        // apply button is_active() to logic active status
        for (mut logic, button, ent) in (&mut logic_res, &button_res, &entities).join() {
            let active = button.is_active();
            println!("Button {} is active: {}", &logic.id, &active);
            logic.set_active(active);
            //connection.set_flagged(&logic.id, false);
            connection.set_value(&logic.id, active, false);
        }

        // 
        for (mut logic, button, ent) in (&mut logic_res, &button_res, &entities).join() {
            //logic.set_active(active);
            let input_key = &logic.id;
            let active = logic.is_active;
            connection.apply_value(input_key);
        }

        for _ in 0..3 {

            for (mut logic, ent) in (&mut logic_res, &entities).join() {
                //logic.set_active(active);
                let input_key = &logic.id;
                let flag = connection.get_flagged(input_key);
                if flag {
                    connection.apply_value(input_key);
                }
                connection.set_flagged(input_key, false); // clear flag
            }
    
        }


        for (mut logic, ent) in (&mut logic_res, &entities).join() {
            let input_key = &logic.id;
            let val = connection.get_value(input_key);
            let flag = connection.get_flagged(input_key);
            println!("Logic {} - Value {} - flag {}", &logic.id, &val, &flag);

            logic.set_input_value(val);
            println!("Logic result value {} - Value {}, input val {}, active {}", &logic.id, &logic.value, &logic.input_value, &logic.is_active);

            if let Some(mut portal) = portal_res.get_mut(ent) {
                println!("Updating portal enabled from logic {}", &val);
                portal.is_enabled = val;
            }
        }
    }

}