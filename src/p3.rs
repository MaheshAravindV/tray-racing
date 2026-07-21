use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use crate::color::Color;

pub struct P3 {
    max_color: isize,
    out_file: BufWriter<File>,
}

impl P3 {
    pub fn new(width: isize, height: isize, output_file_path: &String) -> P3 {
        const MAX_COLOR: isize = 255;

        let mut out_file = BufWriter::new(File::create(output_file_path).unwrap());
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
    }
}
