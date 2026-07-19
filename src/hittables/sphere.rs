use crate::color::Color;
use super::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }

    pub fn get_color_at(&self, hit: &HitRecord) -> Color {
        (hit.normal() + Vec3::new(1.0, 1.0, 1.0)) / 2.0f64
    }
}

impl Hittable for Sphere {
    fn get_hit(self: &Self, ray: &Ray, t_range: std::range::Range<f64>) -> Option<HitRecord> {
        let base = ray.base();
        let center = self.center;
        let direction = ray.direction();
        let radius = self.radius;

        let a = direction * direction;
        let b = 2.0 * direction * (base - center);
        let c = (base - center) * (base - center) - radius * radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let t1 = -b + discriminant.sqrt();
        let t2 = -b - discriminant.sqrt();
        let t = if t2 >= 0.0 { t2 } else { t1 };
        let t = t / (2.0 * a);

        if !t_range.contains(&t) {
            return None;
        }

        let position = ray.at(t);
        let outward_normal = (position - self.center).unit_vector();

        Some(HitRecord::new(t, position, outward_normal, &ray))
    }
}
