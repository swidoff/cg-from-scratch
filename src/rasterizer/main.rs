use crate::log;
use crate::vec3::Color;
use itertools::Itertools;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn rasterizer(canvas_height: usize, canvas_width: usize) -> Vec<u8> {
    let mut canvas = Canvas::new(canvas_height, canvas_width);
    let black = Color::new(0., 0., 0.);
    let green = Color::new(0., 255., 0.);

    // Chapter 6
    // canvas.draw_line(&Point::new(-200, -100), &Point::new(240, 120), &black);
    // canvas.draw_line(&Point::new(-50, -200), &Point::new(60, 240), &black);

    // Chapter 7
    // let p0 = Point::new(-200, -250, 1.0);
    // let p1 = Point::new(200, 50, 1.0);
    // let p2 = Point::new(20, 250, 1.0);
    // canvas.draw_filled_triangle(&p0, &p1, &p2, &green);
    // canvas.draw_wire_frame_triangle(&p0, &p1, &p2, &black);

    // Chapter 8
    let p0 = Point::new(-200, -250, 0.3);
    let p1 = Point::new(200, 50, 0.1);
    let p2 = Point::new(20, 250, 1.0);
    canvas.draw_shaded_triangle(&p0, &p1, &p2, &green);

    canvas.pixels
}

struct Canvas {
    height: i32,
    width: i32,
    pixels: Vec<u8>,
}

impl Canvas {
    fn new(height: usize, width: usize) -> Canvas {
        let capacity = width * height * 4;
        let mut pixels = Vec::with_capacity(capacity);
        for _i in 0..capacity {
            pixels.push(0);
        }
        Canvas {
            height: height as i32,
            width: width as i32,
            pixels,
        }
    }

    fn put_pixel(&mut self, x: i32, y: i32, color: &Color) {
        let x = self.width / 2 + x;
        let y = self.height / 2 - y - 1;
        // log!("x: {}, y: {}", x, y);

        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            let offset = (y * self.width * 4 + x * 4) as usize;
            self.pixels[offset] = color[0].clamp(0., 255.) as u8;
            self.pixels[offset + 1] = color[1].clamp(0., 255.) as u8;
            self.pixels[offset + 2] = color[2].clamp(0., 255.) as u8;
            self.pixels[offset + 3] = 255;
        }
    }

    fn draw_line(&mut self, p0: &Point, p1: &Point, color: &Color) {
        if (p1.x - p0.x).abs() > (p1.y - p0.y).abs() {
            // line is horizontal-ish
            let (p0, p1) = if p0.x > p1.x { (p1, p0) } else { (p0, p1) };
            for (x, y) in interpolate(p0.x, p0.y as f64, p1.x, p1.y as f64) {
                self.put_pixel(x, y as i32, color);
            }
        } else {
            // line is vertical-ish
            let (p0, p1) = if p0.y > p1.y { (p1, p0) } else { (p0, p1) };
            for (y, x) in interpolate(p0.y, p0.x as f64, p1.y, p1.x as f64) {
                self.put_pixel(x as i32, y, color);
            }
        }
    }

    fn draw_wire_frame_triangle(&mut self, p0: &Point, p1: &Point, p2: &Point, color: &Color) {
        self.draw_line(p0, p1, color);
        self.draw_line(p1, p2, color);
        self.draw_line(p0, p2, color);
    }

    fn draw_filled_triangle(&mut self, p0: &Point, p1: &Point, p2: &Point, color: &Color) {
        let (p0, p1, p2) = sort_points_by_y(p0, p1, p2);

        // Find the points along the sides of the triangle.
        let x01 = interpolate(p0.y, p0.x as f64, p1.y, p1.x as f64);
        let x02 = interpolate(p0.y, p0.x as f64, p2.y, p2.x as f64).collect_vec(); // Long size
        let x12 = interpolate(p1.y, p1.x as f64, p2.y, p2.x as f64);

        // Concatenate the two short sides.
        let x012 = x01.dropping(1).chain(x12).collect_vec();

        // Determine which side is the left and which is the right.
        let midpoint = x02.len() / 2;
        let (left_side, right_side) = if x02[midpoint].0 < x012[midpoint].0 {
            (x02, x012)
        } else {
            (x012, x02)
        };

        // Draw the horizontal line segments.
        for (i, &(y, x_left)) in left_side.iter().enumerate() {
            for x in (x_left as i32)..(right_side[i].1 as i32 + 1) {
                self.put_pixel(x as i32, y, color);
            }
        }
    }

    fn draw_shaded_triangle(&mut self, p0: &Point, p1: &Point, p2: &Point, color: &Color) {
        let (p0, p1, p2) = sort_points_by_y(p0, p1, p2);

        // Find the points along the sides of the triangle.
        let x01 = interpolate(p0.y, p0.x as f64, p1.y, p1.x as f64);
        let h01 = interpolate(p0.y, p0.h as f64, p1.y, p1.h as f64);
        let x02 = interpolate(p0.y, p0.x as f64, p2.y, p2.x as f64).collect_vec(); // Long size
        let h02 = interpolate(p0.y, p0.h, p2.y, p2.h).collect_vec(); // Long size
        let x12 = interpolate(p1.y, p1.x as f64, p2.y, p2.x as f64);
        let h12 = interpolate(p1.y, p1.h, p2.y, p2.h);

        // Concatenate the two short sides.
        let x012 = x01.dropping(1).chain(x12).collect_vec();
        let h012 = h01.dropping(1).chain(h12).collect_vec();

        // Determine which side is the left and which is the right.
        let midpoint = x02.len() / 2;
        let (left_side, right_side, h_left, h_right) = if x02[midpoint].0 < x012[midpoint].0 {
            (x02, x012, h02, h012)
        } else {
            (x012, x02, h012, h02)
        };

        // Draw the horizontal line segments.
        for (i, &(y, x_left)) in left_side.iter().enumerate() {
            let x_left = x_left as i32;
            let x_right = right_side[i].1 as i32;
            let h_left = h_left[i].1;
            let h_right = h_right[i].1;
            let h_segment = interpolate(x_left, h_left, x_right, h_right);

            for (x, h) in h_segment {
                self.put_pixel(x as i32, y, &(color * h));
            }
        }
    }
}

struct Point {
    x: i32,
    y: i32,
    h: f64,
}

impl Point {
    fn new(x: i32, y: i32, h: f64) -> Point {
        Point { x, y, h }
    }
}

fn interpolate(i0: i32, d0: f64, i1: i32, d1: f64) -> impl Iterator<Item = (i32, f64)> {
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

fn sort_points_by_y<'a>(
    p0: &'a Point,
    p1: &'a Point,
    p2: &'a Point,
) -> (&'a Point, &'a Point, &'a Point) {
    let mut points = [p0, p1, p2];
    points.sort_by_key(|&p| p.y);
    let [p0, p1, p2] = points;
    (p0, p1, p2)
}
