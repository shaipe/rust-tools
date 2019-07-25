#[macro_use]
extern crate log;
extern crate log4rs;

fn main() {
    log4rs::init_file("log.yaml", Default::default()).unwrap();

    info!("booting up");
    error!("this is a test");
    // ...
}