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

    pub fn cross_product(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(
            (self.e[1] * rhs.e[2]) - (self.e[2] * rhs.e[1]),
            (self.e[2] * rhs.e[0]) - (self.e[0] * rhs.e[2]),
            (self.e[0] * rhs.e[1]) - (self.e[1] * rhs.e[0]),
        )
    }

    pub fn len(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        normal * normal.dot(self) * 2. - self
    }

    pub fn to_vec4(&self, e4: f64) -> Vec4 {
        Vec4::new(self.e[0], self.e[1], self.e[2], e4)
    }

    pub fn to_homogenous_scale(&self) -> Mat4 {
        Mat4 {
            e: [
                [self.e[0], 0., 0., 0.],
                [0., self.e[1], 0., 0.],
                [0., 0., self.e[2], 0.],
                [0., 0., 0., 1.],
            ],
        }
    }

    pub fn to_homogenous_translation(&self) -> Mat4 {
        Mat4 {
            e: [
                [1., 0., 0., self.e[0]],
                [0., 1., 0., self.e[1]],
                [0., 0., 1., self.e[2]],
                [0., 0., 0., 1.],
            ],
        }
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

impl Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        &self + rhs
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

    pub fn identity() -> Mat3 {
        Mat3 {
            e: [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
        }
    }

    pub fn new_oy_rotation_matrix(degrees: f64) -> Mat3 {
        let cos = (degrees * std::f64::consts::PI / 180.0).cos();
        let sin = (degrees * std::f64::consts::PI / 180.0).sin();
        Mat3::new([[cos, 0., -sin], [0., 1., 0.], [sin, 0., cos]])
    }

    pub fn to_homogenous_rotation(&self) -> Mat4 {
        Mat4 {
            e: [
                [self.e[0][0], self.e[0][1], self.e[0][2], 0.],
                [self.e[1][0], self.e[1][1], self.e[1][2], 0.],
                [self.e[2][0], self.e[2][1], self.e[2][2], 0.],
                [0., 0., 0., 1.],
            ],
        }
    }

    pub fn transpose(&self) -> Mat3 {
        Mat3 {
            e: [
                [self.e[0][0], self.e[1][0], self.e[2][0]],
                [self.e[0][1], self.e[1][1], self.e[2][1]],
                [self.e[0][2], self.e[1][2], self.e[2][2]],
            ],
        }
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec4 {
    e: [f64; 4],
}

impl Vec4 {
    pub fn new(e1: f64, e2: f64, e3: f64, e4: f64) -> Vec4 {
        Vec4 {
            e: [e1, e2, e3, e4],
        }
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        (self.e[0] * rhs.e[0]) + (self.e[1] * rhs.e[1]) + (self.e[2] * rhs.e[2])
    }

    pub fn to_vec3(&self) -> Vec3 {
        return Vec3::new(self.e[0], self.e[1], self.e[2]);
    }
}

impl Index<usize> for Vec4 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Mat4 {
    e: [[f64; 4]; 4],
}

impl Mat4 {
    pub fn identity() -> Mat4 {
        Mat4 {
            e: [
                [1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ],
        }
    }

    pub fn new_homogeneous_scaling_matrix(scale: f64) -> Mat4 {
        Mat4 {
            e: [
                [scale, 0., 0., 0.],
                [0., scale, 0., 0.],
                [0., 0., scale, 0.],
                [0., 0., 0., 1.],
            ],
        }
    }
}

impl Mul<Vec4> for &Mat4 {
    type Output = Vec4;

    fn mul(self, v: Vec4) -> Self::Output {
        Vec4::new(
            self.e[0][0] * v[0] + self.e[0][1] * v[1] + self.e[0][2] * v[2] + self.e[0][3] * v[3],
            self.e[1][0] * v[0] + self.e[1][1] * v[1] + self.e[1][2] * v[2] + self.e[1][3] * v[3],
            self.e[2][0] * v[0] + self.e[2][1] * v[1] + self.e[2][2] * v[2] + self.e[2][3] * v[3],
            self.e[3][0] * v[0] + self.e[3][1] * v[1] + self.e[3][2] * v[2] + self.e[3][3] * v[3],
        )
    }
}

impl Mul<&Mat4> for &Mat4 {
    type Output = Mat4;

    fn mul(self, m: &Mat4) -> Self::Output {
        let mut e = [[0.; 4]; 4];
        for r in 0..4 {
            for c in 0..4 {
                for k in 0..4 {
                    e[r][c] += self.e[r][k] * m.e[k][c];
                }
            }
        }
        Mat4 { e }
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, m: Mat4) -> Self::Output {
        &self * &m
    }
}
