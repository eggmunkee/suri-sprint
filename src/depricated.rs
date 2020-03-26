







// Check if two points are within a given radius using squared distances
//  If they are, it returns true and the actual distance
//  Otherwise, it returns false the radius value
pub fn radius_check_points(point: &Point2<f32>, check_point: &Point2<f32>, radius: f32) -> (bool, f32, Point2<f32>, Point2<f32>) {

    if distance_squared(point, check_point) > radius * radius {
        (false, radius, na::Point2::new(point.x, point.y), na::Point2::new(check_point.x, check_point.y))
    }
    else {
        let d = distance(point, check_point);
        (d < radius, d, na::Point2::new(point.x, point.y), na::Point2::new(check_point.x, check_point.y))
    }
}


pub fn radius_square_check_points(point: &Point2<f32>, check_point: &Point2<f32>, combined_radius: f32) -> (bool, f32, Point2<f32>, Point2<f32>) {

    let x_dif = point.x - check_point.x;
    let y_dif = point.y - check_point.y;
    let half_radius = combined_radius / 2.0;
    println!("=========== RAD SQUARE CHECK POINTS=================");
    // distance is min of either dimension given aligned squares
    // use only one axis for the return points
    let (x_dif_abs, y_dif_abs) = (x_dif.abs(), y_dif.abs());
    let (act_x_dif, act_y_dif) = (x_dif_abs - combined_radius, y_dif_abs - combined_radius);
    // pick farther away axis to just distance
    if act_x_dif > act_y_dif {
        let new_dist = act_x_dif;
        let new_pt1 = na::Point2::new(point.x, point.y);
        let mut xd = 0.0;
        if x_dif > 0.0 {
            xd = x_dif + combined_radius;
        }
        else {
            xd = x_dif - combined_radius;
        }
        let new_pt2 = na::Point2::new(point.x-xd, point.y);
        //let new_dist = dist; //distance(&new_pt1, &new_pt2);
        println!("radSqrChkPts - X orig: {:?} {:?}", &point, &check_point);
        println!("radSqrChkPts - X dist:{} {:?} {:?}", &new_dist, &new_pt1, &new_pt2);
        // return points vector only in this axis direction
        (new_dist < 0.0, new_dist, new_pt1, new_pt2)
    }
    else {
        let new_dist = act_y_dif;
        //println!("radSqrChkPts - Y orig: {:?} {:?}", &na::Point2::new(point.x, point.y), &na::Point2::new(check_point.x, check_point.y));
        //println!("radSqrChkPts - Y dist:{} {:?} {:?}", &dist, &na::Point2::new(point.x, point.y), &na::Point2::new(point.x, point.y-y_dif));
        let new_pt1 = na::Point2::new(point.x, point.y);
        let mut yd = 0.0;
        if y_dif > 0.0 {
            yd = y_dif + combined_radius;
        }
        else {
            yd = y_dif - combined_radius;
        }

        let new_pt2 = na::Point2::new(point.x, point.y-yd);
        //let new_dist = dist; // distance(&new_pt1, &new_pt2);
        println!("radSqrChkPts - X orig: {:?} {:?}", &point, &check_point);
        println!("radSqrChkPts - X dist:{} {:?} {:?}", &new_dist, &new_pt1, &new_pt2);
        // return points vector only in this axis direction
        (new_dist < 0.0, new_dist, new_pt1, new_pt2)
    }
    
}


pub fn rect_check_points(point: &Point2<f32>, check_point: &Point2<f32>, dim_1: &[f32;2], dim_2: &[f32;2]) -> (bool, f32, Point2<f32>, Point2<f32>) {

    let x_radius = dim_1[0] + dim_2[0];
    let y_radius = dim_1[1] + dim_2[1];
    let x_dif = point.x - check_point.x;
    let y_dif = point.y - check_point.y;
    // distance is min of either dimension given aligned squares
    // use only one axis for the return points
    let (x_dif_abs, y_dif_abs) = (x_dif.abs(), y_dif.abs());
    let (eff_x_dif, eff_y_dif) = (x_dif_abs - x_radius, y_dif_abs - y_radius);

    // pick farther away axis to just distance
    if eff_x_dif > eff_y_dif {
        let dist = eff_x_dif;
        //println!("radSqrChkPts - X orig: {:?} {:?}", &na::Point2::new(point.x, point.y), &na::Point2::new(check_point.x, check_point.y));
        //println!("radSqrChkPts - X dist:{} {:?} {:?}", &dist, &na::Point2::new(point.x, point.y), &na::Point2::new(point.x-x_dif, point.y));

        // return points vector only in this axis direction
        (dist < x_radius, dist, na::Point2::new(point.x, point.y), na::Point2::new(point.x-x_dif, point.y))
    }
    else {
        let dist = eff_y_dif;
        //println!("radSqrChkPts - Y orig: {:?} {:?}", &na::Point2::new(point.x, point.y), &na::Point2::new(check_point.x, check_point.y));
        //println!("radSqrChkPts - Y dist:{} {:?} {:?}", &dist, &na::Point2::new(point.x, point.y), &na::Point2::new(point.x, point.y-y_dif));

        // return points vector only in this axis direction
        (dist < y_radius, dist, na::Point2::new(point.x, point.y), na::Point2::new(point.x, point.y-y_dif))
    }
    
}

// fn pt_vector_check(_check_point: &Point2<f32>, _vector: &Vector2<f32>) -> bool {
//     true
// }

pub fn actors_push(pos_i: &na::Point2<f32>, pos_j: &na::Point2<f32>, 
    svel_i : Option<&mut na::Vector2<f32>>, svel_j : Option<&mut na::Vector2<f32>>,
    mass_i: f32, mass_j: f32, friction_i: f32, friction_j: f32, combined_obj_radius: f32) {
    //let impulse : f32 = 220.0;
    let touch_dist : f32 = combined_obj_radius;
    //let friction_ratio : f32 = 0.90;
    //let pt = na::Point2::new(pos_i.x,pos_i.y);
    let (check, dist, pos_i, pos_j) = self::radius_check_points(&pos_i, &pos_j, touch_dist);
    if check {
        let mut imp = 0.0;
        if dist > 0.1 {
            let x_dif = (pos_j.x - pos_i.x) / dist;
            let y_dif = (pos_j.y - pos_i.y)  / dist;
            let overlap_len = dist;
            //imp = overlap_;
            //let overlap_ratio = overlap_len / touch_dist;
            imp = overlap_len * 2.0; //overlap_ratio * imp;
            let mut x_imp = imp * x_dif;
            let mut y_imp = imp * y_dif;
            //let inv_len = (touch_dist - dist).max(0.0).min(51.0);
            //let frac = 1.0 / (30.0 * dist); //inv_len / 51.0;
            if imp > 0.0 {
                //imp *= 1.0 - frac;

                // Check velocity status of two entities
                let no_i = match svel_i {
                    Some(_) => true,
                    _ => false
                };
                let no_j = match svel_j {
                    Some(_) => true,
                    _ => false
                };
                // Default to full strength impulses from mass
                let mut mass_frac_i = 0.5;
                let mut mass_frac_j = 0.5;
                // Double impulse per object if one doesn't have velocity
                if no_i || no_j {
                    x_imp *= 2.0;
                    y_imp *= 2.0;
                }

                mass_frac_i = mass_j / (mass_i + mass_j);
                mass_frac_j = mass_i / (mass_i + mass_j);

                let total_friction = (friction_i + friction_j);

                // if mass_i > 0.0 && mass_j > 0.0 {
                //     // having masses for both items, calc fraction of mass
                //     // for I and J, to multiply impulse by
                //     // frac is the magnitude of mass applied by the other object
                //     // i's mass frac is j's mass - J's mass applies in the force on I
                //     // j's mass frac is i's mass - I's mass applies in the force on J
                // }

                // If I has velocity to update, apply impulse
                if let Some(vel_i) = svel_i {
                    vel_i.x *= 1.0 - total_friction;
                    vel_i.y *= 1.0 - total_friction;

                    // apply impulse for
                    //if vel_i.x <
                    vel_i.x += x_imp * mass_frac_i;
                    vel_i.y += y_imp * mass_frac_i;
                    // vel_i.x = 0.0;
                    // vel_i.y = 0.0;
                }

                // If J has velocity to update, apply impulse
                if let Some(vel_j) = svel_j {
                    vel_j.x *= 1.0 - total_friction;
                    vel_j.y *= 1.0 - total_friction;

                    vel_j.x -= x_imp * mass_frac_j;
                    vel_j.y -= y_imp * mass_frac_j;
                    // vel_j.x = 0.0;
                    // vel_j.y = 0.0;
                }
                //}
            }
            // else {
            //     imp = 0.0;
            // }
            
            //println!("Impulse dist: {}, frac: {}, imp: {}", &dist, &frac, &imp);
        }
    }
}


// Actors push at each others boundaries based on two square shapes
pub fn actors_push_squares(pos_i: &na::Point2<f32>, pos_j: &na::Point2<f32>, 
    svel_i : Option<&mut na::Vector2<f32>>, svel_j : Option<&mut na::Vector2<f32>>,
    mass_i: f32, mass_j: f32, friction_i: f32, friction_j: f32, combined_obj_radius: f32) {
    //let impulse : f32 = 220.0;
    let touch_dist : f32 = combined_obj_radius;
    //let friction_ratio : f32 = 0.90;
    //let pt = na::Point2::new(pos_i.x,pos_i.y);
    //let (check, dist, pos_i, pos_j) = self::rect_check_points(&pos_i, &pos_j, &[touch_dist/2.0,touch_dist/2.0], &[touch_dist/2.0,touch_dist/2.0]);
    let (check, dist, pos_i, pos_j) = self::radius_square_check_points(&pos_i, &pos_j, touch_dist);
    if check {
        let mut imp = 0.0;
        let overlap_len = -dist;
        println!("check, dist {}, p1 {:?}, p2 {:?}, m1 = {}, m2 = {}", &dist, &pos_i, &pos_j, &mass_i, &mass_j);
        if overlap_len > 0.0 {
            let x_dif = (pos_i.x - pos_j.x) / overlap_len;
            let y_dif = (pos_i.y - pos_j.y)  / overlap_len;
            println!("overlap len: {}, touch dist: {}, dist: {}, xdif: {}, ydif: {}", &overlap_len, &touch_dist, &dist
                ,&x_dif, &y_dif);
            //imp = overlap_;
            let overlap_ratio = overlap_len / (touch_dist / 2.0);
            imp = overlap_len * overlap_ratio * 3.0; //overlap_ratio * imp;
            //imp = imp * 2.0;
            let x_imp = imp * x_dif;
            let y_imp = imp * y_dif;
            //let inv_len = (touch_dist - dist).max(0.0).min(51.0);
            //let frac = 1.0 / (30.0 * dist); //inv_len / 51.0;
            if imp > 0.0 {
                //imp *= 1.0 - frac;

                // Check velocity status of two entities
                let no_i = match svel_i {
                    Some(_) => true,
                    _ => false
                };
                let no_j = match svel_j {
                    Some(_) => true,
                    _ => false
                };
                // Default to full strength impulses from mass
                let mut mass_frac_i = 0.5;
                let mut mass_frac_j = 0.5;
                // Double impulse per object if one doesn't have velocity
                if no_i || no_j {
                    mass_frac_i *= 2.0;
                    mass_frac_j *= 2.0;
                }

                if !no_i && !no_j {
                    mass_frac_i = mass_j / (mass_i + mass_j);
                    mass_frac_j = mass_i / (mass_i + mass_j);
                }

                let total_friction = 0.0;// (friction_i + friction_j);

                // if mass_i > 0.0 && mass_j > 0.0 {
                //     // having masses for both items, calc fraction of mass
                //     // for I and J, to multiply impulse by
                //     // frac is the magnitude of mass applied by the other object
                //     // i's mass frac is j's mass - J's mass applies in the force on I
                //     // j's mass frac is i's mass - I's mass applies in the force on J
                // }

                let i_x_imp_final = x_imp * mass_frac_i;
                let i_y_imp_final = y_imp * mass_frac_i;
                let j_x_imp_final = x_imp * mass_frac_j;
                let j_y_imp_final = y_imp * mass_frac_j;

                if !no_i || !no_j {
                    println!("Impulse: {}, friction: {}, mass_frac_i: {}, mfj: {}, ({},{}), ({},{})", 
                    &imp, &total_friction, &mass_frac_i, &mass_frac_j,
                    &i_x_imp_final, &i_y_imp_final, &j_x_imp_final, &j_y_imp_final);
                }
                

                // If I has velocity to update, apply impulse
                if let Some(vel_i) = svel_i {
                    vel_i.x *= 1.0 - total_friction;
                    vel_i.y *= 1.0 - total_friction;

                    // apply impulse for
                    //if vel_i.x <
                    vel_i.x += i_x_imp_final;
                    vel_i.y += i_y_imp_final;
                    println!("Final i vel: {}, {}", &vel_i.x, &vel_i.y);
                    // vel_i.x = 0.0;
                    // vel_i.y = 0.0;
                }

                // If J has velocity to update, apply impulse
                if let Some(vel_j) = svel_j {
                    vel_j.x *= 1.0 - total_friction;
                    vel_j.y *= 1.0 - total_friction;

                    vel_j.x -= j_x_imp_final;
                    vel_j.y -= j_y_imp_final;
                    println!("Final j vel: {}, {}", &vel_j.x, &vel_j.y);
                    // vel_j.x = 0.0;
                    // vel_j.y = 0.0;
                }
                //}
            }
            // else {
            //     imp = 0.0;
            // }
            
            //println!("Impulse dist: {}, frac: {}, imp: {}", &dist, &frac, &imp);
        }
    }
}


