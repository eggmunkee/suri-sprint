
use ggez::conf::{WindowMode};
use ggez::nalgebra as na;
use specs::{Entity,WorldExt};

use crate::core::world::{SuriWorld};
use crate::components::{Position};
use crate::entities::level_builder::{LevelBounds,LevelType};
use crate::core::game_state::{GameState,State,GameMode,RunningState,MenuItem};

#[derive(Default,Debug)]
pub struct Camera {
    pub display_offset: (f32, f32),
    pub snap_view: bool,
    pub target_offset: (f32, f32),
    pub following: Option<Entity>,
}

impl Camera {

    pub fn set_snap_view(game_state: &GameState, snap: bool) {
        let mut camera = game_state.world.fetch_mut::<Camera>();

        camera.snap_view = snap;
        drop(camera);

        Self::update(game_state, 0.001);
    }

    pub fn update(game_state: &GameState, time_delta: f32) {

        let mut camera = game_state.world.fetch_mut::<Camera>();

        let mut target_offset_x = camera.target_offset.0;
        let mut target_offset_y = camera.target_offset.1;

        let current_player = game_state.world.get_player();
        camera.following = current_player;

        // IF following - update target offset from following entity
        if let Some(entity) = camera.following {
            if entity.gen().is_alive() {
                //let pos = game_state.world.()::<Position>(entity);
                let pos_res = game_state.world.read_storage::<Position>();
                if let Some(pos) = pos_res.get(entity) {
                    target_offset_x = -pos.x;
                    target_offset_y = -pos.y;
                }
            }
        }

        //self.display_offset.0 = target_offset_x;
        //self.display_offset.1 = target_offset_y;

        // Set freeze-camera states - when camera should not move towards target
        let mut move_camera : bool = true;
        // FREEZE IN MENU
        if game_state.in_menu_system() {
            move_camera = false;
        }
        match &game_state.running_state {
            // DISPLAY DIALOG TEXT
            RunningState::Dialog{..} => {
                move_camera = false;
            },
            _ => {}
        }
        match &game_state.current_state {
            State::Paused => {
                move_camera = false;
            },
            _ => {}
        }        

        if game_state.game_frame_count % 60 == 1 {
            println!(" Calculate camera update ------------------");
        }
        let targ_x_mag = (target_offset_x - camera.display_offset.0).abs();
        let targ_y_mag = (target_offset_y - camera.display_offset.1).abs();
        let targ_axes_sum = targ_x_mag * targ_x_mag + targ_y_mag * targ_y_mag;
        if !camera.snap_view && move_camera == true {

            /*if targ_axes_sum < 10000.0 {
            }
            else */
            if targ_axes_sum < 200000.0 {
                // Create sliding scale of pan speed - must line up with max axes_sum in if
                // Should not result in -x_div or -y_div, as long as initial values and what axes_sum is div. by make sense
                let x_div = (12000.0 - (targ_axes_sum/20.0)) * 2.5; // range 2000-11500
                let y_div = (10001.0 - (targ_axes_sum/20.0)) * 2.5; // range 1-10000

                // let midpoint = (self.display_offset.x - target_offset_x) * (targ_x_mag / 20.0);
                // self.display_offset.x -= midpoint;
                let midpoint_x = (camera.display_offset.0 - target_offset_x) * (targ_x_mag / x_div); //8000.0);
                camera.display_offset.0 -= midpoint_x;
                let midpoint_y = (camera.display_offset.1 - target_offset_y) * (targ_y_mag / y_div); //2500.0);
                camera.display_offset.1 -= midpoint_y;
                
                // // let midpoint = (self.display_offset.x - target_offset_x) * (targ_x_mag / 20.0);
                // // self.display_offset.x -= midpoint;
                // let midpoint_x = (camera.display_offset.0 - target_offset_x) * (targ_x_mag / 6000.0); //8000.0);
                // camera.display_offset.0 -= midpoint_x;
                // let midpoint_y = (camera.display_offset.1 - target_offset_y) * (targ_y_mag / 2500.0); //2500.0);
                // camera.display_offset.1 -= midpoint_y;
            }
            else {
                //self.display_offset.x = target_offset_x;
                //let midpoint = (self.display_offset.x - target_offset_x) * 0.95;
                camera.display_offset.0 = target_offset_x; //midpoint;
                camera.display_offset.1 = target_offset_y;
            }
        }
        else if game_state.snap_view {
            camera.display_offset.0 = target_offset_x; //midpoint;
            camera.display_offset.1 = target_offset_y;
            camera.snap_view = false;
        }

        // save offset back to game state
        //game_state.current_offset.x = self.display_offset.0;
        //game_state.current_offset.y = self.display_offset.1;
    }
}