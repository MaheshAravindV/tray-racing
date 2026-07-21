use crate::ray::Ray;
use crate::vec3::Vec3;
use std::range::Range;

pub trait Hittable {
    fn get_hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<HitRecord>;
}

pub struct HitRecord {
    t: f64,
    point: Vec3,
    normal: Vec3,
    front_face: bool,
}

impl HitRecord {
    pub fn new(t: f64, point: Vec3, outward_normal: Vec3, ray: &Ray) -> Self {
        let same_direction = (outward_normal * ray.direction()) >= 0.0;
        Self {
            t,
            point,
            normal: outward_normal,
            front_face: !same_direction,
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn point(&self) -> Vec3 {
        self.point
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }
}
