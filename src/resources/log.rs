
use ggez::graphics::{Color};
//use ggez::conf::{WindowMode};

//use crate::entities::level_builder::{LevelBounds,LevelType};


#[derive(Default,Debug)]
pub struct GameLogEntry {
    pub on_screen: bool,
    pub msg: String,
    pub bg_color: Option<Color>,
    pub entry_time: f32,
}

#[derive(Default,Debug)]
pub struct GameLog {
    pub entries: Vec::<GameLogEntry>,
    pub max_keep: usize,
}

impl GameLog {
    pub fn add_entry(&mut self, on_scrn: bool, message: String, background_color: Option<Color>, entry_time: f32) {

        // remove elements past or at max
        while self.entries.len() >= self.max_keep {
            self.entries.remove(0);
        }

        // Add new entry
        self.entries.push(GameLogEntry {
            on_screen: on_scrn,
            msg: message,
            bg_color: background_color,
            entry_time: entry_time,
        });
    }
}

