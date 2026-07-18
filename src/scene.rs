use crate::color::Color;
use crate::hittable::Hittable;
use crate::p3::p3;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

pub struct Scene {
    camera: Vec3,
    focal_len: f64,
    width: isize,
    height: isize,
    vp_width: f64,
    vp_height: f64,
    items: Vec<Sphere>,
}

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
            items: vec![],
        }
    }

    pub fn add_item(&mut self, item: Sphere) {
        self.items.push(item);
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

        let mut out = p3::new(self.width, self.height);

        for i in 0..self.height {
            for j in 0..self.width {
                let direction =
                    p00 + i as f64 * self.delta_vv() + j as f64 * self.delta_vu() - self.camera;
                let ray = Ray::new(self.camera, direction);
                out.write_color(self.ray_color(&ray))
            }
        }
    }

    fn ray_color(&self, ray: &Ray) -> Color {
        for item in &self.items {
            let hit = item.get_hit(ray, 0.0..f64::INFINITY);
            if let Some(hit) = hit {
                return item.get_color_at(&hit);
            }
        }

        let start: Color = Color::new(0.5, 0.7, 1.0);
        let end: Color = Color::new(1.0, 1.0, 1.0);

        let a = (ray.direction().unit_vector().y + 1.0) / 2.0;
        start * a + end * (1.0 - a)
    }
}
