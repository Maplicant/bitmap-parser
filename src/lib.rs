extern crate ws;

use std::io::BufReader;
use std::io::Read;
use std::fs::File;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

#[derive(Debug)]
pub struct Bitmap {
    raw_data: Vec<u8>
}

pub struct Matrix {
    matrix: [[u8; 1000]; 1000]
}

impl Bitmap {
    pub fn from_file(filename: &str) -> Option<Bitmap> {
        let mut file = match File::open(filename) {
            Ok(file) => file,
            Err(_) => return None,
        };
        let mut file_contents: Vec<u8> = Vec::new();
        file.read_to_end(&mut file_contents);
        Some(Bitmap {
            raw_data: file_contents
        })
    }
    
    pub fn to_matrix(&self) -> Matrix {
        let iter = self.raw_data.iter();
        let mut matrix = [[0u8; 1000]; 1000];
        for y in 0..1000 {
            for x in 0..500 {
                let datum = iter.next();
                println!("{:?}", datum);

            }
        }
        Matrix{
            matrix: matrix
        }
    }
}
