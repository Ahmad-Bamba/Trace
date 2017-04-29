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
use std::process;

fn pass() {
    thread::sleep(Duration::from_millis(0));
}

fn scale_deadband(x: f32) -> f32 {
    if x.abs() < globals::DEADBAND { 0f32 } else { x }
}

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
        globals::PI.output_pin(tracelib::globals::RSL_PIN).digital_write(pin::Value::Low);
        process::exit(0);
    }).expect("Error setting ctrl-c handler!");

    let mut stick1 = 0f32;
    let mut stick2 = 0f32;
    let mut stick3 = 0f32;
    let mut tank = false;
    let mut x = false;

    let mut trace = rrb3::RaspiRobot::new(9f32, 6f32, 2i8);

    //thread::sleep(Duration::from_secs(2));

    loop {
        for ev in &mut stick {
            match ev {
                joy::Event::Axis(a, pos) => {
                    // position from -32767 to 32767
                    match a {
                        1 => stick1 = {
                            let percent = pos as f32 / 32767 as f32;
                            //println!("Axis {}: {} {}",  a, pos, percent);
                            -percent
                        },
                        3 => stick3 = {
                            let percent = pos as f32 / 32767 as f32;
                            //println!("Axis {}: {} {}",  a, pos, percent);
                            -percent
                        },
                        4 => stick2 = {
                            let percent = pos as f32 / 32767 as f32;
                            //println!("Axis {}: {} {}",  a, pos, percent);
                            -percent
                        },
                        _ => pass(),
                    };
                },
                joy::Event::Button(b, state) => {
                    //if state { println!("Button {} pressed!", b); } else { println!("Button {} released!", b) }
                    match b {
                        1 => {
                            if state {
                                println!("Beginning playback from file: {}...", "output.txt");
                                trace.playback_from_file("output.txt", x);
                                println!("Done playing back from file!!")
                            }
                        },
                        2 => {
                            if state {
                                x = !x;
                                println!("Toggled playback reverse: {}", x);
                            }
                        },
                        7 => {
                            if state {
                                tank = !tank;
                                println!("Switched modes!");
                            }
                        },
                        _ => pass(),
                    };
                },
            }
        }
        if tank {
            trace.tank_drive(scale_deadband(stick1), scale_deadband(stick2));
        } else {
            trace.arcade_drive(scale_deadband(stick1), scale_deadband(stick3));
        }

    }
}
