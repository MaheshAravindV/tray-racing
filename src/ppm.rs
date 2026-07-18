use std::fs::File;
use std::io::Write;
use crate::color::Color;

struct p3 {
    height: isize,
    width: isize,
    max_color: isize,
    out_file: File
}

impl p3 {
    fn new(height: isize, width: isize) -> p3 {
        const MAX_COLOR: isize = 255;

        let mut out_file = File::create("p3.ppm").unwrap();
        out_file.write(format!("P3\n{} {}\n{}\n", height, width, MAX_COLOR).as_bytes()).unwrap();

        p3 {
            height,
            width,
            max_color: MAX_COLOR,
            out_file: File::create("p3.ppm").unwrap()
        }
    }

    fn write_color(&mut self, color: Color) {
        self.out_file.write(format!("{} {} {}\n", color.r(), color.g(), color.b()).as_bytes()).unwrap();
    }
}
