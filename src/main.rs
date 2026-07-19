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
    let height = 400;
    let width = 800;
    let mut scene = Scene::new(width, height);
    scene.add_object(StructObject::new(
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Matte {}),
    ));
    scene.add_object(StructObject::new(
        Box::new(Sphere::new(Vec3::new(0.0, -500.5, -1.0), 500.0)),
        Box::new(Matte {}),
    ));
    scene.draw();
    println!("done");
}
