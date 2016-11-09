extern crate opc;
extern crate rand;
extern crate sidewalk;
use sidewalk::*;

fn main() {
    fill_full(|pixel: &mut [u8; 3]| {
        pixel[0] = 20;
        pixel[1] = 120;
        pixel[2] = 90;
    })
}
