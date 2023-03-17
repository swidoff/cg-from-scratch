use crate::rasterizer::camera::Camera;
use crate::rasterizer::light::{Light, Scatter};
use crate::rasterizer::model::Model;
use crate::rasterizer::point::Point;
use crate::rasterizer::scene::Scene;
use crate::rasterizer::shading::ShadingModel;
use crate::rasterizer::triangle::Triangle;
use crate::rasterizer::util;
use crate::vec3::{Color, Mat3, Vec3, Vec4};
use itertools::Itertools;

pub struct Canvas {
    pub height: i64,
    pub width: i64,
    pub pixels: Vec<u8>,
    pub depth_buffer: Vec<f64>,
    pub shading_model: ShadingModel,
}

impl Canvas {
    pub fn new(height: usize, width: usize, shading_model: ShadingModel) -> Canvas {
        let dim = width * height;
        let pixel_bytes = dim * 4;
        let mut pixels = Vec::with_capacity(pixel_bytes);
        for _i in 0..pixel_bytes {
            pixels.push(0);
        }

        let mut depth_buffer = Vec::with_capacity(dim);
        for _i in 0..dim {
            depth_buffer.push(f64::NEG_INFINITY);
        }

        Canvas {
            height: height as i64,
            width: width as i64,
            pixels,
            depth_buffer,
            shading_model,
        }
    }

    fn put_pixel(&mut self, x: i64, y: i64, inv_z: f64, color: &Color) {
        let x = self.width / 2 + x;
        let y = self.height / 2 - y - 1;

        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            let depth_offset = (y * self.width + x) as usize;
            if inv_z > self.depth_buffer[depth_offset] {
                let pixel_offset = (y * self.width * 4 + x * 4) as usize;
                self.pixels[pixel_offset] = color[0].clamp(0., 255.) as u8;
                self.pixels[pixel_offset + 1] = color[1].clamp(0., 255.) as u8;
                self.pixels[pixel_offset + 2] = color[2].clamp(0., 255.) as u8;
                self.pixels[pixel_offset + 3] = 255;
                self.depth_buffer[depth_offset] = inv_z;
            }
        }
    }

    fn render_triangle(
        &mut self,
        triangle: &Triangle,
        vertices: &Vec<Vec3>,
        projected: &Vec<Point>,
        camera: &Camera,
        lights: &Vec<Light>,
        scatter: &Scatter,
        orientation: &Mat3,
    ) {
        // Backface Culling
        let normal = triangle.normal(vertices);
        let center = -triangle.center(vertices);
        if center.dot(&normal) <= 0. {
            return;
        }

        // Find the points along the sides of the triangle.
        let [i0, i1, i2] = triangle.sorted_indexes_by_y(projected);
        let v0 = &vertices[triangle.vertex_indices[i0]];
        let v1 = &vertices[triangle.vertex_indices[i1]];
        let v2 = &vertices[triangle.vertex_indices[i2]];
        let p0 = &projected[triangle.vertex_indices[i0]];
        let p1 = &projected[triangle.vertex_indices[i1]];
        let p2 = &projected[triangle.vertex_indices[i2]];

        let x_edges =
            util::edge_interpolate(p0.y, p0.x as f64, p1.y, p1.x as f64, p2.y, p2.x as f64);
        let iz_edges = util::edge_interpolate(p0.y, 1. / v0[2], p1.y, 1. / v1[2], p2.y, 1. / v2[2]);

        // Determine which side is the left and which is the right.
        let midpoint = x_edges[0].len() / 2;
        let (left, right) = if x_edges[0][midpoint].1 < x_edges[1][midpoint].1 {
            (0, 1)
        } else {
            (1, 0)
        };

        let vertex_normals = triangle.normals.map(|normals| {
            let transformation = camera.orientation.transpose().to_homogenous_rotation()
                * orientation.to_homogenous_rotation();
            [
                (&transformation * normals[i0].to_vec4(1.0)).to_vec3(),
                (&transformation * normals[i1].to_vec4(1.0)).to_vec3(),
                (&transformation * normals[i2].to_vec4(1.0)).to_vec3(),
            ]
        });

        let normals = match &vertex_normals {
            None => [&normal, &normal, &normal],
            Some(normals) => [&normals[0], &normals[1], &normals[2]],
        };
        // let normals = [&normal, &normal, &normal];

        let shader =
            self.shading_model
                .shader([v0, v1, v2], [p0, p1, p2], normals, camera, lights, scatter);

        // Draw the horizontal line segments.
        for (yi, &(y, x_left)) in x_edges[left].iter().enumerate() {
            let x_left = x_left as i64;
            let x_right = x_edges[right][yi].1 as i64;
            let iz_left = iz_edges[left][yi].1;
            let iz_right = iz_edges[right][yi].1;
            let iz_segment = util::interpolate(x_left, iz_left, x_right, iz_right)
                .map(|(_x, inv_z)| inv_z)
                .collect_vec();

            let intensities = shader.intensities(
                left,
                right,
                yi,
                y,
                [x_edges[0][yi].1 as i64, x_edges[1][yi].1 as i64],
                &iz_segment,
                self.width,
                self.height,
                camera,
                lights,
                scatter,
            );

            for (xi, x) in (x_left..(x_right + 1)).enumerate() {
                let inv_z = iz_segment[xi];
                self.put_pixel(x as i64, y, inv_z, &(&triangle.color * intensities[xi]));
            }
        }
    }

    fn project(&self, v: &Vec4) -> Point {
        util::project_vertex(v, self.width, self.height)
    }

    pub fn render_scene(&mut self, scene: &Scene) {
        for instance in scene.instances.iter() {
            if let Some(model) = instance
                .transform_and_clip(&scene.camera.transformation, &scene.camera.clipping_planes)
            {
                self.render_model(&model, &scene.camera, &scene.lights, &instance.rotation);
            }
        }
    }

    fn render_model(
        &mut self,
        model: &Model,
        camera: &Camera,
        lights: &Vec<Light>,
        orientation: &Mat3,
    ) {
        let projected = model
            .vertices
            .iter()
            .map(|v| self.project(&v.to_vec4(1.0)))
            .collect_vec();

        for triangle in model.triangles.iter() {
            self.render_triangle(
                triangle,
                &model.vertices,
                &projected,
                camera,
                lights,
                &model.scatter,
                orientation,
            );
        }
    }
}
