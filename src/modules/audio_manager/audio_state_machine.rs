use crate::modules::audio_manager::audio_controller::{AudioManager, Manager};
use zbus::{Result};


pub enum AudioState<'a> {
    OffState(Off<'a>),
    PlayingState(Playing<'a>),
    PausedState(Paused<'a>),
}

impl<'a> AudioState<'a> {
    pub async fn on_event(self, event: AudioEvent) -> Result<AudioState<'a>> {
        match self {
            AudioState::OffState(state) => Ok(state.react(event).await?),
            AudioState::PlayingState(state) => Ok(state.react(event).await?),
            AudioState::PausedState(state) => Ok(state.react(event).await?),
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


pub struct Off<'a> {
    audio_manager: AudioManager<'a>
}

impl<'a> Off<'a> {
    pub fn new(audio_manager: AudioManager) -> Off { Off { audio_manager } } 

    async fn react(self, event: AudioEvent) -> Result<AudioState<'a>> {
        match event {
            AudioEvent::Play => {
                match self.audio_manager.play().await {
                    Err(err) => {
                        println!("Did not manage to start playing. Error: {err}");
                        Err(err)
                    },
                    Ok(_) => { Ok(AudioState::playing(self.audio_manager, 30)) }
                }
            },
            AudioEvent::Stop => Ok(AudioState::off(self.audio_manager)),
            AudioEvent::VolUp => Ok(AudioState::off(self.audio_manager)),
            AudioEvent::VolDown => Ok(AudioState::off(self.audio_manager)),
            AudioEvent::Forward => Ok(AudioState::off(self.audio_manager)),
            AudioEvent::Back => Ok(AudioState::off(self.audio_manager)),
            AudioEvent::TrackForward => Ok(AudioState::off(self.audio_manager)),
            AudioEvent::TrackBack => Ok(AudioState::off(self.audio_manager)),
        }
    }

}


pub struct Playing<'a> {
    audio_manager: AudioManager<'a>,
    volume: i32
}

impl<'a> Playing<'a> {
    pub fn new(audio_manager: AudioManager, volume: i32) -> Playing { Playing { audio_manager, volume } }

    async fn react(self, event: AudioEvent) -> Result<AudioState<'a>> {
        match event {
            AudioEvent::Play => {
                self.audio_manager.pause().await?;
                Ok(AudioState::paused(self.audio_manager, 30 ))
            },
            AudioEvent::Stop => {
                // todo: need to implement stop somehow maybe?
                Ok(AudioState::off(self.audio_manager))
            },
            AudioEvent::VolUp => {
                let new_vol: i32 = mod_volume(&self.audio_manager, self.volume, 5).await?;
                Ok(AudioState::playing(self.audio_manager, new_vol))
            },
            AudioEvent::VolDown => {
                let new_vol: i32 = mod_volume(&self.audio_manager, self.volume, -5).await?;
                Ok(AudioState::playing(self.audio_manager, new_vol))
            },
            AudioEvent::Forward => {
                next_track(&self.audio_manager).await?;
                Ok(AudioState::playing(self.audio_manager, self.volume))
            },
            AudioEvent::Back => {
                previous_track(&self.audio_manager).await?;
                Ok(AudioState::playing(self.audio_manager, self.volume))
            },
            AudioEvent::TrackForward => {
                modify_timer(&self.audio_manager, 15).await?;
                Ok(AudioState::playing(self.audio_manager, self.volume))
            }
            AudioEvent::TrackBack => {
                modify_timer(&self.audio_manager, -15).await?;
                Ok(AudioState::playing(self.audio_manager, self.volume))
            }
        }
    }
}


pub struct Paused<'a> {
    audio_manager: AudioManager<'a>,
    volume: i32
}

impl<'a> Paused<'a> {
    pub fn new(audio_manager: AudioManager, volume: i32) -> Paused { Paused { audio_manager, volume } }

    async fn react(self, event: AudioEvent) -> Result<AudioState<'a>> {
        match event {
            AudioEvent::Play => {
                match self.audio_manager.play().await {
                    Err(err) => {
                        println!("Did not manage to start playing. Error: {err}");
                        Err(err)
                    },
                    Ok(_) => { Ok(AudioState::playing(self.audio_manager, 30 )) }
                }
            },
            AudioEvent::Stop => {
                // todo: maybe implement something with stop later
                Ok(AudioState::off(self.audio_manager))
            },
            AudioEvent::VolUp => {
                let new_vol: i32 = mod_volume(&self.audio_manager, self.volume, 5).await?;
                Ok(AudioState::paused(self.audio_manager, new_vol))
            },
            AudioEvent::VolDown => {
                let new_vol: i32 = mod_volume(&self.audio_manager, self.volume, -5).await?;
                Ok(AudioState::paused(self.audio_manager, new_vol))
            },
            AudioEvent::Forward => {
                next_track(&self.audio_manager).await?;
                Ok(AudioState::paused(self.audio_manager, self.volume))
            },
            AudioEvent::Back => {
                previous_track(&self.audio_manager).await?;
                Ok(AudioState::paused(self.audio_manager, self.volume))
            },
            AudioEvent::TrackForward => {
                modify_timer(&self.audio_manager, 15).await?;
                Ok(AudioState::paused(self.audio_manager, self.volume))
            }
            AudioEvent::TrackBack => {
                modify_timer(&self.audio_manager, -15).await?;
                Ok(AudioState::paused(self.audio_manager, self.volume))
            }
        }
    }
}


async fn mod_volume<'a>(audio_manager: &AudioManager<'a>, volume: i32, diff: i32) -> Result<i32> {
    let new_volume: i32 = volume + diff;
    match audio_manager.set_volume(new_volume).await {
        Err(err) => {
            println!("Could not increase volume. Error: {err}");
            Ok(volume)
        },
        Ok(_) => { Ok(new_volume) }
    }
}

async fn modify_timer<'a>(audio_manager: &AudioManager<'a>, seconds: i64) -> Result<()> {
    if seconds >= 0 {

        match audio_manager.move_track_forward(seconds).await {
            Err(err) => {
                println!("Failed to move track forward. Error: {err}");
                Err(err)
            }
            Ok(_) => { Ok(()) }
        }
    } else {
        match audio_manager.move_track_backward(seconds).await {
            Err(err) => {
                println!("Failed to move track backward. Error: {err}");
                Err(err)
            },
            Ok(_) => { Ok(()) }
        }
    }
}

async fn next_track<'a>(audio_manager: &AudioManager<'a>) -> Result<()> {
    match audio_manager.next_track().await {
        Err(err) => {
            println!("Failed to switch to next track. Error: {err}");
            Err(err)
        },
        Ok(_) => { Ok(()) }
    }
}

async fn previous_track<'a>(audio_manager: &AudioManager<'a>) -> Result<()>{
    match audio_manager.previous_track().await {
        Err(err) => {
            println!("Failed to switch to previous track. Error: {err}");
            Err(err)
        },
        Ok(_) => { Ok(()) }
    }
}
