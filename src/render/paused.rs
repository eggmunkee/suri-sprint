
use ggez::{Context, GameResult, GameError};

use ggez::graphics;
use ggez::nalgebra as na;
use ggez::graphics::{Color,DrawParam,Scale,set_window_title};


use crate::game_state::{GameState};
use super::dialog::{DialogRenderer};


pub struct PausedRenderer {
    pub anim_position: f32,

}

impl PausedRenderer {
    pub fn new() -> Self {
        PausedRenderer {
            anim_position: 0.0,
        }
    }

    pub fn render(&self, game_state: &GameState, ctx: &mut Context) {


        let level_name_content = String::from(format!("Paused on Level \"{}\"", &game_state.level.name));
        let border_color = ggez::graphics::Color::new(0.75, 0.75, 0.75, 0.75);
        let bg_color = ggez::graphics::Color::new(0.0, 0.0, 0.0, 0.75);

        let anim_pos = self.anim_position.min(0.2) * 5.0;
        let anim_y = 1.1 - anim_pos * 0.2;

        // Render a dialog with custom loc/size and colors for the pause display
        DialogRenderer::render_at(game_state, ctx, level_name_content,
            0.5, anim_y, 0.75, 0.08, border_color, bg_color );

    }
}