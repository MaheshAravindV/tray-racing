use super::material::Material;
use super::material::ScatterRecord;
use crate::color::Color;
use crate::hittables::HitRecord;
use crate::materials::diffused_direction;
use crate::ray::Ray;

pub struct Matte {
    attenuation: Color,
}

impl Matte {
    pub fn new(attenuation: Color) -> Self {
        Self { attenuation }
    }

    pub fn with_color(color: Color) -> Self {
        Self::new(color.invert())
    }
}

impl Material for Matte {
    fn scatter(&self, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let diffused_direction =
            diffused_direction(hit_record.normal(), hit_record.front_face());

        let refractive_index = hit_record.source_ray().refractive_index_of_medium();
        let reflected_ray = Ray::new(hit_record.point(), diffused_direction, refractive_index);

        Some(ScatterRecord::new(reflected_ray, self.attenuation))
    }
}
