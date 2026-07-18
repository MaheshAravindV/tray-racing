use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }

    pub fn will_be_hit(&self, ray: &Ray) -> bool {
        let base = ray.base;
        let center = self.center;
        let direction = ray.direction;
        let radius = self.radius;

        let a = direction * direction;
        let b = 2.0 * direction * (base - center);
        let c = (base - center) * (base - center)  - radius * radius;

        let discriminant = b * b - 4.0 * a * c;

        discriminant >= 0.0
    }
}