use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn from_tup(rgb: (f32, f32, f32)) -> Self {
        Self {
            x: rgb.0,
            y: rgb.1,
            z: rgb.2,
        }
    }

    pub fn r(&self) -> f32 {
        self.x
    }

    pub fn g(&self) -> f32 {
        self.y
    }

    pub fn b(&self) -> f32 {
        self.z
    }
}

