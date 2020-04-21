


use specs::prelude::*;


use crate::game_state::{GameState,RunningState};
use crate::resources::{InputResource,WorldAction,GameStateResource};
use crate::components::logic::{LogicComponent};
use crate::components::button::{ButtonComponent};
use crate::resources::{ConnectionResource};

struct LogicSystem {

}

impl<'a> System<'a> for LogicSystem {
    type SystemData = (WriteStorage<'a, LogicComponent>,
                        ReadStorage<'a, ButtonComponent>,
                        Read<'a, GameStateResource>,
                        Write<'a, ConnectionResource>,
                        Entities<'a>);

    fn run(&mut self, (mut logic_res, button_res, game_state, mut connection, mut entities): Self::SystemData) {
        use specs::Join;

        let time_delta = game_state.delta_seconds;

        // Update active field of logic nodes
        // apply button is_active() to logic active status
        for (mut logic, button, ent) in (&mut logic_res, &button_res, &entities).join() {
            let active = button.is_active();
            logic.set_active(active);
        }

        // 
        for (mut logic, button, ent) in (&mut logic_res, &button_res, &entities).join() {
            //logic.set_active(active);
            let input_key = &button.name;
            let active = logic.is_active;
            connection.set_value(input_key, active);
            connection.apply_value(input_key);
        }
        
    }

}