
use ggez::conf::{WindowMode};

use crate::entities::level_builder::{LevelBounds,LevelType};


#[derive(Default,Debug)]
pub struct GameStateResource {
    pub window_w: f32,
    pub window_h: f32,
    pub window_mode: WindowMode,
    pub display_offset: (f32, f32),
    pub delta_seconds: f32,
    pub level_world_seconds: f32,
    pub level_frame_num: i32,
    pub game_run_seconds: f32,
    pub level_bounds: LevelBounds,
    pub level_type: LevelType,

    // global player stats
    pub player_count: i32,
    pub player_1_char_num: i32,

    // ai info
    pub player_target_loc: (f32, f32),

    // game status info
    pub points: i32,
}