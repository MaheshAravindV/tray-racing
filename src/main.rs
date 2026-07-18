mod color;
mod p3;
mod position;
mod ray;
mod scene;
mod vec3;

fn main() {
    let height = 400;
    let width = 800;
    let mut img = p3::p3::new(width, height);
    for i in 0..height {
        for j in 0..width {
            img.write_color(color::Color::from_tup((i as f32, (i + j) as f32, i as f32)));
        }
    }

    println!("done");
}
