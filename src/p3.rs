use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use crate::color::Color;

pub struct P3 {
    max_color: isize,
    out_file: BufWriter<File>,
}

impl P3 {
    pub fn new(width: isize, height: isize) -> P3 {
        const MAX_COLOR: isize = 255;

        let mut out_file = BufWriter::new(File::create("p3.ppm").unwrap());
        out_file
            .write(format!("P3\n{} {}\n{}\n", width, height, MAX_COLOR).as_bytes())
            .unwrap();

        P3 {
            max_color: MAX_COLOR,
            out_file,
        }
    }

    pub fn write_color(&mut self, color: Color) {
        let scaled_color = color.transform_to_gamma() * self.max_color as f64;
        self.out_file
            .write(
                format!(
                    "{} {} {}\n",
                    scaled_color.r() as i32,
                    scaled_color.g() as i32,
                    scaled_color.b() as i32
                )
                .as_bytes(),
            )
            .unwrap();
    }
}
