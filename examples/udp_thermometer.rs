use std::{thread, time::Duration};

use smarthouse_homework::prelude::UdpSmartThermometer;

fn main() {
    let receiver_address = "127.0.0.1:3005";
    let thermo = UdpSmartThermometer::new(receiver_address).unwrap();
    for _ in 0..120 {
        thread::sleep(Duration::from_secs(1));
        let temperature = thermo.get_temperature();
        println!("The temperature is {temperature}");
    }
}
