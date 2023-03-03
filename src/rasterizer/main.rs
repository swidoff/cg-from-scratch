use crate::vec3::{Color, Mat3, Mat4, Vec3, Vec4};
use crate::{log, utils};
use itertools::Itertools;
use wasm_bindgen::prelude::*;
use web_sys::console::trace;

const VIEWPORT_SIZE: f64 = 1.;
const PROJECTION_PLANE_Z: f64 = 1.;

#[wasm_bindgen]
pub fn rasterizer(canvas_height: usize, canvas_width: usize) -> Vec<u8> {
    utils::set_panic_hook();
    let mut canvas = Canvas::new(canvas_height, canvas_width);
    let black = Color::new(0., 0., 0.);
    let red = Color::new(255., 0., 0.);
    let green = Color::new(0., 255., 0.);
    let blue = Color::new(0., 0., 255.);
    let yellow = Color::new(255., 255., 0.);
    let purple = Color::new(255., 0., 255.);
    let cyan = Color::new(0., 255., 255.);

    // Chapter 6
    // canvas.draw_line(&Point::new(-200, -100), &Point::new(240, 120), &black);
    // canvas.draw_line(&Point::new(-50, -200), &Point::new(60, 240), &black);

    // Chapter 7
    // let p0 = Point::new(-200, -250, 1.);
    // let p1 = Point::new(200, 50, 1.);
    // let p2 = Point::new(20, 250, 1.);
    // canvas.draw_filled_triangle(&p0, &p1, &p2, &green);
    // canvas.draw_wire_frame_triangle(&p0, &p1, &p2, &black);

    // Chapter 8
    // let p0 = Point::new(-200, -250, 0.3);
    // let p1 = Point::new(200, 50, 0.1);
    // let p2 = Point::new(20, 250, 1.);
    // canvas.draw_shaded_triangle(&p0, &p1, &p2, &green);

    // Chapter 9
    // let v_a = canvas.project(&Vec3::new(-2., -0.5, 5.));
    // let v_b = canvas.project(&Vec3::new(-2., 0.5, 5.));
    // let v_c = canvas.project(&Vec3::new(-1., 0.5, 5.));
    // let v_d = canvas.project(&Vec3::new(-1., -0.5, 5.));
    // let v_ab = canvas.project(&Vec3::new(-2., -0.5, 6.));
    // let v_bb = canvas.project(&Vec3::new(-2., 0.5, 6.));
    // let v_cb = canvas.project(&Vec3::new(-1., 0.5, 6.));
    // let v_db = canvas.project(&Vec3::new(-1., -0.5, 6.));
    //
    // canvas.draw_line(&v_a, &v_b, &blue);
    // canvas.draw_line(&v_b, &v_c, &blue);
    // canvas.draw_line(&v_c, &v_d, &blue);
    // canvas.draw_line(&v_a, &v_d, &blue);
    //
    // canvas.draw_line(&v_ab, &v_bb, &red);
    // canvas.draw_line(&v_bb, &v_cb, &red);
    // canvas.draw_line(&v_cb, &v_db, &red);
    // canvas.draw_line(&v_ab, &v_db, &red);
    //
    // canvas.draw_line(&v_a, &v_ab, &green);
    // canvas.draw_line(&v_b, &v_bb, &green);
    // canvas.draw_line(&v_c, &v_cb, &green);
    // canvas.draw_line(&v_d, &v_db, &green);

    // Chapter 10 + 11
    let cube_model = Model::new(
        vec![
            Vec3::new(1., 1., 1.),
            Vec3::new(-1., 1., 1.),
            Vec3::new(-1., -1., 1.),
            Vec3::new(1., -1., 1.),
            Vec3::new(1., 1., -1.),
            Vec3::new(-1., 1., -1.),
            Vec3::new(-1., -1., -1.),
            Vec3::new(1., -1., -1.),
        ],
        vec![
            Triangle::new(0, 1, 2, red),
            Triangle::new(0, 2, 3, red),
            Triangle::new(1, 5, 6, yellow),
            Triangle::new(1, 6, 2, yellow),
            Triangle::new(2, 6, 7, cyan),
            Triangle::new(2, 7, 3, cyan),
            Triangle::new(4, 0, 3, green),
            Triangle::new(4, 1, 0, purple),
            Triangle::new(4, 3, 7, green),
            Triangle::new(4, 5, 1, purple),
            Triangle::new(5, 4, 7, blue),
            Triangle::new(5, 7, 6, blue),
        ],
    );
    let sqrt_2 = 2.0_f64.sqrt();
    let camera = Camera::new(
        Vec3::new(-3., 1.0, 2.0),
        Mat3::new_oy_rotation_matrix(-30.),
        vec![
            Plane::new(Vec3::new(0., 0., 1.), -PROJECTION_PLANE_Z), // Near
            Plane::new(Vec3::new(sqrt_2, 0., sqrt_2), 0.),          // Left
            Plane::new(Vec3::new(-sqrt_2, 0., sqrt_2), 0.),         // Right
            Plane::new(Vec3::new(0., sqrt_2, sqrt_2), 0.),          // Bottom
            Plane::new(Vec3::new(0., -sqrt_2, sqrt_2), 0.),         // Top
        ],
    );
    let scene = Scene {
        camera,
        instances: vec![
            Instance::new(&cube_model, 0.75, Mat3::identity(), Vec3::new(-1.5, 0., 7.)),
            Instance::new(
                &cube_model,
                1.0,
                Mat3::new_oy_rotation_matrix(195.),
                Vec3::new(1.25, 2.5, 7.5),
            ),
            Instance::new(
                &cube_model,
                1.0,
                Mat3::new_oy_rotation_matrix(195.),
                Vec3::new(0., 0., -10.),
            ),
            Instance::new(&cube_model, 1.0, Mat3::identity(), Vec3::new(3., -1.5, 6.5)),
        ],
    };

    canvas.render_scene(&scene);
    canvas.pixels
}

struct Canvas {
    height: i64,
    width: i64,
    pixels: Vec<u8>,
    depth_buffer: Vec<f64>,
}

impl Canvas {
    fn new(height: usize, width: usize) -> Canvas {
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
        }
    }

    fn put_pixel(&mut self, x: i64, y: i64, inv_z: f64, color: &Color) {
        let x = self.width / 2 + x;
        let y = self.height / 2 - y - 1;

        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            let depth_offset = (y * self.width + x) as usize;
            if inv_z > self.depth_buffer[depth_offset] {
                println!("{} {} {}", x, y, inv_z);
                let pixel_offset = (y * self.width * 4 + x * 4) as usize;
                self.pixels[pixel_offset] = color[0].clamp(0., 255.) as u8;
                self.pixels[pixel_offset + 1] = color[1].clamp(0., 255.) as u8;
                self.pixels[pixel_offset + 2] = color[2].clamp(0., 255.) as u8;
                self.pixels[pixel_offset + 3] = 255;
                self.depth_buffer[depth_offset] = inv_z;
            }
        }
    }

    fn draw_line(&mut self, p0: &Point, p1: &Point, color: &Color) {
        if (p1.x - p0.x).abs() > (p1.y - p0.y).abs() {
            // line is horizontal-ish
            let (p0, p1) = if p0.x > p1.x { (p1, p0) } else { (p0, p1) };
            for (x, y) in interpolate(p0.x, p0.y as f64, p1.x, p1.y as f64) {
                self.put_pixel(x, y as i64, 0., color);
            }
        } else {
            // line is vertical-ish
            let (p0, p1) = if p0.y > p1.y { (p1, p0) } else { (p0, p1) };
            for (y, x) in interpolate(p0.y, p0.x as f64, p1.y, p1.x as f64) {
                self.put_pixel(x as i64, y, 0., color);
            }
        }
    }

    fn draw_wire_frame_triangle(&mut self, p0: &Point, p1: &Point, p2: &Point, color: &Color) {
        self.draw_line(p0, p1, color);
        self.draw_line(p1, p2, color);
        self.draw_line(p0, p2, color);
    }

    fn draw_filled_triangle(
        &mut self,
        triangle: &Triangle,
        vertices: &Vec<Vec3>,
        projected: &Vec<Point>,
    ) {
        // Find the points along the sides of the triangle.
        let [i0, i1, i2] = triangle.sorted_indexes_by_y(projected);
        let p0 = &projected[i0];
        let p1 = &projected[i1];
        let p2 = &projected[i2];
        let v0 = &vertices[i0];
        let v1 = &vertices[i1];
        let v2 = &vertices[i2];

        let (x02, x012) = edge_interpolate(p0.y, p0.x as f64, p1.y, p1.x as f64, p2.y, p2.x as f64);
        let (iz02, iz012) = edge_interpolate(p0.y, 1. / v0[2], p1.y, 1. / v1[2], p2.y, 1. / v2[2]);

        // Determine which side is the left and which is the right.
        let midpoint = x02.len() / 2;
        let (x_left, x_right, iz_left, iz_right) = if x02[midpoint].1 < x012[midpoint].1 {
            (x02, x012, iz02, iz012)
        } else {
            (x012, x02, iz012, iz02)
        };

        // Draw the horizontal line segments.
        for (i, &(y, x_left)) in x_left.iter().enumerate() {
            let iz_segment = interpolate(
                x_left as i64,
                iz_left[i].1,
                x_right[i].1 as i64,
                iz_right[i].1,
            );

            for (x, inv_z) in iz_segment {
                self.put_pixel(x as i64, y, inv_z, &triangle.color);
            }
        }
    }

    fn draw_shaded_triangle(&mut self, triangle: &Triangle, vertices: &Vec<Point>) {
        let [i0, i1, i2] = triangle.sorted_indexes_by_y(vertices);
        let p0 = &vertices[i0];
        let p1 = &vertices[i1];
        let p2 = &vertices[i2];
        let (x02, x012) = edge_interpolate(p0.y, p0.x as f64, p1.y, p1.x as f64, p2.y, p2.x as f64);
        let (h02, h012) = edge_interpolate(p0.y, p0.h, p1.y, p1.h, p2.y, p2.h);

        // Determine which side is the left and which is the right.
        let midpoint = x02.len() / 2;
        let (left_side, right_side, h_left, h_right) = if x02[midpoint].1 < x012[midpoint].1 {
            (x02, x012, h02, h012)
        } else {
            (x012, x02, h012, h02)
        };

        // Draw the horizontal line segments.
        for (i, &(y, x_left)) in left_side.iter().enumerate() {
            let x_left = x_left as i64;
            let x_right = right_side[i].1 as i64;
            let h_left = h_left[i].1;
            let h_right = h_right[i].1;
            let h_segment = interpolate(x_left, h_left, x_right, h_right);

            for (x, h) in h_segment {
                self.put_pixel(x as i64, y, 0., &(triangle.color * h));
            }
        }
    }

    fn project(&self, v: &Vec4) -> Point {
        let x = v[0] * PROJECTION_PLANE_Z / v[2] * self.width as f64 / VIEWPORT_SIZE;
        let y = v[1] * PROJECTION_PLANE_Z / v[2] * self.height as f64 / VIEWPORT_SIZE;
        Point::new(x as i64, y as i64, 1.)
    }

    fn render_scene(&mut self, scene: &Scene) {
        for instance in scene.instances.iter() {
            if let Some(model) = instance
                .transform_and_clip(&scene.camera.transformation, &scene.camera.clipping_planes)
            {
                self.render_model(&model);
            }
        }
    }

    fn render_model(&mut self, model: &Model) {
        let projected = model
            .vertices
            .iter()
            .map(|v| self.project(&v.to_vec4(1.0)))
            .collect_vec();

        for triangle in model.triangles.iter() {
            // self.draw_wire_frame_triangle(
            //     &projected[triangle.vertex_indices[0]],
            //     &projected[triangle.vertex_indices[1]],
            //     &projected[triangle.vertex_indices[2]],
            //     &triangle.color,
            // )
            self.draw_filled_triangle(triangle, &model.vertices, &projected);
        }
    }
}

struct Point {
    x: i64,
    y: i64,
    h: f64,
}

impl Point {
    fn new(x: i64, y: i64, h: f64) -> Point {
        Point { x, y, h }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Triangle {
    vertex_indices: [usize; 3],
    color: Color,
}

impl Triangle {
    fn new(v1: usize, v2: usize, v3: usize, color: Color) -> Triangle {
        Triangle {
            vertex_indices: [v1, v2, v3],
            color,
        }
    }

    fn sorted_indexes_by_y(&self, vertexes: &Vec<Point>) -> [usize; 3] {
        let mut indexes = self.vertex_indices.clone();
        indexes.sort_by_key(|&i| vertexes[i].y);
        indexes
    }
}

struct Model {
    vertices: Vec<Vec3>,
    triangles: Vec<Triangle>,
}

impl Model {
    fn new(vertices: Vec<Vec3>, triangles: Vec<Triangle>) -> Model {
        Model {
            vertices,
            triangles,
        }
    }

    fn bounding_sphere(&self) -> (Vec3, f64) {
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

struct Instance<'a> {
    model: &'a Model,
    transformation: Mat4,
    scale: f64,
}

impl<'a> Instance<'a> {
    fn new(model: &'a Model, scale: f64, rotation: Mat3, translation: Vec3) -> Instance<'a> {
        let scaling_mat4 = Mat4::new_homogeneous_scaling_matrix(scale);
        let rotation_mat4 = rotation.to_homogenous_rotation();
        let translation_mat4 = translation.to_homogenous_translation();
        let transformation = translation_mat4 * rotation_mat4 * scaling_mat4;

        Instance {
            model,
            transformation,
            scale,
        }
    }

    fn transform_and_clip(
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

            Some(Model::new(vertices, triangles))
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

            triangles.push(Triangle::new(
                v[a_index],
                b_prime_index,
                c_prime_index,
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

            triangles.push(Triangle::new(
                v[a_index],
                v[b_index],
                a_prime_index,
                triangle.color,
            ));
            triangles.push(Triangle::new(
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

struct Plane {
    normal: Vec3,
    distance: f64,
}

impl Plane {
    fn new(normal: Vec3, distance: f64) -> Plane {
        Plane { normal, distance }
    }

    fn segment_intersection(&self, a: &Vec3, b: &Vec3) -> Vec3 {
        let b_minus_a = b - a;
        let t = -self.distance - self.normal.dot(a) / self.normal.dot(&b_minus_a);
        a + b_minus_a * t
    }
}

struct Camera {
    transformation: Mat4,
    clipping_planes: Vec<Plane>,
}

impl Camera {
    fn new(position: Vec3, orientation: Mat3, clipping_planes: Vec<Plane>) -> Camera {
        let transformation = orientation.transpose().to_homogenous_rotation()
            * (position * -1.).to_homogenous_translation();
        Camera {
            transformation,
            clipping_planes,
        }
    }
}

struct Scene<'a> {
    camera: Camera,
    instances: Vec<Instance<'a>>,
}

fn interpolate(i0: i64, d0: f64, i1: i64, d1: f64) -> impl Iterator<Item = (i64, f64)> {
    let a = if i0 == i1 {
        0.
    } else {
        ((d1 - d0) as f64) / ((i1 - i0) as f64)
    };

    (i0..(i1 + 1)).scan(d0 as f64, move |d, i| {
        let res = Some((i, *d));
        *d += a;
        res
    })
}

fn edge_interpolate(
    y0: i64,
    v0: f64,
    y1: i64,
    v1: f64,
    y2: i64,
    v2: f64,
) -> (Vec<(i64, f64)>, Vec<(i64, f64)>) {
    let v01 = interpolate(y0, v0, y1, v1);
    let v02 = interpolate(y0, v0, y2, v2).collect_vec(); // Long size
    let v12 = interpolate(y1, v1, y2, v2);

    // Concatenate the two short sides.
    let x012 = v01.dropping(1).chain(v12).collect_vec();
    (v02, x012)
}

#[cfg(test)]
mod tests {
    use crate::rasterizer::main::rasterizer;

    #[test]
    fn test_rasterizer() {
        let res = rasterizer(600, 600);
    }
}
