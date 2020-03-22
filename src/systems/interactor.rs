
use std::collections::hash_map::*;
use ggez::nalgebra as na;
use rand::prelude::*;
use specs::prelude::*;
use specs::{WorldExt};
use ggez::nalgebra::{Point2,Vector2,distance,distance_squared};

use crate::resources::{InputResource};
use crate::components::*;
use crate::components::collision::*;
use crate::physics;


// handle interactions between interactive actors
pub struct InterActorSys;
impl InterActorSys {
    pub fn new() -> InterActorSys {
        InterActorSys
    }
}

impl InterActorSys {

    // fn dist_check(pt_1: &Point2<f32>, pt_2: &Point2<f32>, touch_dist: f32) -> (bool, f32) {
    //     //let pt = Point2::new(500.0f32,500.0);
    //     //let radius = 50.0f32;
    //     let d = distance_squared(&pt_1.clone(), &pt_2.clone());
    //     //if d < radius {
    //         //println!("Block passed for (0,0) and {:?}", check_point);
    //     //}
    //     if d < touch_dist.powi(2) {
    //         (true, d.sqrt())
    //     }
    //     else {
    //         (false, d)
    //     }
       
    // }

    
}

impl<'a> System<'a> for InterActorSys {
    type SystemData = (WriteStorage<'a, Position>,
                        WriteStorage<'a, Velocity>,                        
                        ReadStorage<'a, Collision>,
                        Entities<'a>);

    fn run(&mut self, (mut pos, mut vel, collision, ent): Self::SystemData) {
        use specs::Join;
        let mut rng = rand::thread_rng();
        //let mut blocks = Vec::<(u32,f32,f32,i32,i32)>::new();
        // key is col,val of block, value is entity id, position x/y, 
        let mut block_hash = HashMap::<(i32,i32),Vec<(u32,f32,f32,f32,f32)>>::new();
        let mut check_hash = HashMap::<(u32,u32),bool>::new();

        // iterator over velocities with player components and input
        for (pos, coll, e) in (&pos, &collision, &ent).join() {     

            let ent_id = e.id();
            let mass = coll.mass;
            let friction = coll.friction;
            let px = pos.x;
            let py = pos.y;
            let buc_a_sz = 100.0f32;
            let buc_a_x : i32 = (px / buc_a_sz) as i32;
            let buc_a_y : i32 = (py / buc_a_sz) as i32;

            // find which neighbor cells to include in
            let rad = 45.0;
            let l_rem = px - (buc_a_x as f32) * buc_a_sz;
            let r_rem = ((buc_a_x + 1) as f32) * buc_a_sz - px;
            let t_rem = py - (buc_a_y as f32) * buc_a_sz;
            let b_rem = ((buc_a_y + 1) as f32) * buc_a_sz - py;

            let mut buc_a_xs = buc_a_x;
            let mut buc_a_xe = buc_a_x;
            let mut buc_a_ys = buc_a_y;
            let mut buc_a_ye = buc_a_y;
            if l_rem < rad {
                buc_a_xs -= 1;
            }
            if r_rem < rad {
                buc_a_xe += 1;
            }
            if t_rem < rad {
                buc_a_ys -= 1;
            }
            if b_rem < rad {
                buc_a_ye += 1;
            }

            //println!("Bucket layout for {}, ({},{})-({},{})", &ent_id, &buc_a_xs, &buc_a_ys, &buc_a_xe, &buc_a_ye);

            for i in buc_a_xs..buc_a_xe+1 {
                for j in buc_a_ys..buc_a_ye+1 {
                    if block_hash.contains_key(&(i,j)) {
                        let mut ent_vec = block_hash.get_mut(&(i,j)).unwrap();                
                        ent_vec.push((
                            ent_id,
                            px, py,
                            mass,
                            friction,
                        ));
                    }
                    else {
                        let ent_vec = vec![(
                            ent_id,
                            px, py,
                            mass,
                            friction,
                        )];
                        //let hash_key = buc_a_str.clone().as_str();
                        block_hash.insert((i,j), ent_vec);
                    }
                }
            }

            

            //} 
            //println!("InterActor sys proc for entity: {}", &e.id());   
        }
        //println!("Blocks: {:?}", &blocks);

        {
            for &key in block_hash.keys() {
                let v = block_hash.get(&key).unwrap();
                // consider all buckets with multiple items
                if v.len() > 1 {
                    //println!("BlockHash {:?} - len {}", key, &v.len());
                   
                    let mut ind_i = 0usize;
                    let mut ind_j = 0usize;
                    for &(entid_i,pos_ix,pos_iy,mass_i,fric_i) in v.iter() {
                        for &(entid_j,pos_jx,pos_jy,mass_j,fric_j) in v.iter() {
                            if ind_j != ind_i && entid_i != entid_j {
                                //println!("Hit: i={} and j={}", &entid_i, &entid_j);
                                // Check if pair was already processed and skip if so
                                if check_hash.contains_key(&(entid_i,entid_j)) ||
                                    check_hash.contains_key(&(entid_j,entid_i)) {
                                    //println!("Skipping i={} and j={}", &entid_i, &entid_j);
                                    continue;
                                }
                                //println!("Checking i={} and j={}", &entid_i, &entid_j);
                                check_hash.insert((entid_i,entid_j), true);

                                let ent_i = ent.entity(entid_i);
                                let ent_j = ent.entity(entid_j);
                                // if both alive
                                if ent_i.gen().is_alive() && ent_j.gen().is_alive() {

                                    // get velocity resources from entity pair
                                    let svel_i = vel.get(ent_i);
                                    let svel_j = vel.get(ent_j);
                                    // build points for each's position
                                    let pos_i = na::Point2::new(pos_ix, pos_iy);
                                    let pos_j = na::Point2::new(pos_jx, pos_jy);
                                    // build holders for possible vector component values
                                    let mut vel_i : Option<&mut na::Vector2<f32>> = None;
                                    let mut vel_j : Option<&mut na::Vector2<f32>> = None;
                                    // build vectors to pass to actors_push - to edit
                                    let mut vel_veci = na::Vector2::new(0.0,0.0);
                                    let mut vel_vecj = na::Vector2::new(0.0,0.0);
                                    // get possibilities of velocity components from
                                    //  each entity
                                    let mut any_movable = false;
                                    if let Some(velocity_i) = svel_i {
                                        // copy velocity value into edit vel
                                        if velocity_i.frozen == false {
                                            vel_veci.x = velocity_i.x;
                                            vel_veci.y = velocity_i.y;
                                            vel_i = Some(&mut vel_veci);
                                            any_movable = true;
                                        }
                                    }
                                    if let Some(velocity_j) = svel_j {
                                        // copy velocity value into edit vel
                                        if velocity_j.frozen == false {
                                            vel_vecj.x = velocity_j.x;
                                            vel_vecj.y = velocity_j.y;
                                            vel_j = Some(&mut vel_vecj);
                                            any_movable = true;
                                        }
                                    }

                                    if any_movable {
                                        // Calculcate and update the velocities of these two entities

                                        physics::actors_push(&pos_i, &pos_j, vel_i, vel_j,
                                            mass_i, mass_j, fric_i, fric_j, 50.0);
                                        // physics::actors_push_squares(&pos_i, &pos_j, vel_i, vel_j,
                                        //     mass_i, mass_j, fric_i, fric_j, 40.0);

                                        // Update velocities of pair of entities
                                        // if they have velocity components, update the velocity
                                        let svel_i = vel.get_mut(ent_i);
                                        if let Some(mut velocity_i) = svel_i {
                                            if velocity_i.frozen == false {
                                                velocity_i.x = vel_veci.x;
                                                velocity_i.y = vel_veci.y;
                                            }
                                            else {
                                                //velocity_i.frozen = false;
                                            }
                                        }
                                        let svel_j = vel.get_mut(ent_j);
                                        if let Some(mut velocity_j) = svel_j {
                                            if velocity_j.frozen == false {
                                                velocity_j.x = vel_vecj.x;
                                                velocity_j.y = vel_vecj.y;
                                            }
                                            else {
                                                //velocity_j.frozen = false;
                                            }
                                        }
                                    }
                                    
                                    
                                }

                            }
                            ind_j += 1;
                        }
                        ind_i += 1;
                    }
                }
            }
        }

        // let mut interact_ct = 0usize;
        // let mut ind_i = 0usize;
        // let mut ind_j = 0usize;
        // for &(entid_i,pos_ix,pox_iy,buc_ix,buc_iy) in blocks.iter() {
        //     for &(entid_j,pos_jx,pox_jy,buc_jx,buc_jy) in blocks.iter() {
        //         if ind_j < ind_i && entid_i != entid_j {
        //             //let _i = i.clone();
        //             let ent_i = ent.entity(entid_i);
        //             let ent_j = ent.entity(entid_j);
        //             // If both entities are still alive --
        //             if ent_i.gen().is_alive() && ent_j.gen().is_alive() {
                        // get possible positions and collisions for i & j entities
                        // let spos_i = pos.get(ent_i);
                        // let spos_j = pos.get(ent_j);
                        // let scol_i = collision.get(ent_i);
                        // let scol_j = collision.get(ent_j);

                        // // If both have both position and collision components
                        // if let (Some(pos_i), Some(pos_j), Some(col_i), Some(col_j))
                        //         = (spos_i, spos_j, scol_i, scol_j) {

                            // CONSIDER THIS PAIR FOR INTERACTIONS


        //                     let dist = distance(&na::Point2::new(pos_i.x,pos_i.y), &na::Point2::new(pos_j.x, pos_j.y));
        //                     if dist < 30.0 {
        //                         //println!("Entity: {}:{},{} - {}:{},{}", &i, pos_i.x, pos_i.y, &j, pos_j.x, pos_j.y);


        //                         let impulse : f32 = 40.0;
        //                         let pt = na::Point2::new(pos_i.x,pos_i.y);
        //                         //let (check, dist) = coll.pt_block_check(&pt);
        //                         if true {
        //                             let mut imp = impulse;
        //                             if dist > 1.0 {
        //                                 let x_dif = (pos_i.x - pos_j.x) / dist;
        //                                 let y_dif = (pos_i.y - pos_j.y) / dist;
        //                                 let frac = (1.0/30.0 * dist);
        //                                 if frac < 1.0 {
        //                                     imp *= 1.0 - frac;

        //                                     {
        //                                         interact_ct += 1;
        //                                         let svel_i = vel.get_mut(ent_i);
        //                                         if let Some(vel_i) = svel_i {
        //                                             vel_i.x *= 0.95;
        //                                             vel_i.x += (x_dif * imp) - (imp / 2.0);
        //                                             vel_i.y *= 0.95;
        //                                             vel_i.y += (y_dif * imp) - (imp / 2.0);
        //                                         }

        //                                         let svel_j = vel.get_mut(ent_j);
        //                                         if let Some(vel_j) = svel_j {
        //                                             vel_j.x *= 0.95;
        //                                             vel_j.x -= (x_dif * imp) - (imp / 2.0);
        //                                             vel_j.y *= 0.95;
        //                                             vel_j.y -= (y_dif * imp) - (imp / 2.0);
        //                                         }
        //                                     }
        //                                 }
        //                                 else {
        //                                     imp = 0.0;
        //                                 }
                                        
        //                                 //println!("Impulse dist: {}, frac: {}, imp: {}", &dist, &frac, &imp);
        //                             }
        //                         }


                        // }

                            // bucketize each object to two grid sizes
                            // let buc_i_x : i32 = (pos_ix / 100.0).round() as i32;
                            // let buc_a_i_y : i32 = (pos_iy / 100.0).round() as i32;
                            // let buc_a_j_x : i32 = (pos_jx / 100.0).round() as i32;
                            // let buc_a_j_y : i32 = (pos_jy / 100.0).round() as i32;


                        // }

        //                 if buc_ix == buc_jx && buc_iy == buc_jy {
        //                     //println!("Hit: i={} and j={}", &entid_i, &entid_j);
        //                 }

        //             }


        //         }
        //         ind_j += 1;
        //     }
        //     ind_i += 1;
        // }

        //println!("Physics obj count: {}", &blocks.len());

    }
}