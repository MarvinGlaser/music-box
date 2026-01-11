use std::io::Error;



struct AudioManager {

}


trait Manager {
    fn set_colume(level: i32) -> Result<(), Error>;
    fn play() -> Result<(), Error>;
    fn pause() -> Result<(), Error>;
    fn forward() -> Result<(), Error>;
    fn backward() -> Result<(), Error>;
}


impl Manager for AudioManager {
    fn set_colume(level: i32) -> Result<(), Error> {
        todo!()
    }

    fn play() -> Result<(), Error> {
        todo!()
    }

    fn pause() -> Result<(), Error> {
        todo!()
    }

    fn forward() -> Result<(), Error> {
        todo!()
    }

    fn backward() -> Result<(), Error> {
        todo!()
}
