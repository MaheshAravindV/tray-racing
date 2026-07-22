use crate::color::Color;
use crate::materials::Material;
use crate::materials::ScatterRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Metal {
    attenuation: Color,
    fuzziness: f64
}

impl Metal {
    pub fn new(attenuation: Color, fuzziness: f64) -> Self {
        Self { attenuation, fuzziness }
    }

    fn reflected_direction(source_ray: &Ray, normal: Vec3) -> Vec3 {
        let source_direction = source_ray.direction();
        let reflected_direction = source_direction - 2.0 * (source_direction * normal) * normal;
        reflected_direction
    }
}

impl Material for Metal {
    fn scatter(&self, hit_record: &crate::hittables::HitRecord) -> Option<ScatterRecord> {
        let reflected_direction = Self::reflected_direction(hit_record.source_ray(), hit_record.normal());
        let fuzz_direction = self.fuzziness * Vec3::random_unit_vector();
        let fuzzy_reflected_direction = reflected_direction + fuzz_direction;
        let absorbed = reflected_direction * hit_record.normal() <= 0.0;

        if absorbed {
            None
        } else {
            Some(ScatterRecord {
                scattered_ray: Ray::new(hit_record.point(), fuzzy_reflected_direction), 
                attenuation: self.attenuation 
            })
        }
    }
}
