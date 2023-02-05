use crate::raytracer::light::{Light, Reflection};
use crate::raytracer::objects::Sphere;
use crate::raytracer::ray::Ray;
use crate::raytracer::vec3::{Color, Point, Vec3};
use crate::utils;
use wasm_bindgen::prelude::*;

struct Scene {
    lights: Vec<Light>,
    objects: Vec<Sphere>,
    viewport_width: f64,
    viewport_height: f64,
    projection_pane_d: f64,
}

fn new_scene() -> Scene {
    let objects = vec![
        Sphere::new(
            Point::new(0.0, -1.0, 3.0),
            1.0,
            Color::new(255.0, 0.0, 0.0),
            Reflection::Specular { shininess: 500. },
        ),
        Sphere::new(
            Point::new(2.0, 0.0, 4.0),
            1.0,
            Color::new(0.0, 0.0, 255.0),
            Reflection::Specular { shininess: 500. },
        ),
        Sphere::new(
            Point::new(-2.0, 0.0, 4.0),
            1.0,
            Color::new(0.0, 255.0, 0.0),
            Reflection::Specular { shininess: 10. },
        ),
        Sphere::new(
            Point::new(0., -5001.0, 0.0),
            5000.,
            Color::new(255.0, 255.0, 0.0),
            Reflection::Specular { shininess: 1000. },
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
        lights,
        objects,
        viewport_width: 1.0,
        viewport_height: 1.0,
        projection_pane_d: 1.0,
    }
}

#[wasm_bindgen]
pub fn render(canvas_height: usize, canvas_width: usize) -> Vec<u8> {
    utils::set_panic_hook();

    let mut res = Vec::with_capacity(canvas_width * canvas_height * 4);
    let Scene {
        lights,
        objects,
        viewport_width,
        viewport_height,
        projection_pane_d,
    } = new_scene();

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

            let color = objects
                .iter()
                .filter_map(|obj| obj.intersect(&ray, 1.0, f64::INFINITY))
                .min_by(|int1, int2| int1.t.partial_cmp(&int2.t).unwrap())
                .map(|int| int.color * int.reflection.intensity(&lights, &ray, &int.p, &int.normal))
                .unwrap_or_else(|| background_color.clone());

            res.push(color[0] as u8);
            res.push(color[1] as u8);
            res.push(color[2] as u8);
            res.push(255);
        }
    }

    res
}
