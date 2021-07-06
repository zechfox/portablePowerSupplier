use stm32f0xx_hal as mcu_hal;
use panic_halt as _;
use mcu_hal::{
    pac,
    rcc,
    prelude::*,
    serial::{Serial},
    gpio::{gpioa, Alternate, AF1},
    stm32,
};

use crate::common::{
    constants,
};

pub struct BoardBuilder {
    sysclk: u32,
}
pub struct Board {
    flash: pac::FLASH,
    rcc: rcc::Rcc,
    gpioa: gpioa::Parts,
}

impl BoardBuilder {
    pub fn new(sysclk: u32) -> Self {
        Self { sysclk }
    }
    pub fn take(self) -> Board {
        let p = pac::Peripherals::take().unwrap();
        let mut flash = p.FLASH;
        let mut rcc = p.RCC.configure().sysclk(self.sysclk.mhz()).freeze(&mut flash);
        let gpioa = p.GPIOA.split(&mut rcc);

        Board { flash:flash, rcc:rcc, gpioa:gpioa}
    }
}
