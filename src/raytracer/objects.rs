use crate::raytracer::light::Reflection;
use crate::raytracer::ray::Ray;
use crate::raytracer::vec3::{Color, Point, Vec3};

pub struct Intersection {
    pub p: Point,
    pub t: f64,
    pub normal: Vec3,
    pub color: Color,
    pub reflection: Reflection,
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
    pub reflection: Reflection,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, color: Color, reflection: Reflection) -> Sphere {
        Sphere {
            center,
            radius,
            color,
            reflection,
        }
    }

    pub fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
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
                .map(|&t| {
                    let p = ray.at(t);
                    let normal = (&p - &self.center) / self.radius;
                    let color = self.color.clone();
                    Intersection {
                        p,
                        t,
                        normal,
                        color,
                        reflection: self.reflection,
                    }
                })
        }
    }
}
