// std and main are not available for bare metal software
#![no_std]
#![no_main]

mod services;
mod adapters;
mod devices;
mod common;

use cortex_m_rt::entry; // The runtime
use embedded_hal::digital::v2::OutputPin; // the `set_high/low`function
use stm32f0xx_hal::{delay::Delay, pac, prelude::*}; // STM32F1 specific functions
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
    let uart_conf = UartConfiguration {
        baud_rate: Some(115200),
        parity: Some(UartParity::None),
        stop_bits: Some(UartStopBits::Stop1),
    };
    //let mut uart = Uart2::new(uart_conf);

    //let uart = UartBuilder::new(uart_conf).take_uart2();
    let mut tr: Transceiver<SerialPort> = Transceiver::new();
    let sent = b'Y';
    // Now, enjoy it
    loop {
       tr.write_byte(sent); 
    }
}
