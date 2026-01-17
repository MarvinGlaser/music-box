mod modules;

use std::{error::Error, future::pending};
use zbus::connection;
use modules::audio_manager::audio_state_machine::{AudioStates, AudioEvent, Off, Playing, Paused};

use crate::modules::audio_manager::audio_controller::AudioManager;
//use crate::modules::audio_manager::audio_controller::Manager;
use crate::modules::audio_manager::dbus_service::Greeter;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
//fn main() {
    println!("Hello");
    //let audio_manager: AudioManager = AudioManager {};
    //audio_manager.set_volume(33).unwrap();
    //let connection = Connection::session();

    //let greeter = Greeter::new(0);
    //let _conn = connection::Builder::session()?
    //    .name("org.zbus.MyGreeter")?
    //    .serve_at("/org/zbus/MyGreeter", greeter)?
    //    .build()
    //    .await?;
    //pending::<()>().await;
    
    let manager: AudioManager = AudioManager {};
    let mut state: AudioStates = AudioStates::Off(Off::new(manager));
    state.print_state();
    state = state.on_event(AudioEvent::PressPlay);
    state.print_state();

    Ok(())
    
}
