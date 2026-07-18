mod ppm;
mod vec3;
mod color;
mod position;

fn main() {
    println!("Hello, world!");
}

fn draw() {
    let (width, height) = (256, 256);

    println!("P3");
    println!("{} {}", width, height);
    println!("255");

    for i in 0..height {
       for j in 0..width {
           let (r, g, b): (f32, f32, f32) = (i as f32, (i + j) as f32, i as f32);
       }
    }
}
