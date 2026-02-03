mod modules;

use std::{error::Error, future::pending};
use zbus::{Connection, connection};
use modules::audio_manager::audio_state_machine::{AudioState};
use modules::audio_manager::audio_state_machine::AudioEvent::{Play, Stop, VolUp, VolDown, Forward, Back, TrackForward, TrackBack};

use crate::modules::audio_manager::audio_controller::{AudioManager};
use crate::modules::audio_manager::dbus_scanner::DbusScanner;
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
        state = state.on_event(event).await?;
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
    let connection = Connection::session().await?;
    //test_state_machine(&connection).await?;
    //let res = test_audio_manager().await;
    //if let Err(err) = res { println!("Something terrible happend! Error: {err}") };
    let dbus_scanner = DbusScanner::new(&connection).await?;
    let target: String = String::from("org.mpris.MediaPlayer2.");
    let my_names = dbus_scanner.get_server_vec(&target).await?;
    if my_names.is_empty() {
        println!("The returned vector was emtpy!");
    } else {
        for name in my_names {
            println!("My vector item is {:#?}", name);
        }
    }


    Ok(())
}
