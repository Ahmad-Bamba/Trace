extern crate wiringpi;
use wiringpi::*;
extern crate joy;
extern crate ctrlc;
#[macro_use]
extern crate lazy_static;

mod tracelib;
use tracelib::*;

use std::thread;
use std::time::Duration;

fn main() {
    /// blink rsl while running
    /// automatically ended when main thread exits
    thread::spawn(move || {
        let light = globals::PI.output_pin(tracelib::globals::RSL_PIN);
        loop {
            light.digital_write(pin::Value::High);
            thread::sleep(Duration::from_millis(600));
            light.digital_write(pin::Value::Low);
            thread::sleep(Duration::from_millis(600));
        }
    });

    let mut stick = joy::Device::open("/dev/input/js0\0".as_bytes()).unwrap();

    ctrlc::set_handler(move || {
        println!("message: Exited on user request");
        println!("Exiting...");
    }).expect("Error setting ctrl-c handler!");

    loop {
        for ev in &mut stick {
            match ev {
                joy::Event::Axis(a, p) => println!("Axis {}: {}",  a, p),
                joy::Event::Button(b, true) => println!("Button {} pressed", b),
                joy::Event::Button(b, false) => println!("Button {} released", b),
            }
        }
    }
}
