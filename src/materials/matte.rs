use crate::{hittables::HitRecord, ray::Ray, vec3::Vec3};

use super::material::{ScatterRecord, Material};

pub struct Matte;

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

        let reflected_ray = Ray::new(hit_record.point(), new_dir);

        ScatterRecord::new(reflected_ray, Vec3::uniform(0.5))
    }
}

