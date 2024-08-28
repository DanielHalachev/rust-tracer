use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
#[derive(Clone, PartialEq)]
pub struct Triple(pub f32, pub f32, pub f32);

impl Add for Triple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Triple(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Add<f32> for Triple {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Triple(self.0 + rhs, self.1 + rhs, self.2 + rhs)
    }
}

impl AddAssign for Triple {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl AddAssign<f32> for Triple {
    fn add_assign(&mut self, rhs: f32) {
        self.0 += rhs;
        self.1 += rhs;
        self.2 += rhs;
    }
}

impl Sub for Triple {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Triple(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Sub<f32> for Triple {
    type Output = Self;
    fn sub(self, rhs: f32) -> Self::Output {
        Triple(self.0 - rhs, self.1 - rhs, self.2 - rhs)
    }
}

impl SubAssign for Triple {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl SubAssign<f32> for Triple {
    fn sub_assign(&mut self, rhs: f32) {
        self.0 -= rhs;
        self.1 -= rhs;
        self.2 -= rhs;
    }
}

impl Mul<f32> for Triple {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Triple(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Triple> for f32 {
    type Output = Triple;
    fn mul(self, rhs: Triple) -> Self::Output {
        Triple(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl MulAssign<f32> for Triple {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

pub use Triple as Vector;
impl Vector {
    pub fn get_length(&self) -> f32 {
        f32::sqrt(self.0 * self.0 + self.1 * self.1 + self.2 * self.2)
    }
    pub fn normalize(&mut self) {
        let mut length = self.get_length();
        if length > f32::EPSILON {
            length = 1.0 / length;
            *self *= length;
        }
    }

    pub fn get_normalized(&self) -> Self {
        let mut temp = self.clone();
        temp.normalize();
        temp
    }

    pub fn dot_product(&self, rhs: &Vector) -> f32 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross_product(&self, rhs: &Vector) -> Self {
        Vector(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    pub fn reflect(&self, normal: &Vector) -> Self {
        self.clone() - ((2.0 * self.dot_product(normal)) * normal.clone())
    }
}
pub use Triple as Coordinates;
pub use Triple as Point;
