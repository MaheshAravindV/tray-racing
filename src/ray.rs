use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Ray {
    base: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(base: Vec3, direction: Vec3) -> Self {
        Self {
            base,
            direction: direction.unit_vector(),
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.base() + self.direction() * t
    }

    pub fn base(&self) -> Vec3 {
        self.base
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }
}
