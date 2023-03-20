use crate::rasterizer::point::Point;
use crate::rasterizer::texture::Texture;
use crate::rasterizer::util;
use crate::vec3::{Color, Vec3};
use itertools::Itertools;
use std::iter;

#[derive(Debug, PartialEq, Clone)]
pub enum Surface {
    Texture { index: usize, uvs: [(f64, f64); 3] },
    Color(Color),
}

impl Surface {
    pub fn color_generator(
        &self,
        indexes: [usize; 3],
        vertices: [&Vec3; 3],
        points: [&Point; 3],
    ) -> ColorGenerator {
        match &self {
            Surface::Texture { index, uvs } => {
                let uz = util::edge_interpolate(
                    points[0].y,
                    uvs[indexes[0]].0 as f64 / vertices[0][2],
                    points[1].y,
                    uvs[indexes[1]].0 as f64 / vertices[1][2],
                    points[2].y,
                    uvs[indexes[2]].0 as f64 / vertices[2][2],
                );
                let vz = util::edge_interpolate(
                    points[0].y,
                    uvs[indexes[0]].1 as f64 / vertices[0][2],
                    points[1].y,
                    uvs[indexes[1]].1 as f64 / vertices[1][2],
                    points[2].y,
                    uvs[indexes[2]].1 as f64 / vertices[2][2],
                );
                ColorGenerator::Texture {
                    index: *index,
                    uz,
                    vz,
                }
            }
            Surface::Color(color) => ColorGenerator::Color(*color),
        }
    }
}

pub enum ColorGenerator {
    Texture {
        index: usize,
        uz: [Vec<(i64, f64)>; 2],
        vz: [Vec<(i64, f64)>; 2],
    },
    Color(Color),
}

impl ColorGenerator {
    pub fn colors(
        &self,
        left: usize,
        right: usize,
        i: usize,
        x: [f64; 2],
        inv_z: &Vec<f64>,
        textures: &Vec<Texture>,
    ) -> Vec<Color> {
        let x_left = x[left].floor();
        let x_right = x[right].ceil();
        match &self {
            ColorGenerator::Texture { index, uz, vz } => {
                let texture = &textures[*index];
                let uzscan =
                    util::interpolate(x_left as i64, uz[left][i].1, x_right as i64, uz[right][i].1)
                        .map(|(_i, d)| d);
                let vzscan =
                    util::interpolate(x_left as i64, vz[left][i].1, x_right as i64, vz[right][i].1)
                        .map(|(_i, d)| d);

                uzscan
                    .zip(vzscan)
                    .zip(inv_z.iter())
                    .map(|((u, v), &inv_z)| {
                        let u = u / inv_z;
                        let v = v / inv_z;
                        texture.color_at(u, v)
                    })
                    .collect_vec()
            }
            ColorGenerator::Color(color) => iter::repeat(*color)
                .take((x_right - x_left + 1.0).max(0.0) as usize)
                .collect_vec(),
        }
    }
}
