use std::process;
use std::time::Duration;
use log::{error, info};
use paho_mqtt::{Client, ConnectOptionsBuilder};

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

    match client.subscribe("test", 0) {
        Err(e) => {
            error!("Can't subscribe to broker: {}", e);
            process::exit(1);
        },
        Ok(response) => {
            info!("{}", response.subscribe_response().unwrap_or(-1));
        },
    };

    for msg in client.start_consuming() {
        if let Some(msg) = msg {
            info!("{}", msg);
        } else if !client.is_connected() {
            error!("Got empty message and got disconnected");
            break;
        }
    }

    client.stop_consuming();

    if let Err(e) = client.disconnect(None) {
        error!("Can't disconnect from broker: {}", e);
        process::exit(1);
    }
}
