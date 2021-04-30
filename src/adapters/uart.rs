#![deny(unsafe_code)]
#![no_main]
#![no_std]

use stm32f1xx_hal as mcu_hal;
use panic_halt as _;
use mcu_hal::{
    pac,
    prelude::*,
    serial::{Config, Serial, StopBits},
    gpio::{Alternate, Floating, Input, PushPull},
    gpio::gpioa::{PA2, PA3},
    stm32,
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
    baud_rate: Option<u32>,
    parity: Option<UartParity>,
    stop_bits: Option<UartStopBits>,
}

pub struct Uart2 {
    serial: Serial<stm32::USART2, (PA2<Alternate<PushPull>>, PA3<Input<Floating>>)>,
}

impl Uart2 {
    fn new(self, uart_config: UartConfiguration) -> Self {
        let mut config = Config::default();
        let p = pac::Peripherals::take().unwrap();
        let mut flash = p.FLASH.constrain();
        let mut rcc = p.RCC.constrain();
        let clocks = rcc.cfgr.freeze(&mut flash.acr);
        let mut afio = p.AFIO.constrain(&mut rcc.apb2);

        config = match uart_config.baud_rate {
            None => config.baudrate(115200.bps()),
            Some(i) => config.baudrate(i.bps()),
        };
        config = match uart_config.parity {
            None => config.parity_none(),
            Some(i) => match i {
                UartParity::Even => config.parity_even(),
                UartParity::Odd  => config.parity_odd(),
                UartParity::None => config.parity_none(),
            }
        };
        config = match uart_config.stop_bits {
            None => config.stopbits(StopBits::STOP1),
            Some(stop_bits) => match stop_bits {
                UartStopBits::Stop1   => config.stopbits(StopBits::STOP1),
                UartStopBits::Stop0P5 => config.stopbits(StopBits::STOP0P5),
                UartStopBits::Stop2   => config.stopbits(StopBits::STOP2),
                UartStopBits::Stop1P5 => config.stopbits(StopBits::STOP1P5),
            },
        };
        let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
        let tx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
        let rx = gpioa.pa3;
        let serial = Serial::usart2(
            p.USART2,
            (tx, rx),
            &mut afio.mapr,
            config,
            clocks,
            &mut rcc.apb1,
        );

        Self { serial }
    }
}
