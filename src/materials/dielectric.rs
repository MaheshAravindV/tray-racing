use crate::color::Color;
use crate::hittables::HitRecord;
use crate::materials::{reflected_direction, refracted_direction, Material};
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

        let refracted_ray = match refracted_direction {
            Some(refracted_direction) => {
                Ray::new(
                    hit_record.point(),
                    refracted_direction,
                    self.refractive_index,
                )
            },
            None => {
                let reflected_direction = reflected_direction(source_ray, normal);
                Ray::new(
                    hit_record.point(),
                    reflected_direction,
                    source_ray.refractive_index_of_medium()
                )
            }
        };

        Some(ScatterRecord::new(refracted_ray, Color::uniform(1.0)))
    }
}
