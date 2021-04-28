
use ggez::{Context};
use ggez::graphics::{Color};
use specs::{World,WorldExt,Builder,Entity};
use serde::{Deserialize,Serialize};
use rand::prelude::*;

use crate::components::{Position};
use crate::components::geometry::{GeometryComponent};
use crate::resources::{ImageResources};
use crate::components::flags::{RenderFlag,RenderLayerType,RenderAnimSpriteFlag};
use crate::core::physics::{PhysicsWorld};

#[derive(PartialEq,Clone,Debug)]
pub enum PatchCellShape {
    Square,
    TriangleLeftTop,
    TriangleRightTop,
    TriangleLeftBottom,
    TriangleRightBottom,
    DotLeftTop,
    DotRightTop,
    DotLeftBottom,
    DotRightBottom,
    Empty
}

#[derive(PartialEq,Clone,Debug,Deserialize,Serialize)]
pub struct LevelPatch {
    pub center: (f32, f32),
    pub size: (f32, f32),    
    #[serde(default="LevelPatch::default_cell_nums")]
    pub cell_nums: (i32, i32),
    #[serde(default="LevelPatch::default_cell_data")]
    pub cell_data: Vec::<i32>,
    #[serde(default="LevelPatch::default_z_order")]
    pub z_order: f32,
    #[serde(default)]
    pub image_path: String,
    #[serde(default="LevelPatch::default_color")]
    pub color: (f32, f32, f32, f32),
}

impl LevelPatch {
    pub fn default_cell_nums() -> (i32, i32) {
        (4, 4)
    }
    pub fn default_cell_data() -> Vec::<i32> {
        vec![1, 1, 1, 1, 1,
        1, 0, 0, 1, 1,
        1, 1, 0, 1, 1,
        1, 1, 1, 1, 1,
        0, 1, 1, 1, 0 ]
    }
    pub fn default_z_order() -> f32 { 500.0 }
    pub fn default_color() -> (f32, f32, f32, f32) { (1.0, 1.0, 1.0, 1.0) }

    pub fn new_filled(center: (f32, f32,), size: (f32, f32,), cell_nums: (i32, i32,), z_order: f32 ) -> Self {
        let x_points = cell_nums.0 + 1;
        let y_points = cell_nums.1 + 1;
        LevelPatch {
            center: (center.0, center.1),
            size: (size.0, size.1),
            cell_nums: (cell_nums.0, cell_nums.1),
            //cell_size: (cell_size_x, cell_size_y),
            cell_data: vec![1; (x_points * y_points) as usize],
            z_order: z_order, //LevelPatch::default_z_order(),
            image_path: String::new(),
            color: LevelPatch::default_color(),
        }
    }

    pub fn new_empty(center: (f32, f32,), size: (f32, f32,), cell_nums: (i32, i32,), z_order: f32 ) -> Self {
        let x_points = cell_nums.0 + 1;
        let y_points = cell_nums.1 + 1;
        LevelPatch {
            center: (center.0, center.1),
            size: (size.0, size.1),
            cell_nums: (cell_nums.0, cell_nums.1),
            //cell_size: (cell_size_x, cell_size_y),
            cell_data: vec![0; (x_points * y_points) as usize],
            z_order: z_order, //LevelPatch::default_z_order(),
            image_path: String::new(),
            color: LevelPatch::default_color(),
        }
    }

    pub fn get_cell_sizes(&self) -> (f32, f32) {
        let cell_size_x = self.size.0 / (self.cell_nums.0.max(1)) as f32;
        let cell_size_y = self.size.1 / (self.cell_nums.1.max(1)) as f32;
        (cell_size_x, cell_size_y)
    }

    pub fn valid_point(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x > self.cell_nums.0 || y > self.cell_nums.1 {
            return false;
        }
        true
    }

    pub fn get_value(&self, x: i32, y: i32) -> i32 {
        if !self.valid_point(x, y) {
            return 0;
        }
        let index = y * (self.cell_nums.0 + 1) + x;
        if index >= 0 && index < self.cell_data.len() as i32 {
            self.cell_data[index as usize]
        }
        else {
            0
        }
    }

    pub fn get_line_value(&self, x: i32, y: i32, x2: i32, y2: i32) -> bool {
        if !self.valid_point(x, y) || !self.valid_point(x2, y2) {
            return false;
        }
        
        self.get_value(x, y) > 0 && self.get_value(x2, y2) > 0
    }

    pub fn set_value(&mut self, x: i32, y: i32, val: i32) -> bool {
        let index = y * (self.cell_nums.0 + 1) + x;
        if index >= 0 && index < self.cell_data.len() as i32 {
            self.cell_data[index as usize] = val;
            true
        }
        else {
            false
        }
    }

    pub fn random_point(&self) -> (i32, i32) {
        let mut rnd = rand::thread_rng();

        let x = rnd.gen::<i32>() % (self.cell_nums.0 + 1);
        let y = rnd.gen::<i32>() % (self.cell_nums.1 + 1);

        (x, y)
    }

    pub fn modify_random(&mut self) -> (i32, i32, i32) {
        let mut rng = rand::thread_rng();
        let (x, y) = self.random_point();
        let val = rng.gen::<i32>() % 2;
        let block = (rng.gen::<i32>() % 20 - 13).max(0) + 3;
        let form = rng.gen::<i32>() % 3;

        self.set_value(x, y, val);

        if block > 1 {
            // how many cells left of x
            let before_cell = ((block as f32 - 1.0) / 2.0).ceil() as i32;
            // col of first block cell on left
            let i_start = if form > 0 { x - before_cell } else { x };
            let j_start = if form < 2 { y - before_cell } else { y };
            // col of last block cell on right
            let i_end = if form > 0 { i_start + block } else { x + 1 };
            let j_end = if form < 2 { j_start + block } else { y + 1};
            
            for i in i_start..i_end {
                for j in j_start..j_end {
                    if !(i == x && j == y) {
                        self.set_value(i, j, val);
                    }
                }
            }
        }

        (x, y, val)
    }

    pub fn modify_random_many(&mut self, mod_count: i32) {
        for i in 0..mod_count {
            let (_x, _y, _val) = self.modify_random();
        }
    }

    pub fn triangulate(&self) -> Vec::<(i32, i32, PatchCellShape)> {
        let mut shapes = vec![];

        for row in 0..self.cell_nums.1 {
            let top_row = row;
            let bottom_row = row + 1;
            //print!("Row {},", &row);

            // go through the cells
            for col in 0..self.cell_nums.0 {
                let left_col = col;
                let right_col = col + 1;
                //print!("Col {} ", &col);

                let val_tl = self.get_value(left_col, top_row);
                let val_tr = self.get_value(right_col, top_row);
                let val_br = self.get_value(right_col, bottom_row);
                let val_bl = self.get_value(left_col, bottom_row);
                //print!("[{:?}-{:?} / {:?}-{:?}],", &val_tl, &val_tr, &val_bl, &val_br);

                let left_filled = val_tl > 0 && val_bl > 0;
                let right_filled = val_tr > 0 && val_br > 0;
                let top_filled = val_tl > 0 && val_tr > 0;
                let bottom_filled = val_bl > 0 && val_br > 0;

                // find which shape is present
                // FULLY Filled if 3 sides are filled

                let cell_shape = if left_filled && top_filled && right_filled {
                    PatchCellShape::Square
                }
                else if left_filled && top_filled {
                    PatchCellShape::TriangleLeftTop
                }
                else if right_filled && top_filled {
                    PatchCellShape::TriangleRightTop
                }
                else if left_filled && bottom_filled {
                    PatchCellShape::TriangleLeftBottom
                }
                else if right_filled && bottom_filled {
                    PatchCellShape::TriangleRightBottom
                }
                else {
                    PatchCellShape::Empty
                };

                if cell_shape != PatchCellShape::Empty {
                    shapes.push((left_col,top_row,cell_shape));
                }
                // Handle lone point geometry - if it's not part of any shape around it
                else if val_tl > 0 || val_tr > 0 || val_bl > 0 || val_br > 0 {
                    let rightmost_cell = right_col == self.cell_nums.0 + 1;
                    let bottommost_row = bottom_row == self.cell_nums.1 + 1;

                    // Top-left Dot is "Our" cell's dot - it should be added for all cells if needed
                    if val_tl > 0 {
                        // Check edges on left and top
                        if !self.get_line_value(left_col - 1, top_row, left_col - 1, bottom_row) &&
                                !self.get_line_value(left_col - 1, top_row - 1, left_col - 1, top_row) &&
                                !self.get_line_value(left_col - 1, top_row - 1, left_col, top_row - 1) &&
                                !self.get_line_value(left_col, top_row - 1, right_col, top_row - 1) &&
                                !self.get_line_value(right_col, top_row - 1, right_col, top_row) &&

                                !(self.get_line_value(left_col - 1, bottom_row, left_col, bottom_row) && // bl horiz
                                    self.get_line_value(left_col, top_row, left_col, bottom_row)) && // cell left vertial

                                !(self.get_line_value(left_col - 1, top_row, left_col, top_row) && // left horiz
                                    self.get_line_value(left_col, top_row, left_col, bottom_row)) && // cell left vertial

                                !(self.get_line_value(left_col - 1, top_row, left_col, top_row) && // left horiz
                                    self.get_line_value(left_col, top_row - 1, left_col, top_row)) && // top vertical

                                !(self.get_line_value(left_col, top_row, right_col, top_row) && // cell top horiz
                                    self.get_line_value(left_col, top_row - 1, left_col, top_row)) // top vertical

                                {
                            let shape = PatchCellShape::DotLeftTop;
                            shapes.push((left_col, top_row, shape));
                        }
                    }
                    // Only add dots on right side when at the last cell on the row
                    if rightmost_cell && val_tr > 0 {
                        // On right col, add top-right dat as long as there's no line to the top of the cell
                        if !self.get_line_value(left_col, top_row - 1, right_col, top_row - 1) {
                            let shape = PatchCellShape::DotRightTop;
                            shapes.push((left_col, top_row, shape));
                        }
                    }

                    if bottommost_row && val_bl > 0 {
                        // On bottom row, add bottom-left dot as long as there's no line to the left of the cell
                        if !self.get_line_value(left_col - 1, top_row, left_col - 1, bottom_row) {
                            let shape = PatchCellShape::DotRightTop;
                            shapes.push((left_col, top_row, shape));
                        }
                    }

                    if rightmost_cell && bottommost_row && val_br > 0 {
                        // In bottom-right corner cell, add dot because there are no shape possibilities
                        let shape = PatchCellShape::DotRightTop;
                        shapes.push((left_col, top_row, shape));
                    }





                    /*if rightmost_cell && val_tr && !val_br {
                        let right_chk = if right_col + 1 < self.cell_nums.0 + 1 {
                            self.get_value(right_col + 1, top_row)
                        }
                        else {
                            false
                        };
                        let top_chk = if top_row - 1 >= 0 {
                            self.get_value(right_col, top_row - 1)
                        }
                        else {
                            false
                        };
                        if !right_chk && !top_chk {
                            let shape = PatchCellShape::DotRightTop;
                            shapes.push((left_col, top_row, shape));
                        }
                    }
                    if rightmost_cell && bottommost_row && val_br && !val_bl {
                        let right_chk = if right_col + 1 < self.cell_nums.0 + 1 {
                            self.get_value(right_col + 1, bottom_row)
                        }
                        else {
                            false
                        };
                        let bottom_chk = if bottom_row + 1 < self.cell_nums.1 + 1 {
                            self.get_value(right_col, bottom_row + 1)
                        }
                        else {
                            false
                        };
                        if !right_chk && !bottom_chk {
                            let shape = PatchCellShape::DotRightBottom;
                            shapes.push((left_col, top_row, shape));
                        }
                    }
                    if bottommost_row && val_bl {
                        let left_chk = if left_col - 1 >= 0 {
                            self.get_value(left_col - 1, bottom_row)
                        }
                        else {
                            false
                        };
                        let bottom_chk = if bottom_row + 1 < self.cell_nums.1 + 1 {
                            self.get_value(left_col, bottom_row + 1)
                        }
                        else {
                            false
                        };
                        if !left_chk && !bottom_chk {
                            let shape = PatchCellShape::DotLeftBottom;
                            shapes.push((left_col, top_row, shape));
                        }
                    }*/

                    
                    

                    


                }
            }

            //println!("");

        }

        shapes
    }

}

#[derive(PartialEq,Clone,Debug,Deserialize,Serialize)]
pub struct LevelGridData {
    pub patches : Vec::<LevelPatch>,    
}

impl LevelGridData {
    pub fn new() -> Self {
        Self {
            patches: vec![],
        }
    }

    pub fn new_test_data() -> Self {

        let mut this = Self::new();

        // pos / size / cells

        let mut patch = LevelPatch::new_filled(
            (600.0, 600.0),
            (1800.0, 1800.0),
            (15, 15),
            100.0,
        );
        patch.image_path = "/images/dirt-grid-1.png".to_string();
        patch.modify_random_many(350);
        this.add_patch(patch);

        let mut patch = LevelPatch::new_empty(
            (600.0, 600.0),
            (1800.0, 1800.0),
            (30, 30),
            300.0,
        );
        patch.modify_random_many(350);
        this.add_patch(patch);


        let mut patch = LevelPatch::new_empty(
            (600.0, 600.0),
            (1800.0, 1800.0),
            (45, 45),
            500.0,
        );
        patch.modify_random_many(350);
        this.add_patch(patch);

        this
    }

    pub fn add_patch(&mut self, patch: LevelPatch) {
        self.patches.push(patch);
    }

    pub fn clear(&mut self) {
        self.patches.clear();
    }

    pub fn build(world: &mut World, ctx: &mut Context, physics_world: &mut PhysicsWorld, grid_data: LevelGridData) -> Entity {

        let mut geometry_comp = GeometryComponent::new();
        geometry_comp.geometry = grid_data;

        if let Some(mut images) = world.get_mut::<ImageResources>() {
            for patch in &geometry_comp.geometry.patches {
                // geometry_comp.colors.push(Color::new(1.0, 1.0, 1.0, 0.5));
                // geometry_comp.z_orders.push(patch.z_order);
                // geometry_comp.image_paths.push("/images/dirt-grid-1.png".to_string());
                if !patch.image_path.is_empty() {
                    images.load_image(patch.image_path.clone(), ctx);
                }
            }
        }

        // let mut collision = Collision::new_specs(0.05,0.750, width, height);
        // // collision.dim_1 = width;
        // // collision.dim_2 = height;
        // collision.pos.x = x;
        // collision.pos.y = y;
        // collision.is_sensor = true;
        // collision.entity_type = EntityType::PickupItem(PickupItemType::Point);
        // collision.collision_category = CollisionCategory::Level;
        // collision.collision_mask.clear();
        // collision.collision_mask.push(CollisionCategory::Player);
        // collision.collision_mask.push(CollisionCategory::Level);
        // collision.create_dynamic_body_circle(physics_world);

        //let body_handle_clone = collision.body_handle.clone();

        let entity = world.create_entity()
        .with(geometry_comp)
        .with(Position { x: 0.0, y: 0.0 })
        //.with(collision)
        .with(RenderFlag::from_layer(RenderLayerType::LevelLayer))
        //.with(RenderAnimSpriteFlag)
        .build();

        // let entity_id = entity.id();
        // if let Some(body_handle) = body_handle_clone {
        //     let mut collision_body = physics_world.body_mut(body_handle);
        //     let body_data = &mut *collision_body.user_data_mut();
        //     //let data = &*data_ref;
        //     body_data.entity_id = entity_id;
        // }

        entity
    }
}