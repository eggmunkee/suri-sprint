// external crates
use std::env;
use std::path;
use ggez;
use ggez::event;
//use ggez::filesystem;
use ggez::{GameResult};

// ================== ROOT MODULES ========================

mod conf;
// Core modules like physics, audio, input, events, game system, and world
mod core;
// Components available to entities
mod components;
// Builders for entity types
mod entities;
// Shared world data
mod resources;
// Systems which process world state updates
mod systems;
// Render methods for app
mod render;
// creates game state with world and dispatcher, handles event loop
//   Update, Draw, KeyDown KeyUp, etc.
//   Events are forwarded to specs dispatcher and render/input modules

use crate::conf::*;
use crate::core::{GameState};

// ======================== MAIN INIT APP ============================

// Do setup and start main event loop
pub fn main() -> GameResult {

    // Get config from file, get windows info
    let config : ConfigData = get_game_config().unwrap();    
    let win_title = config.window_setup.title.clone();
    //let win_setup = get_window_setup();
    let win_mode = config.window_mode.clone();

    // get ggez context build - builds window app
    let mut cb = ggez::ContextBuilder::new("suri_sprint", "ggez")
        .window_setup(config.window_setup)
        .window_mode(config.window_mode);

    // insert cargo manifest dir /resources into resources paths
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        //println!("Adding path {:?}", path);
        cb = cb.add_resource_path(path);
    }        
    // build
    let (ctx, event_loop) = &mut cb.build()?;

    // create app's state
    let game_state = &mut GameState::new(ctx, win_title, win_mode, config.music_volume, config.audio_volume, config.gravity, config.start_level)?;

    // Load start level
    //state.load_level(ctx, config.start_level, "".to_string());

    //filesystem::print_all(ctx);

    // run event loop
    println!("Running event loop...");
    let run_result = event::run(ctx, event_loop, game_state);

    println!("Quitting...");

    run_result
}
