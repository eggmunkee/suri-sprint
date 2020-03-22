use ggez::{ContextBuilder};
use ggez::conf::{WindowSetup,WindowMode,FullscreenType,NumSamples};

pub fn get_window_setup() -> WindowSetup {
    WindowSetup {
        title: "GGEZ ~~~ DEMO".to_owned(),
        samples: NumSamples::Eight,
        vsync: true,
        icon: "/icon.png".to_owned(), // set OS window icon
        srgb: true,
    }
}

pub fn get_window_mode() -> WindowMode {
    WindowMode {
        width: 1000.0,
        height: 800.0,
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        max_width: 0.0,
        min_height: 0.0,
        max_height: 0.0,
        resizable: true,
    }
}