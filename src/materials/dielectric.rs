use crate::color::Color;
use crate::hittables::HitRecord;
use crate::materials::{refracted_direction, Material};
use crate::materials::ScatterRecord;
use crate::ray::Ray;

pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let source_ray = hit_record.source_ray();
        let normal = hit_record.normal();
        let hit_front_direction = hit_record.front_face();

        let refracted_direction =
            refracted_direction(source_ray, normal, hit_front_direction, self.refractive_index);

        let refracted_ray = Ray::new(
            hit_record.point(),
            refracted_direction,
            self.refractive_index,
        );

        Some(ScatterRecord::new(refracted_ray, Color::uniform(1.0)))
    }
}
