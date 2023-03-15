use crate::rasterizer::camera::Camera;
use crate::rasterizer::instance::Instance;
use crate::rasterizer::light::Light;

pub struct Scene<'a> {
    pub camera: Camera,
    pub instances: Vec<Instance<'a>>,
    pub lights: Vec<Light>,
}
