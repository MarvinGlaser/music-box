use crate::modules::audio_manager::audio_controller::{AudioManager, Manager};



pub enum AudioStates {
    Off(Off),
    Playing(Playing),
    Paused(Paused),
}

impl AudioStates {
    pub fn on_event(self, event: AudioEvent) -> AudioStates {
        match self {
            AudioStates::Off(state) => state.react(event),
            AudioStates::Playing(state) => state.react(event),
            AudioStates::Paused(state) => state.react(event)
        }
    }

    pub fn print_state(&self) {
        let mut state: String = String::from(""); 
        match self {
            AudioStates::Off(_) => state.push_str("Off"),
            AudioStates::Playing(_) => state.push_str("Playing"),
            AudioStates::Paused(_) => state.push_str("Paused"),
        }
        println!("Current state: {}", state)
    }

    fn off(audio_manager: AudioManager) -> AudioStates {
        AudioStates::Off(Off { audio_manager })
    }

    fn playing(audio_manager: AudioManager, volume: i32) -> AudioStates {
        AudioStates::Playing(Playing { audio_manager, volume })
    }

    fn paused(audio_manager: AudioManager, volume: i32) -> AudioStates {
        AudioStates::Paused(Paused { audio_manager, volume })
    }

}

pub enum AudioEvent {
    Play,
    VolUp,
    VolDown,
    Forward,
    Back,
    TrackForward,
    TrackBack,
}


pub struct Off {
    audio_manager: AudioManager
}

pub struct Playing {
    audio_manager: AudioManager,
    volume: i32
}

pub struct Paused {
    audio_manager: AudioManager,
    volume: i32
}



fn mod_volume(audio_manager: &AudioManager, volume: i32, diff: i32) -> i32 {
                let new_volume: i32 = volume + diff;
                let res = audio_manager.set_volume(new_volume);
                if let Err(err) = res {
                    println!("Could not increase volume. Error: {}", err);
                    new_volume
                } else {
                    volume
                }
}

fn modify_timer(audio_manager: &AudioManager, seconds: i32) -> () {
    if seconds >= 0 {
        let res = audio_manager.move_track_forward(seconds)
        if let Err(err) = res { println!("Failed to move track forward. Error: {}", err) }
    } else {
        let res = audio_manager.move_track_backward(seconds)
        if let Err(err) = res { println!("Failed to move track backward. Error: {}", err) }
    }
}

// todo: add constructors to event handling!!



impl Off {
    pub fn new(manager: AudioManager) -> Off { Off {audio_manager: manager} }

    fn react(self, event: AudioEvent) -> AudioStates {
        match event {
            AudioEvent::Play => AudioStates::Playing(Playing { audio_manager: self.audio_manager, volume: 30 }),
            //AudioEvent::VolUp => AudioStates::Off(Off { audio_manager: self.audio_manager }),
            AudioEvent::VolUp => AudioStates::off(self.audio_manager),
            AudioEvent::VolDown => AudioStates::Off(Off {audio_manager: self.audio_manager }),
            AudioEvent::Forward => AudioStates::Off(Off {audio_manager: self.audio_manager }),
            AudioEvent::Back => AudioStates::Off(Off {audio_manager: self.audio_manager }),
            AudioEvent::TrackForward => AudioStates::Off(Off {audio_manager: self.audio_manager }),
            AudioEvent::TrackBack => AudioStates::Off(Off {audio_manager: self.audio_manager }),
        }
    }

}

impl Playing {
    fn react(self, event: AudioEvent) -> AudioStates {
        match event {
            AudioEvent::Play => AudioStates::Paused(Paused { audio_manager: self.audio_manager, volume: 30 }),
            AudioEvent::VolUp => {
                let new_vol: i32 = mod_volume(&self.audio_manager, self.volume, 5);
                AudioStates::Playing(Playing { audio_manager: self.audio_manager, volume: new_vol })
            },
            AudioEvent::VolDown => {
                let new_vol: i32 = mod_volume(&self.audio_manager, self.volume, -5);
                AudioStates::Playing(Playing { self.audio_manager, new_vol })
            },
            AudioEvent::Forward => AudioStates::Playing(Playing { audio_manager: self.audio_manager, volume: self.volume }),
            AudioEvent::Back => AudioStates::Playing(Playing { audio_manager: self.audio_manager, volume: self.volume }),
            AudioEvent::TrackBack => {
                modify_timer(&self.audio_manager, 15);
                AudioStates::Playing(Playing { audio_manager: self.audio_manager, volume: self.volume })
            }
            AudioEvent::TrackForward => {}
        }
    }
}

impl Paused {
    fn react(self, event: AudioEvent) -> AudioStates {
        match event {
            AudioEvent::Play => AudioStates::Playing(Playing { audio_manager: self.audio_manager, volume: 30 }),
            AudioEvent::VolUp => {
                let new_vol: i32 = mod_volume(&self.audio_manager, self.volume, 5);
                AudioStates::Paused(Paused { audio_manager: self.audio_manager, volume: new_vol })
            },
            AudioEvent::VolDown => {
                let new_vol: i32 = mod_volume(&self.audio_manager, self.volume, -5);
                AudioStates::Paused(Paused { audio_manager: self.audio_manager, volume: new_vol })
            },
            AudioEvent::Forward => AudioStates::Paused(Paused { audio_manager: self.audio_manager, volume: self.volume }),
            AudioEvent::Back => AudioStates::Paused(Paused { audio_manager: self.audio_manager, volume: self.volume }),
        }
    }
}
