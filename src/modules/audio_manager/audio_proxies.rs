use zbus::Result;


#[zbus::proxy(
    interface = "org.mpris.MediaPlayer2.Player",
    default_service = "org.mpris.MediaPlayer2.Player",
    default_path = "/org/mpris/MediaPlayer2/Player"
)]
pub trait Player {
    async fn play_pause(&self) -> Result<()>;
    async fn seek(&self, nano_seconds: i64) -> Result<()>;

    #[zbus(property)]
    fn volume(&self) -> Result<f64>;
    #[zbus(property)]
    fn set_volume(&self, volume: f64) -> Result<()>;
}


#[zbus::proxy(
    interface = "org.mpris.MediaPlayer2",
    default_service = "org.mpris.MediaPlayer2",
    default_path = "/org/mpris/MediaPlayer2"
)]
pub trait Media {
    async fn raise(&self) -> Result<()>;
    async fn quit(&self) -> Result<()>;
}


