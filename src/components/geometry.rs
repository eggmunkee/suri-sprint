
use ggez::nalgebra as na;
use ggez::{Context};
use ggez::graphics;
use ggez::graphics::{Color,FillOptions,Rect,DrawMode,DrawParam};
use specs::{Component, DenseVecStorage, World, WorldExt, Entity};
use specs_derive::*;
//use wrapped2d::b2;
//use rand::prelude::*;
//use serde::{Deserialize,de::DeserializeOwned,Serialize};

use crate::core::game_state::{GameState};
use crate::entities::geometry::{LevelGridData,PatchCellShape};
use crate::components::anim_sprite::{AnimSpriteComponent};
use crate::resources::{ImageResources};


#[derive(Debug,Component)]
#[storage(DenseVecStorage)]
pub struct GeometryComponent {
    // Store the patches of level geometry
    pub geometry: LevelGridData,
    // // Z indices for the list of pathces
    // pub z_orders: Vec::<f32>,
    // // image source paths or color values
    // pub image_paths: Vec::<String>,
    // pub colors: Vec::<Color>,
}

impl GeometryComponent {
    pub fn new() -> Self {
        Self {
            geometry: LevelGridData::new(),
            // z_orders: vec![],
            // image_paths: vec![],
            // colors: vec![],
        }
    }

}


impl super::RenderItemTarget for GeometryComponent {
    fn render_item(game_state: &GameState, ctx: &mut Context, entity: &Entity,
        pos: &na::Point2<f32>, item_index: usize) {
            //print!("GeometryComponent::render_item(),");
            let world = &game_state.world;
            let geometry_res = world.read_storage::<GeometryComponent>();

            // Get Sprite Component to call draw method            
            if let Some(geometry_comp) = geometry_res.get(entity.clone()) {
                use crate::components::{RenderTrait};
                geometry_comp.draw(ctx, world, Some(entity.id()), pos.clone(), item_index);
            }
        }
}


impl super::RenderTrait for GeometryComponent {
    fn draw(&self, ctx: &mut Context, world: &World, _ent: Option<u32>, _pos: na::Point2::<f32>, item_index: usize) {
        //let mut rng = rand::thread_rng();
        let mut image_res = world.fetch_mut::<ImageResources>();
        let patches_len = self.geometry.patches.len(); //.min(self.image_paths.len()).min(self.colors.len()).min(self.z_orders.len());
        //println!("Rendering Geometry with length: {}", &patches_len);

        if item_index < patches_len {

            if let Some(patch) = //, Some(image_path), Some(color), Some(z_order)) = 
                self.geometry.patches.get(item_index) //, self.image_paths.get(item_index),
                //self.colors.get(item_index), self.z_orders.get(item_index)) 
                {

                let solid_color = patch.image_path.is_empty();

                // Setup generic patch variables
                let (x_size, y_size) = patch.get_cell_sizes();
                let (x_cells, y_cells) = patch.cell_nums;
                let start_x = patch.center.0 - x_size * (x_cells as f32 / 2.0);
                let start_y = patch.center.1 - y_size * (y_cells as f32 / 2.0);

                // Create shapes from grid data
                let shapes = patch.triangulate();

                for (i, j, shape) in shapes {
                    let pos_x = start_x + (i as f32 * x_size);
                    let pos_y = start_y + (j as f32 * y_size);

                    //println!("Cell Result ({}, {}, {:?})", &i, &j, &shape);

                    //let mut draw_shape = true;

                    // Solid color rendering
                    if solid_color {
                        let points = if shape == PatchCellShape::Square {
                            vec![
                                na::Point2::new(0.0, 0.0),
                                na::Point2::new(x_size, 0.0),
                                na::Point2::new(x_size,  y_size),
                                na::Point2::new(0.0, y_size)
                            ]
                        }
                        else if shape == PatchCellShape::TriangleLeftTop {
                            vec![
                                na::Point2::new(0.0, 0.0),
                                na::Point2::new(x_size, 0.0),
                                na::Point2::new(0.0, y_size)
                            ]
                        } 
                        else if shape == PatchCellShape::TriangleRightTop {
                            vec![
                                na::Point2::new(0.0, 0.0),
                                na::Point2::new(x_size, 0.0),
                                na::Point2::new(x_size, y_size)
                            ]
                        }
                        else if shape == PatchCellShape::TriangleLeftBottom {
                            vec![
                                na::Point2::new(0.0, 0.0),
                                na::Point2::new(x_size, y_size),
                                na::Point2::new(0.0, y_size)
                            ]
                        }
                        else if shape == PatchCellShape::TriangleRightBottom {
                            vec![
                                na::Point2::new(0.0, y_size),
                                na::Point2::new(x_size, 0.0),
                                na::Point2::new(x_size, y_size)
                            ]
                        }
                        else if shape == PatchCellShape::DotLeftTop {
                            vec![
                                na::Point2::new(x_size*0.15, y_size*0.15),
                                na::Point2::new(x_size*0.85, y_size*0.15),
                                na::Point2::new(x_size*0.85, y_size*0.85),
                                na::Point2::new(x_size*0.15, y_size*0.85)
                            ]
                        } 
                        else if shape == PatchCellShape::DotRightTop {
                            vec![
                                na::Point2::new(x_size*0.7, 0.0),
                                na::Point2::new(x_size*1.0, 0.0),
                                na::Point2::new(x_size*1.0,  y_size*0.3),
                                na::Point2::new(x_size*0.7, y_size*0.3)
                            ]
                        }
                        else if shape == PatchCellShape::DotLeftBottom {
                            vec![
                                na::Point2::new(0.0, y_size*0.7),
                                na::Point2::new(x_size*0.3, y_size*0.7),
                                na::Point2::new(x_size*0.3,  y_size*1.0),
                                na::Point2::new(0.0, y_size*1.0)
                            ]
                        }
                        else if shape == PatchCellShape::DotRightBottom {
                            vec![
                                na::Point2::new(x_size*0.7, y_size*0.7),
                                na::Point2::new(x_size*1.0, y_size*0.7),
                                na::Point2::new(x_size*1.0,  y_size*1.0),
                                na::Point2::new(x_size*0.7, y_size*1.0)
                            ]
                        }
                        else {
                            //draw_shape = false;
                            vec![]
                        };
                        if points.len() > 0 {
                            if let Ok(poly) = ggez::graphics::Mesh::new_polygon(ctx, 
                                DrawMode::Fill(FillOptions::default()), 
                                &points, 
                                Color::new(patch.color.0,patch.color.1,patch.color.2,patch.color.3)
                            ) {
                                let _err = graphics::draw(ctx, &poly, DrawParam::new()
                                    .dest(na::Point2::new(pos_x, pos_y))
                                ).is_err();
                            }
                        }                        
                    }
                    // Image tileset display
                    else if let Ok(image_ref) = image_res.image_ref(patch.image_path.clone()) {
                        let img_w = image_ref.width() as f32 / 4.0;
                        let img_h = image_ref.height() as f32 / 8.0;
                        let scale = (x_size / img_w, y_size / img_h);
    
                        // Setup rows and columns of the spritesheet for calculating cell/frame boundaries
                        let anim_cols = 4;
                        let anim_rows = 8;
                        // Calculate / map shape to spritesheet cell as source of that section in texture space (0-1,0-1)
                        let src = match shape {
                            PatchCellShape::Square => AnimSpriteComponent::calc_frame_src(0, 0, anim_rows, anim_cols),
                            PatchCellShape::TriangleLeftTop => AnimSpriteComponent::calc_frame_src(0, 1, anim_rows, anim_cols),
                            PatchCellShape::TriangleRightTop => AnimSpriteComponent::calc_frame_src(1, 1, anim_rows, anim_cols),
                            PatchCellShape::TriangleLeftBottom => AnimSpriteComponent::calc_frame_src(2, 1, anim_rows, anim_cols),
                            PatchCellShape::TriangleRightBottom => AnimSpriteComponent::calc_frame_src(3, 1, anim_rows, anim_cols),
                            PatchCellShape::DotLeftTop | PatchCellShape::DotRightTop
                                | PatchCellShape::DotLeftBottom | PatchCellShape::DotRightBottom
                                => AnimSpriteComponent::calc_frame_src(0, 2, anim_rows, anim_cols),
                            _ => AnimSpriteComponent::calc_frame_src(3, 3, anim_rows, anim_cols),

                        };
                        //println!("Shape {:?} frame source: {:?}", &shape, &src);

                        let _err = graphics::draw(ctx, image_ref, DrawParam::new()
                                .dest(na::Point2::new(pos_x, pos_y))
                                .scale(na::Vector2::new(scale.0, scale.1))
                                .src(Rect::new(src.0, src.1, src.2, src.3))
                                .color(Color::new(patch.color.0,patch.color.1,patch.color.2,patch.color.3))
                            ).is_err();
                    }
                    
                }

            }
        }

    }
}

// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    world.register::<GeometryComponent>();
}