// Type State Pattern: Implement compile-time state transitions
struct Connected;
struct Disconnected;

struct Connection<S> {
    host: Option<String>,
    _state: std::marker::PhantomData<S>
}

impl Connection<Disconnected> {
    fn new() -> Self {
        Connection {
            host: None,
            _state: std::marker::PhantomData
        }
    }

    fn connect(self, host: &str) -> Connection<Connected> {
        Connection {
            host: Some(host.to_string()),
            _state: std::marker::PhantomData
        }
    }
}

impl Connection<Connected> {
    fn send(&self, data: &str) {
        println!("Sending {} to {}", data, self.host.as_ref().unwrap());
    }

    fn disconnect(self)-> Connection<Disconnected> {
        Connection {
            host: None,
            _state: std::marker::PhantomData
        }
    }
}

fn main() {
    let conn = Connection::new();
    let conn = conn.connect("eample.com");
    conn.send("Hello");
    let _conn = conn.disconnect();
    // conn.send("Sending after disconnect!!")    
}
