
use ggez::{Context, GameResult, GameError};

use ggez::graphics;
use ggez::nalgebra as na;
use ggez::graphics::{Color,DrawParam,Scale,set_window_title};

use crate::game_state::{GameState};



pub struct DialogRenderer {



}

impl DialogRenderer {

    pub fn render(game_state: &GameState, ctx: &mut Context, msg: String) {
        let (w, h) = (game_state.window_w, game_state.window_h);
        let cent_x = w as f32 / 2.0;
        let cent_y = h as f32 / 4.0;
        let dialog_w = w as f32 * 0.75;
        let dialog_h = h as f32 * 0.25;
        let border_color = ggez::graphics::Color::new(0.7, 0.2, 0.7, 1.0);
        let bg_color = ggez::graphics::Color::new(0.3, 0.1, 0.3, 0.5);

        Self::render_dialog(game_state, ctx, msg, cent_x, cent_y, dialog_w, dialog_h, border_color, bg_color);
    }

    pub fn render_at(game_state: &GameState, ctx: &mut Context, msg: String,
        x: f32, y: f32, dw: f32, dh: f32,
        border_color: Color, bg_color: Color) {
        let (w, h) = (game_state.window_w, game_state.window_h);
        let cent_x = w as f32 * x;
        let cent_y = h as f32 * y;
        let dialog_w = w as f32 * dw;
        let dialog_h = h as f32 * dh;

        Self::render_dialog(game_state, ctx, msg, cent_x, cent_y, dialog_w, dialog_h, border_color, bg_color);
    }

    pub fn render_dialog(game_state: &GameState, ctx: &mut Context, msg: String,
        cent_x: f32, cent_y: f32, dialog_w: f32, dialog_h: f32,
        border_color: Color, bg_color: Color) {

        let mut draw_ok = true;
        let (w, h) = (game_state.window_w, game_state.window_h);

        let mut stroke_options = ggez::graphics::StrokeOptions::DEFAULT;
        stroke_options.line_width = 5.0;

        // dark transparent background
        if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
            ggez::graphics::DrawMode::Fill(ggez::graphics::FillOptions::DEFAULT),
            ggez::graphics::Rect::new(0.0, 0.0, dialog_w, dialog_h),
            bg_color
        ) {
            ggez::graphics::draw(ctx, &rect, DrawParam::default()
                .dest(na::Point2::new(cent_x - dialog_w * 0.5, cent_y - dialog_h * 0.5)) );
        }
        // thick purple border
        if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
            ggez::graphics::DrawMode::Stroke(stroke_options),
            ggez::graphics::Rect::new(0.0, 0.0, dialog_w, dialog_h),
            border_color
        ) {
            ggez::graphics::draw(ctx, &rect, DrawParam::default()
                .dest(na::Point2::new(cent_x - dialog_w * 0.5, cent_y - dialog_h * 0.5)) );
        }
        // thin outside black border
        stroke_options.line_width = 1.5;
        if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
            ggez::graphics::DrawMode::Stroke(stroke_options),
            ggez::graphics::Rect::new(0.0, 0.0, dialog_w + 6.0, dialog_h + 6.0),
            Color::new(0.0,0.0,0.0,1.0)
        ) {
            ggez::graphics::draw(ctx, &rect, DrawParam::default()
                .dest(na::Point2::new(cent_x - 3.0 - dialog_w * 0.5, cent_y - 3.0 - dialog_h * 0.5)) );
        }

        if !msg.is_empty() {
            let dialog_content = msg.clone();
            //level_name_content.pusgame_state.level.name.clone();
            let mut dialog_text = ggez::graphics::Text::new(dialog_content);
            dialog_text.set_font(game_state.font, Scale { x: 20.0, y: 20.0 });
            let text_w = dialog_text.width(ctx);
            let text_h = dialog_text.height(ctx);
            if let Err(_) = graphics::draw(ctx, &dialog_text,
                DrawParam::new()
                .dest(na::Point2::new(cent_x-(text_w as f32 / 2.0),cent_y-(text_h as f32 / 2.0)))
                //.scale(na::Vector2::new(2.0,2.0))
            ) {
                draw_ok = false;
            }
            if !draw_ok {
                println!("Draw error occurred");
            }
        }
        

    }
}