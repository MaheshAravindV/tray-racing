use crate::color::Color;
use crate::hittables::Sphere;
use crate::materials::Matte;
use crate::object::StructObject;
use crate::scene::Scene;
use crate::vec3::Vec3;

mod color;
mod hittables;
mod materials;
mod object;
mod p3;
mod ray;
mod scene;
mod vec3;

fn main() {
    let height = 2160;
    let width = 3840;
    let mut scene = Scene::new(width, height);
    scene.add_object(StructObject::new(
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Matte::with_color(Color::from_tup((0.9, 0.8, 0.5)))),
    ));
    scene.add_object(StructObject::new(
        Box::new(Sphere::new(Vec3::new(0.0, -500.5, -1.0), 500.0)),
        Box::new(Matte::new(Color::from_tup((0.2, 1.0, 0.2)))),
    ));
    scene.draw();
    println!("done");
}
