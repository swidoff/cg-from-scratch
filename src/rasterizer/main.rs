use crate::rasterizer::camera::Camera;
use crate::rasterizer::canvas::Canvas;
use crate::rasterizer::instance::Instance;
use crate::rasterizer::light::{Light, Scatter};
use crate::rasterizer::model::Model;
use crate::rasterizer::plane::Plane;
use crate::rasterizer::scene::Scene;
use crate::rasterizer::shading::ShadingModel::Flat;
use crate::rasterizer::triangle::Triangle;
use crate::rasterizer::util::PROJECTION_PLANE_Z;
use crate::utils;
use crate::vec3::{Color, Mat3, Vec3};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn rasterizer(canvas_height: usize, canvas_width: usize) -> Vec<u8> {
    utils::set_panic_hook();
    let mut canvas = Canvas::new(canvas_height, canvas_width, Flat);

    // let black = Color::new(0., 0., 0.);
    let red = Color::new(255., 0., 0.);
    let green = Color::new(0., 255., 0.);
    let blue = Color::new(0., 0., 255.);
    let yellow = Color::new(255., 255., 0.);
    let purple = Color::new(255., 0., 255.);
    let cyan = Color::new(0., 255., 255.);

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
            Triangle::new(
                0,
                1,
                2,
                red,
                Vec3::new(0., 0., 1.),
                Vec3::new(0., 0., 1.),
                Vec3::new(0., 0., 1.),
            ),
            Triangle::new(
                0,
                2,
                3,
                red,
                Vec3::new(0., 0., 1.),
                Vec3::new(0., 0., 1.),
                Vec3::new(0., 0., 1.),
            ),
            Triangle::new(
                1,
                5,
                6,
                yellow,
                Vec3::new(1., 0., 0.),
                Vec3::new(1., 0., 0.),
                Vec3::new(1., 0., 0.),
            ),
            Triangle::new(
                1,
                6,
                2,
                yellow,
                Vec3::new(1., 0., 0.),
                Vec3::new(1., 0., 0.),
                Vec3::new(1., 0., 0.),
            ),
            Triangle::new(
                2,
                6,
                7,
                cyan,
                Vec3::new(0., 0., -1.),
                Vec3::new(0., 0., -1.),
                Vec3::new(0., 0., -1.),
            ),
            Triangle::new(
                2,
                7,
                3,
                cyan,
                Vec3::new(0., 0., -1.),
                Vec3::new(0., 0., -1.),
                Vec3::new(0., 0., -1.),
            ),
            Triangle::new(
                4,
                0,
                3,
                green,
                Vec3::new(-1., 0., 0.),
                Vec3::new(-1., 0., 0.),
                Vec3::new(-1., 0., 0.),
            ),
            Triangle::new(
                4,
                1,
                0,
                purple,
                Vec3::new(-1., 0., 0.),
                Vec3::new(-1., 0., 0.),
                Vec3::new(-1., 0., 0.),
            ),
            Triangle::new(
                4,
                3,
                7,
                green,
                Vec3::new(0., 1., 0.),
                Vec3::new(0., 1., 0.),
                Vec3::new(0., 1., 0.),
            ),
            Triangle::new(
                4,
                5,
                1,
                purple,
                Vec3::new(0., 1., 0.),
                Vec3::new(0., 1., 0.),
                Vec3::new(0., 1., 0.),
            ),
            Triangle::new(
                5,
                4,
                7,
                blue,
                Vec3::new(0., -1., 0.),
                Vec3::new(0., -1., 0.),
                Vec3::new(0., -1., 0.),
            ),
            Triangle::new(
                5,
                7,
                6,
                blue,
                Vec3::new(0., -1., 0.),
                Vec3::new(0., -1., 0.),
                Vec3::new(0., -1., 0.),
            ),
        ],
        Scatter::Specular { shininess: 50.0 },
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
        lights: vec![
            Light::Ambient { intensity: 0.2 },
            Light::Directional {
                intensity: 0.2,
                direction: Vec3::new(-1., 0., 1.),
            },
            Light::Point {
                intensity: 0.6,
                position: Vec3::new(-3., 2., -10.),
            },
        ],
    };

    canvas.render_scene(&scene);
    canvas.pixels
}

#[cfg(test)]
mod tests {
    use crate::rasterizer::main::rasterizer;

    #[test]
    fn test_rasterizer() {
        let _res = rasterizer(600, 600);
    }
}
