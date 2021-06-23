use stm32f0xx_hal as mcu_hal;
use mcu_hal::{
    pac,
    prelude::*,
    gpio::gpiob::{PB12, PB13, PB14, PB15},
    gpio::gpioa::{PA8},
    gpio::{OpenDrain, Output, Input, PullUp, PullDown, Floating},
    stm32,
};
use paste::paste;
use crate::common::{
    constants,
};

pub struct GpioBuilder {
}

impl GpioBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

macro_rules! prepare_gpio {

    ($GPIOX:ident, $pxi:ident, $PXi:ident, Output, $mode:ident) => {
        paste! {
            pub struct [<Gpio $PXi>] {
                gpio: $PXi<Output<$mode>>,
            }

            impl [<Gpio $PXi>] {
                pub fn set_high(&mut self) {
                    self.gpio.set_high().ok();
                }
                pub fn set_low(&mut self) {
                    self.gpio.set_low().ok();
                }
            }

            impl GpioBuilder {
                    pub fn [<take_gpio_ $pxi>](self) -> [<Gpio $PXi>] {
                        let mut p = pac::Peripherals::take().unwrap();
                        let mut rcc = p.RCC.configure().freeze(&mut p.FLASH);
                        let gpio = p.$GPIOX.split(&mut rcc);
                        let pin = cortex_m::interrupt::free(move |cs| gpio.$pxi.[<into_ $mode:snake _output>](cs));

                        [<Gpio $PXi>] {gpio:pin}
                    }
            }
        }
    };

    ($GPIOX:ident, $pxi:ident, $PXi:ident, Input, $mode:ident) => {
        paste! {
            pub struct [<Gpio $PXi>]{
                gpio: $PXi<Input<$mode>>,
            }

            impl [<Gpio $PXi>] {
                pub fn is_high(&self) -> bool {
                    self.gpio.is_high().unwrap()
                }
                pub fn is_low(&self) -> bool {
                    self.gpio.is_low().unwrap()
                }
            }
            impl GpioBuilder {
                    pub fn [<take_gpio_ $pxi>](self) -> [<Gpio $PXi>] {
                        let mut p = pac::Peripherals::take().unwrap();
                        let mut rcc = p.RCC.configure().freeze(&mut p.FLASH);
                        let gpio = p.$GPIOX.split(&mut rcc);
                        let pin = cortex_m::interrupt::free(move |cs| gpio.$pxi.[<into_ $mode:snake _input>](cs));

                        [<Gpio $PXi>]{gpio:pin}
                    }
            }
        }
    };

}
/// GPIOs used in board.
prepare_gpio!(GPIOB, pb12, PB12, Output, OpenDrain);
prepare_gpio!(GPIOB, pb13, PB13, Output, OpenDrain);
prepare_gpio!(GPIOB, pb14, PB14, Output, OpenDrain);
prepare_gpio!(GPIOB, pb15, PB15, Output, OpenDrain);
prepare_gpio!(GPIOA,  pa8,  PA8, Output, OpenDrain);
