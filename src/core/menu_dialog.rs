


#[derive(Clone,Debug,PartialEq)]
#[allow(dead_code)]
pub enum DialogType {
    LevelEntry, // message only - level entry style
    DialogInfo, // message only - dialog / thought bubble style
    DialogChoices, // message + choice of responses - dialog
    WorldDialog, // custom styled dialog representing a world object
    WorldChoices, // custom styled dialog representing a world object + choice of responses - dialog
}

#[derive(Clone,Debug,PartialEq)]
pub struct DialogChoice {
    pub message: String,
    pub key: String,
}


#[derive(Clone,Debug,PartialEq)]
pub enum MenuItem {
    Header(String),
    ToggleItem{ name: String, key: String, value: bool },
    RangeItem{ name: String, key: String, min: f32, max: f32, incr: f32, value: f32 },
    ButtonItem{ name: String, key: String},
}


#[derive(Clone,Debug,PartialEq)]
pub struct Menu {
    pub items : Vec<MenuItem>,
    pub selected_index : i32,
}

impl Menu {
    pub fn new(header_name: String) -> Self {
        Menu {
            items: vec![MenuItem::Header(header_name)],
            selected_index: -1,
        }
    }
}