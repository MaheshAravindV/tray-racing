use crate::color::Color;
use crate::hittables::HitRecord;
use crate::ray::Ray;

pub trait Material: Send + Sync {
    fn scatter(&self, hit_record: &HitRecord) -> Option<ScatterRecord>;
}

pub struct ScatterRecord {
    pub scattered_ray: Ray,
    pub attenuation: Color,
}

impl ScatterRecord {
    pub fn new(scattered_ray: Ray, attenuation: Color) -> Self {
        Self {
            scattered_ray,
            attenuation,
        }
    }
}
