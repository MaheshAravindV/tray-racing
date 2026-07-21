use crate::{color::Color, hittables::HitRecord, ray::Ray, vec3::Vec3};

use super::material::{ScatterRecord, Material};

pub struct Matte { 
    attenuation: Color    
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
    fn scatter(&self, hit_record: &HitRecord) -> ScatterRecord {
        let mut new_dir = Vec3::random_unit_vector();
        let directed_normal = if hit_record.front_face() {
            hit_record.normal()
        } else {
            -hit_record.normal()
        };
        let same_dir = new_dir * directed_normal;
        if same_dir < 0.0 {
            new_dir = -new_dir
        }

        new_dir += directed_normal;

        const EPSILON_MAGNITUDE: f64 = 1.73 * 1e-8;
        if new_dir.magnitude() < EPSILON_MAGNITUDE {
            new_dir = directed_normal
        }

        let reflected_ray = Ray::new(hit_record.point(), new_dir);

        ScatterRecord::new(reflected_ray, self.attenuation)
    }
}

