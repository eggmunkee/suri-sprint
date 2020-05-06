
use ggez::{Context};
use ggez::audio::*;


pub struct Audio {
    pub music_source: Option<Source>,
    pub song_path: String,
    pub base_music_volume: f32,
    pub dimmed_mult: f32,
    pub is_dimmed: bool,
}


impl Audio {
    pub fn new() -> Self {
        Audio {
            music_source: None,
            song_path: String::new(),
            base_music_volume: 0.05, // .3
            dimmed_mult: 0.5,
            is_dimmed: false,
        }
    }

    pub fn set_dimmed(&mut self, dimmed: bool) {
        self.is_dimmed = dimmed;

        let vol = self.curr_volume();
        if let Some(ref mut source) = &mut self.music_source {
            source.set_volume(vol);
        }
    }

    fn curr_volume(&self) -> f32 {
        match self.is_dimmed {
            true => self.base_music_volume * self.dimmed_mult,
            false => self.base_music_volume,
        }
    }

    pub fn set_volume(&mut self, new_level: f32) {
        if self.base_music_volume == new_level { return; }
        self.base_music_volume = new_level;
        let vol = self.curr_volume();
        if let Some(ref mut source) = &mut self.music_source {
            source.set_volume(vol);
        }
    }

    pub fn play_music(&mut self, ctx: &mut Context, music_path: String) {
        println!("Playing music...");
        let vol = self.curr_volume();
        let song_path = music_path.clone();

        if song_path != self.song_path && song_path != "" {
            if let Ok(mut source) = Source::new(ctx, music_path) {
                source.set_volume(vol);
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
                source.set_volume(vol);
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
}

