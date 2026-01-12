mod modules;

use zbus::{Connection, Result, proxy};

use crate::modules::audio_manager::audio_controller::AudioManager;
use crate::modules::audio_manager::audio_controller::Manager;


fn main() {
    println!("Hello");
    let audio_manager: AudioManager = AudioManager {};
    audio_manager.set_volume(33).unwrap();
    let connection = Connection::session();

   
    
}
