
use ggez::{Context, GameResult, GameError};

use ggez::graphics;
use ggez::nalgebra as na;
use ggez::graphics::{Color,DrawParam,Scale,FillOptions,set_window_title};

use crate::core::{GameState};
use crate::resources::{ImageResources};


pub struct DialogRenderer {



}

impl DialogRenderer {
    pub fn get_default_text_scale(game_state: &GameState) -> f32 {
        let (win_w, win_h) = (game_state.window_w, game_state.window_h);
        let text_scale = (win_w.min(win_h) / 50.0).max(5.0).min(50.0);
        text_scale
    }

    pub fn render(game_state: &GameState, ctx: &mut Context, msg: String) {
        let (w, h) = (game_state.window_w, game_state.window_h);
        let cent_x = w as f32 / 2.0;
        let cent_y = h as f32 / 4.0;
        let dialog_w = w as f32 * 0.75;
        let dialog_h = h as f32 * 0.25;
        let border_color = ggez::graphics::Color::new(0.7, 0.2, 0.7, 1.0);
        let bg_color = ggez::graphics::Color::new(0.3, 0.1, 0.3, 0.5);
        let text_color = ggez::graphics::Color::new(1.0, 1.0, 1.0, 1.0);

        Self::render_dialog(game_state, ctx, msg, cent_x, cent_y, dialog_w, dialog_h, border_color, bg_color, text_color);
    }

    pub fn render_at(game_state: &GameState, ctx: &mut Context, msg: String,
        x: f32, y: f32, dw: f32, dh: f32,
        border_color: Color, bg_color: Color, text_color: Color) {
        let (w, h) = (game_state.window_w, game_state.window_h);
        let cent_x = w as f32 * x;
        let cent_y = h as f32 * y;
        let dialog_w = w as f32 * dw;
        let dialog_h = h as f32 * dh;

        Self::render_dialog(game_state, ctx, msg, cent_x, cent_y, dialog_w, dialog_h, border_color, bg_color, text_color);
    }

    pub fn render_dialog(game_state: &GameState, ctx: &mut Context, msg: String,
        cent_x: f32, cent_y: f32, dialog_w: f32, dialog_h: f32,
        border_color: Color, bg_color: Color, text_color: Color) {

        let mut draw_ok = true;
        let (w, h) = (game_state.window_w, game_state.window_h);
        let text_scale = DialogRenderer::get_default_text_scale(game_state);

        let mut stroke_options = ggez::graphics::StrokeOptions::DEFAULT;
        stroke_options.line_width = 5.0;

        // dark transparent background
        if bg_color.a > 0.0 {
            if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
                ggez::graphics::DrawMode::Fill(ggez::graphics::FillOptions::DEFAULT),
                ggez::graphics::Rect::new(0.0, 0.0, dialog_w, dialog_h),
                bg_color
            ) {
                ggez::graphics::draw(ctx, &rect, DrawParam::default()
                    .dest(na::Point2::new(cent_x - dialog_w * 0.5, cent_y - dialog_h * 0.5)) );
            }
        }
        
        // thick purple border
        if border_color.a > 0.0 {
            if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
                ggez::graphics::DrawMode::Stroke(stroke_options),
                ggez::graphics::Rect::new(0.0, 0.0, dialog_w, dialog_h),
                border_color
            ) {
                ggez::graphics::draw(ctx, &rect, DrawParam::default()
                    .dest(na::Point2::new(cent_x - dialog_w * 0.5, cent_y - dialog_h * 0.5)) );
            }
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
            dialog_text.set_font(game_state.font, Scale { x: text_scale, y: text_scale });
            let text_w = dialog_text.width(ctx);
            let text_h = dialog_text.height(ctx);
            if let Err(_) = graphics::draw(ctx, &dialog_text,
                DrawParam::new()
                .dest(na::Point2::new(cent_x-(text_w as f32 / 2.0),cent_y-(text_h as f32 / 2.0)))
                .color(text_color)
                //.scale(na::Vector2::new(2.0,2.0))
            ) {
                draw_ok = false;
            }
            if !draw_ok {
                println!("Draw error occurred");
            }
        }
        

    }

    pub fn render_dialog_text(game_state: &GameState, ctx: &mut Context, msg: String,
        x: f32, y: f32, dw: f32, dh: f32, text_color: Color) {

        // println!("RenderDialogText {}", &msg);
        // println!("TextColor: {:?}", &text_color);

        let mut draw_ok = true;
        let (win_w, win_h) = (game_state.window_w, game_state.window_h);

        let cent_x = win_w as f32 * x;
        let cent_y = win_h as f32 * y;
        let dialog_w = win_w as f32 * dw;
        let dialog_h = win_h as f32 * dh;

        let text_scale = DialogRenderer::get_default_text_scale(game_state);

        let mut stroke_options = ggez::graphics::StrokeOptions::DEFAULT;
        stroke_options.line_width = 5.0;

        if !msg.is_empty() {
            let dialog_content = msg.clone();
            //level_name_content.pusgame_state.level.name.clone();
            // Set text bounds
            let dialog_txt_w = dialog_w - 10.0f32.max(win_w * 0.0125);
            let dialog_txt_h = dialog_h - 10.0f32.max(win_h * 0.0125);
            let mut dialog_text = ggez::graphics::Text::new(dialog_content);
            dialog_text.set_bounds(na::Point2::new(dialog_txt_w, dialog_txt_h), ggez::graphics::Align::Left);
            dialog_text.set_font(game_state.font, Scale { x: text_scale, y: text_scale });
            let text_w = dialog_text.width(ctx);
            let text_h = dialog_text.height(ctx);
            if let Err(_) = graphics::draw(ctx, &dialog_text,
                DrawParam::new()
                .dest(na::Point2::new(cent_x-(text_w as f32 / 2.0),cent_y-(text_h as f32 / 2.0)))
                .color(text_color)
                .offset(na::Point2::new(0.5f32,0.5f32))
                //.scale(na::Vector2::new(2.0,2.0))
            ) {
                draw_ok = false;
            }
            // if !draw_ok {
            //     println!("Draw error occurred");
            // }
        }
        

    }

    pub fn render_dialog_bg_textured(game_state: &GameState, ctx: &mut Context,
        x: f32, y: f32, dw: f32, dh: f32, dialog_texture: String) {

        let (win_w, win_h) = (game_state.window_w, game_state.window_h);

        let cent_x = win_w as f32 * x;
        let cent_y = win_h as f32 * y;
        let dialog_w = win_w as f32 * dw;
        let dialog_h = win_h as f32 * dh;

        let world = &game_state.world;
        let mut images = world.fetch_mut::<ImageResources>();
        let texture_ref = images.image_ref(dialog_texture.clone());

        let draw_pos = na::Point2::<f32>::new(cent_x, cent_y); //na::Point2::<f32>::new(win_w as f32 * cent_x, win_h as f32 * cent_y);

        if let Ok(mut texture) = texture_ref {
            let w = texture.width() as f32;
            let h = texture.height() as f32;
            //println!("Dialog texture sizes: {}, {}", &w, &h);
            let tex_scale = na::Vector2::new(dialog_w / w, dialog_h / h);
            //println!("Dialog texture scale: {}, {}", &tex_scale.x, &tex_scale.y);

            if let Err(_) = ggez::graphics::draw(ctx, texture, DrawParam::new()
                    //.src(ggez::graphics::Rect { x: 0.0, y: 0.0, w: 1.0, h: 1.0 })
                    .dest(draw_pos.clone())
                    //.rotation(angle) //rotation
                    .offset(na::Point2::new(0.5f32,0.5f32))
                    .scale(tex_scale)
                    .color(Color::new(1.0,1.0,1.0,1.0))) { 
                
                println!("Failed to render dialog texture");
            }

        }

    }

    pub fn render_dialog_textured(game_state: &GameState, ctx: &mut Context, msg: String,
        x: f32, y: f32, dw: f32, dh: f32,
        dialog_texture: String, text_color: Color) {

        let (win_w, win_h) = (game_state.window_w, game_state.window_h);
        let text_scale = DialogRenderer::get_default_text_scale(game_state); //(win_w.min(win_h) / 50.0).max(5.0).min(50.0);

        let cent_x = win_w as f32 * x;
        let cent_y = win_h as f32 * y;
        let dialog_w = win_w as f32 * dw;
        let dialog_h = win_h as f32 * dh;

        let world = &game_state.world;
        let mut images = world.fetch_mut::<ImageResources>();
        let texture_ref = images.image_ref(dialog_texture.clone());

        //let curr_transform = ggez::graphics::transform(ctx);

        let mut stroke_options = ggez::graphics::StrokeOptions::DEFAULT;
        stroke_options.line_width = 5.0;



        let draw_pos = na::Point2::<f32>::new(cent_x, cent_y); //na::Point2::<f32>::new(win_w as f32 * cent_x, win_h as f32 * cent_y);

        if let Ok(mut texture) = texture_ref {
            let w = texture.width() as f32;
            let h = texture.height() as f32;
            //println!("Dialog texture sizes: {}, {}", &w, &h);
            let tex_scale = na::Vector2::new(dialog_w / w, dialog_h / h);
            //println!("Dialog texture scale: {}, {}", &tex_scale.x, &tex_scale.y);

            if let Err(_) = ggez::graphics::draw(ctx, texture, DrawParam::new()
                    //.src(ggez::graphics::Rect { x: 0.0, y: 0.0, w: 1.0, h: 1.0 })
                    .dest(draw_pos.clone())
                    //.rotation(angle) //rotation
                    .offset(na::Point2::new(0.5f32,0.5f32))
                    .scale(tex_scale)
                    .color(Color::new(1.0,1.0,1.0,1.0))) { 
                
                println!("Failed to render dialog texture");
            }

        }

        if !msg.is_empty() {
            let dialog_content = msg.clone();
            //level_name_content.pusgame_state.level.name.clone();
            // Set text bounds
            let dialog_txt_w = dialog_w - 10.0f32.max(win_w * 0.0125);
            let dialog_txt_h = dialog_h - 10.0f32.max(win_h * 0.0125);
            let mut dialog_text = ggez::graphics::Text::new(dialog_content);
            dialog_text.set_bounds(na::Point2::new(dialog_txt_w, dialog_txt_h), ggez::graphics::Align::Left);
            dialog_text.set_font(game_state.font, Scale { x: text_scale, y: text_scale });
            let text_w = dialog_text.width(ctx);
            let text_h = dialog_text.height(ctx);
            if let Err(_) = graphics::draw(ctx, &dialog_text,
                DrawParam::new()
                .dest(na::Point2::new(cent_x-(text_w as f32 / 2.0),cent_y-(text_h as f32 / 2.0)))
                .color(text_color)
                .offset(na::Point2::new(0.5f32,0.5f32))
                //.scale(na::Vector2::new(2.0,2.0))
            ) {
                //draw_ok = false;
            }
            // if !draw_ok {
            //     println!("Draw error occurred");
            // }
        }


        // thin outside black border
        // stroke_options.line_width = 1.5;
        // if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
        //     ggez::graphics::DrawMode::Stroke(stroke_options),
        //     ggez::graphics::Rect::new(0.0, 0.0, dialog_w + 6.0, dialog_h + 6.0),
        //     Color::new(0.0,0.0,0.0,1.0)
        // ) {
        //     ggez::graphics::draw(ctx, &rect, DrawParam::default()
        //         .dest(na::Point2::new(cent_x - 3.0 - dialog_w * 0.5, cent_y - 3.0 - dialog_h * 0.5)) );
        // }



    }

    pub fn render_progress_area(game_state: &GameState, ctx: &mut Context, 
        x: f32, y: f32, dw: f32, dh: f32,
        bg_color: Color, border_color: Color,
        min_val: f32, max_val: f32, curr_val: f32) {

        let (win_w, win_h) = (game_state.window_w, game_state.window_h);
        let text_scale = DialogRenderer::get_default_text_scale(game_state); //(win_w.min(win_h) / 50.0).max(5.0).min(50.0);

        let cent_x = win_w as f32 * x;
        let cent_y = win_h as f32 * y;
        let dialog_w = win_w as f32 * dw;
        let dialog_h = win_h as f32 * dh;

        let prog_span = max_val - min_val;
        let progress_ratio = ((curr_val - min_val) / prog_span).max(0.0).min(1.0);

        let mut draw_ok = true;

        // let mut stroke_options = ggez::graphics::StrokeOptions::DEFAULT;
        // stroke_options.line_width = 2.0;

        // progress bar background
        if bg_color.a > 0.0 {
            if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
                ggez::graphics::DrawMode::Fill(ggez::graphics::FillOptions::DEFAULT),
                ggez::graphics::Rect::new(0.0, 0.0, dialog_w, dialog_h),
                bg_color
            ) {
                ggez::graphics::draw(ctx, &rect, DrawParam::default()
                    .dest(na::Point2::new(cent_x - dialog_w * 0.5, cent_y - dialog_h * 0.5)) );
            }
        }
        
        // progress bar indicator bar
        if progress_ratio > 0.0 && border_color.a > 0.0 {
            if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
                ggez::graphics::DrawMode::Fill(ggez::graphics::FillOptions::DEFAULT),
                ggez::graphics::Rect::new(0.0, 0.0, dialog_w * progress_ratio, dialog_h),
                border_color
            ) {
                ggez::graphics::draw(ctx, &rect, DrawParam::default()
                    .dest(na::Point2::new(cent_x - dialog_w * 0.5, cent_y - dialog_h * 0.5)) );
            }
        }
    }

    pub fn render_cursor(ctx: &mut Context, /* game_state: &GameState,*/ x: f32, y: f32, color: Color) {
    {
        // if game_state.game_frame_count % 60 == 1 {
        //     println!("  Render Cursor indicator ------------");
        // }

        let (mx, my) = (x, y);

        if let Ok(circle) = ggez::graphics::Mesh::new_circle(ctx, ggez::graphics::DrawMode::Fill(FillOptions::default()),
            na::Point2::new(0.0, 0.0), 10.0, 0.8, Color::new(1.0, 1.0, 1.0, 1.0) 
            // ggez::graphics::DrawMode::Stroke(stroke_opt),
            // ggez::graphics::Rect::new(0.0, 0.0, width, height),
            // ggez::graphics::Color::new(0.0, 0.0, 0.0, 0.5)
        ) {
            ggez::graphics::draw(ctx, &circle, DrawParam::default()
                .dest(na::Point2::new(mx, my))
                .color(Color::new(color.r, color.g, color.b, 0.3)) );

            ggez::graphics::draw(ctx, &circle, DrawParam::default()
                .dest(na::Point2::new(mx, my))
                .scale(na::Vector2::<f32>::new(0.5, 0.5)) 
                .color(Color::new(color.r, color.g, color.b, 0.5)));

            ggez::graphics::draw(ctx, &circle, DrawParam::default()
                .dest(na::Point2::new(mx, my))
                .scale(na::Vector2::<f32>::new(0.25, 0.25))
                .color(Color::new(color.r, color.g, color.b, 1.0)) );
                //.dest(na::Point2::new(mx, my)) );
        }

    }
    }
}