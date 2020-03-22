// external crates
use std::env;
use std::path;
use ggez;
use ggez::event;
use ggez::filesystem;
use ggez::{GameResult};

// ================== ROOT MODULES ========================

mod conf;
// Builders for entity types
mod entities;
// Simple physics module
mod physics;
// Components available to entities
mod components;
// Shared world data
mod resources;
// Systems which process world state updates
mod systems;
// Sets up the world
mod world;
// Render methods for app
mod render;
// Input key mapping from codes to actions, handle actions
mod input;
// creates game state with world and dispatcher, handles event loop
//   Update, Draw, KeyDown KeyUp, etc.
//   Events are forwarded to specs dispatcher and render/input modules
mod game_state;

use crate::conf::*;

// ======================== MAIN INIT APP ============================

// Do setup and start main event loop
pub fn main() -> GameResult {

    let win_setup = get_window_setup();
    let win_mode = get_window_mode();

    // get ggez context build - builds window app
    let mut cb = ggez::ContextBuilder::new("super_simple", "ggez")
        .window_setup(win_setup)
        .window_mode(win_mode);

    // insert cargo manifest dir /resources into resources paths
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {:?}", path);
        cb = cb.add_resource_path(path);
    }        
    // build
    let (ctx, event_loop) = &mut cb.build()?;

    //ggez::graphics::set_window_icon(ctx, Some("/icon.png"))?;
    // create app's state
    let state = &mut crate::game_state::GameState::new(ctx, win_mode)?;

    
    filesystem::print_all(ctx);
    // run event loop
    event::run(ctx, event_loop, state)
}
