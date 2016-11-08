extern crate opc;
extern crate rand;

use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use std::cell::Cell;
use opc::*;
use rand::Rng;


pub fn random<F>(mut set_pixels: F, delay_time: u64) where F: FnMut(&mut [u8; 3]) {
    let mut stream = TcpStream::connect("192.168.1.51:7890").unwrap();
    let mut client = Client::new(stream);
    let mut pixels = vec![[0,0,0]; 1000];
    let mut rng = rand::thread_rng();

    loop {
        for pixel in pixels.iter_mut() {
            set_pixels(pixel);
        }

        let pixel_msg = Message {
            channel: 0,
            command: Command::SetPixelColors { pixels: pixels.clone() },
        };
        client.send(pixel_msg);
        thread::sleep(Duration::from_millis(delay_time));
    }
}

pub fn fill_full<F>(mut set_pixels: F) where F: FnMut(&mut [u8; 3]) {
    let mut stream = TcpStream::connect("192.168.1.51:7890").unwrap();
    let mut client = Client::new(stream);
    let mut pixels = vec![[0,0,0]; 1000];

    for pixel in pixels.iter_mut() {
        set_pixels(pixel);
    }

    let pixel_msg = Message {
        channel: 0,
        command: Command::SetPixelColors { pixels: pixels.clone() },
    };
    client.send(pixel_msg);
}
