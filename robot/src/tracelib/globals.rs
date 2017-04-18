use wiringpi::*;

lazy_static! {
    pub static ref PI: WiringPi<pin::Gpio> = setup_gpio();
}

pub const RSL_PIN: u16 = 8u16;
pub const DEADBAND: f32 = 0.15f32;
//pub const PWM_CLOCK: u16 = 50u16;
