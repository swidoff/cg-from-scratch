use crate::rasterizer::camera::Camera;
use crate::rasterizer::instance::Instance;
use crate::rasterizer::light::Light;
use crate::rasterizer::texture::Texture;

pub struct Scene<'a> {
    pub camera: Camera,
    pub instances: Vec<Instance<'a>>,
    pub lights: Vec<Light>,
    pub textures: Vec<Texture>,
}
