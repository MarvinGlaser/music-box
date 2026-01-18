use crate::modules::audio_manager::audio_controller::{AudioManager, Manager};


pub enum AudioStates {
    OffState(Off),
    PlayingState(Playing),
    PausedState(Paused),
}

impl AudioStates {
    pub fn on_event(self, event: AudioEvent) -> AudioStates {
        match self {
            AudioStates::OffState(state) => state.react(event),
            AudioStates::PlayingState(state) => state.react(event),
            AudioStates::PausedState(state) => state.react(event)
        }
    }

    pub fn print_state(&self) {
        let mut state: String = String::from(""); 
        match self {
            AudioStates::OffState(_) => state.push_str("Off"),
            AudioStates::PlayingState(_) => state.push_str("Playing"),
            AudioStates::PausedState(_) => state.push_str("Paused"),
        }
        println!("Current state: {}", state)
    }

    fn off(audio_manager: AudioManager) -> AudioStates {
        AudioStates::OffState(Off { audio_manager })
    }

    fn playing(audio_manager: AudioManager, volume: i32) -> AudioStates {
        AudioStates::PlayingState(Playing { audio_manager, volume })
    }

    fn paused(audio_manager: AudioManager, volume: i32) -> AudioStates {
        AudioStates::PausedState(Paused { audio_manager, volume })
    }

}

pub enum AudioEvent {
    Play,
    Stop,
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
        volume
    } else {
        new_volume
    }
}

fn modify_timer(audio_manager: &AudioManager, seconds: i32) {
    if seconds >= 0 {
        let res = audio_manager.move_track_forward(seconds);
        if let Err(err) = res { println!("Failed to move track forward. Error: {}", err) }
    } else {
        let res = audio_manager.move_track_backward(seconds);
        if let Err(err) = res { println!("Failed to move track backward. Error: {}", err) }
    }
}

fn next_track(audio_manager: &AudioManager) {
    let res = audio_manager.next_track();
    if let Err(err) = res { println!("Failed to switch to next track. Error: {}", err) }
}

fn previous_track(audio_manager: &AudioManager) {
    let res = audio_manager.previous_track();
    if let Err(err) = res { println!("Failed to switch to previous track. Error: {}", err) }
}


impl Off {
    pub fn new(manager: AudioManager) -> Off { Off {audio_manager: manager} }

    fn react(self, event: AudioEvent) -> AudioStates {
        match event {
            AudioEvent::Play => AudioStates::playing(self.audio_manager, 30),
            AudioEvent::Stop => AudioStates::off(self.audio_manager),
            AudioEvent::VolUp => AudioStates::off(self.audio_manager),
            AudioEvent::VolDown => AudioStates::off(self.audio_manager),
            AudioEvent::Forward => AudioStates::off(self.audio_manager),
            AudioEvent::Back => AudioStates::off(self.audio_manager),
            AudioEvent::TrackForward => AudioStates::off(self.audio_manager),
            AudioEvent::TrackBack => AudioStates::off(self.audio_manager),
        }
    }

}

impl Playing {
    fn react(self, event: AudioEvent) -> AudioStates {
        match event {
            AudioEvent::Play => AudioStates::paused(self.audio_manager, 30 ),
            AudioEvent::Stop => AudioStates::off(self.audio_manager),
            AudioEvent::VolUp => {
                let new_vol: i32 = mod_volume(&self.audio_manager, self.volume, 5);
                AudioStates::playing(self.audio_manager, new_vol)
            },
            AudioEvent::VolDown => {
                let new_vol: i32 = mod_volume(&self.audio_manager, self.volume, -5);
                AudioStates::playing(self.audio_manager, new_vol)
            },
            AudioEvent::Forward => {
                next_track(&self.audio_manager);
                AudioStates::playing(self.audio_manager, self.volume)
            },
            AudioEvent::Back => {
                previous_track(&self.audio_manager);
                AudioStates::playing(self.audio_manager, self.volume)
            },
            AudioEvent::TrackForward => {
                modify_timer(&self.audio_manager, 15);
                AudioStates::playing(self.audio_manager, self.volume)
            }
            AudioEvent::TrackBack => {
                modify_timer(&self.audio_manager, -15);
                AudioStates::playing(self.audio_manager, self.volume)
            }
        }
    }
}

impl Paused {
    fn react(self, event: AudioEvent) -> AudioStates {
        match event {
            AudioEvent::Play => AudioStates::playing(self.audio_manager, 30 ),
            AudioEvent::Stop => AudioStates::off(self.audio_manager),
            AudioEvent::VolUp => {
                let new_vol: i32 = mod_volume(&self.audio_manager, self.volume, 5);
                AudioStates::paused(self.audio_manager, new_vol)
            },
            AudioEvent::VolDown => {
                let new_vol: i32 = mod_volume(&self.audio_manager, self.volume, -5);
                AudioStates::paused(self.audio_manager, new_vol)
            },
            AudioEvent::Forward => {
                next_track(&self.audio_manager);
                AudioStates::paused(self.audio_manager, self.volume)
            },
            AudioEvent::Back => {
                previous_track(&self.audio_manager);
                AudioStates::paused(self.audio_manager, self.volume)
            },
            AudioEvent::TrackForward => {
                modify_timer(&self.audio_manager, 15);
                AudioStates::paused(self.audio_manager, self.volume)
            }
            AudioEvent::TrackBack => {
                modify_timer(&self.audio_manager, -15);
                AudioStates::paused(self.audio_manager, self.volume)
            }
        }
    }
}

