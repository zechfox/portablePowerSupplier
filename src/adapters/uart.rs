#![deny(unsafe_code)]

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

use cortex_m::singleton;
use crate::common::{
    constants,
};
#[allow(dead_code)]
pub enum UartParity {
    Even,
    Odd,
    None,
}
#[allow(dead_code)]
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

pub struct UartBuilder {
    configuration: UartConfiguration,
}

pub struct Uart2 {
    serial: Serial<stm32::USART2, PA2<Alternate<AF1>>, PA3<Alternate<AF1>>>,
}

impl UartBuilder {
    pub fn new(conf: UartConfiguration) -> Self {
        Self {
            configuration: conf,
        }
    }
    
    pub fn take_uart2(self) -> &'static mut Uart2 {
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
            self.configuration.baud_rate.unwrap().bps(),
            &mut rcc,
        );

        singleton!(: Uart2 = Uart2{serial}).unwrap()
    }
}


impl Uart2 {
    pub fn write_byte(&mut self, b: u8) {
        block!(self.serial.write(b)).ok();
    }
}
