use crate::rasterizer::light::Scatter;
use crate::rasterizer::triangle::Triangle;
use crate::vec3::Vec3;

pub struct Model {
    pub vertices: Vec<Vec3>,
    pub triangles: Vec<Triangle>,
    pub scatter: Scatter,
}

impl Model {
    pub fn new(vertices: Vec<Vec3>, triangles: Vec<Triangle>, scatter: Scatter) -> Model {
        Model {
            vertices,
            triangles,
            scatter,
        }
    }

    pub fn bounding_sphere(&self) -> (Vec3, f64) {
        let bounding_sphere_center = self
            .vertices
            .iter()
            .fold(Vec3::new(0., 0., 0.), |v1, v2| v1 + v2)
            / self.vertices.len() as f64;
        let bounding_sphere_radius = self
            .vertices
            .iter()
            .map(|v| (v - &bounding_sphere_center).len())
            .max_by(|v1, v2| v1.partial_cmp(v2).unwrap())
            .unwrap();
        (bounding_sphere_center, bounding_sphere_radius)
    }
}
