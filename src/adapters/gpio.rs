


pub trait Output {
    fn set_high(&self) {
    }
    fn set_low(&self) {
    }
}
pub trait Input {
    fn is_high(&self) -> bool {
        false 
    }
    fn is_low(&self) -> bool {
        false
    }
}

pub struct GpioBPin12<Direction> {
    gpio: Direction,
}

pub struct GpioBPin13<Direction> {
    gpio: Direction,
}

pub struct GpioBPin14<Direction> {
    gpio: Direction,
}

pub struct GpioBPin15<Direction> {
    gpio: Direction,
}

pub struct GpioBuilder <Direction>;

impl GpioBuilder <Direction> {
    pub fn new() -> Self {
        Self
    }

    pub fn take_gpio_b_12() -> GpioBPin12<Direction> {
    }
}
