use embedded_graphics::{
    prelude::*,
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    primitives::Circle,
    style::{PrimitiveStyle, TextStyle},
};

use crate::{
    utils::logger::Logger,
    devices::lcd::Lcd,
};

pub fn display_init() {
    log!("Display Start to Init!\n");
    let mut lcd_device = Lcd::new();
    log!("Display Init!\n");
    let c = Circle::new(Point::new(20, 20), 8).into_styled(PrimitiveStyle::with_fill(BinaryColor::On));
    let t = Text::new("Hello Rust!", Point::new(40, 16))
        .into_styled(TextStyle::new(Font6x8, BinaryColor::On));
    c.draw(&mut lcd_device).unwrap();
    t.draw(&mut lcd_device).unwrap();

    log!("Display Init Done!\n");
    
}

