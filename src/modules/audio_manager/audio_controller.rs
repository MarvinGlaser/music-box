use std::io::Error;



pub struct AudioManager { }


pub trait Manager {
    fn set_volume(&self, level: i32) -> Result<(), Error>;
    fn play(&self) -> Result<(), Error>;
    fn pause(&self) -> Result<(), Error>;
    fn forward(&self) -> Result<(), Error>;
    fn backward(&self) -> Result<(), Error>;
}


impl Manager for AudioManager {

    fn set_volume(&self, level: i32) -> Result<(), Error> {
        println!("set audio to level {level}");
        Ok(())
    }

    fn play(&self) -> Result<(), Error> {
        Ok(())
    }

    fn pause(&self) -> Result<(), Error> {
        Ok(())
    }

    fn forward(&self) -> Result<(), Error> {
        Ok(())
    }

    fn backward(&self) -> Result<(), Error> {
        Ok(())
    }
}
