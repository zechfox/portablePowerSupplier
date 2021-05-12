use core::{
    convert::Infallible,
    str,
};

use crate::{
    devices::transceiver::{Transceiver, SerialPort},
};

use ufmt_write::uWrite;

pub struct Logger; 
macro_rules! log {
    // NEW!
    ($($string:expr), +) => {
        ufmt::uwrite!(Logger, $($string)+).unwrap();
    };
}

impl uWrite for Logger {
    type Error = Infallible;
    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        unsafe {
            static mut SENDER: Option<Transceiver<SerialPort>> = None;
            if SENDER.is_none() {
                SENDER = Some(Transceiver::new());
            }
            SENDER.as_mut().unwrap().write_str(s);
        }
        Ok(())
    }
}
