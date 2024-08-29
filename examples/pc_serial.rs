use std::time::Duration;

use serialport::{self, available_ports};
use sim_modem::{sim7600::SIM7600, SimModem};

fn main() {
    colog::init();

    let _ports = available_ports().expect("No ports found");

    let mut uart = serialport::new("/dev/ttyACM0", 115200)
        .timeout(Duration::from_millis(500))
        .open()
        .expect("Failed to open port /dev/ttyACM0");

    // uart.clear(ClearBuffer::All).unwrap();
    let mut sim_modem = SIM7600::new();
    let mut buff = [0u8; 256];

    match sim_modem.negotiate(&mut uart, &mut buff) {
        Err(x) => log::error!("Error = {}", x),
        Ok(_x) => log::info!("Success"),
    }
}
