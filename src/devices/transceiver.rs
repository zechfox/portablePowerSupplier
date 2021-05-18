use crate::{
    board::uart::{Uart2, UartBuilder, UartConfiguration, UartParity, UartStopBits},
    common::constants::{BAUD_RATE_115200},
};


pub trait Device {
    fn write_byte(&mut self, b:u8);
    fn write_str(&mut self, s: &str);
}

pub struct SerialPort {
    peripheral: Uart2,
}

impl Device for SerialPort {
    fn write_byte(&mut self, b:u8) {
        self.peripheral.write_byte(b);
    }
    fn write_str(&mut self, s: &str) {
        self.peripheral.write_str(s);
    }
}

pub struct Transceiver<DeviceType: Device> {
    device: DeviceType,
}

/// This function may be used on Uart transceiver
impl Transceiver<SerialPort> {
    pub fn new() -> Self {
        let uart_conf = UartConfiguration {
        baud_rate: Some(BAUD_RATE_115200),
        parity: Some(UartParity::None),
        stop_bits: Some(UartStopBits::Stop1),
        };

        let uart = UartBuilder::new(uart_conf).take_uart2();

        Transceiver { device: SerialPort {peripheral:uart} }
    }
}

impl<DeviceType> Transceiver<DeviceType>
where
    DeviceType: Device,
{
    #[allow(dead_code)]
    pub fn write_byte(&mut self, b: u8) {
        self.device.write_byte(b); 
    }
    pub fn write_str(&mut self, s: &str) {
        self.device.write_str(s);
    }
}
