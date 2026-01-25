mod modules;

use std::{error::Error, future::pending};
use zbus::{Connection, connection};
use modules::audio_manager::audio_state_machine::{AudioState};
use modules::audio_manager::audio_state_machine::AudioEvent::{Play, Stop, VolUp, VolDown, Forward, Back, TrackForward, TrackBack};

use crate::modules::audio_manager::audio_controller::{AudioManager, Manager};
use crate::modules::audio_manager::dbus_service::Greeter;

async fn test_state_machine(con: &Connection) -> zbus::Result<()>{
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
    let manager: AudioManager = AudioManager::new(con, None, None).await? ;
    println!("Building intial AudioState");
    let mut state: AudioState = AudioState::off(manager);
    for event in events {
        state.print_state();
        state = state.on_event(event);
    }
    Ok(())
}

async fn test_audio_manager() -> Result<(), zbus::Error> {

    let greeter = Greeter::new(0);
    let _conn = connection::Builder::session()?
        .name("org.zbus.MyGreeter")?
        .serve_at("/org/zbus/MyGreeter", greeter)?
        .build()
        .await?;
    pending::<()>().await;
    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connection = Connection::session();
    test_state_machine();
    let res = test_audio_manager().await;
    if let Err(err) = res { println!("Something terrible happend! Error: {err}") };

    Ok(())
}
