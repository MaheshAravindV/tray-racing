use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    base: Vec3,
    direction: Vec3,
    refractive_index_of_medium: f64,
}

impl Ray {
    pub fn new_in_air(base: Vec3, direction: Vec3) -> Self {
        Self::new(base, direction, 1.0)
    }

    pub fn new(base: Vec3, direction: Vec3, refractive_index: f64) -> Self {
        Self {
            base,
            direction: direction.unit_vector(),
            refractive_index_of_medium: refractive_index,
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

    pub fn refractive_index_of_medium(&self) -> f64 {
        self.refractive_index_of_medium
    }
}
