use std::ops::{Add, Div, Index, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Color = Vec3;
pub type Point = Vec3;

impl Vec3 {
    pub fn new(e1: f64, e2: f64, e3: f64) -> Vec3 {
        Vec3 { e: [e1, e2, e3] }
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        (self.e[0] * rhs.e[0]) + (self.e[1] * rhs.e[1]) + (self.e[2] * rhs.e[2])
    }

    pub fn len(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        normal * normal.dot(self) * 2. - self
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        let e = [-self.e[0], -self.e[1], -self.e[2]];
        Vec3 { e }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        -&self
    }
}

impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        let e = [
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        ];
        Vec3 { e }
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        &self - rhs
    }
}

impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        let e = [
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        ];
        Vec3 { e }
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        self + &rhs
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        &self + &rhs
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        let e = [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs];
        Vec3 { e }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        &self * rhs
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        let e = [self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs];
        Vec3 { e }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        &self / rhs
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Mat3 {
    e: [[f64; 3]; 3],
}

impl Mat3 {
    pub fn new(e: [[f64; 3]; 3]) -> Mat3 {
        Mat3 { e }
    }
}

impl Mul<Vec3> for &Mat3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        Vec3::new(
            self.e[0][0] * v[0] + self.e[0][1] * v[1] + self.e[0][2] * v[2],
            self.e[1][0] * v[0] + self.e[1][1] * v[1] + self.e[1][2] * v[2],
            self.e[2][0] * v[0] + self.e[2][1] * v[1] + self.e[2][2] * v[2],
        )
    }
}
