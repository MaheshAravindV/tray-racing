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
        let scaled_color = color.unit_vector() * self.max_color as f64;
        self.out_file
            .write(format!("{} {} {}\n",
                           scaled_color.r() as i32,
                           scaled_color.g() as i32,
                           scaled_color.b() as i32).as_bytes())
            .unwrap();
    }
}
