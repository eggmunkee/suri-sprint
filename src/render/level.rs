
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
                LevelItem::Player{x, y, .. } => {
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
                LevelItem::PlayerNamed {x, y, .. } => {
                    let mut stroke_opt = ggez::graphics::StrokeOptions::DEFAULT.clone();
                    stroke_opt.line_width = 4.0;
                    if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
                        ggez::graphics::DrawMode::Stroke(stroke_opt),
                        ggez::graphics::Rect::new(0.0, 0.0, 10.0, 10.0),
                        ggez::graphics::Color::new(1.0, 0.2, 0.0, 1.0)
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
                LevelItem::PortalSide {x, y, w, h, normal, ..} => {
                    let radius = (*w * 0.25 + *h * 0.25);

                    let mut stroke_opt = ggez::graphics::StrokeOptions::DEFAULT.clone();
                    stroke_opt.line_width = 4.0;
                    if let Ok(circle) = ggez::graphics::Mesh::new_circle(ctx, 
                        ggez::graphics::DrawMode::Stroke(stroke_opt),
                        na::Point2::<f32>::new(0.0, 0.0),
                        radius, 0.5,
                        ggez::graphics::Color::new(1.0, 0.7, 0.0, 0.5)
                    ) {
                        ggez::graphics::draw(ctx, &circle, DrawParam::default()
                            .dest(na::Point2::new(*x, *y))
                            .offset(na::Point2::new(*w, *w))
                            
                        );
                    }

                    if let Ok(line) = ggez::graphics::Mesh::new_line(ctx, 
                        &[na::Point2::<f32>::new(0.0, 0.0), na::Point2::<f32>::new(normal.0 * radius, normal.1 * radius)],
                        1.5,
                        ggez::graphics::Color::new(1.0, 1.0, 0.0, 0.75)
                    ) {
                        ggez::graphics::draw(ctx, &line, DrawParam::default()
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
                LevelItem::ExitCustom{x, y, w, ..} => {
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
                LevelItem::AnimSprite{x, y, angle, ..} => {
                    let mut stroke_opt = ggez::graphics::StrokeOptions::DEFAULT.clone();
                    stroke_opt.line_width = 4.0;
                    if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
                        ggez::graphics::DrawMode::Stroke(stroke_opt),
                        ggez::graphics::Rect::new(-10.0, -10.0, 20.0, 20.0),
                        ggez::graphics::Color::new(0.5, 1.0, 0.0, 0.5)
                    ) {
                        ggez::graphics::draw(ctx, &rect, DrawParam::default()
                            //.dest(na::Point2::new(x - 5.0, y - 5.0)) );
                            .dest(na::Point2::new(*x, *y))
                            .offset(na::Point2::new(0.0, 0.0))
                            .rotation(*angle));
                    }
                },
                LevelItem::Platform{x, y, w, h, ang, ..} | LevelItem::StaticLevelProp{x, y, w, h, ang, .. } => {
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
                LevelItem::DynPlatform{x, y, w, h, ang, ..} | LevelItem::DynStaticLevelProp{x, y, w, h, ang, .. } => {
                    let mut stroke_opt = ggez::graphics::StrokeOptions::DEFAULT.clone();
                    stroke_opt.line_width = 4.0;
                    if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
                        ggez::graphics::DrawMode::Stroke(stroke_opt),
                        ggez::graphics::Rect::new(0.0, 0.0, w*2.0, h*2.0),
                        ggez::graphics::Color::new(1.0, 0.0, 1.0, 0.5)
                    ) {
                        ggez::graphics::draw(ctx, &rect, DrawParam::default()                                
                            .dest(na::Point2::new(*x-*w, *y-*h))
                            .offset(na::Point2::new(*w, *h))
                            .rotation(*ang)
                             );
                    }
                },
                LevelItem::ParticleSys { x, y, .. } => {
                    let mut stroke_opt = ggez::graphics::StrokeOptions::DEFAULT.clone();
                    stroke_opt.line_width = 4.0;
                    if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
                        ggez::graphics::DrawMode::Stroke(stroke_opt),
                        ggez::graphics::Rect::new(-10.0, -10.0, 20.0, 20.0),
                        ggez::graphics::Color::new(0.0, 0.25, 1.0, 0.5)
                    ) {
                        ggez::graphics::draw(ctx, &rect, DrawParam::default()
                            //.dest(na::Point2::new(x - 5.0, y - 5.0)) );
                            .dest(na::Point2::new(*x, *y))
                            .offset(na::Point2::new(0.0, 0.0)));
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
                LevelItem::Bowl{x, y, ..} | LevelItem::Mouse{x, y, ..} => {
                    let mut stroke_opt = ggez::graphics::StrokeOptions::DEFAULT.clone();
                    stroke_opt.line_width = 4.0;
                    if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
                        ggez::graphics::DrawMode::Stroke(stroke_opt),
                        ggez::graphics::Rect::new(0.0, 0.0, 10.0, 10.0),
                        ggez::graphics::Color::new(0.2, 0.9, 1.0, 0.5)
                    ) {
                        ggez::graphics::draw(ctx, &rect, DrawParam::default()
                            .dest(na::Point2::new(x - 5.0, y - 5.0)) );
                    }
                },
                LevelItem::PointPickup{x, y, ..} => {
                    let mut stroke_opt = ggez::graphics::StrokeOptions::DEFAULT.clone();
                    stroke_opt.line_width = 4.0;
                    if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
                        ggez::graphics::DrawMode::Stroke(stroke_opt),
                        ggez::graphics::Rect::new(0.0, 0.0, 10.0, 10.0),
                        ggez::graphics::Color::new(0.8, 0.3, 1.0, 0.5)
                    ) {
                        ggez::graphics::draw(ctx, &rect, DrawParam::default()
                            .dest(na::Point2::new(x - 5.0, y - 5.0)) );
                    }
                },
                LevelItem::Connection{.. } => {
                    // let mut stroke_opt = ggez::graphics::StrokeOptions::DEFAULT.clone();
                    // stroke_opt.line_width = 4.0;
                    // if let Ok(rect) = ggez::graphics::Mesh::new_rectangle(ctx, 
                    //     ggez::graphics::DrawMode::Stroke(stroke_opt),
                    //     ggez::graphics::Rect::new(0.0, 0.0, 10.0, 10.0),
                    //     ggez::graphics::Color::new(0.2, 0.9, 1.0, 0.5)
                    // ) {
                    //     ggez::graphics::draw(ctx, &rect, DrawParam::default()
                    //         .dest(na::Point2::new(x - 5.0, y - 5.0)) );
                    // }
                },
                // _ => {

                // }
            }
        }
    }
}