use stm32f0xx_hal as mcu_hal;
use panic_halt as _;
use mcu_hal::{
    pac,
    rcc,
    FLASH,
    prelude::*,
    serial::{Serial},
    gpio::{Alternate, AF1},
    gpio::gpioa::{PA2, PA3},
    stm32,
};

use crate::common::{
    constants,
};

pub struct BoardBuilder {
    sysclk: u32,
}
pub struct Board {
    peripherals: pac::Peripherals,
    flash: FLASH,
    rcc: rcc::Rcc,
}

impl BoardBuilder {
    pub fn new(sysclk: u32) -> Self {
        Self { sysclk }
    }
    pub fn take(self) -> Board {
        let p = pac::Peripherals::take().unwrap();
        let mut flash = p.FLASH;
        let mut rcc = p.RCC.configure().sysclk(self.sysclk.mhz()).freeze(&mut flash);
        Board {p}
    }
}
