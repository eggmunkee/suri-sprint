
use ggez::{Context, GameResult, GameError};

use ggez::graphics;
use ggez::nalgebra as na;
use ggez::graphics::{Color,DrawParam,Scale,set_window_title};


use crate::core::{GameState};
use super::dialog::{DialogRenderer};


pub struct PausedRenderer {

}

impl PausedRenderer {

    pub fn render(game_state: &GameState, ctx: &mut Context) {

        let level_name_content = String::from(format!("Paused on Area \"{}\".\n\nThis message is sponsored by Bell Peppers.", &game_state.level.name));
        let border_color = Color::new(0.75, 0.75, 0.75, 0.75);
        let bg_color = Color::new(0.0, 0.0, 0.0, 0.75);
        let text_color = Color::new(1.0, 1.0, 1.0, 1.0);

        let anim_pos = game_state.paused_anim.min(0.2) * 5.0;
        let anim_y = 1.1 - anim_pos * 0.2;

        // Render a dialog with custom loc/size and colors for the pause display
        DialogRenderer::render_at(game_state, ctx, level_name_content,
            0.5, anim_y, 0.75, 0.08, border_color, bg_color, text_color, None);

    }
}