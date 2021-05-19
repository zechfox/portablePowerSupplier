// std and main are not available for bare metal software
#![no_std]
#![no_main]

#[macro_use] mod utils;
mod services;
#[macro_use] mod board;
mod devices;
mod common;

use cortex_m_rt::entry; // The runtime
use panic_halt as _; // When a panic occurs, stop the microcontroller
use crate::{
    utils::logger::Logger,
    services::display::display_init,
};
// This marks the entrypoint of our application. The cortex_m_rt creates some
// startup code before this, but we don't need to worry about this
#[entry]
fn main() -> ! {
    log!("Hello world!\n");
    log!("dadada {}", 0i32);
    display_init();
    // Now, enjoy it
    loop {
    
    }
}
