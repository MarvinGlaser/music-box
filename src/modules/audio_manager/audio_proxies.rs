use zbus::{proxy, Result};


#[proxy(
    interface = "org.mpris.MediaPlayer2",
    default_service = "org.mpris.MediaPlayer2",
    default_path = "/org/mpris/MediaPlayer2"
)]
pub trait Player {
    async fn play_pause(&self) -> Result<()>;
}

