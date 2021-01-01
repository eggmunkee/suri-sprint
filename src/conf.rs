use ggez::{ContextBuilder};
use ggez::conf::{WindowSetup,WindowMode,FullscreenType,NumSamples};
use serde::{Serialize,Deserialize,de::DeserializeOwned};
use std::{fs,fs::File};
use ron::de::from_reader;
use ron::ser::{to_string_pretty, PrettyConfig};


#[derive(Debug,Deserialize)]
pub struct ConfigData {
    pub window_setup: WindowSetup,
    pub window_mode: WindowMode,
    pub start_level: String,
    pub music_volume: f32,
    pub audio_volume: f32,
    pub gravity: f32,
}

pub fn get_ron_config<'a,T>(config_path: String) -> Option<T> 
    where T: DeserializeOwned
{
    let input_path = format!("{}/resources/{}.ron", env!("CARGO_MANIFEST_DIR"), &config_path);
    if let Ok(f) = File::open(&input_path) {
        let config: T = match from_reader(f) {
            Ok(x) => {
                return Some(x);
            },
            Err(e) => {
                return None;
            }
        };
    }

    None
}

pub fn save_ron_config<'a,T>(config_path: String, config: &T) -> bool
    where T: Serialize
{
    let input_path = format!("{}/resources/{}.ron", env!("CARGO_MANIFEST_DIR"), &config_path);

    //if let Ok(mut f) = File::create(&input_path) {
        if let Ok(source) = to_string_pretty(config, PrettyConfig::default()) {
            //let buf = &source;
            fs::write(&input_path, source);
        }
    //}
   
    true
    //None
}


pub fn get_game_config() -> Option<ConfigData> {

    let config_maybe = self::get_ron_config::<ConfigData>("conf".to_string());

    let return_config = match config_maybe {
        Some(config) => Some(config),
        _ => Some(ConfigData {
            window_setup: WindowSetup::default(),
            window_mode: WindowMode::default(),
            start_level: "start".to_string(),
            music_volume: 0.1,
            audio_volume: 0.1,
            gravity: 45.0,
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