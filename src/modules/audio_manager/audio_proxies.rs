use zbus::{proxy, Result};


#[proxy(
    interface = "org.mpris.MediaPlayer2.Player",
    default_service = "org.mpris.MediaPlayer2.Player",
    default_path = "/org/mpris/MediaPlayer2/Player"
)]
pub trait Player {
    async fn play_pause(&self) -> Result<()>;
    async fn seek(&self, nano_seconds: i64) -> Result<()>;
}


#[proxy(
    interface = "org.mpris.MediaPlayer2",
    default_service = "org.mpris.MediaPlayer2",
    default_path = "/org/mpris/MediaPlayer2"
)]
pub trait Media {
    async fn raise(&self) -> Result<()>;
    async fn quit(&self) -> Result<()>;
}


