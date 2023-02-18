use std::process;
use std::thread::sleep;
use std::time::Duration;
use log::error;
use paho_mqtt::{Client, ConnectOptionsBuilder, Message};

fn main() {
    ::std::env::set_var("RUST_LOG", "INFO");
    env_logger::init();

    let addr = "localhost";
    let host = 1883_u16;
    let client = Client::new(format!("tcp://{}:{}", addr, host));
    let client = match client {
        Ok(c) => c,
        Err(e) => {
            error!("Can't create a new client: {}", e);
            process::exit(1);
        }
    };

    let opts = ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .finalize();
    if let Err(e) = client.connect(opts) {
        error!("Can't connect to broker: {}", e);
        process::exit(1);
    }

    for i in 0.. {
        let msg = Message::new("test", format!("Hi anjing {}", i),0 );
        if let Err(e) = client.publish(msg) {
            error!("Can't send message to broker: {}", e);
            break;
        }
        sleep(Duration::from_millis(500))
    }

    if let Err(e) = client.disconnect(None) {
        error!("Can't disconnect from broker: {}", e);
        process::exit(1);
    }
}
