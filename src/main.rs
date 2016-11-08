extern crate opc;
extern crate rand;

use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use std::cell::Cell;
use opc::*;
use rand::Rng;


fn _loop<F>(mut set_pixels: F) where F: FnMut(&mut [u8; 3]) {

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
        thread::sleep(Duration::from_millis(1000));
    }
}

fn main() {
    _loop(|pixel: &mut [u8; 3]| {
        let mut rng = rand::thread_rng();
        pixel[0] = 0;
        pixel[1] = rng.gen();
        pixel[2] = rng.gen();
    })
}
