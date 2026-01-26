use zbus::{Connection, Error};

use crate::modules::audio_manager::audio_proxies::{PlayerProxy, MediaProxy};



pub struct AudioManager<'a> { 
    player_proxy: PlayerProxy<'a>,
    media_proxy: MediaProxy<'a>
}

impl<'a> AudioManager<'a> {
    pub async fn new(connection: &Connection
        , destination: Option<&'a str>
        , path: Option<&'a str>
        ) -> zbus::Result<AudioManager<'a>> {

        let mut player_builder = PlayerProxy::builder(connection);
        let mut media_builder = MediaProxy::builder(connection);

        if let Some(dest) = destination {
            player_builder = player_builder.destination(dest)?;
            media_builder = media_builder.destination(dest)?;
        }

        if let Some(path) = path {
            player_builder = player_builder.path(path)?;
            media_builder = media_builder.path(path)?;
        }

        let player_proxy = player_builder.build().await?;
        let media_proxy = media_builder.build().await?;

        Ok(AudioManager { player_proxy, media_proxy})
    }

}


pub trait Manager {
    async fn set_volume(&self, level: i32) -> Result<(), Error>;
    async fn play(&self) -> Result<(), Error>;
    async fn pause(&self) -> Result<(), Error>;
    async fn move_track_forward(&self, seconds: i64) -> Result<(), Error>;
    async fn move_track_backward(&self, seconds: i64) -> Result<(), Error>;
    async fn next_track(&self) -> Result<(), Error>;
    async fn previous_track(&self) -> Result<(), Error>;
}

// do these need to be async?
impl<'a> Manager for AudioManager<'a> {

    async fn set_volume(&self, level: i32) -> Result<(), Error> {
        if level > 40 {
            //Err(Error::other(format!("Audio too loude, could not set level to {}", level)))
            println!("Audio too loude, could not set level to {}", level);
        } else {
            println!("Set audio to level {level}");
        }
        Ok(())
    }

    // todo: unsure how to deal with play/pause
    async fn play(&self) -> Result<(), Error> {
        println!("Hello, I am playing a song!");
        Ok(())
    }

    async fn pause(&self) -> Result<(), Error> {
        println!("Aww, I have to pause now...");
        Ok(())
    }

    async fn move_track_forward(&self, seconds: i64) -> Result<(), Error> {
        self.player_proxy.seek(seconds * (10^6)).await?;
        println!("Moving the track forward {seconds} seconds");
        Ok(())
    }

    async fn move_track_backward(&self, seconds: i64) -> Result<(), Error> {
        self.player_proxy.seek(seconds * -1 * (10^6));
        println!("Moving the track backwards {seconds} seconds");
        Ok(())
    }

    async fn next_track(&self) -> Result<(), Error> {
        println!("Playing next track");
        Ok(())
    }

    async fn previous_track(&self) -> Result<(), Error> {
        println!("Playing previous track");
        Ok(())
    }

}
