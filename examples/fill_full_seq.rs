extern crate opc;
extern crate rand;
use std::thread;
use std::time::Duration;
use rand::Rng;
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

fn fill_full_seq(pseq: &[&[u8]]) {
    for pat in pseq.iter() {
        fill_full(|pixel: &mut [u8; 3]| {
            pixel[0] = pat[0];
            pixel[1] = pat[1];
            pixel[2] = pat[2];
        });
        thread::sleep(Duration::from_millis(1000));
    }
}

fn main() {
    fill_full_seq(&[
        &[120, 110, 50],
        &[100, 90, 80],
        &[127, 45, 89],
        &[100, 90, 55],
        &[60, 10, 120],
        &[0, 100, 130],
        &[0, 45, 89],
        &[100, 90, 55]
    ]);
}
