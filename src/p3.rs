use crate::color::Color;
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct P3 {
    max_color: isize,
    image: Vec<u8>,
    out_file: BufWriter<File>,
}

impl P3 {
    pub fn new(width: isize, height: isize) -> P3 {
        const MAX_COLOR: isize = 255;

        let mut out_file = BufWriter::with_capacity(25165824, File::create("p3.ppm").unwrap());
        out_file
            .write(format!("P6\n{} {}\n{}\n", width, height, MAX_COLOR).as_bytes())
            .unwrap();

        P3 {
            max_color: MAX_COLOR,
            out_file,
        }
    }

    pub fn write_color(&mut self, color: Color) {
        let scaled_color = color * self.max_color as f64;
        self.out_file
            .write(&[
                scaled_color.r() as u8,
                scaled_color.g() as u8,
                scaled_color.b() as u8,
            ])
            .unwrap();

        // .write(
        //     format!(
        //         "{} {} {}\n",
        //         scaled_color.r() as i32,
        //         scaled_color.g() as i32,
        //         scaled_color.b() as i32
        //     )
        //     .as_bytes(),
        // )
        // .unwrap();
    }
}
