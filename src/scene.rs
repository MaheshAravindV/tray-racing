use crate::color::Color;
use crate::hittables::Hittable;
use crate::object::{Object, StructObject};
use crate::p3::P3;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Scene {
    camera: Vec3,
    focal_len: f64,
    width: isize,
    height: isize,
    vp_width: f64,
    vp_height: f64,
    objects: Vec<StructObject>,
}

const ANTI_ALIASING_SAMPLES: u8 = 10; 
const TRACE_DEPTH: u8 = 10; 

impl Scene {
    pub fn new(width: isize, height: isize) -> Self {
        let vp_height = 2f64;
        Self {
            camera: Vec3::new(0.0, 0.0, 0.0),
            focal_len: 1.0,
            width,
            height,
            vp_height,
            vp_width: (width as f64) / (height as f64) * vp_height,
            objects: vec![],
        }
    }

    pub fn add_object(&mut self, object: StructObject) {
        self.objects.push(object);
    }

    fn viewport_u(&self) -> Vec3 {
        Vec3::new(self.vp_width, 0.0, 0.0)
    }

    fn viewport_v(&self) -> Vec3 {
        Vec3::new(0.0, -self.vp_height, 0.0)
    }

    fn delta_vu(&self) -> Vec3 {
        self.viewport_u() / self.width as f64
    }

    fn delta_vv(&self) -> Vec3 {
        self.viewport_v() / self.height as f64
    }

    fn upper_left(&self) -> Vec3 {
        self.camera
            - Vec3::new(0.0, 0.0, self.focal_len)
            - self.viewport_u() / 2.0
            - self.viewport_v() / 2.0
    }

    pub fn draw(&self) {
        let p00 = self.upper_left() + self.delta_vu() / 2.0 - self.delta_vv() / 2.0;

        let mut out = P3::new(self.width, self.height);

        for i in 0..self.height {
            for j in 0..self.width {
                let color = (0..ANTI_ALIASING_SAMPLES)
                    .map(|_| self.ray_color(&self.get_sampled_ray(p00, i, j), TRACE_DEPTH))
                    .sum::<Vec3>()
                    / 10.0;
                out.write_color(color);
            }
        }
    }

    fn get_sampled_ray(&self, p00: Vec3, i: isize, j: isize) -> Ray {
        let x_offset = rand::random_range(-0.5..0.5);
        let y_offset = rand::random_range(-0.5..0.5);
        let direction =
            p00 + (i as f64 + y_offset) * self.delta_vv() + (j as f64 + x_offset) * self.delta_vu()
                - self.camera;
        let ray = Ray::new(self.camera, direction);
        ray
    }

    fn ray_color(&self, ray: &Ray, depth: u8) -> Color {
        if depth > 0 {
            for object in &self.objects {
                let hittable = object.get_hittable();
                let material = object.get_material();
                let hit_record = hittable.get_hit(ray, (0.0..f64::INFINITY).into());
                if let Some(hit_record) = hit_record {
                    let hit_result = material.get_hit_result(&hit_record);
                    return hit_result.absorption_factor.odot(&self.ray_color(&hit_result.reflected_ray, depth - 1));
                }
            }
            
            let start: Color = Color::new(0.5, 0.7, 1.0);
            let end: Color = Color::new(1.0, 1.0, 1.0);

            let a = (ray.direction().unit_vector().y + 1.0) / 2.0;
            start * a + end * (1.0 - a)
        } else {
            Color::uniform(0.0)
        }   
    }
}
