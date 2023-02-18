use log::info;

fn main() {
    ::std::env::set_var("RUST_LOG", "INFO");
    env_logger::init();

    info!("Hello, world!");
}
