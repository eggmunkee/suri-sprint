
use ggez::{Context};
use ggez::audio::*;


pub struct Audio {
    pub music_source: Option<Source>,
    pub song_path: String,
    pub base_music_volume: f32,
    pub base_sfx_volume: f32,
    pub dimmed_mult: f32,
    pub is_dimmed: bool,
    pub meow_source: Option<Source>,
}


impl Audio {
    pub fn new(ctx: &mut Context) -> Self {
        let default_vol = 0.05;

        let mut meow : Option<Source> = None;
        let meow_path = "/audio/suri-meow-2.ogg";        
        if let Ok(mut source) = Source::new(ctx, meow_path.clone()) {
            source.set_volume(default_vol * 1.0);
            //println!("Loading sound? {}", &source.playing());
            source.set_repeat(false);
            //source.play();

            meow = Some(source);
        }
        else {
            println!("Could not load sound effect: {}", &meow_path);
        }

        Audio {
            music_source: None,
            song_path: String::new(),
            base_music_volume: default_vol, // .3
            base_sfx_volume: default_vol,
            dimmed_mult: 0.5,
            is_dimmed: false,
            meow_source: meow
        }
    }

    pub fn set_dimmed(&mut self, dimmed: bool) {
        self.is_dimmed = dimmed;

        // let vol = self.curr_volume();
        // if let Some(ref mut source) = &mut self.music_source {
        //     source.set_volume(vol);
        // }
        self.set_music_volume_force(self.base_music_volume, true);
    }

    fn curr_music_volume(&self) -> f32 {
        match self.is_dimmed {
            true => self.base_music_volume * self.dimmed_mult,
            false => self.base_music_volume,
        }
    }

    fn curr_sfx_volume(&self) -> f32 {
        match self.is_dimmed {
            true => self.base_sfx_volume * self.dimmed_mult,
            false => self.base_sfx_volume,
        }
    }

    pub fn limit_volume(vol: f32) -> f32 {
        (
            (vol * 20.0).round() / 20.0
        )
        .min(2.0)
        .max(0.0)
    }

    pub fn incr_music_volume(&mut self) {
        let new_level = Self::limit_volume(self.base_music_volume + 0.05);
            //(((self.base_music_volume - 0.05) * 20.0).round()
            /// 20.0).min(2.0);
        self.set_music_volume(new_level);
    }
    pub fn decr_music_volume(&mut self) {
        let new_level = Self::limit_volume(self.base_music_volume - 0.05);
        // let new_level = (((self.base_music_volume - 0.05) * 20.0).round()
        //     / 20.0).max(0.0);
        self.set_music_volume(new_level);
    }

    pub fn set_music_volume(&mut self, new_level: f32) {
        self.set_music_volume_force(new_level, false);
    }

    pub fn set_music_volume_force(&mut self, new_level: f32, force_update: bool) {
        if !force_update && self.base_music_volume == new_level { return; }
        self.base_music_volume = Self::limit_volume(new_level);
        let vol = self.curr_music_volume();

        if let Some(ref mut source) = &mut self.music_source {
            source.set_volume(vol * 0.05 );
        }
    }

    pub fn set_sfx_volume(&mut self, new_level: f32) {
        self.set_sfx_volume_force(new_level, false);
    }

    pub fn set_sfx_volume_force(&mut self, new_level: f32, force_update: bool) {
        if !force_update && self.base_music_volume == new_level { return; }
        self.base_sfx_volume = new_level;
        let vol = self.curr_sfx_volume();

        if let Some(ref mut meow) = &mut self.meow_source {
            meow.set_volume(vol * 1.0 )
        }
    }

    pub fn play_music(&mut self, ctx: &mut Context, music_path: String) {
        let vol = self.curr_music_volume();
        let song_path = music_path.clone();
        println!("Playing music... {} at vol {}", &song_path, &vol);

        if song_path != self.song_path && song_path != "" {
            if let Ok(mut source) = Source::new(ctx, music_path) {
                source.set_volume(vol * 0.05);
                println!("Is music playing? {}", &source.playing());
                source.set_repeat(true);
                source.play();
    
                self.music_source = Some(source);
                self.song_path = song_path;
            }
            else {
                println!("Could not load music");
            }           
        }
        else {
            println!("Already playing song");
            if let Some(ref mut source) = &mut self.music_source {
                source.set_volume(vol * 0.05);
                if source.paused() {
                    source.resume();
                }
                else if !source.playing() {
                    source.play();
                }
            }
        }
    }

    pub fn stop_music(&mut self, ctx: &mut Context) {
        if let Some(ref mut source) = &mut self.music_source {
            source.stop();
        }
    }

    pub fn pause_music(&mut self) {
        if let Some(ref mut source) = &mut self.music_source {
            if source.playing() {
                source.pause();
            }            
        }
    }

    pub fn resume_music(&mut self) {
        if let Some(ref mut source) = &mut self.music_source {
            if source.paused() {
                source.resume();
            }            
        }
    }

    pub fn play_meow(&mut self) {
        if let Some(ref mut meow) = &mut self.meow_source {
            if !meow.playing() {
                meow.play();
            }
            
        }
    }
}

