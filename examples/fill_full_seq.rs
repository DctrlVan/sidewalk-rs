extern crate opc;
extern crate rand;
extern crate sidewalk_rs;
use opc::*;
use sidewalk_rs::*;

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
    ], 1000);
}
