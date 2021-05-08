// std and main are not available for bare metal software
#![no_std]
#![no_main]

mod services;
mod adapters;
mod devices;
mod common;

use cortex_m_rt::entry; // The runtime
use panic_halt as _; // When a panic occurs, stop the microcontroller
use crate::{
    devices::transceiver::{Transceiver, SerialPort},
};
// This marks the entrypoint of our application. The cortex_m_rt creates some
// startup code before this, but we don't need to worry about this
#[entry]
fn main() -> ! {
    let mut tr: Transceiver<SerialPort> = Transceiver::new();
    let sent = b'Y';
    // Now, enjoy it
    loop {
       tr.write_byte(sent); 
    }
}
