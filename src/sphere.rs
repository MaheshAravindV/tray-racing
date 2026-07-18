use crate::color::Color;
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

    pub fn get_hit_point(&self, ray: &Ray) -> Option<Vec3> {
        let base = ray.base();
        let center = self.center;
        let direction = ray.direction() - ray.base();
        let radius = self.radius;

        let a = direction * direction;
        let h = (base - center) * direction;
        let c = (base - center) * (base - center) - radius * radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let t = -h - discriminant.sqrt() / a;
        Some(ray.at(t))
    }

    pub fn normal_at(&self, pos: Vec3) -> Vec3 {
        (pos - self.center).unit_vector()
    }

    pub fn get_color_at(&self, pos: Vec3) -> Color {
        (self.normal_at(pos) + Vec3::new(1.0, 1.0, 1.0)) / 2.0_f32
    }
}

