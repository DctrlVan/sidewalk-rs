extern crate opc;
extern crate rand;
extern crate sidewalk;
use rand::Rng;
use sidewalk::*;

fn main() {
    random(|pixel: &mut [u8; 3]| {
        let mut rng = rand::thread_rng();
        pixel[0] = 20;
        pixel[1] = rng.gen();
        pixel[2] = rng.gen();
    }, 1250)
}
