use crate::rasterizer::light::Scatter;
use crate::rasterizer::triangle::Triangle;
use crate::vec3::{Color, Vec3};
use std::f64::consts::PI;

pub struct Model {
    pub vertices: Vec<Vec3>,
    pub triangles: Vec<Triangle>,
    pub scatter: Scatter,
    pub bounds_center: Vec3,
    pub bounds_radius: f64,
}

impl Model {
    pub fn new(vertices: Vec<Vec3>, triangles: Vec<Triangle>, scatter: Scatter) -> Model {
        let bounds_center = vertices
            .iter()
            .fold(Vec3::new(0., 0., 0.), |v1, v2| v1 + v2)
            / vertices.len() as f64;
        let bounds_radius = vertices
            .iter()
            .map(|v| (v - &bounds_center).len())
            .max_by(|v1, v2| v1.partial_cmp(v2).unwrap())
            .unwrap();

        Model {
            vertices,
            triangles,
            scatter,
            bounds_center,
            bounds_radius,
        }
    }

    pub fn make_sphere(divs: usize, color: Color, scatter: Scatter) -> Model {
        let mut vertices = Vec::new();
        let mut triangles = Vec::new();
        let n_divs = divs as f64;
        let delta_angle = 2.0 * PI / n_divs;

        for d in 0..(divs + 1) {
            let y = (2.0 / n_divs) * (d as f64 - n_divs / 2.0);
            let radius = (1.0 - y * y).sqrt();
            for i in 0..divs {
                let x = radius * (i as f64 * delta_angle).cos();
                let z = radius * (i as f64 * delta_angle).sin();
                vertices.push(Vec3::new(x, y, z));
            }
        }

        for d in 0..divs {
            for i in 0..divs {
                let i0 = d * divs + i;
                let i1 = (d + 1) * divs + (i + 1) % divs;
                let i2 = divs * d + (i + 1) % divs;
                triangles.push(Triangle::new(
                    i0,
                    i1,
                    i2,
                    color,
                    vertices[i0],
                    vertices[i1],
                    vertices[i2],
                ));
                triangles.push(Triangle::new(
                    i0,
                    i0 + divs,
                    i1,
                    color,
                    vertices[i0],
                    vertices[i0 + divs],
                    vertices[i1],
                ));
            }
        }

        Model {
            vertices,
            triangles,
            scatter,
            bounds_center: Vec3::new(0., 0., 0.),
            bounds_radius: 1.0,
        }
    }
}
