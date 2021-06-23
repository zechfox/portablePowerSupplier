use embedded_hal::blocking::delay::DelayUs;
use cortex_m::peripheral::Peripherals;
use num_derive::ToPrimitive;
use num_traits::ToPrimitive;
use stm32f0xx_hal as mcu_hal;
use mcu_hal::{
    pac,
    prelude::*,
    delay::Delay,
    stm32,
};

#[cfg(feature = "graphics")]
use embedded_graphics;

#[cfg(feature = "graphics")]
use self::embedded_graphics::{
    geometry::Point,
    drawable::Pixel,
    pixelcolor::BinaryColor,
    prelude::*,
    DrawTarget,
};

use crate::{
    utils::logger::Logger,
    board::gpio::{GpioPB12, GpioPB13, GpioPB14, GpioPB15, GpioPA8},
    board::gpio::GpioBuilder,
    common::constants,
};

pub const WIDTH_IN_BITS: u8 = 132;
pub const HEIGHT_IN_BITS: u8 = 64;
const BLOCK_WIDTH_IN_BITS: u8 = 8;
const BLOCK_HEIGHT_IN_BITS: u8 = 8;
const PAGE_SIZE:u8 = (HEIGHT_IN_BITS as u8 / BLOCK_HEIGHT_IN_BITS) as u8;

// lcd display buffer:
// 0 1  2  3  ... 131
// 1 u8 u8 u8 ... u8
// 2 u8 u8 u8 ... u8
// 3 u8 u8 u8 ... u8
// 4 u8 u8 u8 ... u8
// 5 u8 u8 u8 ... u8
// 6 u8 u8 u8 ... u8
// 7 u8 u8 u8 ... u8
const BUFFER_IN_BYTES: usize = (PAGE_SIZE as u32 * WIDTH_IN_BITS as u32) as usize;
const ROWS_PER_PAGE: u8 = 8;

pub struct Lcd {
    lcd_sda: GpioPB12,
    lcd_sclk: GpioPB13,
    lcd_rs: GpioPB14,
    lcd_cs: GpioPB15,
    lcd_bg: GpioPA8,
    lcd_buffer: [u8; BUFFER_IN_BYTES],
    lcd_delay: Delay,
}

pub enum Error<CommError, PinError> {
    Comm(CommError),
    Pin(PinError),
}

#[derive(ToPrimitive)]
enum Instruction {
    DisplayOn = 0xAF,
    SetStartLine = 0x40,
    SetPageAddress = 0xB0,
    SetColumnAddressMSB = 0x10,
    SetColumnAddressLSB = 0x00,
    NormalSegDirection = 0xA0,
    ReverseSegDirection = 0xA1,
    NormalDisplay = 0xA6,
    InverseDisplay = 0xA7,
    Bias_1_9 =  0xA2,
    PowerControl = 0x28,
    SwReset = 0xE2,
    SetEv1 = 0x81,
    SetEv2 = 0x2C,
    NormalComDirection = 0xC8,
    RegulationRatio = 0x23,
}



impl Lcd {
    pub fn new() -> Self {

        log!("New Lcd !\n");
        let sda = GpioBuilder::new().take_gpio_pb12();
        log!("LCD take SDA pin\n");
        let sclk = GpioBuilder::new().take_gpio_pb13();
        let rs = GpioBuilder::new().take_gpio_pb14();
        let cs = GpioBuilder::new().take_gpio_pb15();
        let bg = GpioBuilder::new().take_gpio_pa8();
        let buffer = [0; BUFFER_IN_BYTES];

        let mut p = pac::Peripherals::take().unwrap();
        let mut cp = cortex_m::Peripherals::take().unwrap();
        let mut rcc = p.RCC.configure().freeze(&mut p.FLASH);
        let mut delay = Delay::new(cp.SYST, &rcc);

        Self { 
            lcd_sda: sda,
            lcd_sclk: sclk,
            lcd_rs: rs,
            lcd_cs: cs,
            lcd_bg: bg,
            lcd_buffer: buffer,
            lcd_delay: delay,
        }
    }
    pub fn init(&mut self) {


        self.lcd_rs.set_low();
        self.lcd_cs.set_high();

        // sw reset
        self.write_cmd(Instruction::SwReset);
        // 1/9 bias
        self.write_cmd(Instruction::Bias_1_9);
        // display on
        self.write_cmd(Instruction::DisplayOn);
        // SEG direction
        self.write_cmd(Instruction::NormalSegDirection);
        // COM direction
        self.write_cmd(Instruction::NormalComDirection);
        // regulation
        self.write_cmd(Instruction::RegulationRatio);
        // EV.1
        self.write_cmd(Instruction::SetEv1);
        // EV.2
        self.write_cmd(Instruction::SetEv2);
        // power control
        self.write_cmd_param(Instruction::PowerControl, 0x7);

        // start line
        self.write_cmd(Instruction::SetStartLine);
        self.write_cmd(Instruction::SetPageAddress);
        self.write_cmd(Instruction::SetColumnAddressMSB);
        self.write_cmd(Instruction::SetColumnAddressLSB);
        // turn on display
        self.write_cmd(Instruction::DisplayOn);

        self.clear_screen();

    }
    fn lcd_tick(&mut self) {
        self.lcd_delay.delay_us(1_u8);
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
            self.lcd_tick();
            index += 1;
        }

        self.lcd_cs.set_high();
    }

    fn clear_screen(&mut self) {
        let mut page_index = 0;
        while page_index < PAGE_SIZE {
            let mut col_index = 0;
            self.write_cmd_param(Instruction::SetPageAddress, page_index);
            self.write_cmd(Instruction::SetColumnAddressMSB);
            self.write_cmd(Instruction::SetColumnAddressLSB);
            while col_index < WIDTH_IN_BITS {
                self.write_data(0x00);
                col_index += 1;
            }
            page_index += 1;
        }
    }

    fn write_cmd(&mut self, cmd: Instruction) {
        self.lcd_rs.set_low();
        self.write_byte(cmd.to_u8().unwrap());
    }

    fn write_cmd_param(&mut self, cmd: Instruction, param: u8) {
        let command_param = cmd.to_u8().unwrap() | param;
        self.lcd_rs.set_low();
        self.write_byte(command_param);
    }

    fn write_data(&mut self, data: u8) {
        self.lcd_rs.set_high();
        self.write_byte(data);
    }

    fn set_address(&mut self, x: u8, y: u8) {
        self.write_cmd_param(Instruction::SetPageAddress, y / BLOCK_HEIGHT_IN_BITS);
        self.write_cmd_param(Instruction::SetColumnAddressMSB, ((x >> 4) & 0x0f));
        self.write_cmd_param(Instruction::SetColumnAddressLSB, (x & 0x0f));
    }

    /// Draw pixel
    pub fn set_pixel(&mut self, mut x: u8, mut y: u8, val: u8) {
        let pixel_mask = 0x80 >> (y % 8);
        let start_page = WIDTH_IN_BITS * y / BLOCK_HEIGHT_IN_BITS;
        if val != 0 {
            self.lcd_buffer[x as usize + start_page as usize] |= pixel_mask;
        } else {
            self.lcd_buffer[x as usize + start_page as usize] &= !pixel_mask;
        }
    }
    // x: 0~63, y: 0~131
    pub fn update_display_buffer(&self, x: u8, y: u8, block:&[u8] ) {
    }

    /// Flush buffer to update entire display
    pub fn flush(&mut self) {
        self.lcd_cs.set_low();

        for page_index in 0..PAGE_SIZE {
            self.write_cmd_param(Instruction::SetPageAddress, page_index);
            self.write_cmd_param(Instruction::SetColumnAddressMSB, 0);
            self.write_cmd_param(Instruction::SetColumnAddressLSB, 0);

            let page_start = page_index * WIDTH_IN_BITS;
            for column_index in 0..WIDTH_IN_BITS {
                self.write_byte(self.lcd_buffer[page_start as usize + column_index as usize]);
            }

        }

        self.lcd_cs.set_high();
    }

    /// Flush buffer to update region of the display
    pub fn flush_region(
        &mut self,
        x: u8,
        y: u8,
        w: u8,
        h: u8,
    ) {
        self.lcd_cs.set_low();

        let start_page = y / BLOCK_HEIGHT_IN_BITS; 
        let end_page = (y + h) / BLOCK_HEIGHT_IN_BITS;

        for page_index in start_page..end_page {
            self.write_cmd_param(Instruction::SetPageAddress, page_index);
            self.write_cmd_param(Instruction::SetColumnAddressMSB, ((x >> 4) & 0x0f));
            self.write_cmd_param(Instruction::SetColumnAddressLSB, (x & 0x0f));

            let page_start = page_index * WIDTH_IN_BITS;
            for column_index in x..(x + w) {
                self.write_byte(self.lcd_buffer[page_start as usize + column_index as usize]);
            }
        }

        self.lcd_cs.set_high();
    }

    fn flush_display_buffer(x: u8, y: u8) {

    }
}

#[cfg(feature = "graphics")]
impl DrawTarget<BinaryColor> for Lcd {
    type Error = core::convert::Infallible;

    /// Draw a `Pixel` that has a color defined as `Gray8`.
    fn draw_pixel(&mut self, pixel: Pixel<BinaryColor>) -> Result<(), Self::Error> {
        let Pixel(coord, color) = pixel;
        let x = coord.x as u8;
        let y = coord.y as u8;
        let c = match color { BinaryColor::Off => 0 , BinaryColor::On => 1 };
        self.set_pixel(x, y, c);
        Ok(())
    }

    fn size(&self) -> Size {
        Size::new(WIDTH_IN_BITS as u32, HEIGHT_IN_BITS as u32)
    }
}
