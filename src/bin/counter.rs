extern crate byteorder;
extern crate tsp;

use std::env;
use std::thread;
use std::sync::Mutex;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};


struct CounterApp;

impl CounterApp {
    fn new() -> CounterApp {
        CounterApp
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let connection_type: &str = &args[1];
    let listen_addr: &str = &args[2];

    let app = CounterApp::new();

    loop {
        thread::park();
    }
}
