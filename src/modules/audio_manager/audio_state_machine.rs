use crate::modules::audio_manager::audio_controller::AudioManager;



pub enum AudioStates {
    Off(Off),
    Playing(Playing),
    Paused(Paused),
}

pub enum AudioEvent {
    PressPlay,
    PressOther
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


impl Off {
    pub fn new(manager: AudioManager) -> Off { Off {audio_manager: manager} }

    fn react(self, event: AudioEvent) -> AudioStates {
        match event {
            AudioEvent::PressPlay => AudioStates::Playing(Playing { audio_manager: self.audio_manager, volume: 30 }),
            _ => AudioStates::Off(Off {audio_manager: self.audio_manager })
        }
    }

}

impl Playing {
    fn react(self, event: AudioEvent) -> AudioStates {
        match event {
            AudioEvent::PressPlay => AudioStates::Paused(Paused { audio_manager: self.audio_manager, volume: 30 }),
            _ => AudioStates::Playing(Playing { audio_manager: self.audio_manager, volume: self.volume } )
        }
    }
}

impl Paused {
    fn react(self, event: AudioEvent) -> AudioStates {
        match event {
            AudioEvent::PressPlay => AudioStates::Playing(Playing { audio_manager: self.audio_manager, volume: 30 }),
            _ => AudioStates::Paused(Paused { audio_manager: self.audio_manager, volume: self.volume } )
        }
    }
}
