use crate::{
    utils::logger::Logger,
    board::gpio::{GpioPB12, GpioPB13, GpioPB14, GpioPB15, GpioPA8},
    board::gpio::GpioBuilder,
};

pub struct Lcd {
    lcd_sda: GpioPB12,
    lcd_sclk: GpioPB13,
    lcd_rs: GpioPB14,
    lcd_cs: GpioPB15,
    lcd_bg: GpioPA8,
}

impl Lcd {
    pub fn new() -> Self {
        let sda = GpioBuilder::new().take_gpio_pb12();
        let sclk = GpioBuilder::new().take_gpio_pb13();
        let rs = GpioBuilder::new().take_gpio_pb14();
        let cs = GpioBuilder::new().take_gpio_pb15();
        let bg = GpioBuilder::new().take_gpio_pa8();

        Self { 
            lcd_sda: sda,
            lcd_sclk: sclk,
            lcd_rs: rs,
            lcd_cs: cs,
            lcd_bg: bg,
        }
    }

    fn write_byte(&mut self, byte: u8) {
        self.lcd_cs.set_low();

        self.lcd_cs.set_high();
    }

    fn write_cmd(&self, cmd: u8) {
    }

    fn write_data(&self, data: u8) {
    }

    pub fn set_cursor(&self, x: u8, y: u8) {
    }

    pub fn display_block(&self, block:&[u8] ) {
    }
}
