use ggez::{ContextBuilder};
use ggez::conf::{WindowSetup,WindowMode,FullscreenType,NumSamples};
use serde::{Deserialize,de::DeserializeOwned};
use std::{ fs::File};
use ron::de::from_reader;


#[derive(Debug,Deserialize)]
pub struct ConfigData {
    pub window_setup: WindowSetup,
    pub window_mode: WindowMode,
}

pub fn get_ron_config<'de,T>(config_path: String) -> Option<T> 
    where T: DeserializeOwned
{
    let input_path = format!("{}/src/{}.ron", env!("CARGO_MANIFEST_DIR"), &config_path);
    if let Ok(f) = File::open(&input_path) {
        let config: T = match from_reader(f) {
            Ok(x) => {
                return Some(x);
            },
            Err(e) => {
                return None;
            }
        };
    } //.expect("Failed opening file");

    //println!("Config data: {:?}", &config);

    None
}


pub fn get_game_config() -> Option<ConfigData> {

    let config_maybe = self::get_ron_config::<ConfigData>("conf".to_string());

    let return_config = match config_maybe {
        Some(config) => Some(config),
        _ => Some(ConfigData {
            window_setup: WindowSetup::default(),
            window_mode: WindowMode::default(),
        })
    };

    println!("Config data: {:?}", &return_config);

    return_config
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