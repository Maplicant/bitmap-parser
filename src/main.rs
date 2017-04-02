extern crate redditPlaceHeatmap;

use redditPlaceHeatmap::{Bitmap, Matrix};

fn main() {
    let bm = Bitmap::from_file("export.csv").unwrap();
    // println!("{:?}", bm);
    let matrix = bm.to_matrix();
}