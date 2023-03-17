use crate::rasterizer::camera::Camera;
use crate::vec3::Vec3;

pub enum Light {
    // Simulates the light being scattered by other objects without actually computing it.
    Ambient { intensity: f64 },

    // Light emitting from a single point.
    Point { intensity: f64, position: Vec3 },

    // Simulates the Sun, where the source of the light is so far away affectively all light is coming from the same
    // direction.
    Directional { intensity: f64, direction: Vec3 },
}

#[derive(Clone, Copy, Debug)]
pub enum Scatter {
    // Diffuse: A matte reflection. Surface is irregular and so light is scattered equally in every direction.
    Diffuse,

    // Specular: A shiny reflection. Surface is even so light is scattered in a single (or close to a single)
    // direction, the vector R. The greater the shininess the lower the reflected light as you move away from R.
    Specular { shininess: f64 },
}

impl Scatter {
    pub fn intensity(
        &self,
        vertex: &Vec3,
        normal: &Vec3,
        camera: &Camera,
        lights: &Vec<Light>,
    ) -> f64 {
        let mut res = 0.0;
        for light in lights {
            if let &Light::Ambient { intensity } = light {
                res += intensity;
            } else {
                let (v1, intensity) = match light {
                    Light::Point {
                        intensity,
                        position,
                    } => {
                        let transformed_light = &camera.transformation * position.to_vec4(0.0);
                        (transformed_light.to_vec3() - vertex, *intensity)
                    }
                    Light::Directional {
                        intensity,
                        direction,
                    } => {
                        let rotated_light =
                            &camera.orientation.transpose().to_homogenous_rotation()
                                * direction.to_vec4(0.0);
                        (rotated_light.to_vec3(), *intensity)
                    }
                    _ => panic!("Can't happen"),
                };

                // Diffuse component.
                let cos_alpha = v1.dot(normal) / (v1.len() * normal.len());
                if cos_alpha > 0. {
                    res += cos_alpha * intensity;
                }

                // Specular Component
                if let &Scatter::Specular { shininess } = self {
                    let reflected = v1.reflect(normal);
                    let view = camera.position - vertex;
                    let cos_beta = reflected.dot(&view) / (reflected.len() * view.len());
                    if cos_beta > 0. {
                        res += cos_beta.powf(shininess) * intensity;
                    }
                }
            }
        }
        res
    }
}
