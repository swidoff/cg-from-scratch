use crate::raytracer::light::Scatter;
use crate::raytracer::vec3::{Color, Point, Vec3};

pub struct Intersection {
    pub p: Point,
    pub t: f64,
    pub normal: Vec3,
    pub color: Color,
    pub scatter: Scatter,
    pub reflective: f64,
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
    pub scatter: Scatter,
    pub reflective: f64,
}

impl Sphere {
    pub fn new(
        center: Point,
        radius: f64,
        color: Color,
        scatter: Scatter,
        reflective: f64,
    ) -> Sphere {
        Sphere {
            center,
            radius,
            color,
            scatter,
            reflective,
        }
    }

    pub fn intersect(
        &self,
        origin: &Point,
        direction: &Vec3,
        t_min: f64,
        t_max: f64,
    ) -> Option<Intersection> {
        let d = direction;
        let r = self.radius;
        let co = origin - &self.center;

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
                    let p = origin + direction * t;
                    let normal = (&p - &self.center) / self.radius;
                    let color = self.color.clone();
                    Intersection {
                        p,
                        t,
                        normal,
                        color,
                        scatter: self.scatter,
                        reflective: self.reflective,
                    }
                })
        }
    }
}

pub fn closest_intersection(
    objects: &Vec<Sphere>,
    origin: &Point,
    direction: &Vec3,
    t_min: f64,
    t_max: f64,
) -> Option<Intersection> {
    objects
        .iter()
        .filter_map(|obj| obj.intersect(origin, direction, t_min, t_max))
        .min_by(|int1, int2| int1.t.partial_cmp(&int2.t).unwrap())
}
