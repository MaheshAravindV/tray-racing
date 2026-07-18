use crate::scene::Scene;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

mod color;
mod p3;
mod position;
mod ray;
mod scene;
mod vec3;
mod sphere;

fn main() {
    let height = 400;
    let width = 800;
    let mut scene = Scene::new(width, height);
    scene.add_item(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    scene.draw();
    println!("done");
}
