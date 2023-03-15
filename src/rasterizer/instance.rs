use crate::rasterizer::model::Model;
use crate::rasterizer::plane::Plane;
use crate::rasterizer::triangle::Triangle;
use crate::vec3::{Mat3, Mat4, Vec3};
use itertools::Itertools;

pub struct Instance<'a> {
    pub model: &'a Model,
    pub scale: f64,
    pub rotation: Mat3,
    pub translation: Vec3,
    pub transformation: Mat4,
}

impl<'a> Instance<'a> {
    pub fn new(model: &'a Model, scale: f64, rotation: Mat3, translation: Vec3) -> Instance<'a> {
        let scaling_mat4 = Mat4::new_homogeneous_scaling_matrix(scale);
        let rotation_mat4 = rotation.to_homogenous_rotation();
        let translation_mat4 = translation.to_homogenous_translation();
        let transformation = translation_mat4 * rotation_mat4 * scaling_mat4;

        Instance {
            model,
            scale,
            rotation,
            translation,
            transformation,
        }
    }

    pub fn transform_and_clip(
        &self,
        camera_transformation: &Mat4,
        clipping_planes: &Vec<Plane>,
    ) -> Option<Model> {
        let transformation = camera_transformation * &self.transformation;
        let (bounding_sphere_center, bounding_sphere_radius) = self.model.bounding_sphere();
        let bounding_sphere_center = &transformation * bounding_sphere_center.to_vec4(1.0);
        let bounding_sphere_radius = bounding_sphere_radius * self.scale;

        let outside_any_clipping_plane = clipping_planes.iter().any(|plane| {
            let distance = bounding_sphere_center.dot(&plane.normal) + plane.distance;
            distance < -bounding_sphere_radius
        });
        if outside_any_clipping_plane {
            None
        } else {
            let mut vertices = self
                .model
                .vertices
                .iter()
                .map(|v| (&transformation * v.to_vec4(1.0)).to_vec3())
                .collect_vec();

            let mut triangles = Vec::from(&self.model.triangles[..]);
            for plane in clipping_planes {
                let mut new_triangles = Vec::new();
                for triangle in triangles {
                    Self::clip_triangle(triangle, plane, &mut vertices, &mut new_triangles);
                }
                triangles = new_triangles;
            }

            Some(Model::new(vertices, triangles, self.model.scatter))
        }
    }

    fn clip_triangle(
        triangle: Triangle,
        plane: &Plane,
        vertices: &mut Vec<Vec3>,
        triangles: &mut Vec<Triangle>,
    ) {
        let v = triangle.vertex_indices;
        let in_plane = v
            .iter()
            .map(|&v| (plane.normal.dot(&vertices[v]) + plane.distance))
            .collect_vec();
        let in_count = in_plane.iter().filter(|&&d| d > 0.).count();

        if in_count == 1 {
            // Let A be the vertex with a positive distance
            // compute B' = Intersection(AB, plane)
            // compute C' = Intersection(AC, plane)
            // return [Triangle(A, B', C')]
            let a_index = in_plane
                .iter()
                .position_max_by(|&a, &b| a.partial_cmp(b).unwrap())
                .unwrap();
            let (b_index, c_index) = (0..3).filter(|&i| i != a_index).collect_tuple().unwrap();
            let b_prime = plane.segment_intersection(&vertices[v[a_index]], &vertices[v[b_index]]);
            let c_prime = plane.segment_intersection(&vertices[v[a_index]], &vertices[v[c_index]]);

            let b_prime_index = vertices.len();
            let c_prime_index = vertices.len() + 1;
            vertices.push(b_prime);
            vertices.push(c_prime);

            triangles.push(Triangle::new_no_normals(
                v[a_index],
                c_prime_index,
                b_prime_index,
                triangle.color,
            ))
        } else if in_count == 2 {
            // Let C be the vertex with a negative distance
            // compute A' = Intersection(AC, plane)
            // compute B' = Intersection(BC, plane)
            // return [Triangle(A, B, A'), Triangle(A', B, B')]
            let c_index = in_plane
                .iter()
                .position_min_by(|&a, &b| a.partial_cmp(b).unwrap())
                .unwrap();
            let (a_index, b_index) = (0..3).filter(|&i| i != c_index).collect_tuple().unwrap();

            let a_prime = plane.segment_intersection(&vertices[v[a_index]], &vertices[v[c_index]]);
            let b_prime = plane.segment_intersection(&vertices[v[b_index]], &vertices[v[c_index]]);

            let a_prime_index = vertices.len();
            let b_prime_index = vertices.len() + 1;
            vertices.push(a_prime);
            vertices.push(b_prime);

            triangles.push(Triangle::new_no_normals(
                v[a_index],
                v[b_index],
                a_prime_index,
                triangle.color,
            ));
            triangles.push(Triangle::new_no_normals(
                a_prime_index,
                v[b_index],
                b_prime_index,
                triangle.color,
            ));
        } else if in_count == 3 {
            triangles.push(triangle);
        }
    }
}
