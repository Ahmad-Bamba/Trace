use globals;
use wiringpi::*;

use std::mem;
use std::thread;
use std::time::Duration;
use std::cmp;
use std::cmp::Ordering;

const MOTOR_DELAY: u64 = 200;
const RIGHT_PWM_PIN: u16 = 14;
const RIGHT_1_PIN: u16  = 10;
const RIGHT_2_PIN: u16 = 25;
const LEFT_PWM_PIN: u16 = 24;
const LEFT_1_PIN: u16 = 17;
const LEFT_2_PIN: u16 = 4;
const SW1_PIN: i8 = 11;
const SW2_PIN: i8 = 9;
const LED1_PIN: i8 = 8;
const LED2_PIN: i8 = 7;
const OC1_PIN: i8 = 22;
const OC2_PIN: i8 = 27;
const OC2_PIN_R1: i8 = 21;
const OC2_PIN_R2: i8 = 27;
const TRIGGER_PIN: i8 = 18;
const ECHO_PIN: i8 = 23;

struct RaspiRobot {
    left_pwm: pin::SoftPwmPin<pin::Gpio>,
    right_pwm: pin::SoftPwmPin<pin::Gpio>,
    out_left_one: pin::OutputPin<pin::Gpio>,
    out_left_two: pin::OutputPin<pin::Gpio>,
    out_right_one: pin::OutputPin<pin::Gpio>,
    out_right_two: pin::OutputPin<pin::Gpio>,
    pwm_scale: f32,
    revision: i8,
    old_left_dir: bool,
    old_right_dir: bool,
}

//TODO implement IR handling
impl RaspiRobot {
    fn new(battery_voltage: f32, motor_voltage: f32, rev: i8) -> RaspiRobot {
        assert!(battery_voltage > motor_voltage);

        let mut robot = RaspiRobot {
            left_pwm: globals::PI.soft_pwm_pin(LEFT_PWM_PIN),
            right_pwm: globals::PI.soft_pwm_pin(RIGHT_PWM_PIN),
            out_left_one: globals::PI.output_pin(LEFT_1_PIN),
            out_left_two: globals::PI.output_pin(LEFT_2_PIN),
            out_right_one: globals::PI.output_pin(RIGHT_1_PIN),
            out_right_two: globals::PI.output_pin(RIGHT_2_PIN),
            pwm_scale: motor_voltage / battery_voltage,
            revision: rev,
            old_left_dir: false,
            old_right_dir: false,
        };

        robot.left_pwm.pwm_write(0);
        robot.right_pwm.pwm_write(0);
        robot.out_left_one.digital_write(pin::Value::Low);
        robot.out_right_one.digital_write(pin::Value::Low);
        robot.out_left_two.digital_write(pin::Value::Low);
        robot.out_right_two.digital_write(pin::Value::Low);

        robot
    }

    /// expects value from 0 to 1
    fn set_driver_pins(&mut self, left: f32, left_dir: bool, right: f32, right_dir: bool) {
        assert!(left <= 1f32 && left >= 0f32);
        assert!(right <= 1f32 && right >= 0f32);

        self.left_pwm.pwm_write((left * 100f32 * self.pwm_scale) as i32);
        self.out_left_one.digital_write(if left_dir { pin::Value::High } else { pin::Value::Low });
        self.out_left_two.digital_write(if !left_dir { pin::Value::High } else { pin::Value::Low });

        self.right_pwm.pwm_write((right * 100f32 * self.pwm_scale) as i32);
        self.out_right_one.digital_write(if right_dir { pin::Value::High } else { pin::Value::Low });
        self.out_right_two.digital_write(if !right_dir { pin::Value::High } else { pin::Value::Low });
    }

    /// expects left and right from 0 to 1
    fn set_motors(&mut self, left: f32, left_dir: bool, right: f32, right_dir: bool) {
        assert!(left <= 1f32 && left >= 0f32);
        assert!(right <= 1f32 && right >= 0f32);

        // pause to prevent robot from switching directions too quickly
        if self.old_right_dir != left_dir || self.old_right_dir != right_dir {
            self.set_driver_pins(0f32, false, 0f32, false);
            thread::sleep(Duration::from_millis(MOTOR_DELAY));
        }

        self.set_driver_pins(left, left_dir, right, right_dir);
        self.old_right_dir = right_dir;
        self.old_left_dir = left_dir;
    }

    /// expects input from -1 to 1
    fn tank_drive(&mut self, left: f32, right: f32) {
        assert!(left <= 1f32 && left >= -1f32);
        assert!(right <= 1f32 && right >= -1f32);

        self.set_motors(left.abs(), if left > 0f32 { true } else { false }, right.abs(), if right > 0f32 { true } else { false });
    }

    fn arcade_drive(&mut self, throttle: f32, wheel: f32) {
        assert!(throttle >= -1f32 && throttle <= 1f32);
        assert!(wheel >= -1f32 && wheel <= 1f32);

        let mut left = 0f32;
        let mut right = 0f32;

        if throttle > 0f32 {
            if wheel > 0f32 {
                left = throttle - wheel;
                //right = cmp::max(throttle, wheel);
                right = match throttle.partial_cmp(&wheel).unwrap() {
                    Ordering::Less => wheel,
                    _ => throttle,
                };
            } else {
                //left = cmp::max(throttle, -wheel);
                left = match throttle.partial_cmp(&(-wheel)).unwrap() {
                    Ordering::Less => -wheel,
                    _ => throttle,
                };
                right = throttle + wheel;
            }
        } else {
            if wheel > 0f32 {
                //left = -cmp::max(-throttle, wheel);
                left = -match (-throttle).partial_cmp(&wheel).unwrap() {
                    Ordering::Less => wheel,
                    _ => -throttle,
                };
                right = throttle + wheel;
            } else {
                left = throttle - wheel;
                //right = -cmp::max(-throttle, -wheel);
                right = -match (-throttle).partial_cmp(&(-wheel)).unwrap() {
                    Ordering::Less => -wheel,
                    _ => -throttle,
                };
            }
        }
        self.tank_drive(left, right);
    }

    fn stop(&mut self) {
        self.set_motors(0f32, false, 0f32, false);
    }
}

impl Drop for RaspiRobot {
    fn drop(&mut self) {
        self.stop();
        globals::PI.soft_pwm_pin(LEFT_PWM_PIN).pwm_stop();
        globals::PI.soft_pwm_pin(RIGHT_PWM_PIN).pwm_stop();
    }
}
