use std::{error::Error, future::pending};
use zbus::{connection, interface};


pub struct Greeter {
    count: u64
}

impl Greeter {
    pub fn new(counter: u64) -> Self {
        Greeter { count: counter }
    }
}

#[interface(name = "org.zbus.MyGreeter1")]
impl Greeter {
    fn say_hello(&mut self, name: &str) -> String {
        self.count += 1;
        format!("Hello{}! I have been called {} times.", name, self.count)
    }
}

//#[tokio::main]
//async fn main() -> Result<(), Box<dyn Error>> {
//    let greeter = Greeter { count: 0 };
//    let _conn = connection::Builder::session()?
//        .name("org.zbus.MyGreeter")?
//        .serve_at("/org/zbus/MyGreeter", greeter)?
//        .build()
//        .await?;
//    pending::<()>().await;
//
//    Ok(())
//}


