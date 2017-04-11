fn main() {
    let ip_str = match std::env::args().nth(1) {
        Some(input) => input,
        None => {
            println!("Defaulting to raspberrypi.local");
            String::from("raspberrypi.local")
        },
    };
    println!("Connecting to robot at: '{}'...", ip_str);
}
