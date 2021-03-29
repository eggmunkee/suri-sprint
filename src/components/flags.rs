
use specs::{ World };

pub mod event_flags;
pub mod render_flags;

pub use self::event_flags::*;
pub use self::render_flags::*;


// Register all possible components for world
pub fn register_components(world: &mut World) {
    // register components
    event_flags::register_components(world);
    render_flags::register_components(world);
}