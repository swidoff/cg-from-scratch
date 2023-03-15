use crate::vec3::Vec3;

pub struct Plane {
    pub normal: Vec3,
    pub distance: f64,
}

impl Plane {
    pub fn new(normal: Vec3, distance: f64) -> Plane {
        Plane { normal, distance }
    }

    pub fn segment_intersection(&self, a: &Vec3, b: &Vec3) -> Vec3 {
        let b_minus_a = b - a;
        let t = -self.distance - self.normal.dot(a) / self.normal.dot(&b_minus_a);
        a + b_minus_a * t
    }
}
