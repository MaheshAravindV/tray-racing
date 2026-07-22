use crate::color::Color;
use crate::hittables::HitRecord;
use crate::materials::Material;
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

        let relative_refractive_index = if hit_record.front_face() {
            source_ray.refractive_index_of_medium() / self.refractive_index
        } else {
            self.refractive_index / source_ray.refractive_index_of_medium()
        };
        let cos_theta = (-source_ray.direction() * normal).clamp(0.0, 1.0);

        let refracted_perpendicular =
            relative_refractive_index * (source_ray.direction() + cos_theta * normal);
        let refracted_perpendicular_magnitude = refracted_perpendicular.magnitude();
        let refracted_parallel_magnitude =
            (1.0 - refracted_perpendicular_magnitude * refracted_perpendicular_magnitude).sqrt();
        let refracted_parallel = refracted_parallel_magnitude * -normal;

        let refracted_direction = refracted_parallel + refracted_perpendicular;
        let refracted_ray = Ray::new(
            hit_record.point(),
            refracted_direction,
            self.refractive_index,
        );

        Some(ScatterRecord::new(refracted_ray, Color::uniform(1.0)))
    }
}
