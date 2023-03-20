use crate::rasterizer::camera::Camera;
use crate::rasterizer::canvas::Canvas;
use crate::rasterizer::instance::Instance;
use crate::rasterizer::light::{Light, Scatter};
use crate::rasterizer::model::Model;
use crate::rasterizer::plane::Plane;
use crate::rasterizer::scene::Scene;
use crate::rasterizer::shading::ShadingModel::{Flat, Gouraud, Phong};
use crate::rasterizer::surface::Surface;
use crate::rasterizer::texture::Texture;
use crate::rasterizer::triangle::Triangle;
use crate::rasterizer::util::PROJECTION_PLANE_Z;
use crate::utils;
use crate::vec3::{Color, Mat3, Vec3};
use std::io::Bytes;
use wasm_bindgen::prelude::*;

const CRATE_BYTES: &[u8; 318447] = include_bytes!("crate-texture.jpg");

#[wasm_bindgen]
pub fn rasterizer(canvas_height: usize, canvas_width: usize) -> Vec<u8> {
    utils::set_panic_hook();
    let mut canvas = Canvas::new(canvas_height, canvas_width, Phong);
    let crate_texture = Texture::from_bytes(CRATE_BYTES).unwrap();

    // let black = Color::new(0., 0., 0.);
    // let red = Color::new(255., 0., 0.);
    // let green = Color::new(0., 255., 0.);
    // let blue = Color::new(0., 0., 255.);
    // let yellow = Color::new(255., 255., 0.);
    // let purple = Color::new(255., 0., 255.);
    // let cyan = Color::new(0., 255., 255.);

    let scatter = Scatter::Specular { shininess: 50.0 };
    let wood_upper = Surface::Texture {
        index: 0,
        uvs: [(0., 0.), (1., 0.), (1., 1.)],
    };
    let wood_lower = Surface::Texture {
        index: 0,
        uvs: [(0., 0.), (1., 1.), (0., 1.)],
    };
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
                // Surface::Color(red),
                wood_upper.clone(),
                Vec3::new(0., 0., 1.),
                Vec3::new(0., 0., 1.),
                Vec3::new(0., 0., 1.),
            ),
            Triangle::new(
                0,
                2,
                3,
                // Surface::Color(red),
                wood_lower.clone(),
                Vec3::new(0., 0., 1.),
                Vec3::new(0., 0., 1.),
                Vec3::new(0., 0., 1.),
            ),
            Triangle::new(
                1,
                5,
                6,
                // Surface::Color(yellow),
                wood_upper.clone(),
                Vec3::new(-1., 0., 0.),
                Vec3::new(-1., 0., 0.),
                Vec3::new(-1., 0., 0.),
            ),
            Triangle::new(
                1,
                6,
                2,
                // Surface::Color(yellow),
                wood_lower.clone(),
                Vec3::new(-1., 0., 0.),
                Vec3::new(-1., 0., 0.),
                Vec3::new(-1., 0., 0.),
            ),
            Triangle::new(
                2,
                6,
                7,
                // Surface::Color(cyan),
                // wood_upper.clone(),
                wood_lower.clone(),
                Vec3::new(0., -1., 0.),
                Vec3::new(0., -1., 0.),
                Vec3::new(0., -1., 0.),
            ),
            Triangle::new(
                2,
                7,
                3,
                // Surface::Color(cyan),
                wood_lower.clone(),
                Vec3::new(0., -1., 0.),
                Vec3::new(0., -1., 0.),
                Vec3::new(0., -1., 0.),
            ),
            Triangle::new(
                4,
                0,
                3,
                // Surface::Color(green),
                wood_upper.clone(),
                Vec3::new(1., 0., 0.),
                Vec3::new(1., 0., 0.),
                Vec3::new(1., 0., 0.),
            ),
            Triangle::new(
                4,
                1,
                0,
                // Surface::Color(purple),
                wood_upper.clone(),
                Vec3::new(0., 1., 0.),
                Vec3::new(0., 1., 0.),
                Vec3::new(0., 1., 0.),
            ),
            Triangle::new(
                4,
                3,
                7,
                // Surface::Color(green),
                wood_lower.clone(),
                Vec3::new(1., 0., 0.),
                Vec3::new(1., 0., 0.),
                Vec3::new(1., 01., 0.),
            ),
            Triangle::new(
                4,
                5,
                1,
                // Surface::Color(purple),
                wood_lower.clone(),
                Vec3::new(0., 1., 0.),
                Vec3::new(0., 1., 0.),
                Vec3::new(0., 1., 0.),
            ),
            Triangle::new(
                5,
                4,
                7,
                // Surface::Color(blue),
                wood_upper.clone(),
                Vec3::new(0., 0., -1.),
                Vec3::new(0., 0., -1.),
                Vec3::new(0., 0., -1.),
            ),
            Triangle::new(
                5,
                7,
                6,
                // Surface::Color(blue),
                wood_lower.clone(),
                Vec3::new(0., 0., -1.),
                Vec3::new(0., 0., -1.),
                Vec3::new(0., 0., -1.),
            ),
        ],
        scatter,
    );
    // let sphere = Model::make_sphere(15, Surface::Color(green), scatter);

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
            // Instance::new(&sphere, 1.5, Mat3::identity(), Vec3::new(1.75, -0.5, 7.)),
            // Instance::new(
            //     &cube_model,
            //     1.0,
            //     Mat3::new_oy_rotation_matrix(195.),
            //     Vec3::new(0., 0., -10.),
            // ),
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
        textures: vec![crate_texture],
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
