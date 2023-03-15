use crate::rasterizer::point::Point;
use crate::vec3::{Color, Vec3};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Triangle {
    pub vertex_indices: [usize; 3],
    pub color: Color,
    pub normals: Option<[Vec3; 3]>,
}

impl Triangle {
    pub fn new(
        v1: usize,
        v2: usize,
        v3: usize,
        color: Color,
        n1: Vec3,
        n2: Vec3,
        n3: Vec3,
    ) -> Triangle {
        Triangle {
            vertex_indices: [v1, v2, v3],
            color,
            normals: Some([n1, n2, n3]),
        }
    }

    pub fn new_no_normals(v1: usize, v2: usize, v3: usize, color: Color) -> Triangle {
        Triangle {
            vertex_indices: [v1, v2, v3],
            color,
            normals: None,
        }
    }

    pub fn sorted_indexes_by_y(&self, vertexes: &Vec<Point>) -> [usize; 3] {
        let mut indexes = self.vertex_indices.clone();
        indexes.sort_by_key(|&i| vertexes[i].y);
        indexes
    }

    pub fn normal(&self, vertices: &Vec<Vec3>) -> Vec3 {
        let v1 = vertices[self.vertex_indices[1]] - &vertices[self.vertex_indices[0]];
        let v2 = vertices[self.vertex_indices[2]] - &vertices[self.vertex_indices[0]];
        v1.cross_product(&v2)
    }

    pub fn center(&self, vertices: &Vec<Vec3>) -> Vec3 {
        (vertices[self.vertex_indices[0]]
            + vertices[self.vertex_indices[1]]
            + vertices[self.vertex_indices[2]])
            / 3.
    }
}
