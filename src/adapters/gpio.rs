use stm32f0xx_hal as mcu_hal;
use mcu_hal::{
    pac,
    prelude::*,
    gpio::gpiob::{PB12, PB13},
    gpio::{OpenDrain, Output, Input, PullUp, PullDown, Floating},
    stm32,
};
use concat_idents::concat_idents;

use crate::common::{
    constants,
};
trait OutputPin {
    fn set_high(&self);
    fn set_low(&self);
}
trait InputPin {
    fn is_high(&self) -> bool;
    fn is_low(&self) -> bool;
}

pub struct GpioBuilder {
}

impl GpioBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

macro_rules! prepare_gpio {
    ($GPIOX:ident, $pxi:ident, $PXi:ident, $direction:ident, $mode:ident) => {
        concat_idents!(struct_name = "Gpio", $PXi {
            pub struct struct_name {
                gpio: $PXi<$direction<$mode>>,
            }
        });
    };
    (any, $pxi:ident, $PXi:ident, Output, any) => {
        concat_idents!(struct_name = "Gpio", $PXi {
            impl OutputPin for struct_name {
                fn set_high(&self) {
                    self.gpio.set_high();
                }
                fn set_low(&self) {
                    self.gpio.set_low();
                }
            }
        });
    };
    (any, $pxi:ident, $PXi:ident, Input, any) => {
        concat_idents!(struct_name = "Gpio", $PXi {
            impl InputPin for struct_name {
                fn is_high(&self) {
                    self.gpio.is_high().unwrap()
                }
                fn is_low(&self) {
                    self.gpio.is_low().unwrap();
                }
            }
        });
    };
    ($GPIOX:ident, $pxi:ident, $PXi:ident, Output, OpenDrain) => {
        concat_idents!(fn_name = "take_gpio_", $pxi {
            impl GpioBuilder {
                concat_idents!(struct_name = "Gpio", $PXi {
                    pub fn fn_name() -> struct_name {
                        let mut p = pac::Peripherals::take().unwrap();
                        let mut rcc = p.RCC.configure().sysclk(constants::SYSTEM_CLOCK.mhz()).freeze(&mut p.FLASH);
                        let gpio = p.$GPIOX.split(&mut rcc);
                        let mut pin = cortex_m::interrupt::free(move |cs| gpio.$pxi.into_open_drain_output(cs));

                        struct_name {gpio:pin}
                    }
                    impl OutputPin for struct_name {
                        fn set_high(&self) {
                            self.gpio.set_high();
                        }
                        fn set_low(&self) {
                            self.gpio.set_low();
                        }
                    }
                });
            }
        });
    };
    ($GPIOX:ident, $pxi:ident, $PXi:ident, Output, PushPull) => {
        concat_idents!(fn_name = "take_gpio_", $pxi {
            impl GpioBuilder {
                concat_idents!(struct_name = "Gpio", $PXi {
                    pub fn fn_name() -> struct_name {
                        let mut p = pac::Peripherals::take().unwrap();
                        let mut rcc = p.RCC.configure().sysclk(constants::SYSTEM_CLOCK.mhz()).freeze(&mut p.FLASH);
                        let gpio = p.$GPIOX.split(&mut rcc);
                        let mut pin = cortex_m::interrupt::free(move |cs| gpio.$pxi.into_push_pull_output(cs));

                        struct_name {gpio:pin}
                    }
                });
            }
        });
    };
    ($GPIOX:ident, $pxi:ident, $PXi:ident, Input, PullUp) => {
        concat_idents!(fn_name = "take_gpio_", $pxi {
            impl GpioBuilder {
                concat_idents!(struct_name = "Gpio", $PXi {
                    pub fn fn_name() -> struct_name {
                        let mut p = pac::Peripherals::take().unwrap();
                        let mut rcc = p.RCC.configure().sysclk(constants::SYSTEM_CLOCK.mhz()).freeze(&mut p.FLASH);
                        let gpio = p.$GPIOX.split(&mut rcc);
                        let mut pin = cortex_m::interrupt::free(move |cs| gpio.$pxi.into_pull_up_input(cs));

                        struct_name {gpio:pin}
                    }
                });
            }
        });
    };
    ($GPIOX:ident, $pxi:ident, $PXi:ident, Input, PullDown) => {
        concat_idents!(fn_name = "take_gpio_", $pxi {
            impl GpioBuilder {
                concat_idents!(struct_name = "Gpio", $PXi {
                    pub fn fn_name() -> struct_name {
                        let mut p = pac::Peripherals::take().unwrap();
                        let mut rcc = p.RCC.configure().sysclk(constants::SYSTEM_CLOCK.mhz()).freeze(&mut p.FLASH);
                        let gpio = p.$GPIOX.split(&mut rcc);
                        let mut pin = cortex_m::interrupt::free(move |cs| gpio.$pxi.into_pull_down_input(cs));

                        struct_name {gpio:pin}
                    }
                });
            }
        });
    };
    ($GPIOX:ident, $pxi:ident, $PXi:ident, Input, Floating) => {
        concat_idents!(fn_name = "take_gpio_", $pxi {
            impl GpioBuilder {
                concat_idents!(struct_name = "Gpio", $PXi {
                    pub fn fn_name() -> struct_name {
                        let mut p = pac::Peripherals::take().unwrap();
                        let mut rcc = p.RCC.configure().sysclk(constants::SYSTEM_CLOCK.mhz()).freeze(&mut p.FLASH);
                        let gpio = p.$GPIOX.split(&mut rcc);
                        let mut pin = cortex_m::interrupt::free(move |cs| gpio.$pxi.into_floating_input(cs));

                        struct_name {gpio:pin}
                    }
                });
            }
        });
    };
}
/// GPIOs used in board.
prepare_gpio!(GPIOB, pb12, PB12, Output, OpenDrain);
prepare_gpio!(GPIOB, pb13, PB13, Input, PullDown);
