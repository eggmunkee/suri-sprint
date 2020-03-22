use specs::{Component, DenseVecStorage, World, WorldExt};
//use specs::shred::{Dispatcher};
use ggez::nalgebra::{Point2,Vector2,distance};

#[derive(Debug)]
pub enum CollisionShape {
    Circle(f32),
    Square(f32)
}

#[derive(Debug)]
pub struct Collision {
    pub shape: CollisionShape,
    pub mass: f32,
    pub friction: f32,
}

impl Collision {
    #[allow(dead_code)]
    pub fn new() -> Collision {
        Collision {
            shape: CollisionShape::Circle(32.0),
            mass: 1.0,
            friction: 0.05,
        }
    }
    pub fn new_specs(m: f32, f: f32) -> Collision {
        Collision {
            shape: CollisionShape::Circle(32.0),
            mass: m,
            friction: f,
        }
    }
    pub fn new_circle(radius: f32) -> Collision {
        Collision {
            shape: CollisionShape::Circle(radius),
            mass: 1.0,
            friction: 0.05,
        }
    }
    pub fn new_square(radius: f32) -> Collision {
        Collision {
            shape: CollisionShape::Square(radius),
            mass: 1.0,
            friction: 0.05,
        }
    }
}

impl Component for Collision {
    type Storage = DenseVecStorage<Self>;
}

// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    world.register::<Collision>();
}

