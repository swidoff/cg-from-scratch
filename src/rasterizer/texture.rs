use crate::vec3::Color;
use anyhow::Result;
use image::io::Reader as ImageReader;
use image::{ImageFormat, RgbImage};
use std::io::Cursor;

#[derive(Debug, PartialEq, Clone)]
pub struct Texture {
    img: RgbImage,
}

impl Texture {
    pub fn from_bytes(bytes: &[u8]) -> Result<Texture> {
        let img = ImageReader::with_format(Cursor::new(bytes), ImageFormat::Jpeg)
            .decode()?
            .into_rgb8();
        Ok(Texture { img })
    }

    pub fn texel_color(&self, u: f64, v: f64) -> Color {
        let x = u * (self.img.width() - 1) as f64;
        let y = v * (self.img.height() - 1) as f64;
        let fx = x.fract();
        let fy = y.fract();
        let tx = x.floor() as u32;
        let ty = y.floor() as u32;
        let tx1 = (tx + 1).clamp(0, self.img.width() - 1);
        let ty1 = (ty + 1).clamp(0, self.img.height() - 1);

        let tl = self.pixel_color(tx, ty);
        let tr = self.pixel_color(tx1, ty);
        let bl = self.pixel_color(tx, ty1);
        let br = self.pixel_color(tx1, ty1);

        let ct = tr * fx + tl * (1. - fx);
        let cb = br * fx + bl * (1. - fx);
        return cb * fy + ct * (1. - fy);
    }

    fn pixel_color(&self, x: u32, y: u32) -> Color {
        let [r, g, b] = self.img.get_pixel(x, y).0;
        Color::new(r as f64, g as f64, b as f64)
    }
}
