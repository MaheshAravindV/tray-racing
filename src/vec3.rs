use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn unit_vector(&self) -> Vec3 {
        let magnitude = self.magnitude();

        self / magnitude
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}


impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul<Vec3> for i32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self as f32
    }
}

impl<T> Mul<T> for Vec3 where T: Into<f32> + Copy {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self { x: self.x * rhs.into(), y: self.y * rhs.into(), z: self.z * rhs.into() }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = f32;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl<T> Div<T> for &Vec3 where T: Into<f32> + Copy {
    type Output = Vec3;

    fn div(self, rhs: T) -> Self::Output {
        *self / rhs
    }
}

impl<T> Div<T> for Vec3 where T: Into<f32> + Copy {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self { x: self.x / rhs.into(), y: self.y / rhs.into(), z: self.z / rhs.into() }
    }
}