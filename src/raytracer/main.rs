use crate::raytracer::light::{Light, Scatter};
use crate::raytracer::objects;
use crate::raytracer::objects::{Intersection, Sphere};
use crate::raytracer::vec3::{Color, Mat3, Point, Vec3};
use crate::utils;
use wasm_bindgen::prelude::*;

struct Scene {
    camera: Camera,
    lights: Vec<Light>,
    objects: Vec<Sphere>,
    viewport_width: f64,
    viewport_height: f64,
    projection_pane_d: f64,
}

fn new_scene() -> Scene {
    let camera = Camera::new(
        Vec3::new(3., 0., 1.),
        Mat3::new([
            [
                std::f64::consts::FRAC_1_SQRT_2,
                0.,
                -std::f64::consts::FRAC_1_SQRT_2,
            ],
            [0., 1., 0.],
            [
                std::f64::consts::FRAC_1_SQRT_2,
                0.,
                std::f64::consts::FRAC_1_SQRT_2,
            ],
        ]),
    );

    let objects = vec![
        Sphere::new(
            Point::new(0.0, -1.0, 3.0),
            1.0,
            Color::new(255.0, 0.0, 0.0),
            Scatter::Specular { shininess: 500. },
            0.2,
        ),
        Sphere::new(
            Point::new(2.0, 0.0, 4.0),
            1.0,
            Color::new(0.0, 0.0, 255.0),
            Scatter::Specular { shininess: 500. },
            0.3,
        ),
        Sphere::new(
            Point::new(-2.0, 0.0, 4.0),
            1.0,
            Color::new(0.0, 255.0, 0.0),
            Scatter::Specular { shininess: 10. },
            0.4,
        ),
        Sphere::new(
            Point::new(0., -5001.0, 0.0),
            5000.,
            Color::new(255.0, 255.0, 0.0),
            Scatter::Specular { shininess: 1000. },
            0.5,
        ),
    ];

    let lights = vec![
        Light::Ambient { intensity: 0.2 },
        Light::Point {
            intensity: 0.6,
            position: Vec3::new(2., 1., 0.),
        },
        Light::Directional {
            intensity: 0.2,
            direction: Vec3::new(1., 4., 4.),
        },
    ];

    Scene {
        camera,
        lights,
        objects,
        viewport_width: 1.0,
        viewport_height: 1.0,
        projection_pane_d: 1.0,
    }
}

struct Camera {
    position: Vec3,
    rotation: Mat3,
}

impl Camera {
    fn new(position: Vec3, rotation: Mat3) -> Camera {
        Camera { position, rotation }
    }
}

#[wasm_bindgen]
pub fn render(canvas_height: usize, canvas_width: usize) -> Vec<u8> {
    utils::set_panic_hook();

    let mut res = Vec::with_capacity(canvas_width * canvas_height * 4);
    let Scene {
        camera,
        lights,
        objects,
        viewport_width,
        viewport_height,
        projection_pane_d,
    } = new_scene();

    let background_color = Color::new(0., 0., 0.);

    let viewport_width_scale = viewport_width as f64 / canvas_width as f64;
    let viewport_height_scale = viewport_height as f64 / canvas_height as f64;
    for canvas_y in (0..canvas_height as usize).rev() {
        let viewport_y = (canvas_y as f64 - canvas_height as f64 / 2.0) * viewport_height_scale;

        for canvas_x in 0..canvas_width as usize {
            let viewport_x = (canvas_x as f64 - canvas_width as f64 / 2.0) * viewport_width_scale;
            let direction = &camera.rotation * Vec3::new(viewport_x, viewport_y, projection_pane_d);
            let color = trace_ray(
                &camera.position,
                &direction,
                &lights,
                &objects,
                background_color,
                3,
            );

            res.push(color[0].clamp(0., 255.) as u8);
            res.push(color[1].clamp(0., 255.) as u8);
            res.push(color[2].clamp(0., 255.) as u8);
            res.push(255);
        }
    }

    res
}

fn trace_ray(
    origin: &Point,
    direction: &Vec3,
    lights: &Vec<Light>,
    objects: &Vec<Sphere>,
    background_color: Vec3,
    recursions_remaining: u8,
) -> Color {
    if let Some(Intersection {
        p,
        t: _t,
        normal,
        color,
        scatter,
        reflective,
    }) = objects::closest_intersection(&objects, origin, direction, 1.0, f64::INFINITY)
    {
        let local_color = color * scatter.intensity(lights, objects, direction, &p, &normal);
        if recursions_remaining == 0 || reflective == 0.0 {
            local_color
        } else {
            let reflected_direction = (-direction).reflect(&normal);
            let reflected_color = trace_ray(
                &p,
                &reflected_direction,
                lights,
                objects,
                background_color,
                recursions_remaining - 1,
            );
            local_color * (1.0 - reflective) + reflected_color * reflective
        }
    } else {
        background_color
    }
}
