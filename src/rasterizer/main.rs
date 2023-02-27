use crate::log;
use crate::vec3::Color;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn rasterizer(canvas_height: usize, canvas_width: usize) -> Vec<u8> {
    let mut canvas = Canvas::new(canvas_height, canvas_width);
    let black = Color::new(0., 0., 0.);
    canvas.draw_line(&Point::new(-200, -100), &Point::new(240, 120), &black);
    canvas.draw_line(&Point::new(-50, -200), &Point::new(60, 240), &black);
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
            for (x, y) in interpolate(p0.x, p0.y as f32, p1.x, p1.y as f32) {
                self.put_pixel(x, y as i32, color);
            }
        } else {
            // line is vertical-ish
            let (p0, p1) = if p0.y > p1.y { (p1, p0) } else { (p0, p1) };
            for (y, x) in interpolate(p0.y, p0.x as f32, p1.y, p1.x as f32) {
                self.put_pixel(x as i32, y, color);
            }
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

fn interpolate(i0: i32, d0: f32, i1: i32, d1: f32) -> impl Iterator<Item = (i32, f32)> {
    let a = if i0 == i1 {
        0.
    } else {
        (d1 - d0) / ((i1 - i0) as f32)
    };

    (i0..(i1 + 1)).scan(d0, move |d, i| {
        let res = Some((i, *d));
        *d += a;
        res
    })
}
