extern crate redditPlaceHeatmap;

use redditPlaceHeatmap::{Bitmap, Matrix};
use std::env::args;

fn main() {
    let mut bm = Bitmap::from_file(&args().nth(1).expect("did not get input csv argument")).unwrap();
    // println!("{:?}", bm);
    let matrix = bm.to_matrix();
    matrix.to_image("out.png")
}