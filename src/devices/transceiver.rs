use crate::{
    adapters::uart::{Uart2, UartBuilder, UartConfiguration, UartParity, UartStopBits},
};

pub trait Device {
    fn write_byte(&mut self, b:u8);
}

pub struct SerialPort {
    peripheral: &'static mut Uart2,
}

impl Device for SerialPort {
    fn write_byte(&mut self, b:u8) {
        self.peripheral.write_byte(b);
    }
}

pub struct Transceiver<T: Device> {
    device: T,
}

/// This function may be used on Uart transceiver
impl Transceiver<SerialPort> {
    pub fn new() -> Self {
        let uart_conf = UartConfiguration {
        baud_rate: Some(115200),
        parity: Some(UartParity::None),
        stop_bits: Some(UartStopBits::Stop1),
    };

    let uart = UartBuilder::new(uart_conf).take_uart2();

    Transceiver { device: SerialPort {peripheral:uart} }
    }
}

impl<T> Transceiver<T>
where
    T: Device,
{
    pub fn write_byte(&mut self, b: u8) {
       self.device.write_byte(b); 
    }
}
