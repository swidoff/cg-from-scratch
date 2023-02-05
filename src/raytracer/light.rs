use crate::raytracer::ray::Ray;
use crate::raytracer::vec3::{Point, Vec3};

pub enum Light {
    // Simulates the light being scattered by other objects without actually computing it.
    Ambient { intensity: f64 },

    // Light emitting from a single point.
    Point { intensity: f64, position: Point },

    // Simulates the Sun, where the source of the light is so far away affectively all light is coming from the same
    // direction.
    Directional { intensity: f64, direction: Vec3 },
}

#[derive(Clone, Copy, Debug)]
pub enum Reflection {
    // Diffuse: A matte reflection. Surface is irregular and so light is scattered equally in every direction.
    Diffuse,

    // Specular: A shiny reflection. Surface is even so light is scattered in a single (or close to a single)
    // direction, the vector R. The greater the shininess the lower the reflected light as you move away from R.
    Specular { shininess: f64 },
}

impl Reflection {
    /// Calculates the intensity of the light being reflected from an object that intersects with the ray emitting
    /// from the camera.
    ///
    /// # Arguments
    ///
    /// * `lights`: the light sources
    /// * `ray`: the ray from camera
    /// * `point`: the point at which the ray hits the scene object.
    /// * `normal`: the surface normal between the object and the point
    ///
    /// returns: f64 the intensity of the reflection, which is determined by how the material scatters the light from
    /// the light sources and the angle at which the light hits the surface at the point the ray hits.
    ///
    pub fn intensity(&self, lights: &Vec<Light>, ray: &Ray, point: &Point, normal: &Vec3) -> f64 {
        let mut res = 0.;
        for light in lights {
            res += match light {
                Light::Ambient { intensity } => *intensity,
                Light::Point {
                    intensity,
                    position,
                } => {
                    let l = position - point;
                    *intensity * self.reflect(ray, &l, normal)
                }
                Light::Directional {
                    intensity,
                    direction,
                } => *intensity * self.reflect(ray, direction, normal),
            };
        }
        res
    }

    /// Returns the fraction of light that is reflected as a function of the angle between the surface normal and the
    /// direction of the light.
    ///
    fn reflect(&self, ray: &Ray, l: &Vec3, normal: &Vec3) -> f64 {
        let n_dot_l = normal.dot(&l);
        let mut res = if n_dot_l > 0. {
            // We're computing intensity / area, which is the equivalent to the cosine of the angle between the light
            // (l) and the surface normal. That is equal to <l, n> / |n||l|
            n_dot_l / (normal.len() * l.len())
        } else {
            0.
        };

        if let &Reflection::Specular { shininess } = self {
            // v is the vector from the object to the camera, so that's just -ray.direction;
            let v = -ray.direction;

            // r is the light reflected from the surface normal.
            let r = normal * normal.dot(l) * 2. - l;
            let r_dot_v = r.dot(&v);
            if r_dot_v > 0. {
                // The cosine of the angle between r and v, which is the fraction of light reflected back at v.
                // The less shiny the object, the quicker that intensity decreases as the angle increases.
                res += (r_dot_v / (r.len() * v.len())).powf(shininess);
            }
        }

        res
    }
}
