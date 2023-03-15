use crate::rasterizer::point::Point;
use crate::vec3::{Vec3, Vec4};
use itertools::Itertools;

const VIEWPORT_SIZE: f64 = 1.;
pub const PROJECTION_PLANE_Z: f64 = 1.;

pub fn interpolate(i0: i64, d0: f64, i1: i64, d1: f64) -> impl Iterator<Item = (i64, f64)> {
    let a = if i0 == i1 {
        0.
    } else {
        ((d1 - d0) as f64) / ((i1 - i0) as f64)
    };

    (i0..(i1 + 1)).scan(d0 as f64, move |d, i| {
        let res = Some((i, *d));
        *d += a;
        res
    })
}

pub fn edge_interpolate(
    y0: i64,
    v0: f64,
    y1: i64,
    v1: f64,
    y2: i64,
    v2: f64,
) -> [Vec<(i64, f64)>; 2] {
    let v01 = interpolate(y0, v0, y1, v1);
    let v02 = interpolate(y0, v0, y2, v2).collect_vec(); // Long size
    let v12 = interpolate(y1, v1, y2, v2);

    // Concatenate the two short sides.
    let v012 = v01.dropping(1).chain(v12).collect_vec();
    [v02, v012]
}

pub fn project_vertex(v: &Vec4, canvas_width: i64, canvas_height: i64) -> Point {
    let x = v[0] * PROJECTION_PLANE_Z / v[2] * canvas_width as f64 / VIEWPORT_SIZE;
    let y = v[1] * PROJECTION_PLANE_Z / v[2] * canvas_height as f64 / VIEWPORT_SIZE;
    Point::new(x as i64, y as i64, 1.)
}

pub fn unproject_vertex(x: f64, y: f64, inv_z: f64, canvas_width: i64, canvas_height: i64) -> Vec3 {
    let z = 1.0 / inv_z;
    let ux = (x * z / PROJECTION_PLANE_Z) * (VIEWPORT_SIZE / canvas_width as f64);
    let uy = (y * z / PROJECTION_PLANE_Z) * (VIEWPORT_SIZE / canvas_height as f64);
    Vec3::new(ux, uy, z)
}
