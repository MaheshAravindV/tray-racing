use crate::{color::Color, hittables::HitRecord, ray::Ray};

pub trait Material {
    fn scatter(&self, hit_record: &HitRecord) -> ScatterRecord;
}

pub struct ScatterRecord {
    pub scattered_ray: Ray,
    pub attenuation: Color
}

impl ScatterRecord {
    pub fn new(scattered_ray: Ray, attenuation: Color) -> Self {
        Self {
            scattered_ray,
            attenuation
        }
    }
}
