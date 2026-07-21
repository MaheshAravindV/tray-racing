use std::iter::Sum;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn unit_vector(&self) -> Vec3 {
        let magnitude = self.magnitude();

        self / magnitude
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn random_unit_vector() -> Vec3 {
        let x = rand::random_range(-1.0..1.0);
        let y = rand::random_range(-1.0..1.0);
        let z = rand::random_range(-1.0..1.0);
        let d = x * x + y * y + z * z;

        const EPSILON: f64 = 1e-160;

        if d <= 1.0 && d > EPSILON {
            Vec3::new(x, y, z).unit_vector()
        } else {
            Self::random_unit_vector()
        }
    }

    pub fn uniform(scale: f64) -> Self {
        Self::new(scale, scale, scale)
    }

    pub fn odot(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        -1.0 * self
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

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
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

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul<Vec3> for i32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self as f64
    }
}

impl<T> Mul<T> for Vec3
where
    T: Into<f64> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs.into(),
            y: self.y * rhs.into(),
            z: self.z * rhs.into(),
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = f64;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl<T> Div<T> for &Vec3
where
    T: Into<f64> + Copy,
{
    type Output = Vec3;

    fn div(self, rhs: T) -> Self::Output {
        *self / rhs
    }
}

impl<T> Div<T> for Vec3
where
    T: Into<f64> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs.into(),
            y: self.y / rhs.into(),
            z: self.z / rhs.into(),
        }
    }
}

impl Sum<Vec3> for Vec3 {
    fn sum<I: Iterator<Item = Vec3>>(iter: I) -> Vec3 {
        let mut acc = Vec3::new(0.0, 0.0, 0.0);
        for e in iter {
            acc += e;
        }

        acc
    }
}
