
// Graphics engine, math, graphics context, window mode
use ggez;
use ggez::graphics;
use ggez::graphics::{Color};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use ggez::conf::{WindowMode};
// SPECS game world - world and system running support
use specs::{World, WorldExt};
// Physics class support
use wrapped2d::b2;

// =====================================
// GAME STATE internal support
// Audio system
use crate::core::audio::{Audio};
// Create Specs world support
use crate::core::world::{create_world};
// Physics support
use crate::core::physics;
use crate::core::physics::{PhysicsWorld,ContactFilterConfig};
// Game InputKey support
use crate::core::input::{InputKey,InputMap};
// Core Game System - runs other systems and physics simulation
use crate::core::system::{CoreSystem};
// Core Dialog/Menu types
use crate::core::menu_dialog::{DialogType,DialogChoice,Menu,MenuItem};
// Global SPECS Resource classes
use crate::resources::{GameStateResource,ConnectionResource,Camera,GameLog};
// Level definition support
use crate::entities::level_builder::{LevelConfig,LevelBounds,LevelType};
use crate::entities::geometry::{LevelGridData};
// Render class support
use crate::render;



#[derive(Clone,Debug,PartialEq)]
pub enum RunningState {
    Playing, // normal playing state - world & physics running
    Dialog { // dialog being shown state - world paused
        msg: String, 
        dialog_type: DialogType, 
        // choice data
        choices: Option<Vec<DialogChoice>>,
        selected_choice: i32,
        custom_bg: Option<String>, // image path for world dialog/choices
        text_color: Option<Color>,
    },  
}


#[derive(Debug,Clone,PartialEq)]
#[allow(dead_code)]
pub enum GameStateUpdateMode {
    PlayUpdate,
    PausedUpdate,
    MenuUpdate,
    TerminalUpdate,
    NoUpdate
}


impl RunningState {
    pub fn level_dialog(msg: String) -> RunningState {
        RunningState::Dialog{ msg, dialog_type: DialogType::LevelEntry, choices: None, selected_choice: -1,
            custom_bg: None, text_color: Some(Color::new(0.9, 0.9, 0.9, 1.0)) }
    }

    #[allow(dead_code)]
    pub fn game_dialog(msg: String, choices: Option<Vec<DialogChoice>>) -> RunningState {
        let dialog_type = match &choices {
            Some(_) => DialogType::DialogChoices,
            None => DialogType::DialogInfo
        };
        RunningState::Dialog{ msg, dialog_type: dialog_type, choices: choices, selected_choice: -1,
            custom_bg: None, text_color: None }
    }

    pub fn world_dialog(msg: String, choices: Option<Vec<DialogChoice>>, custom_bg: String) -> RunningState {
        let dialog_type = match &choices {
            Some(_) => DialogType::WorldChoices,
            None => DialogType::WorldDialog
        };
        RunningState::Dialog{ msg, dialog_type: dialog_type, choices: choices, selected_choice: -1, 
            custom_bg: Some(custom_bg), text_color: None }
    }

    pub fn get_bg_image(&self) -> String {
        match self {
            RunningState::Playing => "".to_string(),
            RunningState::Dialog { dialog_type, ref custom_bg, .. } => {
                match dialog_type {
                    DialogType::LevelEntry => "/images/purple-dialog-bg.png".to_string(),
                    DialogType::DialogInfo | DialogType::DialogChoices => "/images/cloud-dialog-bordered.png".to_string(),
                    DialogType::WorldDialog | DialogType::WorldChoices => match custom_bg {
                        Some(bg) => {
                            bg.clone()
                        },
                        None => {
                            "/images/purple-dialog-wide-bg.png".to_string()
                        }
                    }
                }
            }
        }
        
    }
}

#[derive(Copy,Clone,Debug)]
pub enum State {
    Running,
    Paused,    
}
#[derive(Copy,Clone,Debug,PartialEq)]
pub enum GameMode {
    Play,
    Edit,
}

const WARP_TIME_LIMIT : f32 = 1.5f32;
const WARP_TIME_SCALE : f32  = 0.035;  //0.025f32;

// Main game state struct
pub struct GameState {
    pub window_title: String,
    pub current_state: State,
    pub running_state: RunningState,
    pub menu_stack: Vec<Menu>,
    pub mode: GameMode,
    pub start_level_name: String,
    pub current_level_name: String,
    pub current_entry_name: String,
    pub level: LevelConfig,
    pub window_w: f32,
    pub window_h: f32,
    //pub dispatcher: Dispatcher<'a,'a>,
    pub world: World,
    pub font: graphics::Font,
    pub phys_world: PhysicsWorld,
    pub input_map: InputMap,
    pub display_scale: f32,
    pub gravity_scale: f32,
    pub gravity_x: f32,
    //pub image_lookup: HashMap<String,usize>,
    //pub images: Vec<Image>
    //pub paused_text: graphics::Text,
    pub cursor_offset: na::Point2::<f32>,
    pub game_frame_count: u32,
    pub ghost_player_contact: bool,
    pub play_time_scale: f32,

    // Current view offset
    pub current_offset: na::Point2::<f32>,
    pub snap_view: bool,
    // what collision item was clicked
    pub click_info: Vec::<(b2::BodyHandle,b2::FixtureHandle)>,
    // level transition
    pub level_warping: bool,
    pub level_warp_timer: f32,
    pub warp_level_name: String,
    pub warp_level_entry_name: String,
    // paused anim
    pub paused_anim: f32,
    // game display zoom animation
    pub ui_game_display_zoom: f32,
    // terminal state
    pub terminal_open: bool,
    // audio
    pub audio: Audio,

    // debug flags
    pub debug_logic_frames: i32,





    // test data
    pub level_geometry: LevelGridData,
}

impl GameState {
    pub fn new(ctx: &mut Context, title: String, window_mode: WindowMode,
        music_volume: f32, sfx_volume: f32, gravity: f32, start_level: String) -> GameResult<GameState> {

        println!("* Creating GameState...");

        let ghost_player_contact = true;
        // Create physics world to place in game state resource
        println!("  o Physics init...");
        let mut physics_world = physics::create_physics_world((gravity, 0.0));
        // FIX CONTACT FILTER ERROR - TOGGLEABLE PLATFORM BUG - still impassable after UPdate body objstruction to false
        // let contact_f : ContactFilterConfig = ContactFilterConfig::new(ghost_player_contact);
        // physics_world.set_contact_filter(Box::from(contact_f));

        // Create game state related to window size/mode
        let (win_w, win_h) = ggez::graphics::drawable_size(ctx);
        let game_state_resource = GameStateResource {
            window_w: win_w, window_h: win_h, window_mode: window_mode, display_offset: (0.0, 0.0),
            delta_seconds: 0.15, level_bounds: LevelBounds::new(-500.0, -500.0, 3000.0, 3000.0),
            level_world_seconds: 0.0, game_run_seconds: 0.0, player_target_loc: (500.0, 500.0),
            player_count: 0, player_1_char_num: -1, level_type: LevelType::default(), points: 0,
            level_frame_num: 0,
        };

        // get window
        println!("  o Window init...");
        let window = ggez::graphics::window(ctx);
        window.set_position((1100.0,50.0).into());

        // hide OS cursor within display
        ggez::input::mouse::set_cursor_hidden(ctx, true);

        let font = graphics::Font::new(ctx, "/FreeMonoBold.ttf")?;
        //let text = graphics::Text::new(("PAUSED", font, 52.0));

        println!("  o World init...");
        let ecs_world = create_world(ctx, game_state_resource, &mut physics_world);

        // Load additional resources from submodules like renderer
        println!("    o Load system resources (images,etc.)...");
        Self::load_submodule_resources(&ecs_world, ctx);

        println!("  o Audio init...");
        let mut audio_engine = Audio::new(ctx);
        audio_engine.set_music_volume(music_volume);
        audio_engine.set_sfx_volume(sfx_volume);

        // Create main state instance with dispatcher and world
        let mut s = GameState { 
            window_title: title,
            current_state: State::Running,
            //running_state: RunningState::Playing, 
            running_state: RunningState::Playing, //RunningState::level_dialog("Loading game".to_string()),
            menu_stack: vec![], // Menu::new("Main Menu".to_string()), Menu::new("Audio Menu".to_string()), Menu::new("Sub-Audio Menu".to_string())
            mode: GameMode::Play,
            start_level_name: start_level,
            current_level_name: "".to_string(),
            current_entry_name: "".to_string(),
            level: LevelConfig::new(),
            window_w: win_w,
            window_h: win_h,
            display_scale: 1.0,
            //dispatcher: create_dispatcher(), 
            world: ecs_world,
            font: font,
            phys_world: physics_world,
            input_map: InputMap::load_or_init_inputmap(),
            gravity_scale: gravity,
            gravity_x: 0.0,
            ghost_player_contact: ghost_player_contact,
            play_time_scale: 1.0,
            // image_lookup: HashMap::<String,usize>::new(),
            // images: Vec::<Image>::new(),
            //paused_text: text,
            game_frame_count: 0,
            cursor_offset: na::Point2::new(10.0,10.0),
            current_offset: na::Point2::new(0.0,0.0),
            snap_view: true,
            click_info: vec![],
            level_warping: false,
            level_warp_timer: 0.0,
            warp_level_name: "".to_string(),
            warp_level_entry_name: "".to_string(),
            paused_anim: 0.0,
            ui_game_display_zoom: 1.0,
            terminal_open: false,

            audio: audio_engine,
            debug_logic_frames: 0,

            // Test data
            level_geometry: LevelGridData::new()
        };

        s.update_contact_filter();

        s.empty_level(ctx);

        Ok(s)
    }

    pub fn load_submodule_resources(world: &World, ctx: &mut Context) {

        // Add default renderer resources
        render::Renderer::add_resources(world, ctx);

    }

    pub fn set_gravity(&mut self, gravity: (f32, f32)) {
        //self.gravity_scale = gravity;
        physics::update_world_gravity(&mut self.phys_world, gravity);
    }

    pub fn play_music(&mut self, ctx: &mut Context) {
        self.audio.set_dimmed(false);
        self.audio.play_music(ctx, "".to_string());
    }

    pub fn dim_music(&mut self, _ctx: &mut Context) {
        //self.audio.stop_music(ctx);
        self.audio.set_dimmed(true);
    }

    #[allow(dead_code)]
    pub fn pause(&mut self) {
        let curr_st = self.current_state;
        match curr_st {
            State::Running => { 
                self.current_state = State::Paused;
                self.paused_anim = 0.0;
                self.audio.pause_music();
            }
            _ => {}
        }        
    }
    #[allow(dead_code)]
    pub fn play(&mut self) {
        let curr_st = self.current_state;
        match curr_st {
            State::Paused => { 
                self.current_state = State::Running; 
                self.audio.resume_music();
            }
            _ => {}
        }        
    }

    pub fn reset_level_time(&mut self) {
        // get game resource to set delta
        let world = &mut self.world;
        let mut game_res = world.fetch_mut::<GameStateResource>();
        game_res.level_world_seconds = 0.0;
        game_res.level_frame_num = 0;

        self.game_frame_count = 0;
    }

    pub fn set_frame_time(&mut self, time_delta: f32, advance_level_world: bool) {
        // get game resource to set delta
        let world = &mut self.world;
        let mut game_res = world.fetch_mut::<GameStateResource>();
        game_res.delta_seconds = time_delta;
        if advance_level_world {
            game_res.level_world_seconds += time_delta;
            game_res.level_frame_num += 1;
        }        
    }

    pub fn update_run_time(&mut self, time_delta: f32) {
        // get game resource to set delta
        let world = &mut self.world;
        let mut game_res = world.fetch_mut::<GameStateResource>();
        game_res.game_run_seconds += time_delta;
    }
  
    pub fn set_running_state(&mut self, ctx: &mut Context, new_state: RunningState) {
        match new_state {
            RunningState::Playing => {
                self.play_music(ctx);
            },
            RunningState::Dialog{..} => {
                self.dim_music(ctx);
            }
        }

        self.running_state = new_state;
    }

    // #[allow(dead_code)]
    // pub fn run_dialog_update(&mut self, _ctx: &mut Context, _time_delta: f32) {
        
    //     {
    //         let world = &mut self.world;
    //         let mut input_sys = InputSystem::new();
    //         input_sys.run_now(&world);
    //     }
    // }

    pub fn process_time_delta(&mut self, ctx: &mut Context, time_delta: f32, advance_world: bool) -> f32 {
        let mut time_delta = time_delta;
        if advance_world {
            if self.level_warping {
                // Advance level warping timer
                self.level_warp_timer += time_delta;
                // Slow time by warp multiplier
                time_delta *= WARP_TIME_SCALE;

                // Detetct warp time finished - load new level
                if self.level_warp_timer > WARP_TIME_LIMIT {
                    let level_name = self.warp_level_name.clone();
                    let entry_name = self.warp_level_entry_name.clone();
                    self.load_level(ctx, level_name, entry_name);
                    self.level_warp_timer = 0.0;
                    self.level_warping = false;
                    self.ui_game_display_zoom = 0.0;
                }
            }
            else {
                // Regular play speed
                time_delta *= self.play_time_scale; //1.0;// 0.3;
            }
        }
        else {
            // Non-play state time speed
            time_delta *= 0.5;
        }
        time_delta
    }

    // pub fn run_update_step(&mut self, ctx: &mut Context, time_delta: f32) {

    //     CoreSystem::run_update_step(self, ctx, time_delta);
    // }

    pub fn clear_world(&mut self) {
        println!("Clearing world...");
        // Clear world of entity and component data
        self.world.delete_all();

        // Clear Connection logic resources
        let mut conn_res = self.world.fetch_mut::<ConnectionResource>();
        conn_res.clear();        
    }

    pub fn clear_physics(&mut self) {
        println!("Clearing physics world...");
        // Drop and replace the physics world
        //self.phys_world = physics::create_physics_world(self.gravity_scale);
        self.phys_world = physics::create_physics_world((self.gravity_x, self.gravity_scale));
        
        self.update_contact_filter();
    }

    pub fn update_contact_filter(&mut self) {
        // FIX CONTACT FILTER ERROR - TOGGLEABLE PLATFORM BUG - still impassable after UPdate body objstruction to false
        let contact_f : ContactFilterConfig = ContactFilterConfig::new(self.ghost_player_contact);
        self.phys_world.set_contact_filter(Box::from(contact_f));
    }

    pub fn set_level_bounds(&mut self) {
        let bounds = &self.level.bounds;
        // get game resource to set delta
        let world = &mut self.world;
        let mut game_res = world.fetch_mut::<GameStateResource>();
        game_res.level_bounds = bounds.clone();

        println!("Level bounds: {},{} through {},{}", &game_res.level_bounds.min_x,
            &game_res.level_bounds.min_y, &game_res.level_bounds.max_x, &game_res.level_bounds.max_y);
    }

    pub fn set_level_type(&mut self) {
        let mut lvl_type = LevelType::default();
        if let Some(level_type) = &self.level.level_type {
            lvl_type = level_type.clone();
        }
        let world = &mut self.world;
        let mut game_res = world.fetch_mut::<GameStateResource>();
        game_res.level_type = lvl_type.clone();
        drop(game_res);
        
        match &lvl_type {
            LevelType::Platformer => {
                self.set_gravity((self.gravity_x, self.gravity_scale));
            },
            LevelType::Overhead | LevelType::Space => {
                self.set_gravity((0.0, 0.0));
            }
        };
    }

    pub fn start_warp(&mut self, level_name: String, entry_name: String) {
        //self.running_state = RunningState::Dialog(format!("Level {}, entry {}", &level_name, &entry_name));

        self.warp_level_name = level_name;
        self.warp_level_entry_name = entry_name.clone();
        self.level_warping = true;
        self.level_warp_timer = 0.0;

        if !entry_name.is_empty() {
            self.level_warp_timer = WARP_TIME_LIMIT * 0.5;
        }
    }

    #[allow(dead_code)]
    pub fn save_level(&self, save_name: String) {
        // TEST CODE TO SAVE LEVEL CONFIG
        let mut save_path = String::from("levels/");
        save_path.push_str(&save_name);
        crate::conf::save_ron_config(save_path, &self.level);
    }

    pub fn actual_load_empty_level(&mut self, _ctx: &mut Context) {
        // load level from file
        self.level = LevelConfig {
            name: "Empty".to_string(),
            level_type: Some(LevelType::Platformer),
            bounds: LevelBounds {
                min_x: 0.0, min_y: 0.0, max_x: 1000.0, max_y: 800.0,
                solid_sides: [false, false, false, false],
            },            
            items: vec![],
            no_game_ui: Some(true),
            soundtrack: "".to_string(),            
            build_index: 0, built_player: false,
        };
        //self.level = LevelConfig::load_level(&level_name);

        // set game state level bounds
        self.set_level_bounds();
        // get game state level type
        self.set_level_type();

        self.reset_level_time();
    }

    pub fn actual_load_level(&mut self, ctx: &mut Context, level_name: String, entry_name: String) {
        // load level from file
        self.level = LevelConfig::load_level(&level_name);

        if self.level.soundtrack != "" {
            self.audio.play_music(ctx, format!("/audio/{}", &self.level.soundtrack));
        }

        // set game state level bounds
        self.set_level_bounds();
        // get game state level type
        self.set_level_type();

        // load level into world        
        let mut world = &mut self.world;
        // Get mut ref to new physics world
        let mut physics_world = &mut self.phys_world;
        &self.level.build_level_content(&mut world, ctx, &mut physics_world, entry_name);

        self.reset_level_time();
    }

    pub fn empty_level(&mut self, ctx: &mut Context) {
        self.current_level_name = String::new();
        self.current_entry_name = String::new();

        self.clear_world();
        self.clear_physics();
        {
            let mut game_state = self.world.fetch_mut::<GameStateResource>();
            game_state.player_count = 0;
            game_state.player_1_char_num = 1;
        }

        self.actual_load_empty_level(ctx);

        let mut log = self.world.fetch_mut::<GameLog>();
        log.add_entry(true, "Empty Level".to_string(), None,
            self.world.fetch_mut::<GameStateResource>().game_run_seconds);

        self.current_state = State::Running;

        Camera::set_snap_view(self, true);
    }

    pub fn load_level(&mut self, ctx: &mut Context, level_name: String, entry_name: String) {
        self.current_level_name = level_name.clone();
        self.current_entry_name = entry_name.clone();
        {
            //let mut camera = self.world.fetch_mut::<Camera>();
            //camera.snap_view = true;

        }

        //self.stop_music(ctx);

        self.clear_world();
        self.clear_physics();
        {
            let mut game_state = self.world.fetch_mut::<GameStateResource>();
            game_state.player_count = 0;
            game_state.player_1_char_num = 1;
        }

        self.actual_load_level(ctx, level_name, entry_name);

        CoreSystem::run_logic_update(self);

        if self.current_level_name == "loading" {
            self.running_state = RunningState::Playing;
            self.open_menu();
        }
        else {
            if self.current_entry_name.is_empty() {
                self.running_state = RunningState::level_dialog(format!("Suri enters {}.\r\n\r\n\r\n\r\nPress Meow to Continue...", &self.level.name));
                self.audio.set_dimmed(true);
            }
            else {
                self.running_state = RunningState::Playing;

                let mut log = self.world.fetch_mut::<GameLog>();
                log.add_entry(true, format!("Entered {}", &self.level.name), None,
                    self.world.fetch_mut::<GameStateResource>().game_run_seconds);
            }
        }

        self.current_state = State::Running;

        Camera::set_snap_view(self, true);
    }

    pub fn game_frame_completed(&mut self) {
        self.game_frame_count = self.game_frame_count + 1;
    }

    pub fn in_menu_system(&self) -> bool {
        self.menu_stack.len() > 0 || self.ui_game_display_zoom < 1.0
    }

    pub fn game_key_down(&mut self, _ctx: &mut Context, _key: &InputKey) {
        
    }

    pub fn game_key_up(&mut self, _ctx: &mut Context, key: &InputKey) {
        match &key {
            InputKey::Exit => {

            },
            InputKey::Pause => {

            },
            InputKey::EditMode => {
                if self.mode == GameMode::Play {
                    self.mode = GameMode::Edit;
                }
                else {
                    self.mode = GameMode::Play;
                }
            },
            _ => {}
        }
    }


    fn get_update_mode(&self) -> GameStateUpdateMode {
        let term_open = self.terminal_open;
        let menus_open = self.menu_stack.len() > 0 || self.ui_game_display_zoom < 1.0;
        let is_game_running = match &self.current_state { State::Running => true, State::Paused => false };
        
        // Select mode by priority - terminal, then menus, then gameplay vs paused
        match (term_open, menus_open, is_game_running) {
            (true, _, _) => GameStateUpdateMode::TerminalUpdate,
            (_, true, _) => GameStateUpdateMode::MenuUpdate,
            (_, _, true) => GameStateUpdateMode::PlayUpdate,
            (_, _, false) => GameStateUpdateMode::PausedUpdate,
        }
    }

    pub fn handle_update_event(&mut self, ctx: &mut Context) -> GameResult {
        let time_scale = 1.0;
        let delta_s = time_scale * (ggez::timer::duration_to_f64(ggez::timer::delta(ctx)) as f32);
        let game_run_secs = self.world.fetch_mut::<GameStateResource>().game_run_seconds;
        
        // MASTER UPDATE SWITCH 
        // UPDATES ONE OF:
        //  PLAY system
        //  PAUSED system
        //  MENU system
        //  TERMINAL system

        let update_mode = self.get_update_mode();

        // Only update world state when game is running (not paused)
        //let menu_lvls = self.menu_stack.len();
        match update_mode {

            GameStateUpdateMode::PlayUpdate => {
        
                //self.run_update_step(ctx, delta_s);
                CoreSystem::run_update_step(self, ctx, delta_s);

                // Auto-open menu after 5 seconds of loading splashscreen
                if self.current_level_name == "" && game_run_secs > 5.0 && game_run_secs < 5.5 {
                    if self.menu_stack.len() == 0 {
                        self.open_menu();

                        self.world.fetch_mut::<GameStateResource>().game_run_seconds = 10.0;
                    }                        
                }

            },
            GameStateUpdateMode::PausedUpdate => {
                CoreSystem::run_pause_step(self, ctx, delta_s);
            },
            GameStateUpdateMode::MenuUpdate => {
                CoreSystem::run_menu_step(self, ctx, delta_s);
            },
            GameStateUpdateMode::TerminalUpdate => {
                CoreSystem::run_terminal_step(self, ctx, delta_s);
            },
            GameStateUpdateMode::NoUpdate => {}
        }

        // always ok result
        Ok(())
    }

    pub fn handle_render_event(&mut self, ctx: &mut Context) -> GameResult {
        
        // Call rendering module        
        let mut renderer = render::Renderer::new();

        // if self.snap_view {
        //     renderer.snap_view = true;
        //     self.snap_view = false;
        // }

        if self.game_frame_count % 60 == 1 {
            println!("D) Run Render Frame ==================");
        }
        let gr = renderer.render_frame(self, ctx);

        //self.current_offset = renderer.display_offset;

        // let mut game_state_writer = self.world.get_mut::<GameStateResource>();
        // if let Some(game_state_res) = game_state_writer {
        //     game_state_res.display_offset.0 = renderer.display_offset.x;
        //     game_state_res.display_offset.1 = renderer.display_offset.y;
        // }

        // Yield process to os
        ggez::timer::yield_now();

        gr

    }

    pub fn get_current_menu_lvl(&self) -> i32 {
        if self.menu_stack.len() > 0 {
            self.menu_stack.len() as i32 - 1
        }
        else {
            -1
        }
    }

    pub fn check_selected_menu_index(&mut self) {
        //println!("Check Selected menu index");
        // Menus open
        if self.menu_stack.len() > 0 {
            //println!(" o Get Curr Menu Lvl");
            // Get current menu level
            let curr_menu_lvl = self.get_current_menu_lvl();
            //println!("   Result: {}", &curr_menu_lvl);
            if curr_menu_lvl > -1 {
                // Get Menu from stack
                let mut curr_menu = &mut self.menu_stack[curr_menu_lvl as usize];
                //println!("   Curr Menu: {:?}", curr_menu);
                let mut sel_index = curr_menu.selected_index;
                //println!("   Curr Sel Index: {}", sel_index);
                let item_count = curr_menu.items.len();
                
                if sel_index < 0 && item_count > 0 {
                    sel_index = 0;
                }
                else if sel_index + 1 >= item_count as i32 {
                    sel_index = item_count as i32 - 1;
                }

                // Find the current sel index, and check if valid
                let mut item_index = 0;
                for item in &curr_menu.items {
                    //println!("   Iterate Item Index: {:?}", &item);
                    if item_index == sel_index {
                        println!("   Check Index: {}", &item_index);
                        //
                        if let &MenuItem::Header {..} = item {
                            //println!("   Header check: {} < {} ?", &(sel_index + 1), &(item_count as i32));
                            if sel_index + 1 < item_count as i32 {
                                sel_index += 1;
                            }
                        }
                    }
                    item_index += 1;
                }

                curr_menu.selected_index = sel_index;
                
                //match &curr_menu.items[curr_menu.selected_index as usize] {
            }

        }
        //println!("END OF Check Selected menu index");
    }

    pub fn close_all_menus(&mut self) {
        self.menu_stack.clear();
        //self.ui_game_display_zoom = 1.0;
    }

    pub fn close_menu(&mut self) {

        if self.menu_stack.len() > 0 {
            self.menu_stack.pop();
        }

        if self.menu_stack.len() == 0 {
            //self.ui_game_display_zoom = 1.0;
        }
    }

    pub fn open_menu(&mut self) {

        // If not open-already
        if self.menu_stack.len() == 0 {
            //self.ui_game_display_zoom = 1.0;
        }

        // Clear menu stack
        self.menu_stack.clear();
    
        let mut new_menu = Menu::new("Suri Sprint".to_string());

        if self.current_level_name != "loading" && !self.current_level_name.is_empty() {
            new_menu.items.push(MenuItem::ButtonItem { name: "Resume Game".to_string(), key: "resume".to_string() });
            new_menu.items.push(MenuItem::ButtonItem { name: "Restart Level".to_string(), key: "restart_level".to_string() });
        }
        new_menu.items.push(MenuItem::ButtonItem { name: "New Game".to_string(), key: "new_game".to_string() });
        new_menu.items.push(MenuItem::ButtonItem { name: "Options...".to_string(), key: "options".to_string() });
        new_menu.items.push(MenuItem::ButtonItem { name: "Advanced...".to_string(), key: "advanced".to_string() });
        new_menu.items.push(MenuItem::ButtonItem { name: "Exit Game".to_string(), key: "exit".to_string() });
        self.menu_stack.push(new_menu);
        // ensure default selected index is valid
        self.check_selected_menu_index();
        
        //}
        // else {
        //     let mut new_menu = Menu::new(format!("New Menu {}", &self.game_frame_count));
        //     new_menu.items.push(MenuItem::ToggleItem { name: "Fullscreen".to_string(), key: "fullscreen".to_string(), value: false });
        //     new_menu.items.push(MenuItem::RangeItem { name: "Audio Volume".to_string(), key: "audio_volume".to_string(), value: 0.05, min: 0.0, max: 1.0, incr: 0.05});
        //     new_menu.items.push(MenuItem::RangeItem { name: "Music Volume".to_string(), key: "music_volume".to_string(), value: 0.05, min: 0.0, max: 1.0, incr: 0.05});
        //     //new_menu.selected_index = 0;
        //     self.menu_stack.push(new_menu);

        //     self.check_selected_menu_index();
        // }
    }

    pub fn open_submenu(&mut self, menu_name: String) {
        if &menu_name == "options" {
            let mut new_menu = Menu::new("Options Menu".to_string());
            new_menu.items.push(MenuItem::Header("Display".to_string()));
            new_menu.items.push(MenuItem::ButtonItem { name: "Toggle Fullscreen".to_string(), key: "fullscreen".to_string() });
            new_menu.items.push(MenuItem::Header("Audio".to_string()));
            new_menu.items.push(MenuItem::RangeItem { name: "Audio Volume".to_string(), key: "audio_volume".to_string(),
                value: self.audio.base_sfx_volume, min: 0.0, max: 2.0, incr: 0.05});
            new_menu.items.push(MenuItem::RangeItem { name: "Music Volume".to_string(), key: "music_volume".to_string(),
                value: self.audio.base_music_volume, min: 0.0, max: 2.0, incr: 0.05});
            //new_menu.items.push(MenuItem::RangeItem { name: "Music Volume".to_string(), key: "music_volume".to_string(), value: 0.05, min: 0.0, max: 2.0, incr: 0.05});
            //new_menu.items.push(MenuItem::ButtonItem { name: "Options...".to_string(), key: "options".to_string() });
            new_menu.items.push(MenuItem::ButtonItem { name: "Close".to_string(), key: "close_menu".to_string() });
            //new_menu.selected_index = 0;
            self.menu_stack.push(new_menu);
            // ensure default selected index is valid
            self.check_selected_menu_index();
        }
        else if &menu_name == "advanced" {
            let mut new_menu = Menu::new("Advanced Menu".to_string());
            new_menu.items.push(MenuItem::Header("Output Gobbaldygook To Console".to_string()));
            new_menu.items.push(MenuItem::ToggleItem { name: "Debug Logic".to_string(), key: "debug_logic".to_string(), value: false });
            new_menu.items.push(MenuItem::ToggleItem { name: "Debug Physics Contacts".to_string(), key: "debug_contacts".to_string(), value: false });
            new_menu.items.push(MenuItem::ToggleItem { name: "Debug Systems".to_string(), key: "debug_systems".to_string(), value: true });
            new_menu.items.push(MenuItem::ToggleItem { name: "Debug Raycasts".to_string(), key: "debug_raycasts".to_string(), value: false });
            new_menu.items.push(MenuItem::Header("Engine Levers".to_string()));
            new_menu.items.push(MenuItem::RangeItem { name: "Time Multiplier".to_string(), key: "time_multiplier".to_string(), 
                value: 1.0, min: 0.1, max: 2.0, incr: 0.05});
            new_menu.items.push(MenuItem::RangeItem { name: "Gravity Multiplier".to_string(), key: "gravity_multiplier".to_string(), 
                value: 1.0, min: 0.1, max: 2.0, incr: 0.05});
            new_menu.items.push(MenuItem::RangeItem { name: "Suri Standard Mass".to_string(), key: "suri_mass_multiplier".to_string(), 
                value: 1.0, min: 0.1, max: 2.0, incr: 0.05});
            new_menu.items.push(MenuItem::ButtonItem { name: "Close".to_string(), key: "close_menu".to_string() });
            //new_menu.selected_index = 0;
            self.menu_stack.push(new_menu);
            // ensure default selected index is valid
            self.check_selected_menu_index();
        }
    }

    pub fn toggle_fullscreen_mode(&mut self, ctx: &mut Context) {
        let mut game_state_writer = self.world.fetch_mut::<GameStateResource>();

        let new_fs_type = match game_state_writer.window_mode.fullscreen_type {
            ggez::conf::FullscreenType::Windowed => ggez::conf::FullscreenType::Desktop,
            ggez::conf::FullscreenType::Desktop => ggez::conf::FullscreenType::True,
            ggez::conf::FullscreenType::True => ggez::conf::FullscreenType::Windowed,
        };
        game_state_writer.window_mode.fullscreen_type = new_fs_type;

        let _ = ggez::graphics::set_fullscreen(ctx, new_fs_type).is_err();

        let window = ggez::graphics::window(ctx);
        window.show();
    }

    pub fn restart_level(&mut self, ctx: &mut Context) {
        self.add_log("Restarting Level".to_string());
        if self.current_level_name.is_empty() {
            self.empty_level(ctx);
        }
        else {
            self.load_level(ctx, self.current_level_name.clone(), self.current_entry_name.clone());
        }
        
    }

    pub fn set_timescale(&mut self, new_time_multiplier: f32) {
        self.play_time_scale = new_time_multiplier;
    }

    pub fn add_log(&self, msg: String) {
        self.world.fetch_mut::<GameLog>().add_entry(true, msg, None, self.world.fetch::<GameStateResource>().game_run_seconds);
    }

    
    // pub fn toggle_terminal(&mut self) {
    //     self.set_terminal_open(!self.terminal_open);
    // }
    // pub fn set_terminal_open(&mut self, open: bool) {
    //     println!("Set terminal open to {}", &open);
    //     if open != self.terminal_open {
    //         // opening terminal
    //         if open {

    //         }
    //         // closing it
    //         else {

    //         }
    //         self.terminal_open = open;
    //         println!("  Set to {} = {}", &open, &self.terminal_open);
    //     }
    // }

}

