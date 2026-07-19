use crate::{hittables::HitRecord, ray::Ray, vec3::Vec3};

pub trait Material {
    fn get_hit_result(&self, hit_record: &HitRecord) -> HitResult;
}

pub struct HitResult {
    pub reflected_ray: Ray,
    pub absorption_factor: AbsorptionFactor
}

impl HitResult {
    pub fn new(reflected_ray: Ray, absorption_factor: AbsorptionFactor) -> Self {
        Self {
            reflected_ray,
            absorption_factor
        }
    }
}

pub type AbsorptionFactor = Vec3;
