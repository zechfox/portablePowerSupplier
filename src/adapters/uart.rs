#![deny(unsafe_code)]
#![no_main]
#![no_std]

use stm32f0xx_hal as mcu_hal;
use panic_halt as _;
use mcu_hal::{
    pac,
    prelude::*,
    serial::{Serial},
    gpio::{Alternate, AF1},
    gpio::gpioa::{PA2, PA3},
    stm32,
};
use nb::block;

use crate::common::{
    constants,
};
pub enum UartParity {
    Even,
    Odd,
    None,
}

pub enum UartStopBits {
    Stop1,
    Stop0P5,
    Stop2,
    Stop1P5,
}

pub struct UartConfiguration {
    pub baud_rate: Option<u32>,
    pub parity: Option<UartParity>,
    pub stop_bits: Option<UartStopBits>,
}

pub struct Uart2 {
    serial: Serial<stm32::USART2, PA2<Alternate<AF1>>, PA3<Alternate<AF1>>>,
}

impl Uart2 {
    pub fn new(uart_config: UartConfiguration) -> Self {
        let p = pac::Peripherals::take().unwrap();
        let mut flash = p.FLASH;
        let mut rcc = p.RCC.configure().sysclk(constants::SYSTEM_CLOCK.mhz()).freeze(&mut flash);

        let gpioa = p.GPIOA.split(&mut rcc);
        let (tx, rx) = cortex_m::interrupt::free(move |cs| {
            (
                gpioa.pa2.into_alternate_af1(cs),
                gpioa.pa3.into_alternate_af1(cs),
            )
        });
        let serial = Serial::usart2(
            p.USART2,
            (tx, rx),
            constants::BAUD_RATE_115200.bps(),
            &mut rcc,
        );

        Self { serial }
    }

    pub fn write_byte(&mut self, b: u8) {
        block!(self.serial.write(b)).ok();
    }
}
