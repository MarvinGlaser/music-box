use std::io::Error;



pub struct AudioManager { }


pub trait Manager {
    fn set_volume(&self, level: i32) -> Result<(), Error>;
    fn play(&self) -> Result<(), Error>;
    fn pause(&self) -> Result<(), Error>;
    fn move_track_forward(&self, seconds: i32) -> Result<(), Error>;
    fn move_track_backward(&self, seconds: i32) -> Result<(), Error>;
    fn next_track(&self) -> Result<(), Error>;
    fn previous_track(&self) -> Result<(), Error>;
}


impl Manager for AudioManager {

    fn set_volume(&self, level: i32) -> Result<(), Error> {
        if level > 40 {
            Err(Error::other(format!("Audio too loude, could not set level to {}", level)))
        } else {
            println!("Set audio to level {level}");
            Ok(())
        }
    }

    fn play(&self) -> Result<(), Error> {
        println!("Hello, I am playing a song!");
        Ok(())
    }

    fn pause(&self) -> Result<(), Error> {
        println!("Aww, I have to pause now...");
        Ok(())
    }

    fn move_track_forward(&self, seconds: i32) -> Result<(), Error> {
        println!("Moving the track forward {seconds} seconds");
        Ok(())
    }

    fn move_track_backward(&self, seconds: i32) -> Result<(), Error> {
        println!("Moving the track backwards {seconds} seconds");
        Ok(())
    }

    fn next_track(&self) -> Result<(), Error> {
        println!("Playing next track");
        Ok(())
    }

    fn previous_track(&self) -> Result<(), Error> {
        println!("Playing previous track");
        Ok(())
    }

}
