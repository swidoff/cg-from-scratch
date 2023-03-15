use crate::rasterizer::plane::Plane;
use crate::vec3::{Mat3, Mat4, Vec3};

pub struct Camera {
    pub position: Vec3,
    pub orientation: Mat3,
    pub transformation: Mat4,
    pub clipping_planes: Vec<Plane>,
}

impl Camera {
    pub fn new(position: Vec3, orientation: Mat3, clipping_planes: Vec<Plane>) -> Camera {
        let transformation = orientation.transpose().to_homogenous_rotation()
            * (position * -1.).to_homogenous_translation();
        Camera {
            position,
            orientation,
            transformation,
            clipping_planes,
        }
    }
}
