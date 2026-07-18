use crate::scene::Scene;

mod color;
mod p3;
mod position;
mod ray;
mod scene;
mod vec3;

fn main() {
    let height = 400;
    let width = 800;
    let scene = Scene::new(width, height);
    scene.draw();
    println!("done");
}
