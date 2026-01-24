use crate::modules::audio_manager::audio_controller::{AudioManager, Manager};


pub enum AudioState {
    OffState(Off),
    PlayingState(Playing),
    PausedState(Paused),
}

impl AudioState {
    pub fn on_event(self, event: AudioEvent) -> AudioState {
        match self {
            AudioState::OffState(state) => state.react(event),
            AudioState::PlayingState(state) => state.react(event),
            AudioState::PausedState(state) => state.react(event)
        }
    }

    pub fn print_state(&self) {
        let mut state: String = String::from(""); 
        match self {
            AudioState::OffState(_) => state.push_str("Off"),
            AudioState::PlayingState(_) => state.push_str("Playing"),
            AudioState::PausedState(_) => state.push_str("Paused"),
        }
        println!("Current state: {}", state)
    }

    pub fn off(audio_manager: AudioManager) -> AudioState {
        AudioState::OffState(Off::new(audio_manager))
    }

    pub fn playing(audio_manager: AudioManager, volume: i32) -> AudioState {
        AudioState::PlayingState(Playing::new(audio_manager, volume))
    }

    pub fn paused(audio_manager: AudioManager, volume: i32) -> AudioState {
        AudioState::PausedState(Paused::new(audio_manager, volume))
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

impl Off {
    pub fn new(manager: AudioManager) -> Off { Off {audio_manager: manager} }

    fn react(self, event: AudioEvent) -> AudioState {
        match event {
            AudioEvent::Play => {
                let res = self.audio_manager.play();
                if let Err(err) = res { println!("Did not manage to start playing. Error: {err}") };
                AudioState::playing(self.audio_manager, 30)
            },
            AudioEvent::Stop => AudioState::off(self.audio_manager),
            AudioEvent::VolUp => AudioState::off(self.audio_manager),
            AudioEvent::VolDown => AudioState::off(self.audio_manager),
            AudioEvent::Forward => AudioState::off(self.audio_manager),
            AudioEvent::Back => AudioState::off(self.audio_manager),
            AudioEvent::TrackForward => AudioState::off(self.audio_manager),
            AudioEvent::TrackBack => AudioState::off(self.audio_manager),
        }
    }

}


pub struct Playing {
    audio_manager: AudioManager,
    volume: i32
}

impl Playing {
    pub fn new(audio_manager: AudioManager, volume: i32) -> Playing { Playing { audio_manager, volume } }

    fn react(self, event: AudioEvent) -> AudioState {
        match event {
            AudioEvent::Play => {
                let res = self.audio_manager.pause();
                if let Err(err) = res { println!("Did not manage to start playing. Error: {err}") };
                AudioState::paused(self.audio_manager, 30 )
            },
            AudioEvent::Stop => AudioState::off(self.audio_manager),
            AudioEvent::VolUp => {
                let new_vol: i32 = mod_volume(&self.audio_manager, self.volume, 5);
                AudioState::playing(self.audio_manager, new_vol)
            },
            AudioEvent::VolDown => {
                let new_vol: i32 = mod_volume(&self.audio_manager, self.volume, -5);
                AudioState::playing(self.audio_manager, new_vol)
            },
            AudioEvent::Forward => {
                next_track(&self.audio_manager);
                AudioState::playing(self.audio_manager, self.volume)
            },
            AudioEvent::Back => {
                previous_track(&self.audio_manager);
                AudioState::playing(self.audio_manager, self.volume)
            },
            AudioEvent::TrackForward => {
                modify_timer(&self.audio_manager, 15);
                AudioState::playing(self.audio_manager, self.volume)
            }
            AudioEvent::TrackBack => {
                modify_timer(&self.audio_manager, -15);
                AudioState::playing(self.audio_manager, self.volume)
            }
        }
    }
}


pub struct Paused {
    audio_manager: AudioManager,
    volume: i32
}

impl Paused {
    pub fn new(audio_manager: AudioManager, volume: i32) -> Paused { Paused { audio_manager, volume } }

    fn react(self, event: AudioEvent) -> AudioState {
        match event {
            AudioEvent::Play => {
                let res = self.audio_manager.play();
                if let Err(err) = res { println!("Did not manage to start playing. Error: {err}") };
                AudioState::playing(self.audio_manager, 30 )
            },
            AudioEvent::Stop => AudioState::off(self.audio_manager),
            AudioEvent::VolUp => {
                let new_vol: i32 = mod_volume(&self.audio_manager, self.volume, 5);
                AudioState::paused(self.audio_manager, new_vol)
            },
            AudioEvent::VolDown => {
                let new_vol: i32 = mod_volume(&self.audio_manager, self.volume, -5);
                AudioState::paused(self.audio_manager, new_vol)
            },
            AudioEvent::Forward => {
                next_track(&self.audio_manager);
                AudioState::paused(self.audio_manager, self.volume)
            },
            AudioEvent::Back => {
                previous_track(&self.audio_manager);
                AudioState::paused(self.audio_manager, self.volume)
            },
            AudioEvent::TrackForward => {
                modify_timer(&self.audio_manager, 15);
                AudioState::paused(self.audio_manager, self.volume)
            }
            AudioEvent::TrackBack => {
                modify_timer(&self.audio_manager, -15);
                AudioState::paused(self.audio_manager, self.volume)
            }
        }
    }
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
