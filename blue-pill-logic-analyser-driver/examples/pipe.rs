//! Read anything coming from the serial port and spit it into the console

extern crate blue_pill_logic_analyser_driver;
extern crate serial;

use blue_pill_logic_analyser_driver::configure;
use std::env;
use std::error::Error;
use std::io::BufRead;
use std::io::BufReader;
use std::time::Duration;

struct DataPoint {
    value: u8,
    time: u32,
}

fn main() {
    let max_time = Duration::from_millis(10000);

    for arg in env::args_os().skip(1) {
        let mut port = serial::open(&arg).unwrap();
        configure(&mut port).unwrap();

        let mut file = BufReader::new(port);

        for line in file.lines() {
            match line {
                Ok(l) => println!("{:?}", l),
                Err(e) => eprintln!("PARSE ERR {}", e.description()),
            }
        }
    }
}
