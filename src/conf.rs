use ggez::{ContextBuilder};
use ggez::conf::{WindowSetup,WindowMode,FullscreenType,NumSamples};
use serde::Deserialize;
use std::{ fs::File};
use ron::de::from_reader;


#[derive(Debug,Deserialize)]
pub struct ConfigData {
    pub window_setup: WindowSetup,
    pub window_mode: WindowMode,
}

pub fn get_ron_config() -> Option<ConfigData> {
    let input_path = format!("{}/src/conf.ron", env!("CARGO_MANIFEST_DIR"));
    let f = File::open(&input_path).expect("Failed opening file");
    let config: ConfigData = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);

            std::process::exit(1);
        }
    };

    println!("Config data: {:?}", &config);

    Some(config)
}

// pub fn get_window_mode() -> WindowMode {
//     WindowMode {
//         width: 1000.0,
//         height: 800.0,
//         maximized: false,
//         fullscreen_type: FullscreenType::Windowed,
//         borderless: false,
//         min_width: 0.0,
//         max_width: 0.0,
//         min_height: 0.0,
//         max_height: 0.0,
//         resizable: true,
//     }
// }