use zbus::interface;

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
        format!("Hello {}! I have been called {} times.", name, self.count)
    }
}


