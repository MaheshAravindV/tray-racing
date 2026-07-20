
use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn from_tup(rgb: (f64, f64, f64)) -> Self {
        Self {
            x: rgb.0.clamp(0.0, 1.0),
            y: rgb.1.clamp(0.0, 1.0),
            z: rgb.2.clamp(0.0, 1.0),
        }
    }

    fn to_gamma(component: f64) -> f64 {
        if component > 0.0 { component.sqrt() } else { 0.0 }
    }

    pub fn transform_to_gamma(&self) -> Self {
        Self::new(Self::to_gamma(self.r()), Self::to_gamma(self.g()), Self::to_gamma(self.b()))
    }

    pub fn invert(&self) -> Self {
        Self::from_tup((1.0 - self.r(), 1.0 - self.g(), 1.0 - self.b()))
    }

    pub fn r(&self) -> f64 {
        self.x
    }

    pub fn g(&self) -> f64 {
        self.y
    }

    pub fn b(&self) -> f64 {
        self.z
    }
}

