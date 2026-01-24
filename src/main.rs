mod modules;

use std::{error::Error/*, future::pending*/};
//use zbus::connection;
use modules::audio_manager::audio_state_machine::{AudioState};
use modules::audio_manager::audio_state_machine::AudioEvent::{Play, Stop, VolUp, VolDown, Forward, Back, TrackForward, TrackBack};

use crate::modules::audio_manager::audio_controller::AudioManager;
//use crate::modules::audio_manager::audio_controller::Manager;
//use crate::modules::audio_manager::dbus_service::Greeter;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
//fn main() {
    //println!("Hello");
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
    
    let events = vec![
        Play,
        Play,
        VolUp,
        VolUp,
        VolUp,
        VolDown,
        VolDown,
        VolDown,
        Forward,
        Stop,
        Play,
        Back,
        TrackForward,
        TrackBack,
    ];

    println!("Initializing AudioManager");
    let manager: AudioManager = AudioManager {};
    println!("Building intial AudioState");
    let mut state: AudioState = AudioState::off(manager);
    for event in events {
        state.print_state();
        state = state.on_event(event);
    }

    Ok(())
    
}
