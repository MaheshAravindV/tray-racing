use crate::color::Color;
use crate::materials::Material;
use crate::materials::ScatterRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Metal {
    attenuation: Color,
}

impl Metal {
    pub fn new(attenuation: Color) -> Self {
        Self { attenuation }
    }

    fn reflect_ray(source_ray: &Ray, normal: Vec3) -> Ray {
        let source_direction = source_ray.direction();
        let reflected_direction = source_direction - 2.0 * (source_direction * normal) * normal;
        return Ray::new(source_ray.base(), reflected_direction);
    }
}

impl Material for Metal {
    fn scatter(&self, hit_record: &crate::hittables::HitRecord) -> ScatterRecord {
        let reflected_ray = Self::reflect_ray(hit_record.source_ray(), hit_record.normal());
        ScatterRecord { scattered_ray: reflected_ray, attenuation: self.attenuation }
    }
}
