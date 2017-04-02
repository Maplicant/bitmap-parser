extern crate ws;
extern crate image;

use std::io::BufReader;
use std::io::Read;
use std::fs::File;
use std::path::Path;

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
        let mut iter = self.raw_data.iter();
        let mut matrix = [[0u8; 1000]; 1000];
        for y in 0..1000 {
            for x in 0..500 {
                let datum = match iter.next() {
                    Some(b) => *b,
                    None => panic!("read too many bytes"),
                };
                // println!("{:?}", datum);
                let color1 = datum >> 4;
                let color2 = datum & 0b1111; // Last 4 bits

                matrix[x*2][y] = color1;
                matrix[x*2 + 1][y] = color2;

            }
        }
        Matrix{
            matrix: matrix
        }
    }
}

impl Matrix {
    pub fn to_image(&self, filename: &str) {
        let mut imgbuf = image::ImageBuffer::new(1000, 1000);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            match self.matrix[x as usize][y as usize]{
                0  => *pixel = image::Rgb([255, 255, 255]),
                1  => *pixel = image::Rgb([228, 228, 228]),
                2  => *pixel = image::Rgb([136, 136, 136]),
                3  => *pixel = image::Rgb([34,  34,  34 ]),
                4  => *pixel = image::Rgb([255, 167, 209]),
                5  => *pixel = image::Rgb([229, 0  , 0  ]),
                6  => *pixel = image::Rgb([229, 149, 0  ]),
                7  => *pixel = image::Rgb([160, 106, 66 ]),
                8  => *pixel = image::Rgb([229, 217, 0  ]),
                9  => *pixel = image::Rgb([148, 224, 68 ]),
                10 => *pixel = image::Rgb([2  , 190, 1  ]),
                11 => *pixel = image::Rgb([0  , 211, 221]),
                12 => *pixel = image::Rgb([0  , 131, 199]),
                13 => *pixel = image::Rgb([0  , 0  , 234]),
                14 => *pixel = image::Rgb([207, 110, 228]),
                15 => *pixel = image::Rgb([130, 0  , 128]),
                _  => unreachable!()
            }
        }

        let ref mut fout = File::create(&Path::new(filename)).unwrap();
        let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);
    }
}