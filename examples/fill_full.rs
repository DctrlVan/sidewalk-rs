extern crate opc;
extern crate rand;

use std::net::TcpStream;
use opc::*;

fn fill_full<F>(mut set_pixels: F)
    where F: FnMut(&mut [u8; 3])
{
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

fn main() {
    fill_full(|pixel: &mut [u8; 3]| {
        pixel[0] = 20;
        pixel[1] = 120;
        pixel[2] = 90;
    })
}
