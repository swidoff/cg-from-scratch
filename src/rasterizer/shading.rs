use crate::rasterizer::camera::Camera;
use crate::rasterizer::light::{Light, Scatter};
use crate::rasterizer::point::Point;
use crate::rasterizer::util;
use crate::vec3::Vec3;
use itertools::Itertools;
use std::iter;

pub enum ShadingModel {
    Flat,
    Gouraud,
    Phong,
}

impl ShadingModel {
    pub fn shader(
        &self,
        vertices: [&Vec3; 3],
        points: [&Point; 3],
        normals: [&Vec3; 3],
        camera: &Camera,
        lights: &Vec<Light>,
        scatter: &Scatter,
    ) -> Shader {
        match self {
            ShadingModel::Flat => {
                let center = (vertices[0] + vertices[1] + vertices[2]) / 3.0;
                let intensity = scatter.intensity(&center, &normals[0], camera, lights);
                Shader::Flat { intensity }
            }
            ShadingModel::Gouraud => {
                let i0 = scatter.intensity(&vertices[0], &normals[0], camera, lights);
                let i1 = scatter.intensity(&vertices[1], &normals[1], camera, lights);
                let i2 = scatter.intensity(&vertices[2], &normals[2], camera, lights);
                let i_edges =
                    util::edge_interpolate(points[0].y, i0, points[1].y, i1, points[2].y, i2);
                Shader::Gouraud { i_edges }
            }
            ShadingModel::Phong => {
                let nx_edges = util::edge_interpolate(
                    points[0].y,
                    normals[0][0],
                    points[1].y,
                    normals[1][0],
                    points[2].y,
                    normals[2][0],
                );
                let ny_edges = util::edge_interpolate(
                    points[0].y,
                    normals[0][1],
                    points[1].y,
                    normals[1][1],
                    points[2].y,
                    normals[2][1],
                );
                let nz_edges = util::edge_interpolate(
                    points[0].y,
                    normals[0][2],
                    points[1].y,
                    normals[1][2],
                    points[2].y,
                    normals[2][2],
                );
                Shader::Phong {
                    nx_edges,
                    ny_edges,
                    nz_edges,
                }
            }
        }
    }
}

pub enum Shader {
    Flat {
        intensity: f64,
    },
    Gouraud {
        i_edges: [Vec<(i64, f64)>; 2],
    },
    Phong {
        nx_edges: [Vec<(i64, f64)>; 2],
        ny_edges: [Vec<(i64, f64)>; 2],
        nz_edges: [Vec<(i64, f64)>; 2],
    },
}

impl Shader {
    pub fn intensities(
        &self,
        left: usize,
        right: usize,
        i: usize,
        y: i64,
        x: [f64; 2],
        inv_z: &Vec<f64>,
        canvas_width: i64,
        canvas_height: i64,
        camera: &Camera,
        lights: &Vec<Light>,
        scatter: &Scatter,
    ) -> Vec<f64> {
        match self {
            &Shader::Flat { intensity } => iter::repeat(intensity)
                .take((x[right].ceil() - x[left].floor() + 1.0).max(0.0) as usize)
                .collect_vec(),
            Shader::Gouraud { i_edges } => util::interpolate(
                x[left].floor() as i64,
                i_edges[left][i].1,
                x[right].ceil() as i64,
                i_edges[right][i].1,
            )
            .map(|(_i, d)| d)
            .collect_vec(),
            Shader::Phong {
                nx_edges,
                ny_edges,
                nz_edges,
            } => {
                let nxscan = util::interpolate(
                    x[left].floor() as i64,
                    nx_edges[left][i].1 as f64,
                    x[right].ceil() as i64,
                    nx_edges[right][i].1 as f64,
                );
                let nyscan = util::interpolate(
                    x[left].floor() as i64,
                    ny_edges[left][i].1 as f64,
                    x[right].ceil() as i64,
                    ny_edges[right][i].1 as f64,
                )
                .map(|(_i, d)| d);
                let nzscan = util::interpolate(
                    x[left].floor() as i64,
                    nz_edges[left][i].1 as f64,
                    x[right].ceil() as i64,
                    nz_edges[right][i].1 as f64,
                )
                .map(|(_i, d)| d);

                iter::zip(iter::zip(iter::zip(nxscan, nyscan), nzscan), inv_z.iter())
                    .map(|((((x, nx), ny), nz), &inv_z)| {
                        let vertex = util::unproject_vertex(
                            x as f64,
                            y as f64,
                            inv_z,
                            canvas_width,
                            canvas_height,
                        );
                        let normal = Vec3::new(nx, ny, nz);
                        scatter.intensity(&vertex, &normal, camera, lights)
                    })
                    .collect_vec()
            }
        }
    }
}
