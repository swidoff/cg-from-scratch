use crate::log;
use crate::utils;
use std::ops::{Index, Sub};
use wasm_bindgen::prelude::*;

#[derive(Debug, PartialEq, Clone)]
struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    fn new(e1: f64, e2: f64, e3: f64) -> Vec3 {
        Vec3 { e: [e1, e2, e3] }
    }

    fn dot(&self, rhs: &Vec3) -> f64 {
        (self.e[0] * rhs.e[0]) + (self.e[1] * rhs.e[1]) + (self.e[2] * rhs.e[2])
    }
}

impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        let e = [
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        ];
        Vec3 { e }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

struct Ray {
    origin: Point,
    direction: Vec3,
}

impl Ray {
    fn new(origin: Point, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }
}

struct Intersection {
    t: f64,
    color: Color,
}

trait SceneObject {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection>;
}

struct Sphere {
    center: Point,
    radius: f64,
    color: Color,
}

impl Sphere {
    fn new(center: Point, radius: f64, color: Color) -> Sphere {
        Sphere {
            center,
            radius,
            color,
        }
    }
}

impl SceneObject for Sphere {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let d = &ray.direction;
        let r = self.radius;
        let co = &ray.origin - &self.center;

        let a = d.dot(d);
        let b = 2.0 * co.dot(d);
        let c = co.dot(&co) - r * r;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            None
        } else {
            let solutions = [
                (-b + discriminant.sqrt()) / (2.0 * a),
                (-b - discriminant.sqrt()) / (2.0 * a),
            ];
            solutions
                .iter()
                .filter(|&&t| t >= t_min && t <= t_max)
                .min_by(|&&t1, &t2| t1.partial_cmp(t2).unwrap())
                .map(|&t| Intersection {
                    t,
                    color: self.color.clone(),
                })
        }
    }
}

type Color = Vec3;
type Point = Vec3;

fn new_scene() -> Vec<Box<dyn SceneObject>> {
    vec![
        Box::new(Sphere::new(
            Point::new(0.0, -1.0, 3.0),
            1.0,
            Color::new(255.0, 0.0, 0.0),
        )),
        Box::new(Sphere::new(
            Point::new(2.0, 0.0, 4.0),
            1.0,
            Color::new(0.0, 0.0, 255.0),
        )),
        Box::new(Sphere::new(
            Point::new(-2.0, 0.0, 4.0),
            1.0,
            Color::new(0.0, 255.0, 0.0),
        )),
    ]
}

#[wasm_bindgen]
pub fn render(canvas_height: usize, canvas_width: usize) -> Vec<u8> {
    utils::set_panic_hook();

    let mut res = Vec::with_capacity(canvas_width * canvas_height * 4);
    let scene = new_scene();
    let viewport_width = 1.0;
    let viewport_height = 1.0;
    let projection_pane_d = 1.0;
    let camera = Vec3::new(0.0, 0.0, 0.0);
    let background_color = Color::new(255., 255., 255.);

    let viewport_width_scale = viewport_width as f64 / canvas_width as f64;
    let viewport_height_scale = viewport_height as f64 / canvas_height as f64;
    for canvas_y in (0..canvas_height as usize).rev() {
        let viewport_y = (canvas_y as f64 - canvas_height as f64 / 2.0) * viewport_height_scale;

        for canvas_x in 0..canvas_width as usize {
            let viewport_x = (canvas_x as f64 - canvas_width as f64 / 2.0) * viewport_width_scale;
            let direction = Vec3::new(viewport_x, viewport_y, projection_pane_d);
            let ray = Ray::new(camera.clone(), direction);

            let color = trace_ray(&ray, &scene, 1.0, f64::INFINITY)
                .unwrap_or_else(|| background_color.clone());

            res.push(color[0] as u8);
            res.push(color[1] as u8);
            res.push(color[2] as u8);
            res.push(255);
        }
    }

    res
}

fn trace_ray(
    ray: &Ray,
    scene: &Vec<Box<dyn SceneObject>>,
    t_min: f64,
    t_max: f64,
) -> Option<Color> {
    scene
        .iter()
        .map(|obj| obj.intersect(ray, t_min, t_max))
        .filter_map(|obj| obj)
        .min_by(|obj1, obj2| obj1.t.partial_cmp(&obj2.t).unwrap())
        .map(|obj| obj.color)
}
