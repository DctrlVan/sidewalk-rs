extern crate opc;
extern crate rand;
use rand::Rng;
mod animate;

fn main() {
    animate::random(|pixel: &mut [u8; 3]| {
        let mut rng = rand::thread_rng();
        pixel[0] = 20;
        pixel[1] = rng.gen();
        pixel[2] = rng.gen();
    }, 1250)

    // animate::fill_full(|pixel: &mut [u8; 3]| {
    //     pixel[0] = 20;
    //     pixel[1] = 120;
    //     pixel[2] = 90;
    // })
}
