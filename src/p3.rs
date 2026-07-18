use crate::color::Color;
use std::fs::File;
use std::io::Write;

pub struct p3 {
    height: isize,
    width: isize,
    max_color: isize,
    out_file: File,
}

impl p3 {
    pub fn new(width: isize, height: isize) -> p3 {
        const MAX_COLOR: isize = 255;

        let mut out_file = File::create("p3.ppm").unwrap();
        out_file
            .write(format!("P3\n{} {}\n{}\n", width, height, MAX_COLOR).as_bytes())
            .unwrap();

        p3 {
            height,
            width,
            max_color: MAX_COLOR,
            out_file,
        }
    }

    pub fn write_color(&mut self, color: Color) {
        self.out_file
            .write(format!("{} {} {}\n", color.r(), color.g(), color.b()).as_bytes())
            .unwrap();
    }
}
