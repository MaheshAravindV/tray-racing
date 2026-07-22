use crate::color::Color;
use crate::materials::{reflected_direction, Material};
use crate::materials::ScatterRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Metal {
    attenuation: Color,
    fuzziness: f64,
}

impl Metal {
    pub fn new(attenuation: Color, fuzziness: f64) -> Self {
        Self {
            attenuation,
            fuzziness,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, hit_record: &crate::hittables::HitRecord) -> Option<ScatterRecord> {
        let reflected_direction = reflected_direction(hit_record.source_ray(), hit_record.normal());
        let fuzz_direction = self.fuzziness * Vec3::random_unit_vector();
        let fuzzy_reflected_direction = reflected_direction + fuzz_direction;
        let absorbed = reflected_direction * hit_record.normal() <= 0.0;

        if absorbed {
            None
        } else {
            Some(ScatterRecord {
                scattered_ray: Ray::new_in_air(hit_record.point(), fuzzy_reflected_direction),
                attenuation: self.attenuation,
            })
        }
    }
}