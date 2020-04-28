
use ggez::{Context, GameResult, GameError};

use ggez::graphics;
use ggez::nalgebra as na;
use ggez::graphics::{Color,DrawParam,Scale,set_window_title};


use crate::game_state::{GameState};
use crate::entities::level_builder::*;


pub struct LevelRenderer {



}

impl LevelRenderer {

    pub fn render(game_state: &GameState, ctx: &mut Context) {

        for item in &game_state.level.items {

            match &item {
                LevelItem::Player{x, y} => {
                    let mut stroke_opt = ggez::graphics::StrokeOptions::DEFAULT.clone();
                    stroke_opt.line_width = 4.0;
                    if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
                        ggez::graphics::DrawMode::Stroke(stroke_opt),
                        ggez::graphics::Rect::new(0.0, 0.0, 10.0, 10.0),
                        ggez::graphics::Color::new(1.0, 0.0, 0.0, 1.0)
                    ) {
                        ggez::graphics::draw(ctx, &rect, DrawParam::default()
                            .dest(na::Point2::new(x - 5.0, y - 5.0)) );
                    }
                },
                LevelItem::Ghost{x, y, ..} => {
                    let mut stroke_opt = ggez::graphics::StrokeOptions::DEFAULT.clone();
                    stroke_opt.line_width = 4.0;
                    if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
                        ggez::graphics::DrawMode::Stroke(stroke_opt),
                        ggez::graphics::Rect::new(0.0, 0.0, 10.0, 10.0),
                        ggez::graphics::Color::new(1.0, 0.0, 1.0, 0.5)
                    ) {
                        ggez::graphics::draw(ctx, &rect, DrawParam::default()
                            .dest(na::Point2::new(x - 5.0, y - 5.0)) );
                    }
                },
                LevelItem::Portal{x, y, w, ..} => {
                    let mut stroke_opt = ggez::graphics::StrokeOptions::DEFAULT.clone();
                    stroke_opt.line_width = 4.0;
                    if let Ok(rect) = ggez::graphics::Mesh::new_circle(ctx, 
                        ggez::graphics::DrawMode::Stroke(stroke_opt),
                        na::Point2::<f32>::new(0.0, 0.0),
                        *w, 0.5,
                        ggez::graphics::Color::new(1.0, 1.0, 0.0, 0.5)
                    ) {
                        ggez::graphics::draw(ctx, &rect, DrawParam::default()
                            .dest(na::Point2::new(*x, *y))
                            .offset(na::Point2::new(*w, *w))
                            
                        );
                    }
                },
                LevelItem::Exit{x, y, w, ..} => {
                    let mut stroke_opt = ggez::graphics::StrokeOptions::DEFAULT.clone();
                    stroke_opt.line_width = 4.0;
                    if let Ok(rect) = ggez::graphics::Mesh::new_circle(ctx, 
                        ggez::graphics::DrawMode::Stroke(stroke_opt),
                        na::Point2::<f32>::new(0.0, 0.0),
                        *w, 0.5,
                        ggez::graphics::Color::new(0.0, 0.0, 1.0, 0.5)
                    ) {
                        ggez::graphics::draw(ctx, &rect, DrawParam::default()
                            .dest(na::Point2::<f32>::new(*x, *y)) );
                    }
                },
                LevelItem::Sprite{x, y, angle, ..} | LevelItem::DynSprite{x,y,angle, ..} => {
                    let mut stroke_opt = ggez::graphics::StrokeOptions::DEFAULT.clone();
                    stroke_opt.line_width = 4.0;
                    if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
                        ggez::graphics::DrawMode::Stroke(stroke_opt),
                        ggez::graphics::Rect::new(-10.0, -10.0, 20.0, 20.0),
                        ggez::graphics::Color::new(1.0, 1.0, 0.0, 0.5)
                    ) {
                        ggez::graphics::draw(ctx, &rect, DrawParam::default()
                            //.dest(na::Point2::new(x - 5.0, y - 5.0)) );
                            .dest(na::Point2::new(*x, *y))
                            .offset(na::Point2::new(0.0, 0.0))
                            .rotation(*angle));
                    }
                },
                LevelItem::Platform{x, y, w, h, ang, ..} | LevelItem::DynPlatform{x, y, w, h, ang, ..} => {
                    let mut stroke_opt = ggez::graphics::StrokeOptions::DEFAULT.clone();
                    stroke_opt.line_width = 4.0;
                    if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
                        ggez::graphics::DrawMode::Stroke(stroke_opt),
                        ggez::graphics::Rect::new(0.0, 0.0, w*2.0, h*2.0),
                        ggez::graphics::Color::new(1.0, 0.0, 0.0, 0.5)
                    ) {
                        ggez::graphics::draw(ctx, &rect, DrawParam::default()                                
                            .dest(na::Point2::new(*x-*w, *y-*h))
                            .offset(na::Point2::new(*w, *h))
                            .rotation(*ang)
                             );
                    }
                },
                LevelItem::Button{x, y, w, h, ang, ..} => {
                    let mut stroke_opt = ggez::graphics::StrokeOptions::DEFAULT.clone();
                    stroke_opt.line_width = 4.0;
                    if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
                        ggez::graphics::DrawMode::Stroke(stroke_opt),
                        ggez::graphics::Rect::new(0.0, 0.0, w*2.0, h*2.0),
                        ggez::graphics::Color::new(0.0, 0.0, 1.0, 0.5)
                    ) {
                        ggez::graphics::draw(ctx, &rect, DrawParam::default()                                
                            .dest(na::Point2::new(*x-*w, *y-*h))
                            .offset(na::Point2::new(*w, *h))
                            .rotation(*ang)
                             );
                    }
                },
                LevelItem::EmptyBox{x, y, w, h, ang, ..} | LevelItem::DynEmptyBox{x, y, w, h, ang, ..} => {
                    let mut stroke_opt = ggez::graphics::StrokeOptions::DEFAULT.clone();
                    stroke_opt.line_width = 4.0;
                    if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
                        ggez::graphics::DrawMode::Stroke(stroke_opt),
                        ggez::graphics::Rect::new(0.0, 0.0, w*2.0, h*2.0),
                        ggez::graphics::Color::new(0.0, 0.5, 1.0, 0.5)
                    ) {
                        ggez::graphics::draw(ctx, &rect, DrawParam::default()                                
                            .dest(na::Point2::new(*x-*w, *y-*h))
                            .offset(na::Point2::new(*w, *h))
                            .rotation(*ang)
                             );
                    }
                },
                _ => {

                }
            }
        }
    }
}