const MOTOR_DELAY: f32 = 0.2;
const RIGHT_PWM_PIN: i8 = 14;
const RIGHT_1_PIN: i8  = 10;
const RIGHT_2_PIN: i8 = 25;
const LEFT_PWM_PIN: i8 = 24;
const LEFT_1_PIN: i8 = 17;
const LEFT_2_PIN: i8 = 4;
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
    left_pwm: i8,
    right_pwm: i8,
    pwm_scale: f32,
    old_left_dir: i8,
    old_right_dir: i8,
}

impl RaspiRobot {
    fn new(battery_voltage: f32, motor_voltage: f32) -> RaspiRobot {
        RaspiRobot {
            left_pwm: 0,
            right_pwm: 0,
            pwm_scale: motor_voltage / battery_voltage,
            old_left_dir: -1,
            old_right_dir: -1,
        }
    }
}
