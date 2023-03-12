use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use num_traits::{Num, NumAssign, Signed};

#[derive(Debug)]
pub struct Vec3<T: Num>(pub T, pub T, pub T);

impl<T: Num> Vec3<T> {
    pub fn new(a: T, b: T, c: T) -> Self {
        Self(a, b, c)
    }
}

impl<T: Num + Copy> Vec3<T> {
    pub fn dot(&self, rhs: &Self) -> T {
        (self.0 * rhs.0) + (self.1 * rhs.1) + (self.2 * rhs.2)
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    pub fn len(&self) -> f64
    where
        T: Into<f64>,
    {
        self.dot(self).into().sqrt()
    }

    pub fn normalize(&self) -> Vec3<f64>
    where
        T: Into<f64>,
    {
        let len = self.len();
        Vec3(
            self.0.into() / len,
            self.1.into() / len,
            self.2.into() / len,
        )
    }
}

impl<T: Signed> Neg for Vec3<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl<T: Num> Add for Vec3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl<T: NumAssign> AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl<T: Signed> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl<T: NumAssign> SubAssign for Vec3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl<T: Num + Copy> Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl<T: NumAssign + Copy> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl<T: Num + Copy> Div<T> for Vec3<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl<T: NumAssign + Copy> DivAssign<T> for Vec3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}
