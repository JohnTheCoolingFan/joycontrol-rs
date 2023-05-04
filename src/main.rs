use log::info;
use log4rs::init_file;

mod button_state;
mod cli;
mod controller;
mod controller_state;
mod device;
mod mcu;
mod memory;
mod nfc_tag;
mod protocol;
mod stick_calibration;
mod stick_state;

fn main() {
    init_file("log_config.yaml", Default::default()).unwrap();
    info!("Starting up!");
    println!("Hello, world!");
}
