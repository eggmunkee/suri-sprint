

use specs::{Component, DenseVecStorage, World, WorldExt};
use specs_derive::*;
use wrapped2d::b2;


use crate::components::sprite::{SpriteComponent};
use crate::components::collision::{Collision};
use crate::physics;
use crate::physics::{PhysicsWorld};

#[derive(Debug,Component)]
#[storage(DenseVecStorage)]
pub struct MeowComponent {
    pub meow_time: f32,
    pub meow_radius: f32,
}

impl MeowComponent {
    pub fn new() -> MeowComponent {

        let mut meow = MeowComponent {
            meow_time: 0.0,
            meow_radius: 20.0,
        };
        // calculate radius from time to start
        meow.meow_radius = meow.calc_radius();

        meow
    }

    fn calc_alpha(&self) -> f32 {
        self.meow_time
    }

    fn calc_radius(&self) -> f32 {
        let size_ratio = 0.5 - self.meow_time;

        250.0 * size_ratio.max(0.1)
    }

    pub fn update(&mut self, delta_time: f32, collision: &mut Collision, sprite: &mut SpriteComponent, physics_world: &mut PhysicsWorld) {
        self.meow_time -= delta_time;

        let new_radius = self.calc_radius();
        let new_physics_radius = physics::create_size(new_radius);
        //println!("Meow radius: {:?}", &new_radius);

        sprite.scale.x = new_radius / 20.0;
        sprite.scale.y = new_radius / 20.0;

        sprite.alpha = self.calc_alpha();

        if let Some(body_handle) = collision.body_handle {
            let body = physics_world.body(body_handle);
            let mut first_fixture : Option<b2::FixtureHandle> = None;

            for (fixture, meta) in body.fixtures() {
                //println!("Meow collision has body handle with a fixture");
                first_fixture = Some(fixture);
                break;
            }

            // Operate on the first fixture/shape
            if let Some(fixture_handle) = first_fixture {
                //println!("Meow collision has body handle with a fixture PART 2");
                // Get mutable ref to fixture
                let mut fixture = body.fixture_mut(fixture_handle);

                // Get mutable unknown shape enum
                let shape : &mut b2::UnknownShape = &mut *fixture.shape_mut();
                // if matches circle - update radius of circle
                match shape {
                    b2::UnknownShape::Circle(ref mut circle) => {
                        //println!("Setting radius of meow circle shape to {}", &new_physics_radius);
                        circle.set_radius(new_physics_radius);
                    },
                    b2::UnknownShape::Polygon(_polygon) => {
                        //circle.set_radius(new_radius);
                        //println!("Meow collision fixture had a polygon");
                    },
                    _ => {
                        //println!("Meow collision fixture had another shape");
                    }
                }
            }


        }
        
    }
}


// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    world.register::<MeowComponent>();
}