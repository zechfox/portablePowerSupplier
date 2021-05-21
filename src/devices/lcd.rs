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

fn lcd_tick() {
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
    pub fn init(&mut self) {
        self.lcd_rs.set_low();
        self.lcd_cs.set_high();

        // sw reset
        self.write_cmd(0xe2);
        // raise voltage 1
        self.write_cmd(0x2c);
        // raise voltage 2
        self.write_cmd(0x2e);
        // raise voltage 3
        self.write_cmd(0x2f);
        // 1/9 bias
        self.write_cmd(0xa2);
        // display on
        self.write_cmd(0xaf);
        // SEG direction
        self.write_cmd(0xa0);
        // COM direction
        self.write_cmd(0xc8);
        // regulation
        self.write_cmd(0x23);
        // EV.1
        self.write_cmd(0x81);
        // EV.2
        self.write_cmd(0x2c);
        // power control
        self.write_cmd(0x2f);

        // start line
        self.write_cmd(0x40);
        self.write_cmd(0xb0);
        self.write_cmd(0x10);
        self.write_cmd(0x00);
        self.write_cmd(0xaf);

        self.clear_screen();

    }

    fn write_byte(&mut self, byte: u8) {
        let mut index = 0;
        self.lcd_cs.set_low();
        while index < 8 {
            self.lcd_sclk.set_low();
            match (byte >> index) & 1u8 {
                0 => self.lcd_sda.set_low(),
                _ => self.lcd_sda.set_high(),
            }
            self.lcd_sclk.set_high();
            lcd_tick();
            index += 1;
        }

        self.lcd_cs.set_high();
    }

    fn clear_screen(&mut self) {
        let mut row_index = 0;
        while row_index < 8 {
            let mut col_index = 0;
            self.write_cmd(0xb0 | row_index);
            self.write_cmd(0x10);
            self.write_cmd(0x00);
            while col_index < 128 {
                self.write_data(0x00);
                col_index += 1;
            }
            row_index += 1;
        }
    }

    fn write_cmd(&mut self, cmd: u8) {
        self.lcd_rs.set_low();
        self.write_byte(cmd);
    }

    fn write_data(&mut self, data: u8) {
        self.lcd_rs.set_high();
        self.write_byte(data);
    }

    pub fn set_ddram_address(&self, x: u8, y: u8) {

    }

    pub fn display_block(&self, block:&[u8] ) {
    }
}
