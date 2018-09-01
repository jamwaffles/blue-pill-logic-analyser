//! Read anything coming from the serial port and spit it into the console

extern crate blue_pill_logic_analyser_driver;
extern crate serial;

use blue_pill_logic_analyser_driver::configure;
use std::env;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    for arg in env::args_os().skip(1) {
        let mut port = serial::open(&arg).unwrap();
        configure(&mut port).unwrap();

        let mut file = BufReader::new(port);
        for line in file.lines() {
            let l = line.unwrap();

            println!("{}", l);
        }

        // let mut buf: [u8; 1024] = [0; 1024];

        // loop {
        //     port.read(&mut buf).ok();

        //     println!("{}NEXT", String::from_utf8(buf.to_vec()).unwrap());
        // }
    }
}
