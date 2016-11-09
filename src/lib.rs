extern crate opc;
extern crate rand;

use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use opc::*;

pub fn random<F>(mut set_pixels: F, delay_time: u64)
    where F: FnMut(&mut [u8; 3])
{
    let stream = TcpStream::connect("192.168.1.51:7890").unwrap();
    let mut client = Client::new(stream);
    let mut pixels = vec![[0,0,0]; 1000];

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

pub fn fill_full<F>(mut set_pixels: F)
    where F: FnMut(&mut [u8; 3])
{
    let stream = TcpStream::connect("192.168.1.51:7890").unwrap();
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

pub fn fill_full_seq(pseq: &[&[u8]], delay_time: u64) {
    for pat in pseq.iter() {
        fill_full(|pixel: &mut [u8; 3]| {
            pixel[0] = pat[0];
            pixel[1] = pat[1];
            pixel[2] = pat[2];
        });
        thread::sleep(Duration::from_millis(delay_time));
    }
}
