// std and main are not available for bare metal software
#![no_std]
#![no_main]

mod services;
mod adapters;
mod devices;
mod common;

use cortex_m_rt::entry; // The runtime
#[allow(unused_imports)]
use panic_halt; // When a panic occurs, stop the microcontroller
#[allow(unused_imports)]
use crate::{
    adapters::uart::{Uart2, UartBuilder, UartConfiguration, UartParity, UartStopBits},
};
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
