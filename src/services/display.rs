use crate::{
    utils::logger::Logger,
    devices::lcd::Lcd,
};

pub fn display_init() {

    let lcd_device = Lcd::new();

    log!("Display Init!\n");
}

